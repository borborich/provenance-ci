use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};
use chrono::{DateTime, SecondsFormat, Utc};
use sha2::{Digest, Sha256};

use crate::{
    c2pa_adapter::{self, AdapterResult, TrustMaterial},
    config::Config,
    fetch,
    model::{
        CheckResult, CheckpointResult, CredentialPresence, CryptographicValidation, Evidence,
        ObservedBreak, PolicyVerdict, Relationship, RunResult, ToolInfo, TrustState,
    },
    policy,
};

#[derive(Clone, Debug)]
pub struct RunOptions {
    pub config_path: PathBuf,
    pub checked_at: Option<DateTime<Utc>>,
}

pub fn run(config: &Config, options: &RunOptions) -> Result<RunResult> {
    let generated_at = options
        .checked_at
        .unwrap_or_else(Utc::now)
        .to_rfc3339_opts(SecondsFormat::Secs, true);
    let config_dir = options
        .config_path
        .parent()
        .unwrap_or_else(|| Path::new("."));
    let trust = load_trust(config, config_dir)?;
    let mut checks = Vec::with_capacity(config.checks.len());
    let mut infrastructure_error = false;

    for configured_check in &config.checks {
        let mut checkpoints = Vec::with_capacity(configured_check.checkpoints.len());
        for configured_checkpoint in &configured_check.checkpoints {
            let mut checkpoint = match fetch::acquire(configured_checkpoint, config_dir) {
                Ok(asset) => {
                    if asset.supported {
                        let adapter = c2pa_adapter::inspect(&asset.bytes, &trust)?;
                        checkpoint_from_adapter(
                            configured_checkpoint.name.clone(),
                            generated_at.clone(),
                            asset,
                            adapter,
                        )
                    } else {
                        unsupported_checkpoint(
                            configured_checkpoint.name.clone(),
                            generated_at.clone(),
                            asset,
                        )
                    }
                }
                Err(error) => {
                    infrastructure_error = true;
                    fetch_error_checkpoint(
                        configured_checkpoint.name.clone(),
                        generated_at.clone(),
                        fetch::source_skeleton(configured_checkpoint),
                        error.to_string(),
                    )
                }
            };
            checkpoint.relationship_to_previous_checkpoint =
                policy::classify_relationship(checkpoints.last(), &checkpoint);
            checkpoints.push(checkpoint);
        }

        let first_observed_break = checkpoints.windows(2).find_map(|pair| {
            let current = &pair[1];
            if matches!(
                current.relationship_to_previous_checkpoint,
                Relationship::CredentialsLost
                    | Relationship::ChangedWithoutVerifiableLineage
                    | Relationship::Inconclusive
            ) {
                Some(ObservedBreak {
                    from: pair[0].checkpoint.clone(),
                    to: current.checkpoint.clone(),
                    relationship: current.relationship_to_previous_checkpoint.clone(),
                    statement: format!(
                        "First observed break occurred between checkpoint '{}' and checkpoint '{}'.",
                        pair[0].checkpoint, current.checkpoint
                    ),
                })
            } else {
                None
            }
        });
        let (check_policy, policy_reasons) =
            policy::evaluate(&checkpoints, &configured_check.policy);
        checks.push(CheckResult {
            id: configured_check.id.clone(),
            checkpoints,
            first_observed_break,
            policy: check_policy,
            policy_reasons,
        });
    }

    let overall_policy = checks
        .iter()
        .map(|check| &check.policy)
        .max_by_key(|verdict| severity(verdict))
        .cloned()
        .unwrap_or(PolicyVerdict::Inconclusive);
    let exit_code = if infrastructure_error {
        3
    } else if overall_policy == PolicyVerdict::Fail {
        2
    } else if overall_policy == PolicyVerdict::Inconclusive {
        3
    } else {
        0
    };

    Ok(RunResult {
        schema_version: 1,
        generated_at,
        tool: ToolInfo {
            name: "provenance-ci".into(),
            version: env!("CARGO_PKG_VERSION").into(),
            c2pa_sdk: c2pa_adapter::C2PA_SDK_VERSION.into(),
            supported_specification: c2pa_adapter::C2PA_SPECIFICATION.into(),
        },
        trust: trust.info,
        checks,
        overall_policy,
        exit_code,
    })
}

fn load_trust(config: &Config, config_dir: &Path) -> Result<TrustMaterial> {
    let Some(path) = &config.trust.anchors_path else {
        return Ok(TrustMaterial::disabled(config.trust.id.clone()));
    };
    let resolved = config_dir.join(path);
    let pem = fs::read_to_string(&resolved)
        .with_context(|| format!("failed to read trust anchors {}", resolved.display()))?;
    Ok(TrustMaterial::from_pem(pem, config.trust.id.clone()))
}

fn checkpoint_from_adapter(
    checkpoint: String,
    checked_at: String,
    asset: fetch::FetchedAsset,
    adapter: AdapterResult,
) -> CheckpointResult {
    let sha256 = hex::encode(Sha256::digest(&asset.bytes));
    let size_bytes = asset.bytes.len() as u64;
    CheckpointResult {
        checkpoint,
        source: asset.source,
        sha256: Some(sha256),
        media_type: Some(asset.media_type),
        size_bytes: Some(size_bytes),
        width: asset.width,
        height: asset.height,
        checked_at,
        credential_presence: adapter.credential_presence,
        cryptographic_validation: adapter.cryptographic_validation,
        trust: adapter.trust,
        relationship_to_previous_checkpoint: Relationship::NotApplicable,
        active_manifest: adapter.active_manifest,
        parent_manifests: adapter.parent_manifests,
        raw_validation_codes: adapter.raw_validation_codes,
        evidence: adapter.evidence,
        limitations: adapter.limitations,
        error: adapter.error,
    }
}

fn unsupported_checkpoint(
    checkpoint: String,
    checked_at: String,
    asset: fetch::FetchedAsset,
) -> CheckpointResult {
    let sha256 = hex::encode(Sha256::digest(&asset.bytes));
    let size_bytes = asset.bytes.len() as u64;
    let media_type = asset.media_type;
    CheckpointResult {
        checkpoint,
        source: asset.source,
        sha256: Some(sha256),
        media_type: Some(media_type.clone()),
        size_bytes: Some(size_bytes),
        width: asset.width,
        height: asset.height,
        checked_at,
        credential_presence: CredentialPresence::Unknown,
        cryptographic_validation: CryptographicValidation::Indeterminate,
        trust: TrustState::Unknown,
        relationship_to_previous_checkpoint: Relationship::NotApplicable,
        active_manifest: None,
        parent_manifests: Vec::new(),
        raw_validation_codes: Vec::new(),
        evidence: vec![Evidence {
            fact: "unsupported_media_type".into(),
            detail: format!("Detected {media_type}; the MVP validates JPEG only."),
        }],
        limitations: vec!["No C2PA conclusion is made for unsupported media.".into()],
        error: None,
    }
}

fn fetch_error_checkpoint(
    checkpoint: String,
    checked_at: String,
    source: crate::model::SourceEvidence,
    message: String,
) -> CheckpointResult {
    CheckpointResult {
        checkpoint,
        source,
        sha256: None,
        media_type: None,
        size_bytes: None,
        width: None,
        height: None,
        checked_at,
        credential_presence: CredentialPresence::Unknown,
        cryptographic_validation: CryptographicValidation::Indeterminate,
        trust: TrustState::Unknown,
        relationship_to_previous_checkpoint: Relationship::NotApplicable,
        active_manifest: None,
        parent_manifests: Vec::new(),
        raw_validation_codes: Vec::new(),
        evidence: vec![Evidence {
            fact: "checkpoint_acquisition_failed".into(),
            detail: message.clone(),
        }],
        limitations: vec!["No asset bytes were available for validation.".into()],
        error: Some(message),
    }
}

fn severity(verdict: &PolicyVerdict) -> u8 {
    match verdict {
        PolicyVerdict::Pass => 0,
        PolicyVerdict::Warn => 1,
        PolicyVerdict::Inconclusive => 2,
        PolicyVerdict::Fail => 3,
    }
}

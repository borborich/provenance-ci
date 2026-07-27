use std::io::Cursor;

use anyhow::{Context as _, Result};
use c2pa::{settings::Settings, Context, Error, Reader, ValidationState};
use serde_json::Value;
use sha2::{Digest, Sha256};

use crate::model::{
    CredentialPresence, CryptographicValidation, Evidence, TrustInfo, TrustState, ValidationCode,
};

pub const C2PA_SDK_VERSION: &str = "0.90.3";
pub const C2PA_SPECIFICATION: &str = "2.4";

#[derive(Clone, Debug)]
pub struct TrustMaterial {
    pub pem: Option<String>,
    pub info: TrustInfo,
}

#[derive(Clone, Debug)]
pub struct AdapterResult {
    pub credential_presence: CredentialPresence,
    pub cryptographic_validation: CryptographicValidation,
    pub trust: TrustState,
    pub active_manifest: Option<String>,
    pub parent_manifests: Vec<String>,
    pub raw_validation_codes: Vec<ValidationCode>,
    pub evidence: Vec<Evidence>,
    pub limitations: Vec<String>,
    pub error: Option<String>,
}

impl TrustMaterial {
    pub fn disabled(id: Option<String>) -> Self {
        Self {
            pem: None,
            info: TrustInfo {
                checked: false,
                id,
                sha256: None,
            },
        }
    }

    pub fn from_pem(pem: String, id: Option<String>) -> Self {
        let digest = hex::encode(Sha256::digest(pem.as_bytes()));
        Self {
            pem: Some(pem),
            info: TrustInfo {
                checked: true,
                id,
                sha256: Some(digest),
            },
        }
    }
}

pub fn inspect(bytes: &[u8], trust: &TrustMaterial) -> Result<AdapterResult> {
    let verify_trust = trust.pem.is_some();
    let mut settings = Settings::new()
        .with_value("verify.remote_manifest_fetch", false)
        .context("failed to disable SDK remote manifest fetch")?
        .with_value("verify.verify_trust", verify_trust)
        .context("failed to configure SDK trust verification")?;
    if let Some(pem) = &trust.pem {
        settings = settings
            .with_value("trust.trust_anchors", pem.clone())
            .context("failed to load trust anchors")?;
    }
    let context = Context::new()
        .with_settings(settings)
        .context("failed to initialize C2PA SDK context")?;

    match Reader::from_context(context).with_stream("image/jpeg", Cursor::new(bytes)) {
        Ok(reader) => Ok(from_reader(&reader, verify_trust)),
        Err(Error::JumbfNotFound) => Ok(absent_result(verify_trust)),
        Err(Error::RemoteManifestUrl(_)) | Err(Error::RemoteManifestFetch(_)) => {
            Ok(remote_manifest_result(verify_trust))
        }
        Err(error) => Ok(unreadable_result(error.to_string(), verify_trust)),
    }
}

fn from_reader(reader: &Reader, verify_trust: bool) -> AdapterResult {
    let state = reader.validation_state();
    let (cryptographic_validation, trust_state) = match state {
        ValidationState::Trusted => (
            CryptographicValidation::Valid,
            if verify_trust {
                TrustState::Trusted
            } else {
                TrustState::NotChecked
            },
        ),
        ValidationState::Valid => (
            CryptographicValidation::Valid,
            if verify_trust {
                TrustState::Untrusted
            } else {
                TrustState::NotChecked
            },
        ),
        ValidationState::Invalid => (
            CryptographicValidation::Invalid,
            if verify_trust {
                TrustState::Unknown
            } else {
                TrustState::NotChecked
            },
        ),
    };

    let active_manifest = reader.active_label().map(str::to_owned);
    let raw_validation_codes = reader
        .validation_results()
        .and_then(|results| serde_json::to_value(results).ok())
        .map_or_else(Vec::new, |value| collect_validation_codes(&value));
    let parent_manifests: Vec<String> = reader
        .active_manifest()
        .map(|manifest| {
            manifest
                .ingredients()
                .iter()
                .filter(|ingredient| ingredient.is_parent())
                .filter_map(|ingredient| ingredient.active_manifest())
                .filter(|label| {
                    raw_validation_codes.iter().any(|status| {
                        matches!(
                            status.code.as_str(),
                            "ingredient.manifest.validated" | "ingredient.claimSignature.validated"
                        ) && status
                            .url
                            .as_deref()
                            .is_some_and(|url| url.contains(*label))
                    })
                })
                .map(str::to_owned)
                .collect()
        })
        .unwrap_or_default();

    let mut evidence = vec![Evidence {
        fact: "credential_store_parsed".into(),
        detail: "The official C2PA Rust SDK parsed an embedded manifest store.".into(),
    }];
    evidence.push(Evidence {
        fact: "sdk_validation_state".into(),
        detail: format!("{state:?}"),
    });
    if let Some(label) = &active_manifest {
        evidence.push(Evidence {
            fact: "active_manifest".into(),
            detail: label.clone(),
        });
    }
    for label in &parent_manifests {
        evidence.push(Evidence {
            fact: "validated_parent_manifest".into(),
            detail: label.clone(),
        });
    }

    AdapterResult {
        credential_presence: CredentialPresence::Present,
        cryptographic_validation,
        trust: trust_state,
        active_manifest,
        parent_manifests,
        raw_validation_codes,
        evidence,
        limitations: vec![
            "Validation evaluates provenance binding and manifest structure; it does not establish content truth, authorship, copyright, or absence of manipulation.".into(),
            "External assertions, remote ingredients, icons, and soft-binding recovery are not fetched by this MVP.".into(),
        ],
        error: None,
    }
}

fn absent_result(verify_trust: bool) -> AdapterResult {
    AdapterResult {
        credential_presence: CredentialPresence::Absent,
        cryptographic_validation: CryptographicValidation::NotApplicable,
        trust: if verify_trust {
            TrustState::Unknown
        } else {
            TrustState::NotChecked
        },
        active_manifest: None,
        parent_manifests: Vec::new(),
        raw_validation_codes: Vec::new(),
        evidence: vec![Evidence {
            fact: "credential_not_found".into(),
            detail: "The official C2PA SDK returned JumbfNotFound for these JPEG bytes.".into(),
        }],
        limitations: vec![
            "Absence of Content Credentials does not prove that the asset is false, manipulated, or AI-generated.".into(),
        ],
        error: None,
    }
}

fn remote_manifest_result(verify_trust: bool) -> AdapterResult {
    AdapterResult {
        credential_presence: CredentialPresence::Present,
        cryptographic_validation: CryptographicValidation::Indeterminate,
        trust: if verify_trust {
            TrustState::Unknown
        } else {
            TrustState::NotChecked
        },
        active_manifest: None,
        parent_manifests: Vec::new(),
        raw_validation_codes: vec![ValidationCode {
            code: "manifest.inaccessible".into(),
            kind: "informational".into(),
            url: None,
            explanation: Some(
                "A remote manifest reference was found, but remote fetching is disabled.".into(),
            ),
        }],
        evidence: vec![Evidence {
            fact: "remote_manifest_not_evaluated".into(),
            detail: "The SDK requested a remote manifest; the adapter denied network retrieval.".into(),
        }],
        limitations: vec![
            "Remote manifests are not evaluated in the MVP because unrestricted SDK fetching would expand SSRF and privacy risk.".into(),
        ],
        error: None,
    }
}

fn unreadable_result(message: String, verify_trust: bool) -> AdapterResult {
    AdapterResult {
        credential_presence: CredentialPresence::Unreadable,
        cryptographic_validation: CryptographicValidation::Indeterminate,
        trust: if verify_trust {
            TrustState::Unknown
        } else {
            TrustState::NotChecked
        },
        active_manifest: None,
        parent_manifests: Vec::new(),
        raw_validation_codes: Vec::new(),
        evidence: vec![Evidence {
            fact: "sdk_read_error".into(),
            detail: message.clone(),
        }],
        limitations: vec![
            "The SDK could not produce a complete validation result; no validity or lineage conclusion is made.".into(),
        ],
        error: Some(message),
    }
}

fn collect_validation_codes(value: &Value) -> Vec<ValidationCode> {
    fn visit(value: &Value, kind: Option<&str>, out: &mut Vec<ValidationCode>) {
        match value {
            Value::Object(map) => {
                if let Some(code) = map.get("code").and_then(Value::as_str) {
                    out.push(ValidationCode {
                        code: code.to_string(),
                        kind: kind.unwrap_or("unknown").to_string(),
                        url: map
                            .get("url")
                            .and_then(Value::as_str)
                            .map(sanitize_validation_url),
                        explanation: map
                            .get("explanation")
                            .and_then(Value::as_str)
                            .map(str::to_owned),
                    });
                }
                for (key, child) in map {
                    let next_kind = match key.as_str() {
                        "success" => Some("success"),
                        "informational" => Some("informational"),
                        "failure" => Some("failure"),
                        _ => kind,
                    };
                    visit(child, next_kind, out);
                }
            }
            Value::Array(values) => {
                for child in values {
                    visit(child, kind, out);
                }
            }
            _ => {}
        }
    }

    let mut out = Vec::new();
    visit(value, None, &mut out);
    out.sort_by(|a, b| {
        (&a.kind, &a.code, &a.url, &a.explanation).cmp(&(&b.kind, &b.code, &b.url, &b.explanation))
    });
    out.dedup_by(|a, b| {
        a.kind == b.kind && a.code == b.code && a.url == b.url && a.explanation == b.explanation
    });
    out
}

fn sanitize_validation_url(value: &str) -> String {
    if value.starts_with("self#") || value.starts_with("urn:") {
        value.to_owned()
    } else if url::Url::parse(value).is_ok() {
        crate::fetch::redact_url(value)
    } else {
        "<redacted-reference>".into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_manifest_is_distinct_from_invalid() {
        let result = inspect(
            include_bytes!("../fixtures/official/no_manifest.jpg"),
            &TrustMaterial::disabled(None),
        )
        .expect("adapter should return a normalized result");
        assert_eq!(result.credential_presence, CredentialPresence::Absent);
        assert_eq!(
            result.cryptographic_validation,
            CryptographicValidation::NotApplicable
        );
    }

    #[test]
    fn collects_stable_raw_codes() {
        let value = serde_json::json!({
            "activeManifest": {
                "failure": [{"code": "assertion.dataHash.mismatch", "url": "self#x"}],
                "success": [{"code": "claimSignature.validated"}]
            }
        });
        let codes = collect_validation_codes(&value);
        assert_eq!(codes.len(), 2);
        assert_eq!(codes[0].kind, "failure");
        assert_eq!(codes[1].kind, "success");
    }

    #[test]
    fn removes_tokens_from_validation_urls() {
        let value = serde_json::json!({
            "failure": [{
                "code": "example",
                "url": "https://example.com/manifest.c2pa?token=secret#internal"
            }]
        });
        let codes = collect_validation_codes(&value);
        assert_eq!(
            codes[0].url.as_deref(),
            Some("https://example.com/manifest.c2pa")
        );
    }
}

use crate::{
    config::{ContinuityPolicy, InconclusivePolicy, PolicyConfig},
    model::{
        CheckpointResult, CredentialPresence, CryptographicValidation, PolicyVerdict, Relationship,
        TrustState,
    },
};

pub fn classify_relationship(
    previous: Option<&CheckpointResult>,
    current: &CheckpointResult,
) -> Relationship {
    let Some(previous) = previous else {
        return Relationship::NotApplicable;
    };
    if previous.sha256.is_some() && previous.sha256 == current.sha256 {
        return Relationship::ByteIdentical;
    }
    if previous.credential_presence == CredentialPresence::Present
        && current.credential_presence == CredentialPresence::Absent
    {
        return Relationship::CredentialsLost;
    }
    if current.cryptographic_validation == CryptographicValidation::Valid {
        if let Some(previous_label) = previous.active_manifest.as_deref() {
            if current
                .parent_manifests
                .iter()
                .any(|parent| manifest_reference_matches(parent, previous_label))
            {
                return Relationship::DeclaredC2paDerivative;
            }
        }
        if current.credential_presence == CredentialPresence::Present {
            return Relationship::ChangedWithoutVerifiableLineage;
        }
    }
    Relationship::Inconclusive
}

fn manifest_reference_matches(reference: &str, label: &str) -> bool {
    reference == label
        || reference
            .strip_prefix("self#jumbf=/c2pa/")
            .is_some_and(|candidate| candidate == label)
        || reference.ends_with(&format!("/c2pa/{label}"))
}

pub fn evaluate(
    checkpoints: &[CheckpointResult],
    policy: &PolicyConfig,
) -> (PolicyVerdict, Vec<String>) {
    let mut verdict = PolicyVerdict::Pass;
    let mut reasons = Vec::new();

    for checkpoint in checkpoints {
        if policy.require_credential {
            match checkpoint.credential_presence {
                CredentialPresence::Present => {}
                CredentialPresence::Absent => record(
                    &mut verdict,
                    PolicyVerdict::Fail,
                    &mut reasons,
                    format!("{}: credential is absent", checkpoint.checkpoint),
                ),
                CredentialPresence::Unreadable | CredentialPresence::Unknown => record(
                    &mut verdict,
                    inconclusive_verdict(policy.on_inconclusive),
                    &mut reasons,
                    format!(
                        "{}: credential presence is inconclusive",
                        checkpoint.checkpoint
                    ),
                ),
            }
        }

        if policy.require_valid_binding {
            match checkpoint.cryptographic_validation {
                CryptographicValidation::Valid => {}
                CryptographicValidation::Invalid => record(
                    &mut verdict,
                    PolicyVerdict::Fail,
                    &mut reasons,
                    format!(
                        "{}: C2PA cryptographic validation failed",
                        checkpoint.checkpoint
                    ),
                ),
                CryptographicValidation::Indeterminate | CryptographicValidation::NotApplicable => {
                    record(
                        &mut verdict,
                        inconclusive_verdict(policy.on_inconclusive),
                        &mut reasons,
                        format!(
                            "{}: cryptographic validation is inconclusive",
                            checkpoint.checkpoint
                        ),
                    )
                }
            }
        }

        if policy.require_trusted {
            match checkpoint.trust {
                TrustState::Trusted => {}
                TrustState::Untrusted => record(
                    &mut verdict,
                    PolicyVerdict::Fail,
                    &mut reasons,
                    format!(
                        "{}: signer does not chain to the configured trust anchors",
                        checkpoint.checkpoint
                    ),
                ),
                TrustState::NotChecked | TrustState::Unknown => record(
                    &mut verdict,
                    inconclusive_verdict(policy.on_inconclusive),
                    &mut reasons,
                    format!("{}: trust was not established", checkpoint.checkpoint),
                ),
            }
        }
    }

    for checkpoint in checkpoints.iter().skip(1) {
        let relationship = &checkpoint.relationship_to_previous_checkpoint;
        match policy.continuity {
            ContinuityPolicy::Any => {}
            ContinuityPolicy::Exact if relationship != &Relationship::ByteIdentical => record(
                &mut verdict,
                if relationship == &Relationship::Inconclusive {
                    inconclusive_verdict(policy.on_inconclusive)
                } else {
                    PolicyVerdict::Fail
                },
                &mut reasons,
                format!(
                    "{}: continuity policy requires byte-identical checkpoints, observed {:?}",
                    checkpoint.checkpoint, relationship
                ),
            ),
            ContinuityPolicy::Exact => {}
            ContinuityPolicy::ExactOrDeclaredDerivative
                if !matches!(
                    relationship,
                    Relationship::ByteIdentical | Relationship::DeclaredC2paDerivative
                ) =>
            {
                record(
                    &mut verdict,
                    if relationship == &Relationship::Inconclusive {
                        inconclusive_verdict(policy.on_inconclusive)
                    } else {
                        PolicyVerdict::Fail
                    },
                    &mut reasons,
                    format!(
                        "{}: no exact bytes or verifiable declared parent lineage",
                        checkpoint.checkpoint
                    ),
                );
            }
            ContinuityPolicy::ExactOrDeclaredDerivative => {}
        }
    }

    if reasons.is_empty() {
        reasons.push("All configured policy requirements passed.".into());
    }
    (verdict, reasons)
}

fn inconclusive_verdict(policy: InconclusivePolicy) -> PolicyVerdict {
    match policy {
        InconclusivePolicy::Pass => PolicyVerdict::Pass,
        InconclusivePolicy::Warn => PolicyVerdict::Warn,
        InconclusivePolicy::Fail => PolicyVerdict::Fail,
    }
}

fn record(
    current: &mut PolicyVerdict,
    candidate: PolicyVerdict,
    reasons: &mut Vec<String>,
    reason: String,
) {
    if severity(&candidate) > severity(current) {
        *current = candidate;
    }
    reasons.push(reason);
}

fn severity(verdict: &PolicyVerdict) -> u8 {
    match verdict {
        PolicyVerdict::Pass => 0,
        PolicyVerdict::Warn => 1,
        PolicyVerdict::Inconclusive => 2,
        PolicyVerdict::Fail => 3,
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::*;
    use crate::model::{Evidence, SourceEvidence};

    fn checkpoint(
        name: &str,
        sha: &str,
        presence: CredentialPresence,
        validation: CryptographicValidation,
    ) -> CheckpointResult {
        CheckpointResult {
            checkpoint: name.into(),
            source: SourceEvidence {
                kind: "path".into(),
                value: format!("{name}.jpg"),
                final_url: None,
                http_status: None,
                http_headers: BTreeMap::new(),
            },
            sha256: Some(sha.into()),
            media_type: Some("image/jpeg".into()),
            size_bytes: Some(4),
            width: None,
            height: None,
            checked_at: "2026-07-26T00:00:00Z".into(),
            credential_presence: presence,
            cryptographic_validation: validation,
            trust: TrustState::NotChecked,
            relationship_to_previous_checkpoint: Relationship::NotApplicable,
            active_manifest: None,
            parent_manifests: Vec::new(),
            raw_validation_codes: Vec::new(),
            evidence: Vec::<Evidence>::new(),
            limitations: Vec::new(),
            error: None,
        }
    }

    #[test]
    fn detects_credentials_lost() {
        let previous = checkpoint(
            "source",
            "a",
            CredentialPresence::Present,
            CryptographicValidation::Valid,
        );
        let current = checkpoint(
            "production",
            "b",
            CredentialPresence::Absent,
            CryptographicValidation::NotApplicable,
        );
        assert_eq!(
            classify_relationship(Some(&previous), &current),
            Relationship::CredentialsLost
        );
    }

    #[test]
    fn valid_changed_manifest_is_not_automatically_continuous() {
        let mut previous = checkpoint(
            "source",
            "a",
            CredentialPresence::Present,
            CryptographicValidation::Valid,
        );
        previous.active_manifest = Some("urn:uuid:parent".into());
        let current = checkpoint(
            "production",
            "b",
            CredentialPresence::Present,
            CryptographicValidation::Valid,
        );
        assert_eq!(
            classify_relationship(Some(&previous), &current),
            Relationship::ChangedWithoutVerifiableLineage
        );
    }

    #[test]
    fn recognizes_declared_parent_reference() {
        let mut previous = checkpoint(
            "source",
            "a",
            CredentialPresence::Present,
            CryptographicValidation::Valid,
        );
        previous.active_manifest = Some("urn:uuid:parent".into());
        let mut current = checkpoint(
            "production",
            "b",
            CredentialPresence::Present,
            CryptographicValidation::Valid,
        );
        current.parent_manifests = vec!["self#jumbf=/c2pa/urn:uuid:parent".to_string()];
        assert_eq!(
            classify_relationship(Some(&previous), &current),
            Relationship::DeclaredC2paDerivative
        );
    }
}

use std::path::{Path, PathBuf};

use chrono::{TimeZone, Utc};
use provenance_ci::{
    c2pa_adapter::{inspect, TrustMaterial},
    model::{CredentialPresence, CryptographicValidation, Relationship, TrustState},
    run, Config, RunOptions,
};

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf()
}

#[test]
fn official_fixtures_cover_required_normalized_states() {
    let trust = TrustMaterial::disabled(Some("test-disabled".into()));
    let fixture = |name: &str| std::fs::read(root().join("fixtures/official").join(name)).unwrap();

    let valid = inspect(&fixture("C.jpg"), &trust).unwrap();
    assert_eq!(valid.credential_presence, CredentialPresence::Present);
    assert_eq!(
        valid.cryptographic_validation,
        CryptographicValidation::Valid
    );

    let invalid = inspect(&fixture("XCA.jpg"), &trust).unwrap();
    assert_eq!(invalid.credential_presence, CredentialPresence::Present);
    assert_eq!(
        invalid.cryptographic_validation,
        CryptographicValidation::Invalid
    );
    assert!(invalid
        .raw_validation_codes
        .iter()
        .any(|status| status.code == "assertion.dataHash.mismatch"));

    let absent = inspect(&fixture("no_manifest.jpg"), &trust).unwrap();
    assert_eq!(absent.credential_presence, CredentialPresence::Absent);
    assert_eq!(
        absent.cryptographic_validation,
        CryptographicValidation::NotApplicable
    );

    let remote = inspect(&fixture("remote_manifest.jpg"), &trust).unwrap();
    assert_eq!(remote.credential_presence, CredentialPresence::Present);
    assert_eq!(
        remote.cryptographic_validation,
        CryptographicValidation::Indeterminate
    );
    assert!(remote
        .evidence
        .iter()
        .any(|item| item.fact == "remote_manifest_not_evaluated"));
}

#[test]
fn cryptographic_validity_and_trust_are_independent() {
    let asset = std::fs::read(root().join("fixtures/official/C.jpg")).unwrap();
    let trusted_pem =
        std::fs::read_to_string(root().join("fixtures/official/test-trust-anchors.pem")).unwrap();
    let nonmatching_pem =
        std::fs::read_to_string(root().join("fixtures/official/nonmatching-test-anchor.pem"))
            .unwrap();

    let trusted = inspect(
        &asset,
        &TrustMaterial::from_pem(trusted_pem, Some("test-roots".into())),
    )
    .unwrap();
    assert_eq!(
        trusted.cryptographic_validation,
        CryptographicValidation::Valid
    );
    assert_eq!(trusted.trust, TrustState::Trusted);

    let untrusted = inspect(
        &asset,
        &TrustMaterial::from_pem(nonmatching_pem, Some("nonmatching-root".into())),
    )
    .unwrap();
    assert_eq!(
        untrusted.cryptographic_validation,
        CryptographicValidation::Valid
    );
    assert_eq!(untrusted.trust, TrustState::Untrusted);
}

#[test]
fn validated_parent_lineage_is_detected() {
    let path = root().join("examples/pass.yml");
    let config = Config::from_path(&path).unwrap();
    let result = run(
        &config,
        &RunOptions {
            config_path: path,
            checked_at: Some(Utc.with_ymd_and_hms(2026, 7, 26, 0, 0, 0).unwrap()),
        },
    )
    .unwrap();
    assert_eq!(
        result.checks[0].checkpoints[1].relationship_to_previous_checkpoint,
        Relationship::DeclaredC2paDerivative
    );
    assert_eq!(result.exit_code, 0);
}

#[test]
fn repeat_run_is_deterministic_with_fixed_evidence_time() {
    let path = root().join("examples/pass.yml");
    let config = Config::from_path(&path).unwrap();
    let options = RunOptions {
        config_path: path,
        checked_at: Some(Utc.with_ymd_and_hms(2026, 7, 26, 0, 0, 0).unwrap()),
    };
    let first = serde_json::to_value(run(&config, &options).unwrap()).unwrap();
    let second = serde_json::to_value(run(&config, &options).unwrap()).unwrap();
    assert_eq!(first, second);
}

#[test]
fn malformed_and_truncated_jpeg_never_become_valid() {
    let trust = TrustMaterial::disabled(None);
    for bytes in [
        vec![0xff, 0xd8, 0xff, 0xe1, 0x00],
        std::fs::read(root().join("fixtures/official/C.jpg"))
            .unwrap()
            .into_iter()
            .take(512)
            .collect(),
    ] {
        let result = inspect(&bytes, &trust).unwrap();
        assert_ne!(
            result.cryptographic_validation,
            CryptographicValidation::Valid
        );
    }
}

#[test]
fn common_unsigned_transformations_are_observed_as_credentials_lost() {
    let path = root().join("examples/transformation-matrix.yml");
    let config = Config::from_path(&path).unwrap();
    let result = run(
        &config,
        &RunOptions {
            config_path: path,
            checked_at: Some(Utc.with_ymd_and_hms(2026, 7, 26, 0, 0, 0).unwrap()),
        },
    )
    .unwrap();

    for check in result.checks {
        let transformed = &check.checkpoints[1];
        assert_eq!(
            transformed.credential_presence,
            CredentialPresence::Absent,
            "{}",
            check.id
        );
        assert_eq!(
            transformed.relationship_to_previous_checkpoint,
            Relationship::CredentialsLost,
            "{}",
            check.id
        );
    }
}

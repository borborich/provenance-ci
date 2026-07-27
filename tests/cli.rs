use std::path::Path;

use assert_cmd::Command;
use predicates::prelude::*;

fn root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
}

#[test]
fn passing_config_exits_zero_and_writes_json() {
    let output = root().join("target/test-pass-result.json");
    Command::cargo_bin("provenance-ci")
        .unwrap()
        .args([
            "--config",
            "examples/pass.yml",
            "--output",
            output.to_str().unwrap(),
            "--checked-at",
            "2026-07-26T00:00:00Z",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("DeclaredC2paDerivative"));
    let result: serde_json::Value =
        serde_json::from_slice(&std::fs::read(output).unwrap()).unwrap();
    assert_eq!(result["exitCode"], 0);
}

#[test]
fn policy_failure_exits_two_but_still_writes_evidence() {
    let output = root().join("target/test-fail-result.json");
    Command::cargo_bin("provenance-ci")
        .unwrap()
        .args([
            "--config",
            "examples/provenance-ci.yml",
            "--output",
            output.to_str().unwrap(),
            "--checked-at",
            "2026-07-26T00:00:00Z",
        ])
        .assert()
        .code(2);
    let result: serde_json::Value =
        serde_json::from_slice(&std::fs::read(output).unwrap()).unwrap();
    assert_eq!(result["overallPolicy"], "fail");
    assert_eq!(result["checks"][0]["firstObservedBreak"]["from"], "build");
}

#[test]
fn missing_config_is_infrastructure_exit_three() {
    Command::cargo_bin("provenance-ci")
        .unwrap()
        .args(["--config", "does-not-exist.yml"])
        .assert()
        .code(3)
        .stderr(predicate::str::contains("failed to read config"));
}

#[test]
fn checkpoint_fetch_error_is_machine_readable_exit_three() {
    let output = root().join("target/test-fetch-error-result.json");
    Command::cargo_bin("provenance-ci")
        .unwrap()
        .args([
            "--config",
            "examples/fetch-error.yml",
            "--output",
            output.to_str().unwrap(),
            "--checked-at",
            "2026-07-26T00:00:00Z",
        ])
        .assert()
        .code(3);
    let result: serde_json::Value =
        serde_json::from_slice(&std::fs::read(output).unwrap()).unwrap();
    assert_eq!(
        result["checks"][0]["checkpoints"][0]["evidence"][0]["fact"],
        "checkpoint_acquisition_failed"
    );
    assert_eq!(result["exitCode"], 3);
}

#[test]
fn local_validation_leaves_custom_temp_directory_empty() {
    let scratch = root().join(format!("target/test-tmp-{}", std::process::id()));
    if scratch.exists() {
        std::fs::remove_dir_all(&scratch).unwrap();
    }
    std::fs::create_dir_all(&scratch).unwrap();

    Command::cargo_bin("provenance-ci")
        .unwrap()
        .env("TMPDIR", &scratch)
        .args([
            "--config",
            "examples/pass.yml",
            "--output",
            "target/test-no-temp-result.json",
            "--checked-at",
            "2026-07-26T00:00:00Z",
        ])
        .assert()
        .success();

    assert_eq!(std::fs::read_dir(&scratch).unwrap().count(), 0);
    std::fs::remove_dir(&scratch).unwrap();
}

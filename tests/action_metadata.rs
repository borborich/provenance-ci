use std::path::Path;

#[test]
fn action_metadata_and_sample_workflow_have_safe_defaults() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let action: serde_yaml::Value =
        serde_yaml::from_slice(&std::fs::read(root.join("action.yml")).unwrap()).unwrap();
    assert_eq!(action["runs"]["using"], "composite");
    assert_eq!(action["inputs"]["upload-artifact"]["default"], "true");

    let workflow =
        std::fs::read_to_string(root.join(".github/workflows/provenance-ci.yml")).unwrap();
    assert!(workflow.contains("contents: read"));
    assert!(!workflow.contains("pull_request_target"));
    assert!(!workflow.contains("secrets:"));
}

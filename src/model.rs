use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CredentialPresence {
    Present,
    Absent,
    Unreadable,
    Unknown,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CryptographicValidation {
    Valid,
    Invalid,
    Indeterminate,
    NotApplicable,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TrustState {
    Trusted,
    Untrusted,
    NotChecked,
    Unknown,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Relationship {
    ByteIdentical,
    DeclaredC2paDerivative,
    CredentialsLost,
    ChangedWithoutVerifiableLineage,
    DifferentAsset,
    Inconclusive,
    NotApplicable,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PolicyVerdict {
    Pass,
    Warn,
    Fail,
    Inconclusive,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RunResult {
    pub schema_version: u32,
    pub generated_at: String,
    pub tool: ToolInfo,
    pub trust: TrustInfo,
    pub checks: Vec<CheckResult>,
    pub overall_policy: PolicyVerdict,
    pub exit_code: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolInfo {
    pub name: String,
    pub version: String,
    pub c2pa_sdk: String,
    pub supported_specification: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrustInfo {
    pub checked: bool,
    pub id: Option<String>,
    pub sha256: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckResult {
    pub id: String,
    pub checkpoints: Vec<CheckpointResult>,
    pub first_observed_break: Option<ObservedBreak>,
    pub policy: PolicyVerdict,
    pub policy_reasons: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckpointResult {
    pub checkpoint: String,
    pub source: SourceEvidence,
    pub sha256: Option<String>,
    pub media_type: Option<String>,
    pub size_bytes: Option<u64>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub checked_at: String,
    pub credential_presence: CredentialPresence,
    pub cryptographic_validation: CryptographicValidation,
    pub trust: TrustState,
    pub relationship_to_previous_checkpoint: Relationship,
    pub active_manifest: Option<String>,
    pub parent_manifests: Vec<String>,
    pub raw_validation_codes: Vec<ValidationCode>,
    pub evidence: Vec<Evidence>,
    pub limitations: Vec<String>,
    pub error: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceEvidence {
    pub kind: String,
    pub value: String,
    pub final_url: Option<String>,
    pub http_status: Option<u16>,
    pub http_headers: BTreeMap<String, String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidationCode {
    pub code: String,
    pub kind: String,
    pub url: Option<String>,
    pub explanation: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Evidence {
    pub fact: String,
    pub detail: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObservedBreak {
    pub from: String,
    pub to: String,
    pub relationship: Relationship,
    pub statement: String,
}

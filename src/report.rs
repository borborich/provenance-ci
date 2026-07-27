use crate::model::{PolicyVerdict, RunResult};

pub fn human(result: &RunResult) -> String {
    let mut output = format!(
        "Provenance CI: {:?} (exit {})\n",
        result.overall_policy, result.exit_code
    );
    for check in &result.checks {
        output.push_str(&format!("\n{}: {:?}\n", check.id, check.policy));
        for checkpoint in &check.checkpoints {
            output.push_str(&format!(
                "  {}: presence={:?}, validation={:?}, trust={:?}, relationship={:?}\n",
                checkpoint.checkpoint,
                checkpoint.credential_presence,
                checkpoint.cryptographic_validation,
                checkpoint.trust,
                checkpoint.relationship_to_previous_checkpoint
            ));
        }
        if let Some(observed_break) = &check.first_observed_break {
            output.push_str(&format!("  {}\n", observed_break.statement));
        }
        for reason in &check.policy_reasons {
            output.push_str(&format!("  - {reason}\n"));
        }
    }
    output
}

pub fn markdown(result: &RunResult) -> String {
    let mut output = format!(
        "# Provenance CI result\n\nOverall policy: **{:?}**  \nSDK: `c2pa {}`  \nChecked: `{}`\n\n",
        result.overall_policy, result.tool.c2pa_sdk, result.generated_at
    );
    output.push_str("| Check | Checkpoint | Presence | Validation | Trust | Relationship |\n");
    output.push_str("|---|---|---|---|---|---|\n");
    for check in &result.checks {
        for checkpoint in &check.checkpoints {
            output.push_str(&format!(
                "| {} | {} | {:?} | {:?} | {:?} | {:?} |\n",
                escape(&check.id),
                escape(&checkpoint.checkpoint),
                checkpoint.credential_presence,
                checkpoint.cryptographic_validation,
                checkpoint.trust,
                checkpoint.relationship_to_previous_checkpoint
            ));
        }
        if let Some(observed_break) = &check.first_observed_break {
            output.push_str(&format!("\n> {}\n\n", observed_break.statement));
        }
    }
    output.push_str(
        "\n## Limitations\n\n- This validates provenance data and cryptographic bindings, not whether content is true, authentic, human-created, unmanipulated, correctly copyrighted, or trustworthy.\n- A first observed break identifies an interval between supplied checkpoints, not the hidden system that caused it.\n- Remote manifests and soft-binding recovery are not evaluated by this MVP.\n",
    );
    output
}

pub fn github_annotations(result: &RunResult) -> String {
    let mut output = String::new();
    for check in &result.checks {
        let command = match check.policy {
            PolicyVerdict::Fail => "error",
            PolicyVerdict::Warn | PolicyVerdict::Inconclusive => "warning",
            PolicyVerdict::Pass => "notice",
        };
        for reason in &check.policy_reasons {
            output.push_str(&format!(
                "::{command} title=Provenance CI ({})::{}\n",
                annotation_escape(&check.id),
                annotation_escape(reason)
            ));
        }
    }
    output
}

fn escape(value: &str) -> String {
    value.replace('|', "\\|").replace('\n', " ")
}

fn annotation_escape(value: &str) -> String {
    value
        .replace('%', "%25")
        .replace('\r', "%0D")
        .replace('\n', "%0A")
        .replace(':', "%3A")
        .replace(',', "%2C")
}

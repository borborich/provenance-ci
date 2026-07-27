#!/usr/bin/env bash
set -uo pipefail

action_root="${GITHUB_ACTION_PATH:?GITHUB_ACTION_PATH is required}"
config_path="${INPUT_CONFIG:-provenance-ci.yml}"
result_path="${INPUT_RESULT_PATH:-provenance-ci-result.json}"
binary_path="${INPUT_BINARY:-}"
summary_path="${GITHUB_STEP_SUMMARY:-}"
output_path="${GITHUB_OUTPUT:-}"

if [[ -z "$config_path" || -z "$result_path" ]]; then
  echo "::error title=Provenance CI::config and result-path must not be empty"
  exit_code=3
elif [[ -n "$binary_path" ]]; then
  if [[ ! -x "$binary_path" ]]; then
    echo "::error title=Provenance CI::Configured binary is not executable"
    exit_code=3
  else
    args=(--config "$config_path" --output "$result_path" --github-annotations)
    if [[ -n "$summary_path" ]]; then
      args+=(--markdown "$summary_path")
    fi
    "$binary_path" "${args[@]}"
    exit_code=$?
  fi
else
  args=(--config "$config_path" --output "$result_path" --github-annotations)
  if [[ -n "$summary_path" ]]; then
    args+=(--markdown "$summary_path")
  fi
  cargo run --quiet --release --locked --manifest-path "$action_root/Cargo.toml" -- "${args[@]}"
  exit_code=$?
fi

if [[ -n "$output_path" ]]; then
  {
    echo "result_path=$result_path"
    echo "exit_code=$exit_code"
  } >>"$output_path"
fi

# Policy failure must not prevent the composite action from uploading evidence.
exit 0


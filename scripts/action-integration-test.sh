#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
work_dir="$(mktemp -d "${TMPDIR:-/tmp}/provenance-ci-action.XXXXXX")"
trap 'rm -rf "$work_dir"' EXIT

cargo build --locked --manifest-path "$repo_root/Cargo.toml"

run_action() {
  local config="$1"
  local expected="$2"
  local stem="$3"
  GITHUB_ACTION_PATH="$repo_root" \
  GITHUB_STEP_SUMMARY="$work_dir/$stem-summary.md" \
  GITHUB_OUTPUT="$work_dir/$stem-output.txt" \
  INPUT_CONFIG="$repo_root/$config" \
  INPUT_RESULT_PATH="$work_dir/$stem-result.json" \
  INPUT_BINARY="$repo_root/target/debug/provenance-ci" \
    "$repo_root/integrations/github-action/run.sh" >/dev/null
  grep -Fx "exit_code=$expected" "$work_dir/$stem-output.txt" >/dev/null
  grep -Fx "result_path=$work_dir/$stem-result.json" "$work_dir/$stem-output.txt" >/dev/null
  [[ -s "$work_dir/$stem-result.json" ]]
  [[ -s "$work_dir/$stem-summary.md" ]]
}

run_action "examples/pass.yml" 0 pass
run_action "examples/provenance-ci.yml" 2 fail
echo "GitHub Action integration test passed."


#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
work_dir="$(mktemp -d "${TMPDIR:-/tmp}/provenance-ci-release.XXXXXX")"
trap 'rm -rf "$work_dir"' EXIT

cargo build --release --locked --manifest-path "$repo_root/Cargo.toml"
binary="$repo_root/target/release/provenance-ci"

"$binary" \
  --config "$repo_root/examples/pass.yml" \
  --output "$work_dir/pass.json" \
  --markdown "$work_dir/summary.md" \
  --checked-at 2026-07-26T00:00:00Z >/dev/null

set +e
"$binary" \
  --config "$repo_root/examples/provenance-ci.yml" \
  --output "$work_dir/fail.json" \
  --checked-at 2026-07-26T00:00:00Z >/dev/null
policy_code=$?
"$binary" --config "$work_dir/missing.yml" >/dev/null 2>&1
infrastructure_code=$?
set -e

[[ "$policy_code" == "2" ]]
[[ "$infrastructure_code" == "3" ]]
[[ -s "$work_dir/pass.json" ]]
[[ -s "$work_dir/fail.json" ]]
[[ -s "$work_dir/summary.md" ]]

python3 - "$work_dir/pass.json" <<'PY'
import json
import sys
result = json.load(open(sys.argv[1]))
assert result["schemaVersion"] == 1
assert result["exitCode"] == 0
assert result["checks"][0]["checkpoints"][1]["relationshipToPreviousCheckpoint"] == "declared_c2pa_derivative"
PY

echo "Release smoke test passed."


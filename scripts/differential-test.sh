#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
c2patool_bin="${C2PATOOL_BIN:-c2patool}"
work_dir="$(mktemp -d "${TMPDIR:-/tmp}/provenance-ci-differential.XXXXXX")"
trap 'rm -rf "$work_dir"' EXIT

if ! command -v "$c2patool_bin" >/dev/null 2>&1 && [[ ! -x "$c2patool_bin" ]]; then
  echo "c2patool not found; set C2PATOOL_BIN to pinned c2patool 0.27.3" >&2
  exit 3
fi

version="$("$c2patool_bin" --version)"
if [[ "$version" != "c2patool 0.27.3" ]]; then
  echo "expected c2patool 0.27.3, got: $version" >&2
  exit 3
fi

cargo run --quiet -- \
  --config "$repo_root/examples/fixture-matrix.yml" \
  --output "$work_dir/provenance.json" \
  --checked-at 2026-07-26T00:00:00Z >/dev/null

"$c2patool_bin" \
  --settings "$repo_root/tests/oracle-settings.toml" \
  "$repo_root/fixtures/official/C.jpg" >"$work_dir/c.json"
"$c2patool_bin" \
  --settings "$repo_root/tests/oracle-settings.toml" \
  "$repo_root/fixtures/official/XCA.jpg" >"$work_dir/xca.json"

python3 - "$work_dir" <<'PY'
import json
import pathlib
import sys

work = pathlib.Path(sys.argv[1])
product = json.loads((work / "provenance.json").read_text())
oracle_valid = json.loads((work / "c.json").read_text())
oracle_invalid = json.loads((work / "xca.json").read_text())

points = {
    point["checkpoint"]: point
    for point in product["checks"][0]["checkpoints"]
}
assert points["c"]["credentialPresence"] == "present"
assert points["c"]["cryptographicValidation"] == "valid"
assert points["invalid-hard-binding"]["credentialPresence"] == "present"
assert points["invalid-hard-binding"]["cryptographicValidation"] == "invalid"
assert points["stripped"]["credentialPresence"] == "absent"

def codes(value):
    found = set()
    if isinstance(value, dict):
        if isinstance(value.get("code"), str):
            found.add(value["code"])
        for child in value.values():
            found |= codes(child)
    elif isinstance(value, list):
        for child in value:
            found |= codes(child)
    return found

assert "claimSignature.validated" in codes(oracle_valid)
assert "assertion.dataHash.mismatch" in codes(oracle_invalid)
assert "assertion.dataHash.mismatch" in {
    item["code"] for item in points["invalid-hard-binding"]["rawValidationCodes"]
}
print("SDK adapter and c2patool 0.27.3 agree on valid and invalid-hard-binding fixtures.")
PY

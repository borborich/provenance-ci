#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
version="$(awk -F '"' '/^version = / {print $2; exit}' "$repo_root/Cargo.toml")"
target="$(rustc -vV | sed -n 's/^host: //p')"
name="provenance-ci-v${version}-${target}"
stage_dir="$(mktemp -d "${TMPDIR:-/tmp}/${name}.XXXXXX")"
trap 'rm -rf "$stage_dir"' EXIT

SOURCE_DATE_EPOCH=0 cargo build --release --locked --manifest-path "$repo_root/Cargo.toml"
"$repo_root/scripts/check-licenses.py"
mkdir -p "$stage_dir/$name/schemas"
cp "$repo_root/target/release/provenance-ci" "$stage_dir/$name/"
cp "$repo_root/README.md" "$repo_root/CHANGELOG.md" "$repo_root/SECURITY.md" "$stage_dir/$name/"
cp "$repo_root/LICENSE-MIT" "$repo_root/LICENSE-APACHE" "$repo_root/THIRD_PARTY_NOTICES.md" "$stage_dir/$name/"
cp "$repo_root/Cargo.lock" "$repo_root/dist/dependency-licenses.json" "$stage_dir/$name/"
cp "$repo_root/schemas/config-v1.schema.json" "$repo_root/schemas/result-v1.schema.json" "$stage_dir/$name/schemas/"

mkdir -p "$repo_root/dist"
archive="$repo_root/dist/$name.tar.gz"
"$repo_root/scripts/deterministic-tar.py" "$stage_dir/$name" "$archive"
(
  cd "$repo_root/dist"
  LC_ALL=C shasum -a 256 "$name.tar.gz" >"$name.tar.gz.sha256"
)
echo "$archive"

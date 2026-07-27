#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
ffmpeg_bin="${FFMPEG_BIN:-ffmpeg}"
jpegtran_bin="${JPEGTRAN_BIN:-jpegtran}"
expected_version="ffmpeg version 7.1.1 "
actual_version="$("$ffmpeg_bin" -version | head -1)"
if [[ "$actual_version" != "$expected_version"* ]]; then
  echo "expected FFmpeg 7.1.1, got: $actual_version" >&2
  exit 2
fi
expected_jpegtran="libjpeg-turbo version 3.2.0 "
actual_jpegtran="$("$jpegtran_bin" -version 2>&1 | head -1)"
if [[ "$actual_jpegtran" != "$expected_jpegtran"* ]]; then
  echo "expected libjpeg-turbo 3.2.0 jpegtran, got: $actual_jpegtran" >&2
  exit 2
fi

source_asset="$repo_root/fixtures/official/C.jpg"
output_dir="$repo_root/fixtures/transforms"
mkdir -p "$output_dir"

"$jpegtran_bin" -copy none -outfile "$output_dir/metadata-stripped.jpg" "$source_asset"
"$ffmpeg_bin" -hide_banner -loglevel error -y -i "$source_asset" \
  -map_metadata -1 -frames:v 1 -q:v 2 \
  "$output_dir/reencoded.jpg"
"$ffmpeg_bin" -hide_banner -loglevel error -y -i "$source_asset" \
  -map_metadata -1 -frames:v 1 -vf "scale=1024:-2" -q:v 2 \
  "$output_dir/resized.jpg"
"$ffmpeg_bin" -hide_banner -loglevel error -y -i "$source_asset" \
  -map_metadata -1 -frames:v 1 -vf "crop=1024:1024:(in_w-1024)/2:(in_h-1024)/2" -q:v 2 \
  "$output_dir/cropped.jpg"
"$ffmpeg_bin" -hide_banner -loglevel error -y -i "$source_asset" \
  -map_metadata -1 -frames:v 1 -vf "transpose=clock" -q:v 2 \
  "$output_dir/orientation-normalized.jpg"

LC_ALL=C shasum -a 256 "$output_dir"/*.jpg

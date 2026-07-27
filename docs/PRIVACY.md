# Privacy

## Current processing

The CLI and Action process assets on the user's machine or GitHub runner. The
project operates no server, account, database, analytics endpoint, or telemetry
collector.

Raw asset bytes are held in process memory for hashing and validation and are
not copied into reports. The current implementation creates no media temporary
files. The GitHub Action uploads only the versioned JSON result unless the
caller independently uploads other files.

## Data in results

Results include:

- checkpoint names and configured local path strings;
- public URL scheme/host/path without query or fragment;
- selected HTTP rendition headers;
- asset SHA-256, byte size, MIME, JPEG dimensions;
- manifest labels, parent references, raw validation codes, evidence, errors,
  SDK/spec/trust metadata, and limitations.

HTTP(S) references attached to raw validation codes are also stored without
query or fragment; opaque non-`self`/URN references are replaced.

Results do not intentionally include image pixels, thumbnails, full manifests,
EXIF/GPS, certificates, cookies, authorization headers, URL query tokens, or
response bodies beyond derived validation evidence.

Manifest labels and local paths can still be identifying. Treat JSON reports as
potentially sensitive and use repository-appropriate artifact access and
retention.

## External services

For a configured URL checkpoint, the runner contacts that public host. Normal
network observers, DNS, the target host, GitHub-hosted runner providers, and
certificate infrastructure may observe the request. No third-party analytics
request is added.

The SDK is not allowed to contact remote manifests, ingredients, assertions,
icons, soft-binding repositories, or OCSP endpoints.

## User commitments

Do not inspect assets or URLs without authority. Avoid signed private URLs in
configuration. Do not publish JSON evidence containing confidential path or
manifest identifiers. A future hosted product requires a separate privacy
notice, deletion mechanism, processor/subprocessor inventory, and legal review.

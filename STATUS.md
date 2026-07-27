# Status

Last updated: 2026-07-27

## Decision

**NARROW**

The safe first release is local JPEG checkpoint validation, explicit
trust-by-pinned-PEM, validated parent-lineage comparison, CLI, and GitHub
Action. Remote manifests, external assertion retrieval, soft-binding recovery,
additional media formats, hosted scanning, signing, SaaS, and billing remain
out of scope.

## Completed

- Current C2PA 2.4, SDK 0.90.3, c2patool 0.27.3, trust model, licenses,
  conformance program, Action requirements, branding constraints, platform
  behavior, and competitors researched from primary sources.
- Official SDK adapter with automatic network access disabled.
- Independent presence, cryptographic validation, trust, continuity, and policy
  result fields.
- Ordered local paths and hardened public-HTTPS checkpoints.
- Valid, trusted, untrusted, absent, invalid hard binding, remote-not-evaluated,
  malformed, unsupported, fetch-error, and declared-parent fixture paths.
- Reproducible metadata-strip, re-encode, resize, crop, and
  orientation-rewrite matrix.
- Stable result/config schemas and exit codes.
- CLI human/JSON/Markdown output and GitHub annotations.
- Root composite Action, artifact upload, Job Summary, and sample workflow.
- Unit, fixture, deterministic, CLI, Action integration, release smoke, and
  differential SDK-versus-c2patool tests.
- Final local verification: 22 Rust tests passed, strict clippy passed, Action
  integration passed, SDK/c2patool differential passed, 382-package license
  inventory passed, release smoke passed, and consecutive release packages
  produced the same checksum.
- Local `0.1.0` artifact:
  `dist/provenance-ci-v0.1.0-aarch64-apple-darwin.tar.gz`, SHA-256
  `64a4ad03647314811397909a9631ba5736c3302d27349531b0987f7c2579a7ee`.

## Publication state

The source repository is public at
`https://github.com/borborich/provenance-ci`. No immutable GitHub release,
package registry publication, Action Marketplace listing, domain, hosted
scanner, analytics, billing plan, or external campaign has been created.

## Known limits

- JPEG only.
- Trust list must be supplied as a local immutable snapshot; no automatic
  refresh.
- OCSP network fetching is disabled.
- Remote manifest references are `present + indeterminate`, not fetched.
- A declared derivative requires validated `parentOf` evidence. If it cannot be
  proved, continuity is inconclusive or changed without verifiable lineage.
- The official SDK is pre-1.0 and implements a subset of C2PA 2.4.
- The first artifact is macOS/Apple Silicon. The checked-in Ubuntu workflow has
  not run on an external GitHub runner because nothing was published.

## Next gate

Verify the first public GitHub CI run. Then, after separate explicit approval,
verify the working name/trademarks, publish an immutable release and checksums,
list the Action, and observe real third-party use. Do not add SaaS features
before the 30–45 day usage gate in `docs/METRICS.md`.

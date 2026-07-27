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
- First public Ubuntu CI run `30297140231` passed all formatting, clippy,
  test, Action integration, license, release-build, and smoke-test gates.
- Local `0.1.0` artifact:
  `dist/provenance-ci-v0.1.0-aarch64-apple-darwin.tar.gz`, SHA-256
  `10743470ec200821d5b167ab3d599a860ec51deb0f5b877c88010c1c6bc79bb4`.

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
- The first packaged binary is macOS/Apple Silicon. Source build and tests have
  also passed on GitHub-hosted Ubuntu.

## Next gate

After separate explicit approval, verify the working name/trademarks, publish
an immutable release and checksums, list the Action, and observe real
third-party use. Do not add SaaS features before the 30–45 day usage gate in
`docs/METRICS.md`.

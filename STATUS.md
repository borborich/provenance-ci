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
- Public Ubuntu CI run `30297714701` passed the same gates after updating the
  full-SHA-pinned checkout dependency to `actions/checkout v6.0.2`.
- Release Ubuntu CI run `30299051776` passed all required gates on commit
  `a6b64a33fc236ba0e0d6cd39f7cb42749f495ebc`.
- Published `0.1.0` artifact:
  `dist/provenance-ci-v0.1.0-aarch64-apple-darwin.tar.gz`, SHA-256
  `ac2bd746ce8936dbf2cb865be24cd875ab443cbbacd4bc69c9a0c96d2e3ec343`.
- GitHub release immutability is enabled. The release API reports `v0.1.0` as
  immutable, and GitHub release-attestation verification passed for the archive
  and its checksum file.
- A preliminary exact-name and repository search found no obvious conflicting
  “Provenance CI” product. This is not legal or trademark clearance, so the
  public working-name caveat remains.

## Publication state

The source repository is public at
`https://github.com/borborich/provenance-ci`. The immutable `v0.1.0` GitHub
Release is published at
`https://github.com/borborich/provenance-ci/releases/tag/v0.1.0`. No package
registry publication, Action Marketplace listing, domain, hosted scanner,
analytics, billing plan, or external campaign has been created.

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

Action Marketplace publication remains a separate explicit gate. Before
listing, complete the remaining repository security/settings review and the
Marketplace-specific copy review. After listing, observe real third-party use;
do not add SaaS features before the 30–45 day usage gate in `docs/METRICS.md`.

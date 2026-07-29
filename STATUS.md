# Status

Last updated: 2026-07-29

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
- Dependabot vulnerability alerts and security updates, secret scanning push
  protection, private vulnerability reporting, CodeQL default setup, and
  non-destructive `main` branch protection are enabled.
- Release CI run `30305889837`, CodeQL run `30305889422`, and both Dependabot
  ecosystem checks passed on commit
  `11fdb101b8e39ab421690c9203311a86e79aeb7a`.
- Published `0.1.1` Marketplace artifact:
  `dist/provenance-ci-v0.1.1-aarch64-apple-darwin.tar.gz`, SHA-256
  `e8aa1e5fca21fd815457264cb6410355d68d5c1b1548d88aed62bbce544284f2`.
- The `v0.1.1` release API reports `immutable: true`, and GitHub
  release-attestation verification passed for the archive and checksum file.
- The Action is published in GitHub Marketplace under the Continuous
  integration and Testing categories.
- An English-language static reference site and reproducible first-break guide
  are prepared for GitHub Pages. The site has no application backend,
  analytics, user accounts, asset upload, or hosted validation.

## Publication state

The source repository is public at
`https://github.com/borborich/provenance-ci`. The immutable `v0.1.1` GitHub
Release is published at
`https://github.com/borborich/provenance-ci/releases/tag/v0.1.1`. The Action is
listed at
`https://github.com/marketplace/actions/provenance-ci-checkpoint-validator`.
No package registry publication, domain, hosted scanner, analytics, billing
plan, or third-party promotional campaign has been created. The GitHub Pages
reference site is published from version-controlled static files in this
repository.

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

Observe real third-party use for the 30–45 day gate in `docs/METRICS.md`.
Track installs, successful checks, repeat use, support load, and inconclusive
rates without adding telemetry or uploading raw assets. Do not add SaaS,
accounts, billing, signing, or hosted scanning before that gate.

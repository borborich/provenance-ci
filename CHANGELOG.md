# Changelog

All notable changes are documented here.

## 0.1.1 - 2026-07-27

- Prepared the root Action metadata and documentation for GitHub Marketplace.
- Documented runner-local processing, privacy, support, and private
  vulnerability-reporting paths.
- Enabled repository release immutability, Dependabot alerts and security
  updates, secret scanning push protection, CodeQL default setup, and
  non-destructive `main` branch protection.
- Added weekly pinned dependency update checks for Cargo and GitHub Actions.
- Published the Action in GitHub Marketplace under the Continuous integration
  and Testing categories.
- Kept the validation core, schemas, policy semantics, and network boundaries
  unchanged from `0.1.0`.

## 0.1.0 - 2026-07-27

- Added a pinned official C2PA Rust SDK adapter for JPEG validation.
- Added versioned YAML configuration and JSON result schemas.
- Added ordered checkpoint comparison and first-observed-break localization.
- Added independent presence, cryptographic validity, trust, continuity, and
  policy dimensions.
- Added path and hardened public-HTTPS acquisition without raw URL query
  retention.
- Added explicit remote-manifest non-evaluation.
- Added CLI, Markdown summary, GitHub annotations, stable exit semantics, and a
  root composite GitHub Action.
- Added official fixtures, a generated declared-derivative fixture,
  reproducible transformation matrix, differential c2patool tests, and release
  tooling.
- Recorded the Phase 0 decision as `NARROW`.
- Published the source preview at
  `https://github.com/borborich/provenance-ci`.
- Updated the full-SHA-pinned checkout dependency to Node.js 24-based
  `actions/checkout v6.0.2`.
- Published an immutable GitHub release with a macOS Apple Silicon binary
  archive and SHA-256 checksum.

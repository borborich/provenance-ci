# Provenance CI project rules

## Product truthfulness

- Treat C2PA as provenance metadata, not as a truth, copyright, authorship, AI-detection, or manipulation oracle.
- State only the first observed break between user-supplied ordered checkpoints.
- Label hypotheses as inferences. Never name a breaking component without a supplied intermediate checkpoint or a controlled integration test.
- Keep credential presence, cryptographic validation, trust, continuity, and policy as separate dimensions.
- A changed file may be a valid declared derivative. Byte equality or perceptual similarity alone never proves C2PA continuity.
- Never describe this project as C2PA certified, compliant, conformant, affiliated, or endorsed.

## Scope and security

- The first release is a local JPEG validation core, CLI, and GitHub Action. Do not add SaaS, accounts, billing, signing, scheduled monitors, plugins, or dashboards before a usage gate.
- Use the official Content Authenticity Initiative C2PA SDK behind `src/c2pa_adapter.rs`; do not implement parsing or cryptography.
- Remote manifests are not evaluated in the MVP. Do not let the SDK fetch arbitrary external resources.
- URL checkpoints accept public HTTPS on ports 443 or implicit 443 only, reject credentials, private/reserved addresses, redirects to disallowed destinations, non-JPEG content, oversized bodies, and excess redirects.
- Never accept signing keys. Do not log URL query strings, credentials, manifest bodies, EXIF, or raw user assets.
- Temporary files must be scoped, permission-restricted where practical, and deleted.

## Engineering

- Public documentation, code, schemas, CLI text, and Action output are English. Owner-facing communication is Russian.
- Pin direct dependencies and release tooling. Commit `Cargo.lock`.
- Version config and result schemas independently; reject unsupported major versions.
- Stable exit codes: `0` pass/warn, `2` policy failure, `3` inconclusive/infrastructure/configuration error.
- Every checkpoint result includes byte SHA-256, media type, size, final redacted URL where applicable, timestamp, SDK version, trust configuration, raw validation codes, evidence, and limitations.
- Tests must be offline and deterministic unless explicitly marked as network/integration tests.
- Do not modify unrelated user files, publish externally, spend money, buy domains, accept legal terms, or contact third parties.
- Update `STATUS.md` when a phase or gate changes and document material decisions in `docs/adr/`.

## Verification

Run before handoff:

```sh
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
cargo build --release --locked
./scripts/release-smoke.sh
```


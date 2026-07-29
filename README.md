# Provenance CI — C2PA Content Credentials checkpoint testing

Provenance CI is an open-source Rust CLI and GitHub Action for local-first
regression and continuity testing of C2PA Content Credentials across ordered
JPEG publishing checkpoints. It reports the first observed interval where
credential presence, cryptographic validation, trust, declared continuity, or
configured policy changes.

> Project status: `v0.1.1` is the Marketplace release of the local JPEG
> validator and GitHub Action, available from the
> [GitHub Marketplace](https://github.com/marketplace/actions/provenance-ci-checkpoint-validator).
> Practical guides are published at
> [borborich.github.io/provenance-ci](https://borborich.github.io/provenance-ci/).
> This project is not affiliated with or endorsed by C2PA, the Content
> Authenticity Initiative, Adobe, or any other vendor. “Provenance CI” is a
> working name only.

## What it does

Given source, build, origin, CDN, or production checkpoints, the tool validates
each JPEG with the official Content Authenticity Initiative Rust SDK and reports
the first observed interval where credentials disappear, fail validation, or
lose verifiable declared continuity.

It keeps five dimensions separate:

- credential presence;
- cryptographic validation;
- trust against an explicitly supplied local PEM snapshot;
- relationship to the previous checkpoint;
- configured policy verdict.

It does **not** determine whether an image is true, authentic, human-created,
AI-generated, unmanipulated, correctly copyrighted, or signed by a person or
organization that deserves trust. It does not identify a hidden system that
caused a break.

## Five-minute local quickstart

Requirements: Rust 1.88 or newer and standard C build tools.

```sh
git clone https://github.com/borborich/provenance-ci.git
cd provenance-ci
cargo run --locked -- --config examples/pass.yml
```

The first build downloads and compiles pinned dependencies. The command writes
`provenance-ci-result.json` and returns:

| Exit | Meaning |
|---:|---|
| `0` | policy passed or produced configured warnings |
| `2` | deterministic policy failure |
| `3` | infrastructure/configuration failure or unresolved inconclusive run |

Try the intentional regression:

```sh
cargo run --locked -- \
  --config examples/provenance-ci.yml \
  --output provenance-ci-result.json \
  --markdown provenance-ci-summary.md
```

The `v0.1.1` GitHub Release also provides a macOS Apple Silicon binary archive
and a sibling SHA-256 file. Other platforms should build from the immutable
release tag with the pinned `Cargo.lock`.

## Configuration

```yaml
version: 1

checks:
  - id: homepage-hero
    checkpoints:
      - name: source
        path: assets/hero.jpg
      - name: build
        path: dist/hero.jpg
      - name: production
        url: https://example.com/images/hero.jpg
    policy:
      requireCredential: true
      requireValidBinding: true
      requireTrusted: false
      continuity: exact-or-declared-derivative
      onInconclusive: warn
```

Paths are resolved relative to the config file. URL checkpoints allow public
HTTPS on port 443 only and apply address, redirect, size, timeout, and MIME
controls. URL query strings are not persisted.

Trust is never downloaded implicitly. To check trust, pin a local PEM snapshot:

```yaml
trust:
  anchorsPath: ./C2PA-TRUST-LIST.pem
  id: https://github.com/c2pa-org/conformance-public/commit/<commit>
```

The result records the PEM SHA-256. Without `anchorsPath`, trust is
`not_checked`, not `untrusted`.

Schemas are in [`schemas/config-v1.schema.json`](schemas/config-v1.schema.json)
and [`schemas/result-v1.schema.json`](schemas/result-v1.schema.json).

## GitHub Action

The local Action has a root `action.yml`, needs only `contents: read`, uploads
the JSON evidence artifact, writes the Job Summary, and emits annotations.

```yaml
permissions:
  contents: read

steps:
  - uses: actions/checkout@de0fac2e4500dabe0009e67214ff5f5447ce83dd # v6.0.2
  - uses: borborich/provenance-ci@v0.1.1
    with:
      config: provenance-ci.yml
```

The `v0.1.1` release tag and its assets are protected by GitHub release
immutability. For the strongest supply-chain pinning, resolve the tag and use
its reviewed full commit SHA. The Action builds from the pinned lockfile unless
a prebuilt binary is explicitly supplied. User assets remain on the runner.

## Security, privacy, and support

The Action processes assets on the caller's runner. The project operates no
backend, account system, analytics endpoint, or telemetry collector. By
default, the Action uploads only the versioned JSON evidence artifact with
14-day retention; set `upload-artifact: "false"` to disable that upload.

- Review the [privacy and data-flow details](docs/PRIVACY.md).
- Report vulnerabilities through
  [GitHub private vulnerability reporting](https://github.com/borborich/provenance-ci/security/advisories/new).
- Use [GitHub Issues](https://github.com/borborich/provenance-ci/issues) for
  non-sensitive support requests after reviewing [SUPPORT.md](SUPPORT.md).

## Verification

```sh
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features --locked
./scripts/action-integration-test.sh
C2PATOOL_BIN=/path/to/c2patool-0.27.3 ./scripts/differential-test.sh
./scripts/release-smoke.sh
./scripts/check-licenses.py
./scripts/package-release.sh
```

See [RESEARCH](docs/RESEARCH.md), [FEASIBILITY](docs/FEASIBILITY.md),
[ARCHITECTURE](docs/ARCHITECTURE.md), and [THREAT MODEL](docs/THREAT_MODEL.md).

## License

Project code is available under `MIT OR Apache-2.0`. Fixtures and dependencies
retain their upstream licenses; see
[THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md).

# Provenance CI

Local-first CI regression and continuity testing for Content Credentials across
ordered publishing checkpoints.

> Project status: public source preview for release candidate `0.1.0`. No
> package or Marketplace listing has been released. This project is not
> affiliated with or endorsed by C2PA, the Content Authenticity Initiative,
> Adobe, or any other vendor. “Provenance CI” is a working name only.

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
  - uses: actions/checkout@34e114876b0b11c390a56381ad16ebd13914f8d5
  # Pre-release preview only. Pin a reviewed release commit for production use.
  - uses: borborich/provenance-ci@main
    with:
      config: provenance-ci.yml
```

The public repository can be referenced directly, but `main` is mutable.
Production consumers should pin a reviewed full commit SHA after the first
immutable release. The Action builds from the pinned lockfile unless a prebuilt
binary is explicitly supplied. User assets remain on the runner.

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

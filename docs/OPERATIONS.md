# Operations

## Local build

```sh
cargo build --release --locked
```

Runtime inputs are a YAML config, referenced local JPEG/PEM files, and optional
public HTTPS assets. Runtime outputs are JSON and optional Markdown. No daemon
or service is operated.

## Routine verification

```sh
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features --locked
./scripts/action-integration-test.sh
./scripts/check-licenses.py
./scripts/release-smoke.sh
./scripts/package-release.sh
```

Run differential tests with the checksum-verified official oracle:

```sh
C2PATOOL_BIN=/absolute/path/to/c2patool ./scripts/differential-test.sh
```

## Trust-list update

1. Download from the official conformance-public repository outside the
   validator process.
2. Verify repository provenance and review upstream changes.
3. Calculate SHA-256 and record immutable commit URL/fetch date.
4. Run trusted/untrusted fixtures and production sample checks.
5. Store/deploy the PEM as an explicit input; never silently replace a list
   during a run.

## SDK update

Review release/security notes and licenses, update exact pin/lockfile and
adapter version constant, run all tests plus c2patool differential tests, inspect
raw-code changes, update schemas/docs if semantics change, and publish a new
project release only after approval.

## Failure triage

- exit `2`: inspect policy reasons and the first observed break.
- exit `3`: inspect acquisition/config/error evidence before treating it as a
  provenance regression.
- unexpected parser result: preserve only an authorized minimized fixture,
record SHA-256/SDK/settings, compare with pinned c2patool, and report privately
if it may be a vulnerability.

## Fixture regeneration

The checked-in transform fixtures are the release inputs. Recreate them only
with the versions enforced by `scripts/generate-transform-fixtures.sh`, compare
all checksums, rerun the fixture matrix, and review any byte change before
committing it.

## Backup and restore

The Git repository plus immutable releases are the complete current state.
Restore by cloning, verifying tags/checksums, and building with `--locked`.
There is no database or customer data backup.

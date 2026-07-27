# Release and publication

The source repository is public at
https://github.com/borborich/provenance-ci. No immutable GitHub release,
package, Action Marketplace listing, or website has been published.

## Build release artifact

```sh
./scripts/check-licenses.py
./scripts/release-smoke.sh
./scripts/package-release.sh
```

Artifacts are written under `dist/` with a sibling SHA-256 file. The current
package is platform-specific and contains the binary, schemas, README,
changelog, security policy, licenses, notices, lockfile, and license inventory.
The packager normalizes archive ordering, ownership, modes, timestamps, and
gzip metadata. Two consecutive local packages produced the same checksum.

Current local artifact:

```text
64a4ad03647314811397909a9631ba5736c3302d27349531b0987f7c2579a7ee  provenance-ci-v0.1.0-aarch64-apple-darwin.tar.gz
```

## Pre-publication checklist

- [ ] Owner approves external publication and account/legal actions.
- [ ] Working name checked for trademarks/existing products.
- [ ] No C2PA/CAI/Adobe logo or implied affiliation.
- [ ] No “certified,” “compliant,” or “conformant” claim.
- [ ] Public repository history contains no secrets or private data.
- [ ] Version, SDK, c2patool, Action dependency SHAs, schemas, and docs agree.
- [ ] Format, clippy, tests, Action integration, differential test, license
      inventory, release smoke, and clean-checkout build pass.
- [ ] Release archive/checksum and third-party notices reviewed.
- [ ] GitHub 2FA, security policy, private vulnerability reporting, branch
      protection, dependency updates, and code scanning configured.
- [ ] Immutable patch release/tag created; any movable major alias managed
      separately.
- [ ] `action.yml` remains at repository root with unique truthful name,
      description, branding from the allowed Feather/color set, and
      `contents: read` quickstart.
- [ ] Marketplace listing explicitly says assets stay on runner, documents
      limitations, support, privacy, and exit codes.
- [ ] Do not transfer the GitHub App installation thresholds or paid-App
      billing rules to an Action listing.

GitHub publication references:

- https://docs.github.com/en/actions/how-tos/create-and-publish-actions/publish-in-github-marketplace
- https://docs.github.com/en/actions/how-tos/create-and-publish-actions/release-and-maintain-actions
- https://docs.github.com/en/actions/reference/security/secure-use

Publication itself requires explicit owner approval.

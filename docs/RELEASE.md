# Release and publication

The source repository is public at
https://github.com/borborich/provenance-ci. The immutable `v0.1.0` release is
published at https://github.com/borborich/provenance-ci/releases/tag/v0.1.0.
The `v0.1.1` Marketplace patch is prepared but not yet published. No package
registry, Action Marketplace listing, or website has been published.

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
e8aa1e5fca21fd815457264cb6410355d68d5c1b1548d88aed62bbce544284f2  provenance-ci-v0.1.1-aarch64-apple-darwin.tar.gz
```

## Pre-publication checklist

- [x] Owner approves this GitHub Release publication.
- [x] Preliminary exact-name/product search completed; the working-name caveat
      remains because this is not legal trademark clearance.
- [x] No C2PA/CAI/Adobe logo or implied affiliation.
- [x] No “certified,” “compliant,” or “conformant” claim.
- [x] Public repository history contains no detected secrets or private data.
- [x] Version, SDK, c2patool, Action dependency SHAs, schemas, and docs agree.
- [x] Format, clippy, tests, Action integration, differential test, license
      inventory, release smoke, and clean-checkout build pass.
- [x] Release archive/checksum and third-party notices reviewed.
- [x] Security policy, private vulnerability reporting, branch protection,
      dependency updates, secret scanning push protection, and code scanning
      configured.
- [ ] Owner completes GitHub 2FA and accepts the Marketplace Developer
      Agreement.
- [ ] Immutable `v0.1.1` release/tag and Marketplace listing published; any
      movable major alias managed separately.
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

Action Marketplace publication requires the owner to personally accept GitHub's
legal terms and complete two-factor confirmation.

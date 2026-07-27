# Research

Research date: 2026-07-26. All mutable facts below were rechecked from primary
sources on that date.

## Versions and official sources

- C2PA Technical Specification **2.4**, April 2026:
  https://spec.c2pa.org/specifications/specifications/2.4/specs/C2PA_Specification.html
- C2PA 2.4 specification index:
  https://spec.c2pa.org/specifications/specifications/2.4/index.html
- Official Rust SDK `c2pa 0.90.3`, released 2026-07-24:
  https://crates.io/crates/c2pa/0.90.3 and
  https://github.com/contentauth/c2pa-rs/tree/c2pa-v0.90.3
- Official reference CLI `c2patool 0.27.3`, released 2026-07-24:
  https://github.com/contentauth/c2pa-rs/releases/tag/c2patool-v0.27.3
- Rust SDK usage and release notes:
  https://opensource.contentauthenticity.org/docs/rust-sdk/docs/usage/ and
  https://opensource.contentauthenticity.org/docs/rust-sdk/docs/release-notes/
- C2PA conformance program and live lists:
  https://c2pa.org/conformance/
- Public trust-list repository:
  https://github.com/c2pa-org/conformance-public/tree/main/trust-list
- GitHub Action metadata and publication:
  https://docs.github.com/en/actions/reference/workflows-and-actions/metadata-syntax
  and
  https://docs.github.com/en/actions/how-tos/create-and-publish-actions/publish-in-github-marketplace

The SDK is beta/pre-1.0, supports C2PA v2 claims by default, requires Rust
1.88+, and implements a subset of the specification. Exact pinning and the
adapter boundary are therefore product requirements.

## Reliable state model

### Presence

- `present`: an embedded store or declared remote reference was detected.
- `absent`: the official SDK returned `JumbfNotFound` for validated JPEG bytes.
- `unreadable`: data resembling a credential could not be parsed.
- `unknown`: acquisition or format evaluation did not complete.

### Cryptographic validation

The current SDK exposes `Invalid`, `Valid`, and `Trusted`.

- `valid`: SDK `Valid` or `Trusted`.
- `invalid`: SDK completed validation and returned `Invalid`; raw failure codes
  remain authoritative evidence.
- `indeterminate`: parsing, remote retrieval, security policy, or another limit
  prevented a complete answer.
- `not_applicable`: credential absence is established.

`Invalid` can cover structural and cryptographic failures, so the normalized
explanation includes raw codes rather than inventing one generic cause.

### Trust

C2PA's nesting is Well-Formed ⊃ Valid ⊃ Trusted. Trust adds a certificate chain
to configured anchors; it does not establish truth. The SDK requires trust
anchors to be supplied explicitly.

- `trusted`: SDK `Trusted` against the recorded PEM snapshot.
- `untrusted`: cryptographically valid, but not chained to that snapshot.
- `not_checked`: no snapshot supplied.
- `unknown`: trust evaluation could not complete.

On 2026-07-26 the official C2PA Trust List PEM snapshot observed in the public
repository had SHA-256
`b1f399a7235f188a22f3db97992f1cc1417517664600335f9d105a6a7cdb46c1`
at upstream commit `8a130b0f01140fd3bb7e6eca7fb7de8676045634`.
This mutable value is research evidence, not bundled runtime state. The Interim
Trust List is frozen for legacy use; it is not silently merged into results.

## Cryptographic validity

The SDK validates claim structure, claim signature and certificate validity
window, assertions, ingredients, and hard binding. Success commonly includes
`claimSignature.validated`, `claimSignature.insideValidity`, and
`assertion.dataHash.match`; hard-binding mutation produces
`assertion.dataHash.mismatch`.

Reproducibility requires exact asset bytes, SDK version/settings, validation
time, trust snapshot digest, and raw status codes. Human explanation strings are
not treated as a stable API.

## Derivative lineage

C2PA 2.4 ingredient v3 is current; v1/v2 are deprecated for generation.
Relationships are `parentOf`, `componentOf`, and `inputTo`.

For the MVP, B is a declared C2PA derivative of A only when:

1. B is cryptographically valid;
2. B has a `parentOf` ingredient referencing A's active manifest; and
3. SDK validation evidence contains `ingredient.manifest.validated` or
   `ingredient.claimSignature.validated` for that reference.

A friendly label match alone is insufficient. Even validated lineage proves a
signed declaration and cryptographic link, not that the described real-world
operation happened or that no other source existed.

With two checkpoints the product can prove byte identity, credential loss
between those observations, an invalid state at the second observation, or
validated declared lineage. It cannot identify a hidden intermediary.

With multiple checkpoints it can localize the first observed adjacent break.
It still cannot name an unsupplied system.

## Remote manifests, external assertions, and soft bindings

C2PA 2.4 uses `application/c2pa` for external manifest stores. Discovery can use
format-specific references, XMP, HTTP Link, or explicit external input.
Unavailable declared remote provenance is not equivalent to absence.

Unrestricted SDK fetching is an SSRF and privacy risk. The SDK is compiled
without its remote-fetch feature and configured with
`verify.remote_manifest_fetch=false`. Remote references are therefore
`present + indeterminate + not_evaluated`.

External assertions and icons are not downloaded. Soft Binding API 2.4 defines
vendor algorithms and repository resolution, but the official Rust SDK does
not provide a confirmed universal resolver for every registered algorithm.
Perceptual hashes are not called “C2PA soft bindings.”

## Supported formats

The C2PA specification defines embedding for many image, media, document, and
text formats. That is broader than any one SDK. The SDK currently lists JPEG,
PNG, WebP, GIF, TIFF/DNG, SVG, AVIF/HEIF, JPEG XL, BMFF media, RIFF media,
MP3/FLAC, PDF read-only, and `.c2pa`, among others:
https://github.com/contentauth/c2pa-rs/blob/main/docs/supported-formats.md

The MVP claims JPEG only because only JPEG has passed this project's fixture
matrix.

## Licenses and transfer

`c2pa 0.90.3` and `c2patool 0.27.3` are `MIT OR Apache-2.0`. Those licenses
permit commercial use, modification, distribution, and asset transfer subject
to notices and conditions. They grant no trademark, affiliation, or conformance
rights. The specification text is CC BY 4.0 with the published patent policy.

Commercial transfer is not blocked at the top level. A lockfile license scan,
notices, SBOM/inventory, source modifications, and trademark separation remain
release duties.

## Market and pipeline evidence

The exact workflow is not uncontested. `Rubiss/art50-ci 0.3.0` already offers a
CLI and Action that compare one source with one delivered asset and walk
ingredient ancestry:
https://github.com/Rubiss/art50-ci

It does not expose arbitrary ordered checkpoints, adjacent-pair classification,
or first-observed-break localization. That narrow wedge remains.

Official platform documentation supports the regression hypothesis:

- WordPress image editors generate derivatives and generally strip most
  metadata:
  https://developer.wordpress.org/reference/classes/wp_image_editor_imagick/thumbnail_image/
- Cloudflare now has an explicit preserve-Content-Credentials mode that can
  create a signed derivative:
  https://developers.cloudflare.com/images/optimization/transformations/preserve-content-credentials/
- Cloudinary transformations normally strip most metadata, while `fl_c2pa`
  can sign delivery transformations:
  https://cloudinary.com/documentation/content_provenance_and_authenticity

These are vendor claims until reproduced in controlled fixtures. They show why
byte inequality cannot be equated with breakage.

## Answers to the feasibility questions

1. Presence, SDK validation state, explicit trust, byte identity, validated
   parent lineage, policy, and acquisition errors are reliable within scope.
2. The SDK currently distinguishes `Invalid`, `Valid`, and `Trusted` and returns
   structured success/informational/failure status sets.
3. Pin SDK/settings/bytes/time/trust digest; retain raw status codes; compare to
   pinned c2patool fixtures.
4. Supply an explicit PEM snapshot and keep trust separate from validity.
5. Require a valid current manifest, `parentOf`, and validated ingredient
   manifest/signature evidence.
6. The SDK returns relationship, active-manifest reference, manifest data,
   validation results/statuses, hashes, and related metadata.
7. Two checkpoints prove only what is observed at those two endpoints.
8. Multiple checkpoints localize the first observed interval, not the hidden
   responsible component.
9. Do not permit SDK network access; use a separate hardened resolver in a
   future opt-in feature.
10. `art50-ci` is a direct adjacent competitor, but not the exact N-checkpoint
    workflow.
11. Top-level licenses permit commercial use and transfer with notices.
12. Safe short-term scope is the implemented local JPEG CLI + Action; remote
    manifests and soft bindings remain out.


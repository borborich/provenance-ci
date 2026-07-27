# Competitors

Reviewed: 2026-07-26. Product capabilities are based on each vendor's official
documentation and are not independently certified here.

| Product | Confirmed role | Gap relative to this MVP |
|---|---|---|
| [art50-ci 0.3.0](https://github.com/Rubiss/art50-ci) | CLI + GitHub Action; source-to-delivered C2PA presence/validation and ingredient-ancestry check | Closest competitor. No arbitrary N checkpoints, adjacent classifications, or first-observed-break localization |
| [CAI Inspect](https://opensource.contentauthenticity.org/docs/getting-started/inspect/) | Manual single file/URL inspection and history display | No pipeline CI or ordered policy |
| [c2patool 0.27.3](https://github.com/contentauth/c2pa-rs/releases/tag/c2patool-v0.27.3) | Official single-asset read/sign/validate CLI | Reference oracle, not multi-checkpoint orchestration |
| [Truepic Lens API](https://vision.truepic.dev/docs/lens-api) | Hosted capture/inspection API and webhooks | Vendor service, not local vendor-neutral publishing checkpoints |
| [Trufo API](https://trufo.ai/documentation) | Hosted C2PA validation and certificate endpoints | Single-content API |
| [DigiCert Content Trust Manager](https://docs.digicert.com/en/content-trust-manager/sign-media.html) | Signing, verification, and credential persistence | Upstream trust/signing system rather than external pipeline regression test |
| [Cloudflare Images](https://developers.cloudflare.com/images/optimization/transformations/preserve-content-credentials/) | Vendor-specific transformation preservation/re-signing | Covers its own hop, not an arbitrary pipeline |
| [Cloudinary C2PA](https://cloudinary.com/documentation/content_provenance_and_authenticity) | Vendor-specific signed transformations | Covers its own delivery layer |

## Decision

The existence of `art50-ci` prevents a broad claim that “C2PA CI does not
exist.” It does not trigger KILL because the ordered N-checkpoint and
first-break workflow remains meaningfully distinct.

The product must not compete on generic “scan a C2PA file.” The narrow promise
is:

> Locate the first observed credential or continuity regression between
> user-supplied publishing checkpoints, with evidence for every adjacent pair.

## Monitoring

Recheck before publication and monthly thereafter:

- `art50-ci` checkpoint support and trust model;
- official `c2patool` automation/report features;
- Cloudflare and Cloudinary delivery signing;
- C2PA conformance product list;
- GitHub Marketplace searches for Content Credentials and C2PA.

If a maintained free tool adds equivalent N-checkpoint classification and
first-break evidence, reassess the KILL gate before investing in hosted
features.


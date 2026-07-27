# ADR 0002: Narrow remote and lineage behavior

Status: accepted  
Date: 2026-07-26

## Context

Remote manifests, external assertions, icons, ingredients, OCSP endpoints, and
soft-binding repositories can cause uncontrolled outbound requests. A valid
changed asset may be a legitimate signed derivative. Label equality alone does
not validate lineage.

## Decision

- Disable all SDK-controlled network retrieval.
- Report a declared remote manifest as present but not evaluated and
  cryptographically indeterminate.
- Fetch only the user-declared top-level HTTPS asset through the hardened
  acquisition layer.
- Require a valid current manifest, `parentOf`, a matching prior active
  manifest reference, and ingredient validation success before reporting
  `declared_c2pa_derivative`.
- Report uncertainty instead of using perceptual similarity.

## Consequences

The MVP has lower SSRF/privacy risk and honest semantics, but cannot validate
remote-only credentials or recover stripped credentials via soft binding.


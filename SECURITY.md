# Security policy

## Supported version

Only the current `0.1.x` source and exact release artifacts are supported. The
project is not yet publicly released.

## Reporting

Do not open a public issue containing a malicious media sample, private URL,
credential, token, personal data, or unpublished vulnerability. After a public
repository exists, configure GitHub private vulnerability reporting and replace
this paragraph with the repository's private reporting link.

Until then, report privately to the repository owner through an already agreed
channel. Do not send raw customer assets unless explicitly requested through a
secure transfer mechanism.

## Boundaries

- Media and manifests are untrusted input.
- The CLI does not accept signing keys.
- SDK remote fetch, OCSP fetch, soft-binding recovery, and external assertion
  retrieval are disabled.
- URL checkpoints are public HTTPS/443 only and apply DNS/IP, redirect, size,
  timeout, and MIME controls. Application checks are defense in depth; CI
  runners should still use egress restrictions for hostile inputs.
- No asset is uploaded to a project-controlled service.

See `docs/THREAT_MODEL.md` and `docs/SECURITY.md` for design details.


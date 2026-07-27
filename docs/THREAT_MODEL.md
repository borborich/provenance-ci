# Threat model

Reviewed: 2026-07-26.

## Assets to protect

- runner credentials and network reachability;
- repository files and build outputs;
- private/signed URL query tokens;
- media metadata, identities, GPS/EXIF, thumbnails, and internal references;
- result integrity and policy exit semantics;
- build/release supply chain.

## Trust boundaries

```text
untrusted config/media/URL
  -> acquisition boundary
  -> untrusted media parser and official SDK
  -> normalized evidence
  -> CI annotations/artifact
```

The trust list and Action/source release are operator-controlled inputs. A
manifest signer is not trusted merely because a signature validates.

## Threats and controls

| Threat | Control | Residual risk |
|---|---|---|
| SSRF through checkpoint URL | HTTPS/443 only; no credentials; ignore proxy environment; resolve A/AAAA; reject any private, loopback, link-local, multicast, documentation, carrier NAT, benchmark, reserved, and IPv4-mapped IPv6 result; pin a vetted IP; repeat for every redirect; five redirects | DNS/TLS/OS resolver and HTTP-stack vulnerabilities; use runner egress controls for hostile inputs |
| SSRF through manifest/assertion/icon/OCSP | SDK remote, OCSP, external assertion, ingredient resource, and soft-binding fetch disabled | Remote-only credentials remain indeterminate |
| DNS rebinding | Resolve once per hop and pin the HTTP client to a vetted address | A compromised resolver/host can still serve hostile bytes |
| Oversized/decompression input | Content-Length precheck, streaming 25 MiB cap, request timeout, JPEG sniffing | Parser CPU complexity inside capped bytes |
| Pixel/decompression bomb | MVP does not decode pixels; it parses JPEG headers and C2PA structures only | SDK JPEG/JUMBF parsing remains untrusted code |
| HTML/MIME spoof | Byte sniffing; reject HTML; only JPEG enters adapter | Polyglot files require ongoing fixture coverage |
| Malformed C2PA/JUMBF | Official SDK only; Rust memory safety; no custom crypto; malformed fixtures | SDK bugs and algorithmic denial of service |
| Secret leakage in URL | Persist URL without query/fragment; no full manifest or EXIF logging | Host/path can still be sensitive; users should avoid private targets |
| GitHub expression/script injection | Inputs passed by environment and quoted arrays; no direct attacker-controlled expression in shell commands | Fork-controlled repository code is still code; do not grant secrets |
| Untrusted PR with secrets | Minimal permissions; no `pull_request_target`; sample has no secrets | Caller can create a less safe workflow |
| Supply-chain substitution | Exact SDK pin, `Cargo.lock`, full commit SHA for upload Action, release checksums, license inventory | Registry/GitHub compromise; add signed provenance after publication |
| Result ambiguity | Independent states, raw codes, limitations, stable schema and exits | Users may ignore limitations |
| Temp-file residue | Assets remain in memory; the current core creates no media temp files | OS crash dumps/swap are outside application control |

## Out of-scope hostile environments

The application cannot make a general-purpose shared runner safe against
arbitrary unreviewed code. Private-origin access, repository secrets, or broad
network reach must not be granted to fork workflows merely because this URL
fetcher applies SSRF checks.

## Hosted scanner gate

A hosted scanner is not implemented. Before one exists it additionally needs:

- network-namespace egress enforcement and a separate safe fetch gateway;
- non-root sandbox, read-only filesystem, temp quota, CPU/RAM/wall limits;
- resolver/connection IP binding proven under redirects and rebinding tests;
- port/scheme restrictions, decompression/pixel limits, rate limits, abuse
  handling, telemetry minimization, deletion operations, and incident response;
- separate remote-manifest policy with the same network controls.

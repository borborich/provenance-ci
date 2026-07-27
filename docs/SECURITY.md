# Security design

## Safe defaults

- Local runner execution; no project-operated backend.
- No signing or private-key input.
- User assets are never uploaded by the product.
- Official C2PA parser/validator; no custom cryptography.
- SDK-controlled HTTP, remote manifest, external resource, soft-binding, and
  OCSP retrieval disabled.
- JPEG only after byte sniffing.
- Public HTTPS/443 URL checkpoints only.
- Exact direct dependency pins and locked transitive graph.

## URL acquisition

For each initial URL and redirect:

1. require `https`, port 443, hostname, and no URL credentials;
2. resolve all addresses;
3. reject the hostname if any address is non-public/reserved;
4. pin the client connection to one vetted address while retaining the hostname
   for TLS verification;
5. ignore proxy environment so it cannot bypass the vetted address;
6. disable automatic redirects and repeat the process explicitly;
7. cap redirects at five, connection at five seconds, request at twenty
   seconds, and body at 25 MiB;
8. reject HTML and pass only sniffed JPEG to the SDK.

Only rendition-relevant response headers are stored. Cookies, authorization,
and arbitrary headers are not stored.

## Parser boundary

The adapter receives immutable bytes and a local trust snapshot. It disables
SDK networking, uses `Reader::with_stream` so sibling sidecars are not
implicitly discovered, captures structured validation results, and catches the
SDK's non-exhaustive error type.

## CI boundary

The Action needs `contents: read`. It passes input via environment variables to
a fixed script, quotes every path, does not evaluate shell supplied by config,
uploads the JSON artifact before returning a policy failure, and pins
`actions/upload-artifact` to a full commit.

The caller must check out its own repository. A fork workflow receives no
special secrets or private-origin access from this Action.

## Release controls

Before release run formatting, clippy, tests, Action integration, c2patool
differential tests, license inventory, locked release build, smoke test, archive
checksum, secret scan, and review of GitHub Action dependency SHAs.

See the root `SECURITY.md` for vulnerability reporting.

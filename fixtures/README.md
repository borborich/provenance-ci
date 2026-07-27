# Fixture corpus

`C.jpg`, `CA.jpg`, `XCA.jpg`, `no_manifest.jpg`, `remote_manifest.jpg`, and
the PEM files in `official/` are copied without modification from
`contentauth/c2pa-rs` tag `c2pa-v0.90.3`, commit
`79c2eae817bd674189066ca716242436d006760e`. `declared_derivative.jpg` was
generated from `C.jpg` with pinned official `c2patool 0.27.3`, Edit intent,
and its development signer.

They are distributed under the upstream `MIT OR Apache-2.0` license. The
upstream license texts are represented in this project by
`THIRD_PARTY_NOTICES.md` and linked from
`docs/DEPENDENCIES_AND_LICENSES.md`.

| File | Upstream role | Expected normalized result |
|---|---|---|
| `C.jpg` | Signed source asset | present, cryptographically valid, test signer untrusted unless a test anchor is configured |
| `CA.jpg` | Signed asset with ingredient history | present, cryptographically valid |
| `declared_derivative.jpg` | Generated from `C.jpg` by pinned `c2patool 0.27.3` in Edit intent using its development signer | present, cryptographically valid, validated `parentOf` lineage to `C.jpg` |
| `XCA.jpg` | Pixel/content bytes changed while the old manifest remains | present, cryptographically invalid with `assertion.dataHash.mismatch` |
| `no_manifest.jpg` | JPEG without a manifest | absent, validation not applicable |
| `remote_manifest.jpg` | JPEG referring to a remote manifest | present, remote manifest not evaluated, validation indeterminate |
| `test-trust-anchors.pem` | Upstream SDK development trust roots | test-only roots for exercising `trusted`; never a production trust list |
| `nonmatching-test-anchor.pem` | Upstream CLI non-matching test root | test-only root for exercising `untrusted` independently of cryptographic validity |

Checksums:

```text
a2d14755db55de67a47c04090340d8266e892367be4104a45626d7a6fa6e9ffd  C.jpg
e71bff58fc57640803e6e65f7534e2fb0c2f99018c85276cc14b30f04427cc76  CA.jpg
02218055e841fe1b3ea0142f925d8c268e504afc51f923416b15734fb22d847b  declared_derivative.jpg
81f20702a2e81611b29aa773f811b59ade6095acd3d089d6ffdae6b591d6d5a5  XCA.jpg
9ac395ca04fc9d348acf6f81920f5e894d336341c3f764ca86c354eec6f7c2d6  no_manifest.jpg
e6ad9f51be5bb83f137322a3f260b5881738e3528aee208727e75b6daaba6092  remote_manifest.jpg
c67864bf6bae91c7df0abbef274abd1b03116063093ec225a8391d744f8356fc  test-trust-anchors.pem
7308d8dd5473ee3b2c451d037a58b092c7383ac329ca650806bf16419e09355a  nonmatching-test-anchor.pem
```

Malformed and truncated media are generated in tests from these checked-in
bytes so that no additional opaque fixture is required. Network and fetch
errors are tested without contacting external services.

## Transformation matrix

The files in `transforms/` are deterministic transformations of `C.jpg`.
`scripts/generate-transform-fixtures.sh` pins and checks FFmpeg `7.1.1` and
libjpeg-turbo `jpegtran 3.2.0` before recreating them. These tools are fixture
generators, not runtime dependencies.

| File | Transformation | Expected result |
|---|---|---|
| `metadata-stripped.jpg` | Lossless JPEG coefficient copy with all extra markers removed | absent, credentials lost |
| `reencoded.jpg` | JPEG pixel decode and encode | absent, credentials lost |
| `resized.jpg` | Resize from 2048×1365 to 1024×682 | absent, credentials lost |
| `cropped.jpg` | Center crop to 1024×1024 | absent, credentials lost |
| `orientation-normalized.jpg` | Clockwise pixel rotation simulating an orientation rewrite | absent, credentials lost |

Checksums:

```text
31d040843d45bcd6253e6aba73d7223bfb8bfe928e35b4dfe629a14eda7e3d5d  cropped.jpg
db9ede5d4dc93d1bcba1f4f7a89ff271981380b43719b0cc4676b08e88823fe0  metadata-stripped.jpg
71c34b4e8c05e43797c7d2d7f25a759da6eab31a6ed3a707f33ea5835fcb67f7  orientation-normalized.jpg
ef219b5e6fd9550a8a046a53bc99eb1d163022d8dce2ac96cbda7d57dd6b82aa  reencoded.jpg
f6de87f2f2346d4e1c9a13a717521096fa7825e3f94dd3ff1604cf71f8911ae3  resized.jpg
```

# Feasibility decision

Decision date: 2026-07-26  
Gate: **NARROW**

## Demonstrated

- Official SDK `0.90.3` compiles in a portable Rust 1.88+ runtime with automatic
  network retrieval disabled.
- The adapter reliably separates absent, present/valid, present/invalid,
  trusted, untrusted, unsupported, unreadable, remote-not-evaluated, and
  acquisition error states in the checked fixture set.
- A pixel/content mutation retaining the old manifest produces
  `assertion.dataHash.mismatch` and normalized `invalid`.
- Test roots demonstrate that cryptographic validity and trust are independent.
- A c2patool-generated `parentOf` derivative produces validated ingredient
  evidence and is distinguished from merely changed signed content.
- Fixed-input repeat runs are byte-for-byte deterministic when the evidence
  timestamp is fixed.
- The transformation matrix covers metadata stripping, JPEG re-encode, resize,
  crop, and orientation rewrite; each unsigned output loses credentials.
- SDK results agree in substance with pinned c2patool `0.27.3` for valid and
  invalid-hard-binding fixtures.
- WordPress/CDN/image-platform documentation confirms plausible metadata-loss
  and legitimate re-signing paths.
- Top-level licenses permit commercial transfer.
- A competitor exists, but the N-checkpoint first-break wedge remains.

## Performance and memory

On the local `aarch64-apple-darwin` feasibility host (Rust `1.93.1`), one warm
optimized run over the two-checkpoint 132,518-byte + 175,647-byte declared
lineage fixture completed in `0.01 s` wall time. This is a smoke measurement,
not a benchmark or cross-platform guarantee. The stripped release binary is
11,238,032 bytes.

Acquisition is capped at 25 MiB per asset and the runner processes checkpoints
sequentially. Asset bytes remain in memory only for the active checkpoint;
there is no decoded-pixel buffer and a subprocess test confirms that a custom
temporary directory remains empty. SDK parser structures add input-dependent
overhead, so peak RSS is not claimed or guaranteed from this single host.

## Not demonstrated

- Universal soft-binding resolution.
- Safe arbitrary remote manifest/external assertion retrieval.
- Visual similarity as provenance evidence.
- Fixture parity for PNG, WebP, video, audio, PDF, or text.
- A hosted scanner security model under real abuse.
- Third-party demand, installs, retention, revenue, or buyer interest.
- Official C2PA conformance.
- A controlled cross-platform benchmark and measured peak-RSS envelope for
  adversarial 25 MiB manifests.

## Why NARROW

The local JPEG workflow is technically useful and honest, but a broad
“provenance continuity platform” would overstate what can be proved. Remote
manifests and soft bindings materially enlarge both security scope and semantic
uncertainty. They remain `indeterminate` instead of being guessed.

## Scope authorized by this gate

```text
local JPEG checkpoints
+ hardened public HTTPS asset retrieval
+ credential-state regression
+ exact bytes or validated declared parent lineage
+ explicit trust snapshot
+ CLI and GitHub Action
```

## Gate reassessment

Move toward GO only after:

- at least one third-party pipeline repeatedly uses the Action;
- inconclusive rate is acceptable;
- a separate safe remote-manifest resolver passes SSRF and privacy tests; or
- users demonstrate that embedded/local checks alone solve a retained job.

Move to KILL if an equivalent maintained free N-checkpoint tool erases the
wedge, the SDK cannot remain reproducible across updates, or published usage
misses the product kill gate.

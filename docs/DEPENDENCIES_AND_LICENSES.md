# Dependencies and licenses

Reviewed: 2026-07-26. This is engineering documentation, not legal advice.

## Critical dependencies

| Dependency | Pin | Role | License |
|---|---:|---|---|
| `c2pa` | `0.90.3` | official parsing, validation, ingredients, trust | MIT OR Apache-2.0 |
| `c2patool` | `0.27.3` external oracle | differential fixture validation | MIT OR Apache-2.0 |
| `reqwest` | `0.12.23` | top-level HTTPS fetch with rustls/native roots | MIT OR Apache-2.0 |
| `serde` / `serde_json` / `serde_yaml` | exact pins | versioned data/config | MIT OR Apache-2.0 |
| `clap` | `4.5.48` | CLI | MIT OR Apache-2.0 |
| `sha2` | `0.10.9` | asset and trust-list evidence digest | MIT OR Apache-2.0 |
| `chrono` | `0.4.45` | evidence timestamps | MIT OR Apache-2.0 |
| `actions/upload-artifact` | commit `043fb46d…` (`v7.0.1`) | JSON Action artifact | MIT |

All direct Rust dependencies are exact-pinned; `Cargo.lock` pins the complete
graph. The SDK crate checksum is
`ac87e7d7aac404cef1cd509f3b14964e7acf37f644f0bfc8bb855a7a428d522d`.

## Compliance process

`scripts/check-licenses.py` reads locked Cargo metadata for all platforms,
fails on missing license metadata or a configured forbidden
strong-copyleft/server-side license, and writes a machine-readable inventory
for release. The current inventory contains 382 transitive packages. A clean
machine may download the exact Cargo-locked sources needed to read their
metadata. Manually review compound expressions and non-code assets before each
public release.

Retain:

- `LICENSE-MIT`, `LICENSE-APACHE`, and `THIRD_PARTY_NOTICES.md`;
- Cargo source checksums and release artifact checksums;
- upstream attribution/notices;
- an indication of any modified upstream source (none is vendored now);
- exact Action dependency commits.

No root `NOTICE` file was present in the pinned SDK tag. Recheck on update.

## Transfer conclusion

Top-level licenses permit commercial use, distribution, modification, and
transfer. Apache-2.0 includes an explicit patent grant and patent-litigation
termination; neither license grants trademarks. Conformance and brand use are
separate from code licensing.

## Update policy

Because the SDK is pre-1.0, update it only in a dedicated change that reruns the
entire fixture matrix, differential oracle, trust tests, Action integration,
license inventory, and release smoke. Never use an unpinned `latest`.

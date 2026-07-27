# ADR 0001: Rust SDK behind an adapter

Status: accepted  
Date: 2026-07-26

## Context

The product requires C2PA parsing, signature verification, hard-binding
validation, ingredients, strict types, portability, and commercial
transferability. Implementing cryptography or JUMBF parsing locally is unsafe.
The official Rust SDK is comprehensive but pre-1.0 and changes API on minor
release trains.

## Decision

Use exact-pinned `c2pa 0.90.3` with Rust-native crypto and no SDK HTTP/remote
manifest feature. All official API calls live in `c2pa_adapter.rs`. Use pinned
`c2patool 0.27.3` only as a differential reference oracle.

## Consequences

- One native binary and strong compile-time types.
- No OpenSSL runtime dependency in the selected feature set.
- Breaking SDK upgrades are isolated but still require fixture/differential
  testing.
- Compile time is higher than a prebuilt JavaScript wrapper.
- The project cannot claim full specification conformance.
- The release target is a platform-specific native binary or a source build on
  the GitHub runner. The first local artifact is
  `aarch64-apple-darwin`; the included workflow is the pending
  `x86_64-unknown-linux-gnu` portability gate.
- No Docker image is required for the local/Action MVP. A container would add
  distribution and patching work without isolating hostile inputs on its own.

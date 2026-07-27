# Architecture

## Runtime

A single Rust binary and a composite GitHub Action. There is no server,
database, queue, account system, signing service, or telemetry endpoint.

```text
versioned YAML
  -> config validation
  -> checkpoint acquisition (path | hardened HTTPS)
  -> byte SHA-256 + JPEG properties
  -> official c2pa SDK adapter (network disabled)
  -> normalized independent states
  -> adjacent checkpoint comparison
  -> policy engine
  -> JSON + human + Markdown + annotations
```

## Components

- `config.rs`: strict, versioned YAML parsing and semantic checks.
- `fetch.rs`: local byte reads and public HTTPS acquisition with URL
  redaction, DNS/IP validation, pinned resolution, proxy bypass, redirect
  revalidation, streaming limit, timeout, and MIME sniffing.
- `c2pa_adapter.rs`: the only direct dependency on official SDK APIs. It
  configures trust and normalizes errors/statuses.
- `policy.rs`: adjacent classification and policy evaluation.
- `model.rs`: stable serialized result types.
- `runner.rs`: orchestration without shell execution or persistent media.
- `report.rs`: terminal, Markdown, and GitHub workflow command rendering.
- `main.rs`: CLI boundary and stable exits.

## State semantics

`presence`, `cryptographicValidation`, `trust`, and
`relationshipToPreviousCheckpoint` never collapse into one “authentic” flag.

`declared_c2pa_derivative` requires:

- current cryptographic state `valid`;
- a `parentOf` reference to the prior active manifest; and
- an SDK success code validating that ingredient manifest/signature.

Otherwise, changed valid credentials are
`changed_without_verifiable_lineage`. The product makes no visual-similarity or
causal claim.

## Determinism

- Direct dependencies and the official SDK are exact-pinned; transitive
  dependencies are locked.
- Checkpoints execute in declared order.
- Maps serialize in stable key order where order matters.
- Validation codes are sorted and deduplicated.
- A single timestamp is used for all checkpoints in a run.
- `--checked-at` fixes evidence time for tests and reproducible fixtures.
- The release packager normalizes tar ordering, ownership, modes, timestamps,
  and gzip metadata; two consecutive packages must produce the same checksum.

## Trust

The runtime never downloads a trust list. A configured PEM file is read once,
hashed, and used for the run. The ID and SHA-256 are recorded. Operational trust
list refresh is an explicit reviewed input change.

## GitHub Action

The root composite Action passes input through environment variables to a fixed
script, builds with `--locked` or invokes an explicitly supplied executable,
writes the Job Summary, emits annotations, uploads JSON using a full-SHA-pinned
official action, and only then returns the validator's stable exit code.

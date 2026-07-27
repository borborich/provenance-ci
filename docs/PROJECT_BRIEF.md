# Project brief

Last reviewed: 2026-07-26

## Objective

Build a minimal, safe, self-service developer tool for deterministic Content Credentials regression testing across ordered publishing checkpoints:

```text
source asset -> build output -> origin -> CDN -> published URL
```

The first release consists of a local validation core, an English-language CLI, and a GitHub Action. It needs no account, SaaS backend, secrets, or external upload.

## User and job

The primary user is a web developer, content engineer, publishing-pipeline maintainer, WordPress developer, or small publishing team already using Content Credentials.

The job is:

> Given an ordered set of user-supplied checkpoints, identify the first observed transition where credentials disappear, become invalid, or no longer have verifiable declared lineage, and emit reproducible evidence suitable for CI.

## Product constraints

- Presence, cryptographic validity, trust, relationship, and policy are independent.
- Two endpoints cannot identify which hidden component caused a break.
- A changed rendition may legitimately carry a new valid manifest and a declared parent ingredient.
- A stale manifest whose hard binding no longer matches the pixels is invalid, not preserved.
- A valid manifest does not prove that content is true, unmanipulated, human-created, correctly copyrighted, or produced by a trustworthy person or organization.
- The product is not C2PA certified, conformant, compliant, affiliated, or endorsed.

## First iteration

Input is a versioned YAML file containing checks, ordered local-path or public-HTTPS checkpoints, and policy. Initial format support is JPEG only.

Output includes:

- versioned JSON;
- human-readable terminal output;
- Markdown suitable for GitHub Job Summary;
- GitHub workflow annotations;
- stable exit codes;
- facts, evidence, limitations, and explicitly labelled inferences.

## Decision gates

Phase 0 ends in `GO`, `NARROW`, or `KILL`. A `GO` or `NARROW` decision authorizes implementation of the CLI and Action without another owner decision. A hosted scanner, publishing, marketplace listing, package release, domain purchase, payment, or external legal acceptance remains out of scope without explicit approval.

## Success evidence

Current local testing and project CI are synthetic and must never be reported as user adoption. Post-publication evidence is third-party installs, successful checks, repeat use, active repositories/sites, retained integrations, organic acquisition, support cost, and paid conversion.

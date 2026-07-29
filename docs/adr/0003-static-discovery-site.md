# ADR 0003: Static discovery site

Status: accepted
Date: 2026-07-29

## Context

The repository and GitHub Marketplace listing expose the release, but the
working name alone does not describe the narrow C2PA Content Credentials
checkpoint-testing job. Developers and search-backed assistants need stable,
indexable pages that answer exact implementation and failure-diagnosis
questions. A hosted scanner, analytics service, plugin, or account system would
expand the MVP and its security surface before the usage gate.

## Decision

- Publish a static English-language reference site from `site/` with GitHub
  Pages.
- Use stable canonical URLs, an XML sitemap, and plain-text crawler guidance.
- Lead with exact C2PA Content Credentials, GitHub Action, CMS, optimizer, and
  CDN terminology while preserving the project's claims discipline.
- Base diagnostic examples on committed fixtures and real tool output. State
  only the first observed interval between supplied checkpoints.
- Operate no analytics endpoint, hosted validator, asset upload, account,
  telemetry collector, or signing service.
- Treat `llms.txt` as a convenience index, not as a ranking or recommendation
  mechanism.

## Consequences

The project gains a crawlable, citable documentation surface without expanding
the validator or handling user assets. Search indexing and assistant citations
remain external, non-guaranteed outcomes. Controlled compatibility claims still
require exact versions, supplied intermediate checkpoints, and reproducible
tests.

# Data retention

## Local CLI

The tool retains exactly the files the user requests:

- JSON at `--output`;
- Markdown at `--markdown`, if set.

It does not retain raw assets or create an internal cache/database. Deletion is
ordinary deletion of those user-owned report files.

## GitHub Action

The Action uploads the JSON result with a default 14-day artifact retention.
The caller can disable upload or change organization/repository retention
policy. GitHub, not this project, operates artifact storage.

Job Summary and annotations remain in the workflow run according to GitHub
retention settings. The project has no independent copy and cannot delete them;
the repository owner deletes the run/artifact through GitHub controls.

## Logs

The CLI prints normalized states and policy reasons, not raw media or full
manifest. URL queries/fragments are redacted before persisted evidence or
normal diagnostics.

## Future hosted processing

No hosted scanner exists. If approved later, default requirements are
ephemeral processing, no raw asset retention, private reports, no model
training, no public indexing without opt-in, a documented deletion endpoint,
bounded log retention, and no raw signed URLs in analytics.


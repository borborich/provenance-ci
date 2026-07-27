# Metrics and event specification

No user, install, retention, revenue, or buyer-interest metric has been
observed. Local tests, project CI, bots, health checks, and synthetic scans are
excluded from product usage.

## Privacy-minimized events after publication

Events should be derived from Action/CLI opt-in telemetry or aggregate
Marketplace/public repository signals. Do not transmit raw asset bytes, full
URLs, manifest contents, path names, identities, or query strings.

| Event | When | Allowed properties |
|---|---|---|
| `check_started` | third-party run begins | tool version, source channel, checkpoint count, repository pseudonym if consented |
| `checkpoint_completed` | one checkpoint normalizes | media type, source kind, presence, crypto, trust checked boolean, relationship, duration bucket |
| `check_completed` | result emitted | policy verdict, exit class, checkpoint count, duration bucket, inconclusive reason class |
| `integration_retained` | same consented integration runs in later window | pseudonymous integration ID, 7/30-day window |
| `support_case` | owner handles support | category, minutes, resolved boolean |
| `paid_conversion` | future automatic payment | plan, non-sensitive channel, amount/currency |

Every event needs `schemaVersion`, `occurredAt`, `toolVersion`, `environment`
(`third_party`, `project_ci`, `synthetic`, `bot`), and an explicit telemetry
consent/source. Project-owned environments are filtered before KPI calculation.

## KPIs

- successful first check;
- completed third-party checks;
- active third-party repositories/sites;
- checks per repository;
- 7- and 30-day repeat usage;
- retained integrations/monitors;
- parser failure and inconclusive rates;
- Action installs;
- organic acquisition channel count;
- support minutes and compute cost per check;
- future paid conversion, MRR, churn.

## Continuation gate at 30–45 days after real publication

Continue feature development only with:

- 1,000 real checks/scans;
- 50 active third-party repositories/sites;
- at least 25% repeat usage;
- at least 10 retained integrations/monitors;
- organic use from two channels.

## Kill gate

After a complete public launch in at least three relevant organic channels, if
there are fewer than 100 real users/check consumers or fewer than 10 retained
integrations, stop adding features. One narrow pivot inside checkpoint/
continuity testing is allowed if one segment has materially better retention.

Commercial continuation after payment requires at least one of: five paying
users, $250+ MRR with positive movement, or demonstrated paid conversion among
active users.


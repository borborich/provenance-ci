# Product

## Job to be done

When a developer publishes an image carrying Content Credentials, automatically
test ordered checkpoints and show the first observed transition where
credentials disappear, fail validation, or no longer have verifiable declared
continuity.

## Primary user

A web developer, content engineer, publishing-pipeline maintainer, WordPress
developer, or small team already using Content Credentials and able to add one
CI workflow without procurement, a sales call, an NDA, signing keys, or a SaaS
account.

## Positioning

> CI regression and continuity testing for Content Credentials across
> publishing pipelines.

The differentiator is adjacent comparison over arbitrary ordered checkpoints:

```text
source -> build -> CMS -> optimizer -> origin -> CDN
                    ^ first observed break
```

The product does not claim to identify the hidden component responsible for a
break. Probable causes belong in a future explicitly labelled `inference`
field, never in factual evidence.

## MVP

- JPEG;
- local path and public HTTPS checkpoints;
- official SDK validation;
- optional local trust snapshot;
- exact-byte and validated declared-parent continuity;
- versioned JSON, terminal, Markdown, annotations;
- local CLI and GitHub Action;
- no external asset upload.

## Explicit non-goals

Truth/authenticity scoring, AI/deepfake detection, copyright or authorship
determination, signing, private-key handling, compliance certification,
universal soft binding, remote-manifest retrieval, dashboard, accounts,
monitoring, WordPress/Shopify plugins, billing, video/audio, and enterprise
onboarding.

## Product principles

1. Facts and inferences are distinguishable.
2. Missing is not invalid; valid is not trusted; trusted is not true.
3. Changed bytes can be a legitimate signed derivative.
4. Inconclusive is an acceptable result.
5. User files stay on the runner by default.
6. Usage gates expansion.


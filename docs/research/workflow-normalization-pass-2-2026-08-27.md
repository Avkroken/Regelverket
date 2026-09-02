# Workflow normalization pass 2 — eight Avkroken repositories

Date: 2026-08-27
Status: empirical production observation

## Scope

This pass compares `.github/workflows` across the eight repositories that formed the v24.2 reference installation:

- docker-idempotent-update
- produkter
- pastebinit
- routines-relay
- politiker
- bastion
- klarsprak
- dumpen

The goal is to separate reusable policy implementation from repository-specific CI and product automation.

## Strong byte-identity findings

Git blob SHA proves that the following files are byte-identical across all sampled non-Bastion repositories, and where present also Bastion:

### auto-fix-review.yml

Observed SHA:

`bcb0e763628ccb74d139bf47accf5dd459fb0fb5`

This SHA is shared across docker-idempotent-update, produkter, pastebinit, routines-relay, politiker, klarsprak, dumpen and Bastion.

Implication: this is a true shared implementation artifact, not per-repository source.

### scope-policy.yml

Seven standard repositories use:

`154a00f9f68e84ab4e3b065091a075d1d67c95a1`

Bastion uses a distinct blob:

`8f5aa0b5cde912dcf520aaaed17437afd2eed5f5`

The reason is structural: Bastion has additional platform/core ref classes and therefore additional path/scope behavior.

Implication: the generic model is one scope-policy implementation plus repository/template data. Bastion is evidence that scope rules belong in configuration/graph data rather than copied workflow source.

### sync-pool.yml

The standard implementation used by docker-idempotent-update, produkter, pastebinit, routines-relay, politiker and klarsprak has SHA:

`24c322fa5dd160f4096849e84328bdcfb67634bd`

Dumpen uses:

`b64f095da4cab1729e71e915fb1ee5ec1c889a11`

Bastion uses:

`8ad66f20d26594a64b43a921b760b5811f2b44ad`

The variation correlates with slot/ref-class configuration. Bastion has six additional permanent platform/core slots. Dumpen differs from the standard pool implementation even though its ruleset topology is now standard, so its exact delta must remain an explicit fixture until normalized semantically.

Implication: slot membership and related explanatory comments/env data must be rendered from policy data. The implementation should not require one source file per repository.

## OSV scanner family

All eight contain `osv-scanner.yml`. The observed files have identical structure and differ in the scheduled cron offset. Examples already verified:

- docker-idempotent-update: `0 5 * * 1`
- produkter: `30 5 * * 1`
- Bastion: same workflow family with another staggered time

All use the same important semantic pattern:

- `pull_request`
- `merge_group: checks_requested`
- `push` on main
- scheduled scan
- event-specific reusable OSV jobs
- one stable wrapper job/check named `osv`

Implication: schedule staggering is template/runtime data. `osv` is an example of a stable capability-facing check backed by event-specific implementations.

## Repository-specific CI is genuinely repository-specific

Every repository has its own build/test surface. Examples from production rulesets and workflow inventories:

- docker-idempotent-update: `lint`, `python`, `docker`
- produkter: `python`, `node`, `docker`, `dependency-review`
- pastebinit: `python`
- routines-relay: `repository-checks`
- politiker: `python`, `typecheck`, `docker`
- klarsprak: `validate`
- dumpen: `test`
- Bastion: fifteen required checks spanning Apple, Linux, packaging, Android, Windows, OSV and scope policy

Therefore the generic system must not ship a fixed `ci.yml` concept as the required-check source. It must discover check providers and bind policy requirements to existing workflows/jobs where possible.

## Repository-specific non-policy automation

Workflow inventories contain product-specific automation that must remain unmanaged by default. Examples:

- pastebinit: release-deb
- routines-relay: send-mail
- politiker: export, release, media sync
- klarsprak: deploy
- docker-idempotent-update: package cleanup, Docker publishing
- produkter: dependency review, package cleanup, Docker workflows
- Bastion: multiple platform build/package workflows, TestFlight, security issue automation

This is empirical support for the ownership states `unmanaged`, `shared`, `adopted`, `managed`.

## Normalized family model

The v24.2 workflow population can be reduced conceptually to:

```text
Shared policy capabilities
  auto-fix-review
  scope-policy
  slot synchronization
  OSV gate/scanner family

Repository-specific providers
  CI/build/test/lint/typecheck
  packaging
  deployment
  release
  product automation
```

The first group is suitable for templates/modules. The second group must be discovered and reused/adapted rather than replaced.

## Detect requirements derived from this pass

`detect` must inventory every workflow and extract at least:

- path and blob SHA
- workflow display name
- triggers
- job IDs and explicit job names
- matrix expansion metadata
- reusable workflow calls
- local action calls
- `needs` dependencies
- permissions
- environments
- branch/path filters
- emitted observed check names from Actions runtime evidence when available

Static YAML alone is not sufficient to prove the final check name/provider mapping.

## Design conclusion

The old installation package is best represented as a small set of policy workflow implementations plus policy/config data and repository-specific provider bindings. This reinforces the compiler/planner design: files are rendered artifacts or discovered providers, never the policy model itself.

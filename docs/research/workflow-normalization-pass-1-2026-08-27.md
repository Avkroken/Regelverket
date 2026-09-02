# Workflow normalization pass 1

Status: observed production research
Date: 2026-08-27

## Purpose

This pass begins normalization of the workflow layer behind the eight-repository Avkroken policy family. It focuses on proving which files are policy-provided, which are project-provided, and which apparent repository variants are really one workflow with parameters.

## Observed policy-provided workflows

Across the inspected repositories, the recurring governance workflows are:

```text
auto-fix-review.yml
osv-scanner.yml
scope-policy.yml
sync-pool.yml
```

Project-specific workflows coexist beside them, for example `ci.yml`, `docker.yml`, packaging/build workflows and dependency review.

This confirms that file inventory alone is insufficient. Regelverket must classify workflow role/capability rather than assume every workflow is managed policy.

## Exact-content evidence

In the directly compared standard repositories `docker-idempotent-update` and `produkter`:

- `auto-fix-review.yml` has the same Git blob SHA: `bcb0e763628ccb74d139bf47accf5dd459fb0fb5`
- `scope-policy.yml` has the same Git blob SHA: `154a00f9f68e84ab4e3b065091a075d1d67c95a1`
- `sync-pool.yml` has the same Git blob SHA: `24c322fa5dd160f4096849e84328bdcfb67634bd`

Therefore these are not merely similar workflows; for this observed pair they are literally identical content reused across repositories.

Architecture consequence:

> A future renderer should maintain one semantic implementation for each common policy capability, not copied repository-specific source files.

## OSV scanner parameterization

`osv-scanner.yml` differs between the two directly compared standard repositories only in the observed scheduled cron time:

```text
docker-idempotent-update: 0 5 * * 1
produkter:                 30 5 * * 1
```

The rest of the observed workflow is structurally identical, including:

- `pull_request` on `main`
- `merge_group` with `checks_requested`
- `push` on `main`
- weekly schedule
- reusable OSV scanner workflows pinned to immutable commit SHA
- PR differential scan
- merge-group scan
- main/scheduled scan
- stable wrapper job named `osv`

This validates the earlier hypothesis that cron staggering is data, not a separate workflow implementation.

A future model could resolve something equivalent to:

```yaml
capability: dependency-vulnerability-gate
provider: osv
schedule:
  strategy: staggered
  resolved_cron: "30 5 * * 1"
```

The public schema remains undecided.

## Stable wrapper check pattern

The OSV workflow exposes one stable required check:

```text
osv
```

behind event-specific reusable jobs.

This is a reusable architecture pattern for Regelverket:

```text
complex/event-specific implementation
        -> stable policy-facing CheckProvider identity
```

The required ruleset need not know which underlying reusable workflow or event-specific job performed the work.

This pattern is particularly valuable for merge queue because provider implementations may differ between `pull_request` and `merge_group` while the ruleset requires one stable check identity.

## Scope policy

Standard repositories observed here share the same `scope-policy.yml` blob, while Bastion has a larger specialization containing branch-to-path rules for its six extra scopes.

Therefore `scope-policy` should be modeled as a capability with generated/resolved scope data rather than as one immutable workflow file for every repository.

Conceptually:

```text
base scope-policy engine
  + documentation policy
  + zero or more branch/path scope bindings
```

The Bastion specialization is evidence that future smart adaptation must preserve project-specific scope mappings rather than overwrite a user's existing gate with a generic copy.

## Sync pool

The standard pair has byte-identical `sync-pool.yml`. Bastion uses the same semantic lifecycle with an expanded slot set and production fixes around conflict handling/open-PR detection.

The normalized capability is not “install sync-pool.yml”. It is closer to:

```text
Maintain persistent slot refs such that:
  idle slot == current default branch SHA
  active slot == current work rebased on current default branch
  destructive reset requires confirmed no-open-PR state
  writes use concurrency-safe ref update semantics
  post-write state is verified
```

The slot list is resolved data.

This capability must remain coupled to the closed-pool ruleset and bypass/maintenance actor capabilities through the Policy Graph.

## Project-provided workflows

Examples observed in standard repositories:

`docker-idempotent-update`:

```text
ci.yml
docker.yml
cleanup-packages.yml
```

`produkter`:

```text
ci.yml
docker.yml
cleanup-packages.yml
dependency-review.yml
```

Their required checks differ accordingly.

This supports the distinction:

```text
policy-provided workflow
project-provided workflow
shared/reused workflow
```

During adaptation, project workflows should normally be discovered and bound to required capabilities/check providers, not overwritten or recreated.

## File naming conclusion

The current family happens to use generic names such as `scope-policy.yml` and `sync-pool.yml`, but a public tool cannot assume those paths are available.

The future ownership resolver should:

1. detect whether the desired path exists
2. identify whether it is managed/shared/unmanaged
3. reuse semantically compatible existing implementations where possible
4. otherwise generate a deterministic Regelverket-owned filename
5. use a stable collision suffix only when necessary

File path is a binding, not capability identity.

## Detection implications

For every workflow, discovery should extract at minimum:

- path
- Git blob SHA
- top-level workflow name
- events/triggers
- branch/path filters
- permissions
- jobs
- explicit job `name`
- `needs`
- job-level `if`
- reusable workflow `uses`
- local action/script references
- environment/deployment references
- check-run evidence and App identity where available

Then it should build:

```text
PolicyRequirement
  -> CheckProvider
     -> Job
        -> Workflow
           -> dependencies
```

## Evidence status

- Exact shared blobs: observed directly in `docker-idempotent-update` and `produkter`.
- OSV cron-only variation between those two: observed directly from current workflow content.
- Bastion OSV stable-wrapper pattern: observed directly in current Bastion workflow.
- Bastion `scope-policy` specialization and `sync-pool` slot expansion: observed directly in current production workflows.
- All-eight exact workflow equivalence/variation table: still to be completed; do not infer uninspected repos solely from historical package assumptions.

## Next pass

Complete an all-eight workflow inventory and build an empirical provider map for every required check:

```text
required check
  -> observed check run App
  -> job name
  -> workflow path
  -> supported events
```

The output becomes a core DG-02 fixture and a test corpus for the future Discovery/Workflow Dependency Analyzer.

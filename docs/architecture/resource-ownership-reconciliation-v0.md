# Resource Ownership & Reconciliation v0

Status: architecture draft
Date: 2026-08-27

## Purpose

Repeated execution must converge safely. Regelverket must know what it owns, what it merely depends on, what the user owns, and when observed state has drifted.

Core invariant:

> Applying the same desired policy twice to an unchanged repository produces no changes on the second run.

## Ownership states

### unmanaged

Resource existed independently of Regelverket and is not modified automatically.

### shared

Resource is unmanaged but satisfies a Regelverket requirement. Regelverket may depend on it but does not own its contents.

### adopted

Pre-existing resource explicitly transferred into Regelverket management after user approval and baseline capture.

### managed

Resource created or fully managed by Regelverket.

Ownership is per resource. A repository is never globally considered managed.

## Stable resource IDs

Every managed/adopted resource receives a stable logical identity, for example:

```text
workflow.slot-sync
workflow.scope-policy
ruleset.default
ruleset.closed-pool
ref.work.feature
```

Path, GitHub numeric ID and current content hash are bindings/observations, not identity.

## Manifest

A repository-local manifest should live outside `.github/workflows`, provisionally:

```text
.github/regelverket/manifest.yaml
```

Example conceptual form:

```yaml
schema: regelverket.manifest/v0
installation_id: rvk_...
template:
  id: sequential-slots
  version: 0.1.0

resources:
  workflow.slot-sync:
    ownership: managed
    kind: workflow
    path: .github/workflows/regelverk-slot-sync.yml
    last_applied_digest: sha256:...

  workflow.ci:
    ownership: shared
    kind: workflow
    path: .github/workflows/ci.yml
    binding:
      capability: ci
      check: build

  ruleset.default:
    ownership: managed
    kind: ruleset
    external_id: 123456
    last_applied_digest: sha256:...
```

Manifest format must support schema migration.

## Generated workflow naming

Normal names should be deterministic and readable:

```text
.github/workflows/regelverk-scope-policy.yml
.github/workflows/regelverk-slot-sync.yml
```

Repository name is normally unnecessary because workflow paths are repository-local.

If the deterministic path is occupied by an unmanaged resource, Regelverket must not overwrite it. A stable collision suffix may be generated from logical resource identity and installation context, for example:

```text
regelverk-slot-sync-a7c31f.yml
```

The suffix must not be a content hash. Content changes must update the same managed resource rather than create another file.

## Reconciliation loop

Every plan/apply cycle:

```text
Discover actual state
  -> load config + manifest
  -> resolve capabilities
  -> resolve template
  -> build desired graph
  -> bind reusable existing resources
  -> compare normalized actual/desired
  -> detect drift/conflicts
  -> produce plan
  -> explicit approval where required
  -> apply transaction steps
  -> verify actual + effective policy
  -> update manifest
```

## Operation classes

- NOOP
- REUSE
- CREATE
- UPDATE
- ADOPT
- MOVE/RENAME
- DETACH
- DELETE
- CONFLICT
- BLOCKED

Every non-NOOP operation needs a reason and affected dependency list.

## Drift

For managed/adopted resources compare:

1. last applied semantic digest
2. current observed semantic digest
3. newly desired semantic digest

This distinguishes:

### no drift

current == last applied

### user drift, desired unchanged

current != last applied and desired == last applied

Default: report and require explicit restore/adopt-new-state decision.

### user drift plus template/config change

All three differ. Planner must perform a three-way semantic comparison and must not silently overwrite.

### external normalization drift

Raw API differs but normalized semantics are equivalent. No change should be produced.

## Shared-resource drift

Shared resources are never restored automatically. If a shared CI workflow stops satisfying a required capability, the plan reports the broken binding and proposes alternatives:

- adapt with approval
- bind another compatible provider
- generate dedicated provider
- block

## Idempotence tests

Required from the first implementation milestone:

1. empty repo -> apply -> apply = second run NOOP
2. existing compatible CI -> adapt -> apply again = NOOP
3. managed file manually changed -> plan reports drift, no silent overwrite
4. unmanaged path collision -> generated stable alternate path; repeated run reuses it
5. API reorders set-like fields -> no false update
6. external numeric ruleset ID changes after controlled recreation -> logical identity remains stable after verified rebind

## Destructive safety

Delete, branch removal, ruleset replacement and ownership takeover are high-impact operations.

Requirements:

- snapshot before mutation
- detect open PR/dependency impact
- block on material unknown/inaccessible policy facts
- explicit approval
- verify after each irreversible boundary
- rollback plan where technically possible

## Concurrency

Plan/apply must account for repository mutation between observation and write.

Design requirements to research/implement:

- optimistic concurrency using SHAs/ETags where APIs support it
- force-with-lease rather than blind force push where Git operations require rewriting
- re-detect before destructive steps
- fail rather than overwrite on stale preconditions

## Effective verification

Verification is two-layered:

1. Resource verification: did managed resources reach desired normalized state?
2. Enforcement verification: do representative refs receive the expected effective rules after GitHub layering?

A successful API write is not sufficient proof of a successful policy installation.

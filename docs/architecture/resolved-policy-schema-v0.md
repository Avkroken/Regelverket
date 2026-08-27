# Resolved Policy Schema v0

Status: architecture draft
Date: 2026-08-27

## Purpose

Resolved Policy is the deterministic bridge between user intent and a concrete plan. It contains the exact semantic governance state Regelverket wants for one repository after combining:

- desired config
- selected template/archetype
- observed state
- capability/cost constraints
- adaptation decisions
- constraint evaluation

It is not a GitHub REST payload and it is not yet a mutation plan.

## Core distinction

```text
Observed State   = what exists / what is known
Desired Config   = what the user wants
Resolved Policy  = exact semantic target after resolution
Plan             = how to move observed -> resolved safely
```

## Required properties

Resolved Policy must be:

- deterministic for identical inputs
- complete enough to plan without rereading intent semantics
- independent of transient GitHub numeric IDs
- explicit about reuse/adoption/generation bindings
- explicit about unsupported/degraded features
- versioned and digestible

## Conceptual top-level shape

```yaml
schema: regelverket.resolved-policy/v0
repository: Avkroken/example

inputs:
  config_digest: sha256:...
  observed_state_digest: sha256:...
  template:
    id: sequential-slots
    version: 0.1.0
    digest: sha256:...
  knowledge_base_version: 2026-08-27

resolution:
  capability_profile: ...
  degradation: []
  warnings: []

resources: ...
requirements: ...
bindings: ...
constraints: ...
verification_expectations: ...
```

## Semantic resources

Resources use stable logical identities.

Examples:

```yaml
resources:
  ref.work.feature:
    kind: ref
    desired_name: work/feature
    class: work

  workflow.scope-policy:
    kind: workflow_capability
    capability: scope-policy

  ruleset.default:
    kind: ruleset
    target: refclass.default
```

A resource can later bind to an existing unmanaged/shared implementation or a generated managed implementation.

## Requirements

Policy requirements remain intent-level assertions, for example:

```yaml
requirements:
  default.no_force_push:
    severity: invariant
    target: refclass.default

  default.required_ci:
    severity: invariant
    requires:
      - check-capability.ci
```

This allows verification to test policy semantics independently of renderer details.

## Bindings

Bindings capture adaptation decisions.

```yaml
bindings:
  check-capability.ci:
    strategy: reuse
    resource: workflow.existing-ci
    evidence:
      state: known
      source: workflow-analysis

  workflow.scope-policy:
    strategy: generate
    resource: workflow.regelverket-scope-policy
```

Allowed conceptual strategies:

- reuse
- compose
- adapt
- generate
- adopt
- conflict
- unavailable

## Resolved GitHub implementation

The schema may carry normalized GitHub implementation intent while keeping it separate from raw API payloads.

Example:

```yaml
resources:
  ruleset.default:
    kind: ruleset
    target:
      type: branch
      include: ["~DEFAULT_BRANCH"]
      exclude: []
    enforcement: active
    rules:
      - deletion_prohibited
      - non_fast_forward_prohibited
      - linear_history_required
      - required_checks:
          strict: false
          checks:
            - binding: check-capability.ci
            - binding: check-capability.osv
            - binding: check-capability.scope-policy
```

Renderer adapters convert this representation into GitHub REST structures.

## Ref classes

Resolved Policy must contain concrete class membership/binding decisions.

```yaml
ref_classes:
  work:
    members:
      - ref.work.feature
      - ref.work.fix
      - ref.work.chore
  documentation:
    members:
      - ref.docs.content
```

For dynamic patterns it must preserve pattern semantics and sample/boundary verification cases.

## Capability resolution

Each requested capability receives a result:

```yaml
capabilities:
  merge_queue:
    state: available
    source: github-plan-probe

  organization_rulesets:
    state: unavailable
    reason: account_or_plan
```

Possible states align with the capability engine, including:

- available
- unavailable
- requires-upgrade
- permission-missing
- inaccessible
- unknown
- substituted

## Degradation and substitution

If a template supports a tested fallback, resolved policy records the explicit substitution:

```yaml
degradation:
  - requested: merge_queue
    resolved_as: protected_serial_merge
    reason: capability_unavailable
    preserves:
      - serialized-default-branch-integration
    loses:
      - native-merge-group-validation
```

A substitution must never be hidden.

## Constraint results

Resolved Policy includes constraint evaluation before planning.

```yaml
constraints:
  - id: github.merge_queue.requires_merge_group
    result: pass
  - id: cost.free_only
    result: pass
```

Blocking constraints prevent plan generation except for a diagnostic plan state.

## Ownership intent

Resolved Policy declares intended ownership without mutating the manifest yet.

```yaml
ownership:
  workflow.existing-ci: shared
  workflow.regelverket-scope-policy: managed
  ruleset.default: managed
```

Actual ownership transition occurs during apply after verification.

## Verification expectations

Every invariant can declare how it should be verified.

```yaml
verification_expectations:
  - requirement: default.required_ci
    methods:
      - resource_state
      - dependency_coherence
      - effective_policy
```

High-risk templates may require behavioral certification evidence separately.

## Semantic digest

Resolved Policy produces a canonical semantic digest.

Digest excludes:

- timestamps
- transient external numeric IDs
- display-only ordering

Digest includes:

- requirements
- normalized rules
- ref topology
- bindings
- capability/degradation decisions
- ownership intent

## Explainability

Every non-trivial resolved choice should be traceable to one or more causes:

```yaml
reason:
  because:
    - template: sequential-slots@0.1.0
    - config: intent.merge.strategy=sequential
    - observed: existing workflow emits build on merge_group
```

This supports `regelverk explain` and plan rationale.

## Acceptance criteria

The same schema must represent:

- Dumpen-style standard Sequential Slots topology
- Bastion's extended platform/core ref classes and larger required-check graph
- a future repository that reuses existing workflows instead of generating Regelverket files

No repository-name special cases are allowed in the schema.

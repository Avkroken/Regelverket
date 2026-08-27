# Plan Schema v0

Status: architecture draft
Date: 2026-08-27

## Purpose

A plan is the explicit, reviewable transition from observed state to resolved policy. It must be deterministic, explainable, stale-state-aware and safe to serialize for later apply.

A plan is not a stream of imperative API calls. It is a dependency-ordered set of semantic operations with preconditions, risk classes, rollback characteristics and verification requirements.

## Inputs

Every plan records immutable references/digests for the material inputs:

```yaml
schema: regelverket.plan/v0
repository: Avkroken/example

inputs:
  observed_state_digest: sha256:...
  resolved_policy_digest: sha256:...
  manifest_digest: sha256:...
  config_digest: sha256:...
  knowledge_base_version: 2026-08-27
```

## Plan status

Possible states:

- ready
- no_changes
- blocked
- requires_approval
- stale
- invalid

A blocked plan can still be useful diagnostically and must explain the blockers.

## Operation model

Each operation has a stable plan-local ID and a logical resource ID.

```yaml
operations:
  - id: op-001
    action: create
    resource: workflow.scope-policy
    kind: workflow
    risk: safe_additive
    depends_on: []
    reason:
      requirement: default.scope-validation
      resolution: generate
```

Conceptual action set:

- noop
- reuse
- create
- update
- adopt
- compose
- move
- rename
- detach
- delete
- conflict
- blocked

`reuse`/`noop` may be included in explain output even if they produce no mutation.

## Risk classes

Every mutating operation is classified:

- `safe_additive`
- `policy_enforcement`
- `ownership_transfer`
- `privilege_sensitive`
- `destructive`

Approval can be scoped to these classes. One approval must not authorize newly discovered operations outside the approved plan digest.

## Preconditions

Operations declare facts that must still be true at apply time.

Examples:

```yaml
preconditions:
  - type: ref_sha_equals
    ref: refs/heads/main
    value: abc123
  - type: file_blob_sha_equals
    path: .github/workflows/ci.yml
    value: def456
  - type: ruleset_semantic_digest_equals
    resource: ruleset.default
    value: sha256:...
```

Material precondition failure makes the plan stale; it triggers re-detection/replanning rather than blind write.

## Dependency ordering

Operations form a DAG.

Required invariant:

> dependencies before enforcement; replacements before removals.

Example:

```text
create workflow provider
  -> verify staged provider
  -> update required-check rule
  -> verify effective enforcement
  -> delete obsolete managed provider
```

Cycles in the mutation DAG are a planning error that must be resolved into staged operations or reported as blocked.

## Impact set

Every operation should expose downstream impact discovered from the repository/policy graph.

```yaml
impact:
  resources:
    - ruleset.default
    - check.build
  open_pull_requests: []
  ref_classes:
    - default
```

Impact informs risk, approval and verification scope.

## File operations

File modifications must identify ownership and merge strategy.

```yaml
file_change:
  path: .github/workflows/ci.yml
  ownership: unmanaged
  strategy: adapt
  before_sha: ...
  proposed_semantic_digest: ...
  requires_explicit_approval: true
```

Unmanaged files are never overwritten because a template uses the same filename.

Generated path collisions are resolved before plan creation and remain stable across replans when inputs are equivalent.

## Ruleset operations

Ruleset operations refer to logical resources and normalized semantics. External GitHub IDs are bindings only.

For replacement/recreation the plan must explain whether identity can be preserved or a new external ID is expected.

## Ref operations

Branch/ref changes include explicit source/target SHA and whether force behavior is involved.

High-risk examples:

- delete branch
- force-update branch
- change default branch

These require destructive approval and open-work impact analysis.

## Ownership transitions

Adoption is explicit:

```yaml
operation:
  action: adopt
  resource: workflow.existing-ci
  from: unmanaged
  to: adopted
```

Shared bindings do not transfer ownership.

## Rollback class

Each mutation declares:

- reversible
- compensatable
- irreversible_or_uncertain

and records the proposed compensation where relevant.

```yaml
rollback:
  class: compensatable
  action: recreate_previous_ruleset_semantics
```

## Verification checkpoints

Operations/phases can require checkpoints:

```yaml
verify:
  after:
    - resource_state
    - dependency_coherence
```

The whole plan ends with effective-policy verification for representative refs when enforcement changed.

## Plan phases

A compiled plan groups operations into ordered phases:

1. preflight
2. snapshot
3. stage_dependencies
4. validate_staged
5. apply_enforcement
6. verify_enforcement
7. cleanup
8. commit_manifest

A phase can be empty.

## No-change plan

If observed normalized semantics already satisfy resolved policy:

```yaml
status: no_changes
operations: []
```

This is the required result of repeated apply against unchanged desired/actual state.

## Conflict and blocker records

Plans must distinguish conflicts from missing capabilities.

Examples:

```yaml
blockers:
  - id: required-check-provider-ambiguous
    resource: check.build
    evidence: [workflow.a, workflow.b]

conflicts:
  - id: unmanaged-path-collision
    path: .github/workflows/regelverk-scope-policy.yml
    resolution_options:
      - generate_alternate_path
      - adopt_existing
```

When a safe deterministic resolution exists, planner should resolve it and explain the choice rather than unnecessarily block.

## Destructive approval token

An apply must bind approval to the exact plan digest and risk classes. If the plan changes after approval, destructive approval is invalidated.

## Multi-repository batch plan

A batch is a container around independent repository plans:

```yaml
schema: regelverket.batch-plan/v0
plans:
  - repository: Avkroken/a
    plan_digest: ...
  - repository: Avkroken/b
    plan_digest: ...
```

Cross-repository atomicity is not implied.

## Explainability requirements

For every create/update/delete/adopt operation, plan output must answer:

- what changes?
- why is it required?
- what does it depend on?
- what might it affect?
- is it reversible?
- what verification proves success?
- which user/template/capability decision led here?

## Acceptance criteria

The v0 planner must be able to serialize a no-op plan for an already-converged Avkroken fixture and a deterministic additive/update plan when one required check, ref class or generated policy workflow is changed in desired state.

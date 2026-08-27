# Plan Schema v0

Status: architecture draft
Date: 2026-08-27

## Purpose

A plan is the explicit, reviewable transition from observed state to resolved policy. It must be deterministic, explainable, stale-state-aware and safe to serialize for later apply.

A plan is not a stream of imperative API calls. It is an immutable, dependency-ordered description of exact semantic mutations plus the verification barriers, preconditions, approvals and rollback characteristics required to execute them safely.

## Inputs and freshness

Every plan records immutable references/digests for material inputs and time-sensitive capability evidence:

```yaml
schema: regelverket.plan/v0
repository: Avkroken/example
created_at: 2026-08-27T18:00:00+02:00

inputs:
  observed_state_digest: sha256:...
  resolved_policy_digest: sha256:...
  manifest_digest: sha256:...
  config_digest: sha256:...
  knowledge_base_version: 2026-08-27
  capability_evidence:
    - evidence_id: ev.merge-queue
      digest: sha256:...
      observed_at: 2026-08-27T17:59:00+02:00
      valid_until: 2026-08-27T18:59:00+02:00
  permission_evidence:
    - evidence_id: ev.repo-admin
      digest: sha256:...
      observed_at: 2026-08-27T17:59:00+02:00
      valid_until: 2026-08-27T18:14:00+02:00
```

Expired or non-reproducible material evidence makes apply re-probe before mutation. If the re-probe changes a material assumption, the saved plan becomes stale and must be regenerated.

## Plan status and precedence

Possible serialized states:

- ready
- no_changes
- requires_approval
- blocked
- stale
- invalid

Precedence from strongest to weakest is:

```text
invalid > stale > blocked > requires_approval > ready/no_changes
```

Invariants:

- `invalid`, `stale`, and `blocked` are never applyable.
- `requires_approval` is applyable only after all required approval classes are satisfied for the current canonical mutation digest.
- `no_changes` requires an empty mutating operation set.
- apply independently evaluates blockers/preconditions; status text alone is never an authorization boundary.

## Operation model

Each operation has a stable plan-local ID, phase membership, logical resource ID, exact target state or immutable artifact reference, risk classes, dependencies, impact and verification requirements.

```yaml
operations:
  - id: op-001
    phase: stage_dependencies
    action: create
    resource: workflow.scope-policy
    kind: workflow
    risk_classes: [safe_additive]
    depends_on: []
    target:
      representation: content_addressed_artifact
      digest: sha256:...
      artifact_id: artifact.workflow.scope-policy
    reason:
      requirement: default.scope-validation
      resolution: generate
    impact:
      resources: [check.scope-policy]
      open_pull_requests:
        state: known
        values: []
    rollback:
      class: reversible
    verify_after: [checkpoint.scope-policy-provider]
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

`reuse`/`noop` may be serialized for explainability even though they produce no mutation.

## Exact target state

A saved mutating operation must be executable without rerunning planner logic or consulting mutable unspecified state.

`target` therefore contains either:

1. the complete normalized desired semantic object, or
2. an immutable content-addressed artifact reference whose digest is part of the plan.

Examples include a complete normalized ruleset object, a generated workflow blob, a ref target SHA, or an explicit semantic patch whose base digest is a precondition.

A digest alone is never sufficient when it cannot reconstruct the reviewed mutation.

## Risk classes and approval scope

Risk is a set because one operation may cross several approval boundaries:

- `safe_additive`
- `policy_enforcement`
- `ownership_transfer`
- `privilege_sensitive`
- `destructive`

Example:

```yaml
risk_classes:
  - ownership_transfer
  - policy_enforcement
```

Approval must cover every applicable class. One approval cannot silently authorize newly discovered classes or operations.

## Preconditions

Operations declare facts that must still be true at apply time.

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

## Dependency and verification DAG

Operations and verification checkpoints share one dependency graph. Enforcement may depend on successful verification, not merely on resource creation.

```yaml
checkpoints:
  - id: checkpoint.scope-policy-provider
    phase: validate_staged
    kind: dependency_coherence
    verifies: [workflow.scope-policy, check.scope-policy]
    depends_on: [op-001]

operations:
  - id: op-002
    phase: apply_enforcement
    action: update
    resource: ruleset.default
    depends_on: [checkpoint.scope-policy-provider]
```

Required invariant:

> dependencies before enforcement; verified replacements before removals.

Cycles in the combined DAG are a planning error and must be staged differently or reported as blocked.

## Plan phases

Every operation/checkpoint belongs to exactly one ordered phase:

1. preflight
2. snapshot
3. stage_dependencies
4. validate_staged
5. apply_enforcement
6. verify_enforcement
7. cleanup
8. commit_manifest

A phase can be empty. The executor may parallelize only nodes whose DAG dependencies and phase barriers permit it.

## Impact set and completeness

Impact data uses the same evidence-state semantics as observed state so `known empty` cannot be confused with `unknown` or `inaccessible`.

```yaml
impact:
  resources:
    - ruleset.default
    - check.build
  open_pull_requests:
    state: known
    values: []
    evidence: [ev.open-prs]
  ref_classes:
    - default
```

Allowed collection states include `known`, `unknown`, `inaccessible`, and `conflicting`. A destructive operation with materially unknown/inaccessible open-work impact is blocked unless a future explicit safety policy proves the uncertainty irrelevant.

## File operations

File modifications identify ownership, strategy, exact target artifact and base precondition.

```yaml
file_change:
  path: .github/workflows/ci.yml
  ownership: unmanaged
  strategy: adapt
  before_sha: ...
  target_artifact:
    digest: sha256:...
    artifact_id: artifact.workflow.ci-adapted
  requires_explicit_approval: true
```

Unmanaged files are never overwritten because a template uses the same filename.

Generated path collisions that have a safe deterministic alternate path are resolved before plan creation and represented as an explained binding decision, not an unresolved conflict.

## Ruleset operations

Ruleset operations refer to logical resources and normalized target semantics. External GitHub IDs are bindings only.

For replacement/recreation the plan explains whether identity can be preserved or a new external ID is expected.

## Ref operations

Branch/ref changes include explicit source/target SHA and whether force behavior is involved.

High-risk examples include delete branch, force-update branch, and change default branch. These require destructive approval and complete open-work impact analysis.

## Ownership transitions

Adoption is explicit and may carry more than one risk class:

```yaml
operation:
  action: adopt
  resource: ruleset.existing
  from: unmanaged
  to: adopted
  risk_classes: [ownership_transfer, policy_enforcement]
```

Shared bindings do not transfer ownership.

## Rollback class

Each mutation declares one of:

- reversible
- compensatable
- irreversible_or_uncertain

and records the proposed compensation where relevant.

## Verification checkpoints

Checkpoints have stable IDs, dependencies, evidence expectations and failure behavior. A checkpoint failure prevents dependent operations.

The whole plan ends with effective-policy verification for representative refs whenever enforcement changed.

## No-change plan

If observed normalized semantics already satisfy resolved policy:

```yaml
status: no_changes
operations: []
```

This is the required result of repeated apply against unchanged desired/actual state.

## Conflict and blocker records

Conflicts are reserved for choices that cannot be safely/deterministically resolved by planner policy.

```yaml
blockers:
  - id: required-check-provider-ambiguous
    resource: check.build
    evidence: [workflow.a, workflow.b]

conflicts:
  - id: unmanaged-workflow-semantic-overlap
    resources: [workflow.user-ci, workflow.generated-ci]
    reason: both can emit the same required context but neither can be safely selected automatically
    resolution_options:
      - bind_existing
      - generate_distinct_check
      - adopt_existing
```

When a safe deterministic resolution exists, planner resolves it and explains the choice.

## Canonical mutation digest and approval token

`plan_digest` is computed over a canonical mutation envelope, not over arbitrary YAML/JSON serialization.

Canonicalization requirements:

- UTF-8 JSON canonical form defined by the implementation contract/ADR before destructive approvals ship
- maps use canonical key ordering
- set-like collections are normalized and sorted
- no timestamps, prose/explain text, UI metadata or other non-semantic fields are included unless they change apply behavior
- all mutation-relevant fields are mandatory in the digest envelope

The digest envelope includes at minimum:

```text
schema version
repository identity
material input digests
capability/permission evidence digests + freshness bounds
all mutating operations:
  id
  phase
  action
  resource/kind
  exact target state/artifact digest
  risk_classes
  preconditions
  dependencies
  rollback class/compensation target where applicable
all verification checkpoints and dependency edges
all blockers/conflicts that affect applyability
```

Approval tokens bind to this digest plus the approved risk-class set and repository identity. Any change to the canonical envelope invalidates prior approval.

## Multi-repository batch plan

A batch is a container around repository plans and can express cross-repository dependencies without implying atomicity.

```yaml
schema: regelverket.batch-plan/v0
plans:
  - id: repo-a
    repository: Avkroken/a
    plan_digest: ...
    depends_on: []
  - id: repo-b
    repository: Avkroken/b
    plan_digest: ...
    depends_on: [repo-a]
```

Dependencies must form a DAG. A v0 executor may choose to reject dependent batches if it cannot honor the ordering, but it must never infer ordering solely from YAML list position.

## Explainability requirements

Every operation, including non-mutating `reuse`, `compose`, and `noop`, explains why that semantic action was selected and which requirement/resource it satisfies.

Every mutating operation additionally answers:

- what exact state changes?
- what does it depend on?
- what might it affect, including completeness/unknown state?
- which risk classes apply?
- is it reversible/compensatable?
- what checkpoint proves success?
- which user/template/capability decision led here?

## Acceptance criteria

The v0 planner must be able to serialize:

1. a no-op plan for an already-converged Avkroken fixture;
2. a deterministic additive/update plan when one required check, ref class or generated policy workflow changes;
3. a staged provider -> verification checkpoint -> enforcement dependency chain;
4. a destructive operation that is blocked when open-work impact is unknown;
5. an approval token whose canonical mutation digest changes whenever any mutation-relevant target, risk class, dependency, precondition or verification barrier changes.

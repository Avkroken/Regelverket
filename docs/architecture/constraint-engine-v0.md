# Constraint Engine v0

Status: architecture draft
Date: 2026-08-27

## Purpose

The constraint engine converts documented GitHub semantics, observed behavior and Regelverket safety requirements into machine-evaluable rules.

It exists to prevent invalid or dangerous combinations before mutation.

## Constraint classes

### Platform hard constraint

A condition GitHub requires for the requested feature to work.

Violation blocks the plan.

### Policy invariant

A property promised by the selected archetype/template.

Violation blocks the resolved policy unless a documented degradation path explicitly changes the promise.

### Safety constraint

Protects repository integrity during detection, migration or reconciliation.

Violation can block only risky/destructive operations while still allowing read-only analysis.

### Compatibility constraint

Expresses interaction between existing repository resources and desired policy.

May yield adapt/generate/conflict.

### Cost/capability constraint

Feature availability depends on plan, account type, visibility, permissions or user budget.

### Advisory rule

Non-blocking best practice or optimization. Must never be presented as a GitHub requirement.

## Constraint record

Conceptual form:

```yaml
id: github.merge_queue.actions_requires_merge_group
class: platform_hard
status: verified
if:
  all:
    - capability.merge_queue == enabled
    - required_check.provider == github_actions
then:
  require:
    - provider.workflow.events contains merge_group
failure:
  severity: blocker
  message: Required GitHub Actions checks used with merge queue must run for merge_group.
provenance:
  - source: github_docs
    checked: 2026-08-27
```

Schema is provisional.

## Provenance states

Every constraint must carry provenance and confidence:

- documented
- observed
- documented_and_observed
- inferred
- project_safety_policy

Experimental findings must link to a reproducible test ID where possible.

## Initial constraint families

### Required checks

- required check needs an identifiable provider before it can be safely enforced
- ambiguous duplicate providers are an error when they can produce indistinguishable required-check identity
- expected-source/App binding must not be invented when source identity is unknown
- required workflows/checks whose triggering filters can skip required execution must be flagged

### Merge queue

- required GitHub Actions checks must support merge-group execution
- all required checks for the protected branch must have a valid queue-time provider
- template cannot claim merge-queue semantics if capability is unavailable and no tested semantic fallback exists

### Ruleset targeting

- include/exclude conditions must be evaluated using GitHub semantics, not generic glob assumptions
- overlapping rulesets must be evaluated as layered effective policy
- unknown higher-level policy blocks destructive replacement if effective enforcement cannot be proven

### Branch/ref lifecycle

- deleting a branch with open PR/dependency relationships is destructive and requires explicit handling
- creating a closed-pool lock before allowed refs/providers exist is forbidden by operation ordering
- forced ref updates require stale-state protection where possible

### Workflows

- generated workflow path cannot overwrite unmanaged content
- local/reusable workflow references must remain resolvable after move/rename
- workflow/job rename that changes required-check identity requires dependent policy update in same plan
- external executable dependencies must satisfy project supply-chain policy

### Ownership

- unmanaged resource cannot be silently converted to managed
- shared resource cannot be silently modified
- managed drift cannot be overwritten without resolving the three-way state
- content hash changes do not create a new logical resource identity

### Billing/capabilities

- `free-only` forbids a plan whose required semantics depend on a paid capability
- `requires-upgrade` must be reported before apply
- permission-missing and feature-unavailable are distinct states

## Rule evaluation phases

### Phase A: factual consistency

Detect contradictions in observed evidence.

### Phase B: template satisfiability

Can requested policy be represented under current capabilities and user constraints?

### Phase C: binding compatibility

Can existing resources safely implement requested requirements?

### Phase D: migration safety

Can proposed operations be executed without violating safety invariants?

### Phase E: post-resolution invariant check

Does the final desired graph still satisfy every template promise?

## Constraint outcomes

- PASS
- WARN
- ADAPT_REQUIRED
- CONFLICT
- BLOCKED
- UNKNOWN_BLOCKING

`UNKNOWN_BLOCKING` is important: lack of evidence is not automatically failure, but for high-risk operations it prevents mutation.

## Explanations

A constraint result must be traceable.

Example:

```text
BLOCKED github.merge_queue.actions_requires_merge_group

Requested:
  Sequential Slots / merge-queue variant

Observed:
  build is required on main
  build is emitted by .github/workflows/ci.yml
  ci.yml triggers pull_request but not merge_group

Why blocked:
  queue candidates would not receive the required build provider

Safe options:
  adapt ci.yml
  select a tested no-queue variant
```

## Constraint conflicts

If constraints themselves appear contradictory, do not silently choose one.

Priority:

1. platform hard facts
2. safety invariants
3. explicit user constraints
4. template invariants
5. advisory preferences

A contradiction among the first four becomes an architecture/research defect to resolve.

## Versioning

Constraints form a versioned knowledge base separate from application releases. Each Regelverket release records the knowledge-base version it evaluated against.

A newer knowledge base may change recommendation or plan results even when user configuration is unchanged; this must be surfaced explicitly.

## Tests

Every blocking constraint needs at least one positive and one negative test. Constraints derived from live behavior should additionally reference an E2E/experiment fixture.

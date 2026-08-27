# Capability Knowledge Model v0

Status: architecture draft
Date: 2026-08-27

The research matrix shows that Regelverket cannot represent GitHub capabilities as booleans. Availability and semantics vary by product, plan, owner type, repository visibility, permission, source level and feature maturity.

## Capability record

```yaml
id: github.rules.required_status_checks
kind: rule
scope:
  products: [github.com]
  targets: [branch, tag]
availability:
  owner_types: [user, organization]
  visibilities: [public, private]
  plans: [] # resolved from versioned evidence, not guessed
maturity: stable
parameters: []
prerequisites: []
conflicts: []
dependencies: []
verification: []
evidence:
  - type: documentation
    observed_at: 2026-08-27
    source: github-docs
uncertainty: []
```

## Capability resolution result

```yaml
capability: github.rules.merge_queue
state: available
confidence: documented
reasons: []
requirements: []
upgrade_options: []
unknowns: []
```

Allowed states initially:

- available
- configured
- misconfigured
- unavailable_plan
- unavailable_account_type
- unavailable_visibility
- permission_missing
- unknown_inaccessible
- unknown_unresearched
- preview

## Rule resource identity

A normalized rule must retain:

```yaml
resource_id: ruleset.main.required-checks
source:
  level: repository # repository|organization|enterprise
  owner: example
  ruleset_id: 123
  editable: true
  visibility: complete # complete|partial|unknown
target: branch
conditions: {}
rule:
  type: required_status_checks
  parameters: {}
evidence_state: documented
```

Source provenance is mandatory. An organization rule affecting a repository must never be treated as if Regelverket can edit it at repository level.

## Uncertainty is data

Examples:

```yaml
bypass_actors:
  state: unknown_inaccessible
  value: null
```

is different from:

```yaml
bypass_actors:
  state: known
  value: []
```

The planner must propagate uncertainty. A destructive operation depending on unknown policy state is blocked by default.

## Constraint record

```yaml
id: constraint.merge_queue.actions_merge_group
severity: error
when:
  all:
    - capability: github.rules.merge_queue
      state: configured
    - provider_kind: github_actions
assert:
  workflow_event: merge_group
message: >-
  Required GitHub Actions checks used by merge queue must be emitted for merge-group execution.
evidence:
  confidence: documented
```

Constraints may have confidence levels `documented`, `observed`, `inferred`, or `experimental`.

## Recommendation record

Recommendations must be separate from constraints:

```yaml
id: recommendation.sequential-slots.ai-heavy
applies_when: []
benefits: []
tradeoffs: []
requires: []
excludes: []
confidence: experimental
```

This prevents community popularity or Regelverket opinion from being confused with GitHub platform truth.

## Versioning

Every research snapshot must record:

- date
- GitHub product
- API version
- docs/product version where relevant
- feature maturity (stable/beta/public-preview)

Knowledge changes independently of Regelverket binary releases. The implementation should eventually support updating the knowledge catalog without rewriting the planner.

## Design consequences

1. Detection produces facts plus uncertainty, not a flat config.
2. Capability resolution occurs before template resolution.
3. Templates declare required/optional capabilities rather than directly assuming plan features.
4. Planner preserves source provenance and editability.
5. Verify checks both desired managed resources and effective GitHub policy.
6. Preview capabilities require explicit template opt-in until promoted.

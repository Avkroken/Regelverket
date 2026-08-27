# Template Specification v0

Status: architecture draft
Date: 2026-08-27

## Purpose

Templates are first-class product artifacts. A user must be able to apply a tested workflow model directly or adapt it safely to an existing repository.

## Three levels

### Archetype

Describes a development/governance model independently of a specific GitHub implementation.

Examples under research may include Sequential Slots, Protected Trunk, GitHub Flow, Release Branching, and deployment-oriented models. Names are not final until real-world research is complete.

### Template

A versioned, tested implementation of an archetype for a capability envelope.

### Resolved Policy

The concrete desired graph after repository detection, user constraints, capability resolution and adaptation.

## Template is intent plus implementation options

A template must not be a directory of files to copy.

It declares requirements such as:

```yaml
schema: regelverket.template/v0
id: sequential-slots
version: 0.1.0
archetype: sequential-slots
summary: Sequential isolated work slots rebased onto the default branch.

requires:
  policy:
    - default.no_force_push
    - default.required_ci
    - slots.closed_pool
    - slots.synchronized

optional:
  capabilities:
    - merge_queue

parameters:
  work_slots:
    type: list
    default: [feature, fix, chore]
  documentation_slot:
    type: boolean
    default: true
```

The exact schema remains provisional until the GitHub capability research is complete.

## Required template metadata

Every public template must document:

- stable ID and semantic version
- archetype
- human-readable purpose
- intended repository/project profiles
- unsuitable profiles
- required capabilities
- optional capabilities and degradation paths
- GitHub account/plan implications
- cost implications where known
- branch/ref topology
- policy requirements
- automation requirements
- security properties
- operational trade-offs
- migration risks
- known failure modes
- test coverage
- provenance for important GitHub assumptions

## Modes

### Apply template

Goal: realize the template as designed.

Existing compatible resources may still be reused, but template invariants win. Any destructive or incompatible change appears in the plan and requires approval.

### Adapt template

Goal: realize the template's semantic properties while preserving useful existing repository structure.

Resolution preference:

1. REUSE
2. COMPOSE
3. ADAPT
4. GENERATE
5. CONFLICT

The planner must explain why it selected an operation.

## Capability requirements, not filenames

A template should request:

```yaml
workflow_requirements:
  ci:
    must_emit:
      - check: build
    events:
      - pull_request
      - merge_group_if_merge_queue
```

It should not require `.github/workflows/ci.yml` unless the path itself has semantic meaning.

Discovery can therefore bind an existing workflow to the requirement.

## Branch/ref requirements

Templates describe semantic classes before concrete names.

```yaml
ref_classes:
  work:
    count: 3
    suggested_names:
      - work/feature
      - work/fix
      - work/chore
  documentation:
    count: 1
    suggested_names:
      - docs/content
```

Adapt mode may bind an existing compatible branch to a class. Apply mode may prefer canonical names. Naming policy is a resolver concern.

## Variants

Templates can contain capability-driven variants rather than being duplicated.

Example concept:

```text
Sequential Slots
  base
  + merge-queue enhancement
  + organization policy enhancement
  + AI-heavy safeguards
```

Variants must declare the condition that activates them and their additional requirements.

## Degradation

A template may define tested fallbacks when a feature is unavailable.

Example:

```yaml
feature: merge_queue
if_unavailable:
  strategy: substitute
  template_capability: protected_serial_merge
```

A fallback is allowed only if it preserves explicitly documented properties. Otherwise the template must report `requires-upgrade` or `unsupported`.

## Recommendations

Templates can declare matching signals but recommendations must distinguish facts from heuristics.

Possible signals:

- solo vs team
- number of active contributors
- AI/bot automation
- monorepo
- release cadence
- deployment model
- expected parallel work
- public/private
- account type
- budget constraint

Recommendation output must include reasons and disqualifiers.

## Template safety

Templates are executable governance inputs and therefore part of the supply chain.

Requirements:

- versioned
- schema validated
- immutable release identity
- provenance/signing strategy to be researched
- no arbitrary code in the declarative template layer
- generated workflows/actions reviewed as executable artifacts
- external Actions references pinned according to project security policy

## Testing contract

A template is not publishable until it has:

- schema tests
- graph resolution tests
- plan golden tests
- idempotence tests
- adaptation/conflict tests
- live E2E for supported capability profiles
- documented unsupported profiles

## Initial reference fixture

Avkroken v24.2 is a reference fixture for the Sequential Slots family. The new implementation is not required to reproduce byte-identical files. It must be able to reproduce the intended governance semantics and explicitly explain any deliberate differences.

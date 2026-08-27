# Desired State / Config Schema v0

Status: architecture draft
Date: 2026-08-27

## Purpose

The Regelverket config expresses user intent. It must not be a thin mirror of GitHub REST payloads.

The config is the durable desired-state input consumed together with observed state, capability facts and a selected template/archetype.

## Core principles

1. Intent first, implementation second.
2. Schema is versioned from day one.
3. Omitted is not always false; defaults belong to schema/template resolution.
4. Repository-specific facts belong in observed state, not copied into config unless the user intentionally overrides them.
5. Cost, capability and preservation constraints are first-class inputs.
6. Multi-repo config is composable without forcing all repositories into one identical policy.

## Top-level conceptual shape

```yaml
schema: regelverket.config/v0

repository:
  target: Avkroken/example

mode: adapt

template:
  id: sequential-slots
  version: 0.1.0

constraints:
  budget: free-only
  destructive_changes: deny
  preserve_unmanaged_workflows: true

intent:
  merge:
    strategy: sequential
    require_linear_history: true
  review:
    required_approvals: 0
    require_thread_resolution: true
  automation:
    dependabot: enabled
    ai_agents: present
  slots:
    work:
      names: [work/feature, work/fix, work/chore]
    documentation:
      names: [docs/content]
```

Exact field names remain provisional until implementation spikes validate ergonomics.

## Repository target

The config can target:

- one existing repository
- a set of existing repositories
- a new repository definition in a later phase

Repository identity is a selector, not ownership state.

## Mode

Supported semantic modes:

- `adopt`
- `augment`
- `replace`
- template convenience modes may map onto these internally

`adapt-template` is expected to resolve primarily through `augment`/selective `adopt` behavior.

## Template selection

Template reference must be immutable enough for reproducibility:

```yaml
template:
  id: sequential-slots
  version: 0.1.0
  digest: sha256:...
```

Digest may be optional for locally authored templates but official release flows should pin it.

## Constraints

Constraints represent user boundaries rather than GitHub settings.

Examples:

```yaml
constraints:
  budget: free-only
  plan_upgrades: deny
  destructive_changes: prompt
  ownership_takeover: prompt
  preserve_unmanaged_workflows: true
  preserve_existing_branches: prefer
  third_party_actions:
    require_sha_pin: true
```

These are consumed by capability resolution and planning.

## Intent domains

### Merge

Express desired behavior:

- serial vs parallel integration
- allowed merge methods
- merge queue preference/requirement
- linear history
- update-before-merge semantics

### Review

- approvals
- code owner requirements
- thread resolution
- stale review behavior
- last-push approval

### Checks

Intent should name capabilities where possible, not workflow filenames.

```yaml
checks:
  required:
    - capability: ci
    - capability: dependency-security
    - capability: scope-policy
```

Explicit contexts are an advanced escape hatch:

```yaml
checks:
  required_contexts:
    - context: build
      provider: github-actions
```

### Ref topology

Ref classes are semantic:

```yaml
refs:
  classes:
    work:
      desired_count: 3
      preferred_names: [work/feature, work/fix, work/chore]
    documentation:
      desired_count: 1
      preferred_names: [docs/content]
```

### Automation

- Dependabot/Renovate preference
- AI/bot presence
- slot synchronization
- security reporting
- release/deployment automation expectations

### Security

Intent-level security properties, not raw rules:

- no force push to default
- deny arbitrary branch creation
- require immutable Action refs
- minimize generated workflow permissions

## Overrides

Templates need controlled override capability.

Overrides are typed and validated. They must not become arbitrary JSON injection into GitHub payloads by default.

Possible model:

```yaml
overrides:
  template_parameters:
    work_slots: 4
  github:
    advanced_rules: []
```

The `github.advanced_rules` escape hatch, if implemented, should be clearly marked expert/less-portable and excluded from some recommendation guarantees.

## Detect-generated starter config

`regelverk detect --init-config` may generate a starter config, but it must separate:

- observed facts
- inferred suggestions
- unanswered questions

It must never silently convert an inference into an explicit user preference.

Example:

```yaml
suggestions:
  project_profile:
    ai_heavy: inferred

questions:
  expected_parallel_work:
    value: null
```

The final durable config should contain only intentional selections plus explicit accepted suggestions.

## Multi-repository composition

Conceptual form:

```yaml
schema: regelverket.config/v0

defaults:
  constraints:
    budget: free-only

repositories:
  Avkroken/a:
    template: { id: sequential-slots, version: 0.1.0 }
  Avkroken/b:
    template: { id: protected-trunk, version: 0.1.0 }
```

Resolution produces one desired/resolved policy per repo.

## Schema migration

Config schema versions are migrated explicitly.

Requirements:

- parser rejects unknown incompatible major schema versions
- migration command supports deterministic upgrades where possible
- migration never changes semantic intent silently
- migration notes explain renamed/deprecated fields

## Acceptance criteria

A v0 implementation must be able to express both current Avkroken standard topology and Bastion's extended ref classes without embedding GitHub numeric IDs or copying full REST ruleset JSON into config.

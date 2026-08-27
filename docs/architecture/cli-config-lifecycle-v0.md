# CLI & Configuration Lifecycle v0

Status: architecture draft
Date: 2026-08-27

## Purpose

Define the user-facing lifecycle without locking implementation language or final command syntax.

The CLI should expose the same engine used by interactive setup, automation and future integrations.

## Core lifecycle

```text
detect -> recommend/configure -> plan -> apply -> verify -> explain
```

Each stage should be independently runnable and machine-readable.

## `detect`

Read-only by default.

Responsibilities:

- inspect repository/account context
- scan branches/tags
- scan rulesets and effective branch rules where accessible
- scan repository merge settings
- scan Actions workflows, reusable workflows, local actions and relevant scripts/config
- detect check providers and workflow dependencies
- detect existing Regelverket manifest
- infer project characteristics with explicit confidence
- probe relevant capabilities and permissions

Outputs:

- normalized observed-state graph
- human summary
- unresolved/unknown facts
- suggested follow-up questions

Detection must not create policy resources.

## `recommend`

Inputs observed graph plus user profile constraints.

Outputs ranked compatible archetypes/templates with:

- why it fits
- required changes
- unavailable features
- paid/upgrade requirements
- security/operational trade-offs
- assumptions

Recommendation is advisory and never mutates.

## `init` / `configure`

Creates or updates desired configuration.

Modes:

### interactive

Ask only questions that detection could not answer and that materially affect policy.

### config-driven

Accept a checked-in config without interactive questions.

### detect-seeded

Generate a proposed config from observed state and recommendations, but do not claim user preferences were detected when they were not.

## Configuration principle

Config expresses intent, not a mirror of GitHub REST payloads.

Provisional example:

```yaml
schema: regelverket.config/v0
repositories:
  - repo: owner/name
    template:
      id: sequential-slots
      version: 0.1.0
    mode: adapt
    constraints:
      budget: free-only
      preserve_unmanaged: true
    parameters:
      work_slots:
        - work/feature
        - work/fix
        - work/chore
      documentation: true
```

Exact fields remain research-dependent.

## Config layering

Potential sources, highest priority last:

1. template defaults
2. organization/project profile defaults
3. repository config file
4. explicit CLI overrides

The final resolved config must be printable so hidden defaults never determine a mutation invisibly.

## Multi-repository config

A single config may target multiple repositories but each repo receives a separately resolved policy and plan.

Shared template/profile defaults should avoid duplication.

Example concept:

```yaml
defaults:
  constraints:
    budget: free-only

repositories:
  - repo: org/api
    template: protected-trunk
  - repo: org/docs
    template: documentation-simple
```

## `plan`

Always read-only.

Plan must show:

- creates/updates/deletes
- reused/shared resources
- ownership changes
- conflicts/blockers
- capability and cost implications
- unknown facts affecting safety
- operation ordering
- dependency impact
- verification strategy

Support both human and machine-readable output.

A saved plan should include fingerprints/preconditions so stale plans cannot be applied blindly.

## `apply`

Applies an approved/current plan.

Principles:

- revalidate preconditions before mutation
- snapshot before destructive operations
- order dependencies safely
- stop on stale/contradictory state
- avoid implicit destructive fallback
- verify critical transitions
- update manifest only after successful verification of relevant resources

Interactive confirmation behavior and non-interactive CI behavior should be separate modes.

## `verify`

Read-only validation of installed policy.

Two levels:

1. desired resource state vs observed resources
2. expected effective policy vs GitHub effective enforcement for representative refs where supported

Outputs drift, missing providers, unexpected effective rules and inaccessible verification areas.

## `explain`

Queries the graph and knowledge base.

Examples:

```text
regelverk explain main
regelverk explain check:build
regelverk explain workflow:ci
regelverk explain template:sequential-slots
```

Explain should answer both current-state and planned-state questions.

## `export`

Candidate command for later versions.

Possible uses:

- export observed state
- export normalized policy graph
- export rendered GitHub resources
- export diagnostic bundle without secrets

## Apply/adapt/replace semantics

Do not encode these as unrelated installers.

They are reconciliation policies:

### adopt

Preserve topology and explicitly take management of selected compatible resources.

### augment

Preserve unmanaged resources and add required policy around them.

### replace

Target the template's canonical topology where possible. Destructive differences require plan + approval.

Template `apply` and `adapt` are higher-level UX choices that resolve into reconciliation policies.

## Non-interactive operation

Automation needs deterministic behavior.

Requirements:

- no prompts when explicit non-interactive mode is selected
- fail on required decisions instead of guessing
- stable exit codes
- machine-readable plan/diagnostics
- explicit flags for destructive approvals
- no secrets in logs/export

## Exit categories

Exact numeric codes are future implementation detail, but semantic categories should exist:

- success/no changes
- success/changes applied
- plan contains changes
- user decision required
- capability unavailable
- conflict
- safety blocked
- authentication/permission failure
- verification failure
- unexpected internal error

## Schema versioning

Configuration, manifest, templates and knowledge base each require independent schema/version identifiers.

Migrations must be explicit. Older configuration should not silently acquire materially different policy semantics after an upgrade.

## Upgrade lifecycle

A future `upgrade` operation should be implemented through the same planner:

```text
old template/config + current state
  -> new template/config
  -> semantic plan
  -> approval
  -> apply
  -> verify
```

No special overwrite-based upgrade path.

## Security boundaries

- detect/recommend/plan/verify/explain are read-oriented and should request minimal permissions
- apply requests only permissions necessary for the specific plan
- config/template input is untrusted data
- arbitrary shell execution must not be part of the declarative config language
- generated workflow code is executable and must be reviewed/tested as such

## UX principle

Users should be able to start simple:

```text
detect -> choose recommended template -> review plan -> apply
```

Advanced users can control every stage with checked-in configuration and machine-readable output, but complexity must not be required for the common path.

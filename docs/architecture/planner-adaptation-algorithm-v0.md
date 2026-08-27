# Planner & Adaptation Algorithm v0

Status: architecture draft
Date: 2026-08-27

## Purpose

The planner converts observed repository state, user intent, selected template and capability constraints into a safe, explainable and idempotent change plan.

## Inputs

- observed repository/policy graph
- effective policy observations where available
- user/environment profile
- selected archetype/template and parameters
- capability/cost matrix
- ownership manifest
- knowledge-base constraints

## Output

A resolved desired graph plus an ordered plan of operations with reasons, dependencies, risk level, blockers, rollback metadata and verification checks.

## Planner phases

### 1. Detect and normalize

Build the observed graph. Preserve `unknown`, `inaccessible`, `inferred` and conflicting evidence.

### 2. Resolve capabilities

Evaluate each template requirement against account type, plan, visibility, permissions, enabled repository features and user cost constraints.

Result per requirement:

- satisfied
- satisfiable
- degraded_with_tested_fallback
- requires_upgrade
- permission_missing
- inaccessible
- unsupported
- unknown

### 3. Expand template intent

Convert archetype/template requirements into policy requirements and resource capabilities, without choosing files or GitHub resource IDs yet.

### 4. Bind existing resources

For every desired capability, search observed graph for compatible providers.

Preference order:

1. exact semantic reuse
2. safe composition with existing resources
3. safe adaptation of an existing resource
4. generation of a dedicated Regelverket resource
5. conflict/block

Binding decisions must be deterministic for unchanged inputs.

### 5. Evaluate constraints

Run hard constraints first, then safety constraints, then advisory optimization rules.

A hard or safety violation can block planning before any mutation is proposed.

### 6. Build desired graph

Instantiate concrete refs, rulesets, workflows, bindings and ownership states. Keep logical IDs stable.

### 7. Semantic diff

Compare normalized observed and desired graphs. Raw YAML/JSON differences alone do not imply an update.

### 8. Impact analysis

Walk dependency edges for every proposed change.

Examples:

- changing job name affects required check bindings
- moving a reusable workflow affects callers
- deleting a branch affects open PRs
- enabling merge queue requires compatible required workflows

### 9. Classify operations

Operations:

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

Risk classes:

- informational
- low
- medium
- high
- destructive

### 10. Order plan

Order by dependency and reversibility.

General principle:

1. preflight/permission checks
2. snapshots/backups
3. non-disruptive prerequisites
4. new providers/resources
5. policy references to those providers
6. destructive replacements/removals
7. verification
8. manifest update

Never enforce a required dependency before the provider is available.

## Adaptation rules

### REUSE

Use when an unmanaged/shared resource already satisfies the semantic requirement.

No content modification.

### COMPOSE

Use when multiple existing resources together satisfy a requirement without mutation.

### ADAPT

Use when a small, mechanically understood change to an existing resource can satisfy the requirement.

Default behavior: propose diff and require approval before modifying unmanaged resources. If approved, ownership may remain unmanaged or become adopted depending on the requested mode.

### GENERATE

Create a dedicated managed resource when reuse/adaptation is unsafe or unavailable.

### CONFLICT

Use when competing semantics cannot be safely reconciled automatically.

A conflict is a first-class plan result, not an exception to hide.

## Example: workflow collision

Desired capability: slot synchronization.

Observed path `.github/workflows/regelverk-slot-sync.yml` exists and is unmanaged.

Planner:

1. determine whether existing workflow satisfies slot-sync capability
2. if yes, bind as shared
3. if no, do not overwrite
4. create deterministic alternate managed path
5. store binding in manifest

## Example: existing CI and merge queue

Observed CI emits required `build` for `pull_request` but has no `merge_group` support.

Template requires merge queue.

Possible plans:

- ADAPT existing CI: add compatible `merge_group` trigger, with explicit diff
- GENERATE separate provider only if the resulting required-check identity is valid and unambiguous
- BLOCK if neither preserves CI semantics safely

Planner must explain the dependency chain.

## Unknown data policy

Unknown facts have different effects by operation.

Examples:

- unknown organization ruleset: may allow read-only recommendation but blocks destructive policy replacement
- unknown billing capability: can permit detection but cannot claim a paid feature is available
- unknown workflow behavior: cannot be silently adopted as required-check provider

The planner uses a risk-dependent evidence threshold.

## Multi-repository planning

A future multi-repo operation must create:

- one repository plan per repo
- one aggregate plan showing shared assumptions and failures

Failure in one repo must not silently change the semantics applied to another.

Atomic cross-repository mutation is not assumed.

## Explainability contract

Every proposed operation must expose:

- what changes
- why it changes
- which template/policy requirement caused it
- what existing resource was reused or rejected
- dependencies
- capability/cost implication
- risk
- verification method

## Idempotence contract

Given equivalent observed state, desired config, template version and capability evidence, planner output must be semantically identical.

After successful apply and verification, immediately replanning must produce NOOP except for explicitly volatile observations.

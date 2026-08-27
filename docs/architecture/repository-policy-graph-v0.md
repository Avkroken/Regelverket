# Repository / Policy Graph v0

Status: architecture draft
Date: 2026-08-27

## Purpose

Regelverket must reason about repositories semantically rather than as collections of JSON and YAML files. The normalized graph is the shared internal representation used by discovery, recommendation, template adaptation, planning, application, verification, and explain.

## Core rule

GitHub resources are observations and deployment targets. They are not the internal model.

The graph must preserve provenance and uncertainty. A missing value is never automatically false.

## Node families

### Repository

Stable identity, owner type, visibility, default branch, enabled merge methods, detected project characteristics, and GitHub capability context.

### Ref and RefClass

A Ref is a concrete branch or tag. A RefClass is a semantic set such as default, work, documentation, release, automation, deployment, or template-defined custom class.

A RefClass can resolve to explicit refs and/or GitHub include/exclude patterns.

### Ruleset and Rule

Rulesets are normalized independently from their raw REST representation. Rules retain target, enforcement, conditions, parameters, source level, bypass policy, and provenance.

### Workflow, Job, CheckProvider

Workflow represents an Actions workflow or compatible external automation. Job represents an executable/check-producing unit. CheckProvider maps a required check identity to the workflow/job/App that can emit it.

### Actor and Capability

Actors include users, teams, Apps, Dependabot-like automation, and GitHub-owned actors when they can be identified. Capabilities represent what an actor or environment can safely perform.

### FileResource

Repository files relevant to governance: workflows, reusable workflows, local actions, scripts, CODEOWNERS, dependency configuration, deployment configuration, and Regelverket manifests.

### Environment / DeploymentTarget

Represents GitHub environments and other deployment relationships required to understand deployment gates.

### PolicyRequirement

An intent-level requirement independent of implementation, for example:

- default branch cannot be force-pushed
- changes to default branch require CI
- work slots must stay synchronized with default
- documentation work must be isolated
- required CI must also execute for merge queue candidates

### Evidence

Every important fact can reference evidence with source, timestamp, confidence and access state.

## Edge families

- `targets`: ruleset -> ref/refclass
- `contains`: ruleset -> rule
- `requires`: rule/policy -> capability/check/deployment
- `emits`: workflow/job -> check
- `calls`: workflow -> reusable workflow/local action/script
- `reads`: workflow/script -> file/config
- `writes`: workflow/script -> ref/file/resource
- `bypasses`: actor -> ruleset/rule
- `implements`: resource -> policy requirement
- `depends_on`: generic semantic dependency
- `conflicts_with`: incompatible resources or requirements
- `equivalent_to`: semantically equivalent implementation
- `derived_from`: desired resource -> template/archetype
- `managed_by`: resource -> Regelverket manifest identity

## Observed, desired, effective

The graph has three overlays rather than three unrelated models.

### Observed

What discovery can prove exists now.

### Desired

What configuration plus selected template requires.

### Effective

What GitHub actually enforces after layering repository and organization policy. Effective policy should use GitHub's effective-rule APIs where possible and otherwise be marked inferred.

This distinction is required for safe planning.

## Provenance model

Every non-trivial property should be able to carry:

```yaml
value: true
state: known
source:
  kind: github_api
  endpoint: rules-for-branch
observed_at: 2026-08-27T00:00:00Z
confidence: authoritative
```

Possible states:

- known
- unknown
- inaccessible
- unsupported
- inferred
- conflicting

## Semantic identities

Resource identity must not depend on filenames or GitHub numeric IDs.

Examples:

```text
repo.default
refclass.work.feature
policy.default.no-force-push
workflow.capability.ci
check.ci.build
ruleset.default-protection
```

External IDs and paths are bindings attached to these identities.

## Normalization

Normalization converts GitHub-specific representations into comparable semantic forms. Examples:

- sort set-like collections
- normalize include/exclude conditions without losing GitHub matching semantics
- separate check identity from workflow filename
- retain expected App/source identity for checks
- represent organization and repository rules as distinct sources even when effective behavior overlaps
- normalize equivalent explicit/default API values

Normalization must never simplify away a distinction known to affect enforcement.

## Dependency graph example

```text
PolicyRequirement: default branch requires build
    -> Rule: required_status_checks
       -> Check: build
          -> Provider: workflow.ci / job.build
             -> Workflow: .github/workflows/ci.yml
                -> trigger: pull_request
                -> trigger: merge_group
```

A rename of `job.build` can therefore be recognized as a policy-impacting change before it is applied.

## Graph queries required by v0

- Which rules effectively apply to ref X?
- Which resources implement requirement Y?
- Which required checks have zero, one, or multiple providers?
- What will break if resource X changes or disappears?
- Which resources are unmanaged/shared/adopted/managed?
- Which observed resources can satisfy template requirement Y without modification?
- Which desired resources cannot be implemented with current capabilities?
- Which unknown/inaccessible facts make a destructive plan unsafe?

## Design constraint

The graph is an internal contract. CLI, YAML schema and GitHub REST payloads must be adapters around it, not replacements for it.

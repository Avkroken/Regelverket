# Core design principles

Status: architecture baseline, subject to revision when research contradicts an assumption.

## 1. New implementation from first principles

The new Regelverket implementation may be written from scratch. Avkroken rulesets/workflows v24.2 is a reference implementation and regression source, not a codebase that constrains the new design.

## 2. Configuration is policy; generated files are artifacts

The desired configuration should be the source of truth. GitHub ruleset JSON, workflow YAML and related files are rendered/adapted artifacts.

Users should not have to manually keep several files semantically synchronized.

## 3. Templates are a core product feature

Regelverket must provide ready-made, versioned, documented templates/archetypes that can be:

1. applied substantially as designed, or
2. adapted to the repository's existing structure.

Each template should document purpose, suitable repository/project types, required capabilities, plan/cost constraints, branch/workflow model, security properties, trade-offs, failure modes and E2E coverage.

Proposed conceptual hierarchy:

```text
Archetype → Template → Resolved Policy
```

An archetype describes a working model. A template is a tested implementation for a class of environments. A resolved policy is the concrete policy after capability detection and repository adaptation.

## 4. Detect before mutate

Detection is a first-class subsystem. It should inspect the repository and relevant GitHub environment before planning changes.

Candidate inputs include:

- default branch and branch topology,
- existing rulesets and branch protection,
- effective organization-level policy where observable,
- workflows, triggers, jobs and required checks,
- reusable workflows and local actions,
- scripts/configuration referenced by workflows,
- Dependabot/Renovate and other bots,
- CODEOWNERS,
- environments/deployments,
- merge settings and merge queue,
- repository visibility/account ownership,
- permissions and feature/capability availability,
- open pull requests where migration safety depends on them.

Detection must distinguish at least: `present`, `absent`, `unknown`, `inaccessible` and `unsupported` where relevant.

## 5. Semantic reuse before generation

Adaptation should prefer:

```text
REUSE → COMPOSE/ADAPT → GENERATE → CONFLICT
```

A pre-existing workflow that already provides a required capability should be reused instead of duplicating it.

File names are not sufficient to decide equivalence. Regelverket should match semantic capability first, path/name second.

## 6. Dependency graph as the internal model

Repository resources should form a normalized graph rather than a loose set of files.

Example:

```text
ruleset.main
  → requires check ci/build
  → emitted by workflow.ci
  → job.build
  → calls reusable workflow/action/script
```

The graph should represent include/exclude rules, branch/ref targets, workflows, jobs/checks, actors, Apps, files, scripts, environments and relevant dependencies.

This graph is the basis for planning, adaptation, conflict detection, explanation and verification.

## 7. Stable resource identity and explicit ownership

Regelverket-created resources need stable internal resource IDs independent of file paths and GitHub-generated IDs.

Resources should be classified as:

- `unmanaged`: pre-existing and not controlled by Regelverket,
- `shared`: used by Regelverket but not owned by it,
- `adopted`: pre-existing but explicitly transferred to Regelverket management,
- `managed`: created/owned by Regelverket.

A manifest can store resource identity, template version, resolved path/GitHub ID and last known managed state.

## 8. Conflict-safe workflow naming

GitHub executable workflow files must remain directly under `.github/workflows/`.

Normal generated names should be deterministic and readable, for example:

```text
.github/workflows/regelverk-scope-policy.yml
.github/workflows/regelverk-slot-sync.yml
```

If an unmanaged resource already occupies the deterministic name, Regelverket should not overwrite it. A stable conflict suffix, potentially derived from resource identity, may be used. Content hashes should not be the normal naming scheme because content changes must not create orphan workflows.

Non-executable Regelverket state/metadata may live under a dedicated directory such as `.github/regelverk/`.

## 9. Idempotent reconciliation

Applying an unchanged desired state repeatedly must not create duplicate branches, rulesets, workflows or other resources.

The fundamental loop is:

```text
detect
  → normalize observed state
  → resolve template/config
  → compute desired state
  → compare desired vs actual
  → plan
  → apply approved changes
  → verify
```

A second run after a successful apply should normally report `no changes`.

## 10. Drift is explicit

If a managed resource is manually modified, Regelverket should report drift rather than silently overwrite it by default.

Shared resources must not be modified without explicit adoption/approval.

## 11. Migration is planned, not destructive by default

Candidate migration modes:

- `adopt`: model and manage selected existing resources,
- `augment`: preserve existing model and add capabilities,
- `replace`: migrate toward a selected policy model.

`replace` does not mean delete-first. It must inventory dependencies, open PRs and other blockers, create a plan/snapshot, and require explicit approval for destructive operations.

## 12. Capabilities and cost are first-class constraints

The system must account for account type, organization/enterprise context, repository visibility, permissions, GitHub feature availability and user budget constraints.

A user preference such as `free-only` is a hard constraint on recommendations, not a cosmetic UI filter.

Capability results should be able to represent states such as:

- available,
- requires upgrade,
- missing permission,
- unavailable for current account/repository type,
- unknown because the current credentials cannot observe it.

## 13. User intent is separate from detected facts

Regelverket should detect what can be observed and ask only questions it cannot safely infer.

Examples of intent/profile data:

- solo developer vs team,
- human/AI/bot mix,
- desired parallelism,
- review strictness,
- security level,
- release/deployment workflow,
- willingness to pay for GitHub capabilities,
- how much existing structure may be changed.

## 14. Constraints encode platform knowledge

A constraint engine should encode documented and experimentally confirmed relations instead of enumerating every possible ruleset combination.

Examples include:

- merge queue + required GitHub Actions check implies merge-group workflow support,
- linear-history requirements imply compatible merge methods,
- a required check should have a resolvable, non-ambiguous provider,
- unknown/inaccessible higher-level policy should block unsafe destructive assumptions,
- workflow/resource naming conflicts require alternate paths or explicit adoption.

## 15. Plan and explain semantically

Plans should explain why a change is required, not just show text diffs.

Example:

```text
UPDATE .github/workflows/ci.yml
Reason: selected policy requires merge queue and the required `build` check
must run for merge-group commits.
Dependent resource: ruleset.main → required check `build`.
```

The same dependency model should support an `explain` command for policy troubleshooting.

## 16. Verification has two perspectives

Verification should compare:

1. actual managed resources against desired state, and
2. the effective policy GitHub reports for representative refs/branches.

This reduces the risk of declaring success merely because a ruleset payload was accepted while overlapping organization/repository rules produce a different effective result.

## 17. Modular, capability-driven loading

Detection should reduce the question and module space. Modules should declare the detectors, questions, capabilities, constraints, templates and renderers they provide.

A simple individual repository should not need to process irrelevant enterprise-only concerns.

## 18. Testing is architectural

Minimum planned layers:

- unit tests for normalized models and constraints,
- golden/render tests,
- simulated reconciliation tests (`observed + desired → plan`),
- live E2E tests in disposable GitHub repositories.

Avkroken v24.2 should become a regression fixture whose intended semantics can be reproduced from a Regelverket configuration without requiring byte-for-byte reproduction of the old files.

## 19. Privileged execution follows least privilege

The project requires a dedicated security/threat-model phase because it will manage repository governance with elevated permissions.

Threats include compromised tokens/Apps, malicious workflow changes, workflow self-modification, action supply chain, bypass abuse, config/template injection, TOCTOU between plan/apply, concurrent mutations, force-push races, API partial failure and rollback failure.

## 20. Shell is not the core architecture

A shell script may be used for bootstrap where appropriate, but the core needs a real typed/structured model capable of parsing YAML/JSON, building dependency graphs, querying APIs, performing deterministic reconciliation and running substantial tests.

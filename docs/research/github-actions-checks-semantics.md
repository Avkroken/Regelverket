# GitHub Actions and required-check semantics

Status: research baseline
Date: 2026-08-27

This document captures behavior that the discovery, dependency-graph, constraint and adaptation engines must understand before Regelverket may safely reuse or modify existing workflows.

## Required checks are runtime resources

A required status check is not equivalent to a workflow filename. Regelverket must model at least:

- workflow identity/path
- workflow display name
- event that produced a run
- job/check identity
- matrix expansion
- producing GitHub App/source where relevant
- commit/SHA against which the check is reported
- branch/merge-group context

Therefore adaptation must match semantic capability and runtime check production, not filename equality.

## Required workflow skipped by filters can deadlock a PR

GitHub documents that when a workflow is skipped due to branch filtering, path filtering, or commit-message skipping, checks associated with the workflow can remain Pending. If such a check is required, the pull request is blocked.

Constraint candidate:

```text
IF check.required = true
AND provider = github_actions
AND provider.workflow_can_be_skipped_by_top_level_filter = true
THEN warn/error unless an always-reporting check design is proven
```

Template guidance: do not make a top-level path-filtered workflow itself the sole provider of a universally required check.

## Merge queue requires merge_group execution

GitHub explicitly requires workflows that provide required GitHub Actions checks for merge queue to listen to `merge_group`. `pull_request` and `push` do not substitute for it.

Hard constraint candidate:

```text
IF merge_queue.enabled
AND required_check.provider = github_actions
THEN provider.workflow.events includes merge_group
```

This must be verified against the resolved provider, not inferred from a workflow name.

## Expected GitHub App/source

GitHub can require a status check from a specific GitHub App. A check with the correct textual name from the wrong source does not satisfy that requirement.

Internal identity should therefore be closer to:

```text
CheckRequirement {
  name
  expected_source? // app/integration identity
}
```

not simply `required_checks: ["build"]`.

## Duplicate/ambiguous providers

GitHub documentation warns against using the same job name in multiple workflows when the name is used as a required status check because ambiguous results can block pull requests.

Discovery must build a provider index:

```text
check-name -> [workflow/job providers]
```

Classification:

- one proven provider: resolvable
- multiple mutually exclusive providers: needs runtime proof
- multiple possible providers: ambiguous
- no provider: unsatisfied

Regelverket should refuse to create a new generated job whose check identity collides with an unmanaged provider unless the user explicitly resolves the conflict.

## Matrix jobs

Matrix expansion can affect the runtime checks presented to branch protection/rulesets. Detection cannot treat the YAML job key alone as sufficient evidence of the final check identities.

Research/live-test requirements:

- exact check names emitted for common matrix forms;
- `name:` interaction at workflow and job levels;
- matrix include/exclude behavior;
- whether a stable aggregator job is preferable as the required check.

Template design hypothesis: prefer one stable, non-matrix aggregator/gate job as the required check while matrix jobs feed it through `needs`.

## Reusable workflows

Reusable workflows are referenced with `jobs.<job_id>.uses`. GitHub requires reusable workflow files to live directly in `.github/workflows`; subdirectories there are not supported for reusable workflow files.

The dependency graph must distinguish:

- caller workflow
- called reusable workflow
- local action
- external action

A reusable workflow may be shared across multiple entry workflows, so file ownership and dependency ownership are separate concepts.

## Ruleset workflows are a different primitive

Ruleset-required workflows are not merely named status checks. GitHub documents support for `pull_request`, `pull_request_target`, and `merge_group` events. When invoked as ruleset workflows, filters on supported events are ignored and the default activity types are used.

Consequences:

- the scanner must classify a workflow as ordinary event-driven, reusable, ruleset-required, or combinations thereof;
- adaptation must not assume ordinary path/branch filters retain their meaning when the workflow becomes a ruleset workflow;
- ruleset workflow requirements must include repository ID + path + ref/SHA identity where appropriate.

## Dependency graph requirements

For each workflow, parse at least:

- path
- `name`
- `on` events
- event branch/path/type filters
- workflow-level permissions
- concurrency
- jobs
- job `name`
- job `needs`
- job `if`
- job `uses`
- environment
- matrix strategy
- runner labels
- step `uses`
- step scripts
- working directories
- referenced local paths
- secrets and permission requirements at a capability level (never store secret values)

The graph should create edges such as:

```text
Ruleset -> CheckRequirement -> CheckProvider -> Job -> Workflow
Workflow -> ReusableWorkflow
Workflow -> LocalAction
Workflow -> Script
Workflow -> Environment
Workflow -> RepositoryPath
```

## Safe adaptation rules

Suggested order:

1. REUSE: existing provider already satisfies capability.
2. COMPOSE: generated Regelverket gate can depend on existing jobs/workflows without changing their semantics.
3. ADAPT: modify an existing workflow only with an explicit semantic diff and ownership/approval rules.
4. GENERATE: create a dedicated Regelverket workflow with deterministic conflict-free identity.
5. CONFLICT: stop when safety cannot be proven.

A filename collision is not enough reason to modify an existing file.

## Generated workflow naming

Default examples:

- `.github/workflows/regelverk-scope-policy.yml`
- `.github/workflows/regelverk-slot-sync.yml`

If the deterministic path is occupied by an unmanaged resource, use a stable identity-derived suffix rather than a content hash, for example:

- `.github/workflows/regelverk-scope-policy-3f19c2.yml`

Changing generated content must not change the resource path.

## Idempotence requirements

Given unchanged configuration and repository state:

```text
apply # creates/updates desired resources
apply # MUST produce no changes
```

If an owned workflow drifts:

```text
plan -> DRIFT, not silent overwrite
```

If a shared workflow changes but still satisfies the capability:

```text
plan -> no managed-file change; refresh observed provider metadata
```

If a shared workflow changes and no longer satisfies a required capability:

```text
plan -> capability unsatisfied + alternatives; never silently mutate shared file
```

## Verification strategy

Static verification:

- YAML parses
- dependency graph resolves
- no generated identity collision
- required check providers are resolvable
- merge-queue providers support merge_group
- local/reusable workflow references resolve

Runtime verification:

- representative PR produces expected checks
- merge-group run produces required checks when queue is used
- expected App/source matches
- required checks report against expected SHA/context
- effective branch rules show expected requirement

## Open experiments

- Matrix runtime check naming across job `name` variants.
- Same check name emitted by two workflows: exact satisfaction/blocking behavior.
- Conditional job skip versus workflow skip for required checks.
- Reusable workflow caller/callee check naming.
- Ruleset workflow behavior when created after a PR is already open.
- `pull_request_target` safety constraints for templates.
- Fork PR permission/secret behavior for generated workflows.
- Concurrency cancellation and required-check terminal states.
- Renaming a workflow/job that currently supplies a required check.
- Interaction between expected App and GitHub Actions-generated checks.
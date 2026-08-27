# Workflow Dependency Analyzer v0

Status: architecture draft
Date: 2026-08-27

## Purpose

Regelverket must understand workflows by behavior and dependency, not filename. This analyzer maps GitHub Actions configuration into the repository graph so templates can reuse compatible automation and planners can predict policy impact.

## Inputs

Primary inputs:

- `.github/workflows/*.yml` and `*.yaml`
- local reusable workflow references
- local actions (`action.yml` / `action.yaml`)
- referenced scripts/config where statically resolvable
- recent Actions/check runtime observations where available

## Parsed workflow model

For each workflow retain:

- logical/path identity
- display `name`
- events under `on`
- event-specific branch/path filters
- workflow permissions
- environment variables relevant to references
- concurrency
- jobs
- reusable workflow call status

For each job retain:

- job ID
- displayed `name` expression
- runner or reusable call
- `needs`
- `if`
- strategy/matrix
- environment
- permissions overrides
- steps
- outputs
- callable workflow inputs/secrets where applicable

## Dependency edges

### workflow -> workflow

Local and external reusable workflow calls via `jobs.<id>.uses`.

Local calls can be analyzed recursively. External calls are represented as external dependencies with pinned/ref metadata.

### workflow/job -> action

`uses:` actions, including local actions and third-party actions.

### workflow/job -> script

Statically resolvable `run` references to repository scripts. The analyzer should avoid pretending arbitrary shell is fully understood.

### workflow/job -> environment

Environment deployment/review dependencies.

### job -> job

`needs` ordering.

### rule -> check -> provider

Bindings discovered by static + runtime analysis connect required status checks to providers.

## Check identity analysis

A critical product goal is to determine whether a required check has:

- zero known providers
- exactly one provider
- multiple possible providers
- dynamically named/ambiguous providers

Static job ID is not sufficient. Display names, matrix expansion, reusable workflows and source App observations can affect check identity.

Confidence must therefore be attached to provider bindings.

## Merge queue compatibility

For a check capability used by a merge-queue-protected target, the analyzer must determine whether the provider can run for `merge_group`.

Possible outcomes:

- compatible
- incompatible
- conditionally compatible
- unknown

A workflow with only `pull_request` support must not silently be reused as merge-queue CI.

## Filter analysis

Branch/path filters influence whether a required check will ever be emitted.

The analyzer records filters and asks constraint evaluation to determine whether a required-check configuration can leave a merge permanently pending.

It must not over-simplify GitHub glob semantics.

## Matrix jobs

Matrix jobs can generate dynamic check names and multiple checks from one definition.

Analyzer responsibilities:

- preserve matrix dimensions/expressions
- identify statically enumerable names where possible
- mark dynamic names as unresolved where not possible
- use runtime observations to refine bindings
- warn when template requirements rely on an unstable or ambiguous matrix-generated check identity

## Reusable workflows

Reusable workflows are treated as executable graph components, not copied inline.

The graph tracks:

- caller workflow/job
- callee reference and version/ref
- inputs
- secrets inheritance/mapping
- permissions interaction
- callee jobs/checks where local or inspectable

Local reusable workflows are strong candidates for reuse by templates.

## Semantic capability matching

The analyzer emits capability evidence such as:

```text
capability: ci.test
provider: workflow.ci/job.test
supports: pull_request, merge_group
paths: all
confidence: runtime_observed
```

Templates bind to capabilities, not filenames.

Possible capability families include:

- build
- test
- lint
- typecheck
- security_scan
- scope_validation
- deployment
- documentation_build
- dependency_update

The catalog remains extensible and is not equivalent to job names.

## Safe adaptation classes

When a required capability is missing:

### Reuse

Existing resource satisfies it without modification.

### Compose

A new Regelverket workflow can call an existing reusable workflow/action/script without modifying the source resource.

### Adapt

Existing file can be safely extended, but this requires explicit ownership/approval policy and a semantic diff.

### Generate

Create a dedicated Regelverket workflow.

### Conflict

No safe automatic resolution.

## Generated workflow path policy

Runnable files remain directly in `.github/workflows/`.

Preferred deterministic paths:

```text
regelverk-<capability>.yml
```

Stable suffixing is used only for collision resolution with unmanaged resources.

## External dependency security

The analyzer records for external Actions/workflows:

- owner/repository
- requested ref
- whether ref is a full commit SHA
- source classification

Security policy can later require SHA pinning or flag mutable references.

## Dynamic-analysis boundaries

Static YAML analysis cannot reliably understand arbitrary scripts, generated workflow content, runtime expression values, or external reusable workflow internals without retrieval.

The correct result is `unknown`, not guessed behavior.

## Required queries

- Which workflow/job emits required check X?
- Does provider X run for event Y?
- What calls file/workflow/action X?
- Which required policy depends on workflow X?
- Can template capability Y reuse existing automation?
- Which workflows are likely duplicates by semantic capability?
- Would renaming/deleting job X break a ruleset?
- Are any required checks vulnerable to filter-induced pending states?

## v0 success criteria

The v0 analyzer should correctly handle ordinary jobs, matrices, local reusable workflows, local actions, filters, `needs`, merge-group compatibility, and runtime check-provider correlation while explicitly reporting unsupported dynamic cases.

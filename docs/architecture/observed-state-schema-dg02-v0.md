# DG-02 Observed State Schema v0

Status: architecture contract draft
Date: 2026-08-27

## Purpose

DG-02 is the machine-readable contract produced by `regelverket detect`. It represents what Regelverket can prove, infer, or cannot access about a repository at one observation point.

It is not desired configuration and not a migration plan.

## Core principles

1. Facts, inferences and unknowns are distinct.
2. Missing data is not false.
3. External GitHub IDs are observations, not stable logical identities.
4. Provenance is retained for every material fact.
5. The document is deterministic after normalization.
6. The schema is versioned independently of templates and CLI versions.

## Top-level shape

Conceptual YAML:

```yaml
schema: regelverket.observed-state/v0
observed_at: 2026-08-27T17:39:00+02:00
repository:
  identity: ...
  account: ...
  settings: ...
capabilities: ...
refs: ...
rulesets: ...
effective_policy: ...
workflows: ...
check_providers: ...
files: ...
project_signals: ...
actors: ...
environments: ...
open_work: ...
uncertainties: ...
evidence: ...
normalization: ...
```

## Observation wrapper

Material fields use an observation wrapper when uncertainty or provenance matters:

```yaml
state: known
value: true
source:
  kind: github_api
  locator: GET /repos/{owner}/{repo}/rulesets
confidence: authoritative
observed_at: ...
```

Allowed states:

- `known`
- `inferred`
- `unknown`
- `inaccessible`
- `unsupported`
- `conflicting`

`known` requires direct evidence. `inferred` requires derivation metadata.

## Repository identity

```yaml
repository:
  logical_id: repo.current
  github:
    id: 1288661408
    full_name: Avkroken/bastion
    owner:
      login: Avkroken
      type: Organization
    visibility: public
    default_branch: main
```

The logical ID is local to the document; GitHub numeric IDs remain external bindings.

## Repository settings

Observed settings should include when accessible:

- default branch
- visibility
- archived state
- merge methods
- auto-merge availability
- update-branch behavior
- Actions enabled/permission policy if accessible
- security/code-scanning features if accessible

Unavailable settings are represented explicitly.

## Capabilities

Capabilities are resolved observations, not product recommendations.

```yaml
capabilities:
  merge_queue:
    state: known
    value: available_and_configured
    evidence: [ev.ruleset.main.merge_queue]
  repository_rulesets:
    state: known
    value: available
```

Possible resolved values are capability-specific but must distinguish at least:

- available
- available_and_configured
- unavailable
- requires_upgrade
- permission_missing
- inaccessible
- unknown

## Refs

Concrete refs:

```yaml
refs:
  - id: ref.main
    kind: branch
    name: main
    sha: ...
    classes: [default]
```

Ref classes are inferred semantic labels and therefore carry derivation evidence.

Examples:

- default
- work
- documentation
- automation
- release
- platform.android
- platform.apple

## Rulesets

Rulesets preserve both external representation and normalized semantic form.

```yaml
rulesets:
  - id: ruleset.main
    github_id: 21637223
    name: Protect main
    source:
      level: repository
      repository: Avkroken/bastion
    target: branch
    enforcement: active
    conditions:
      include: ["~DEFAULT_BRANCH"]
      exclude: []
    rules:
      - type: deletion
      - type: non_fast_forward
      - type: required_linear_history
      - type: required_status_checks
        parameters:
          strict: false
          do_not_enforce_on_create: true
          checks:
            - context: xcodegen-and-build
              integration_id: 15368
```

Rule ordering is normalized where order is not semantically meaningful.

## Effective policy

Effective policy is represented separately from raw rulesets.

```yaml
effective_policy:
  samples:
    - ref: ref.main
      state: known
      rules: [...]
      source: github_effective_rules_api
```

Where the effective API is unavailable, state is `inferred` or `unknown` rather than fabricated.

## Workflows

```yaml
workflows:
  - id: workflow.osv
    path: .github/workflows/osv-scanner.yml
    blob_sha: ...
    name: OSV-Scanner
    events:
      pull_request:
        branches: [main]
      merge_group:
        types: [checks_requested]
      push:
        branches: [main]
      schedule:
        - cron: "0 5 * * 1"
    jobs:
      - id: job.osv.wrapper
        job_key: osv
        display_name: osv
        emits_checks: [check.osv]
```

Workflow IDs are semantic detector identities. Paths are bindings.

## Workflow dependency edges

DG-02 includes normalized edges when detectable:

```yaml
workflow_edges:
  - from: workflow.osv
    relation: calls
    to:
      kind: reusable_workflow
      locator: google/osv-scanner-action/.github/workflows/osv-scanner-reusable-pr.yml@6e4298...
```

Relations include:

- calls
- needs
- reads
- writes
- deploys_to
- emits

## Check providers

Check providers are first-class because ruleset contexts cannot safely be mapped from filenames alone.

```yaml
check_providers:
  - check:
      context: osv
      integration_id: 15368
    providers:
      - workflow: workflow.osv
        job: job.osv.wrapper
        events: [pull_request, merge_group]
        evidence_state: known
```

Provider cardinality is recorded:

- zero providers
- exactly one
- multiple/ambiguous
- unknown

## Files

Relevant repository files are inventoried with semantic classification:

```yaml
files:
  - path: .github/workflows/scope-policy.yml
    blob_sha: ...
    kind: github_workflow
    ownership: unmanaged
```

Ownership here is observed only if a Regelverket manifest exists. Otherwise it remains unmanaged/unknown according to evidence.

## Actors

Actors discovered from bypass/check/App relationships:

```yaml
actors:
  - id: actor.integration.4594645
    github:
      actor_id: 4594645
      actor_type: Integration
    identity:
      state: unknown
```

Names must not be guessed from numeric IDs.

## Project signals

Detector signals are separated from recommendations:

```yaml
project_signals:
  languages: [...]
  package_managers: [...]
  monorepo:
    state: inferred
    value: false
  automation:
    dependabot: ...
    ai_agent_signals: ...
  deployment_targets: [...]
```

Signals include evidence and confidence. They are inputs to recommendation, not conclusions.

## Open work

When available:

- open PRs
- source/target refs
- merge-queue participation
- dependencies on branches/workflows scheduled for change

This section supports migration safety.

## Uncertainties

All material unresolved facts are summarized:

```yaml
uncertainties:
  - id: uncertainty.org-parent-policy
    severity: blocking_for_destructive_change
    subject: effective organization rules
    reason: inaccessible
```

The planner consumes this list directly.

## Evidence registry

Evidence can be deduplicated:

```yaml
evidence:
  ev.ruleset.main:
    kind: github_api
    locator: https://api.github.com/repos/Avkroken/bastion/rulesets/21637223
    observed_at: ...
    authority: authoritative
```

Large raw payloads should not necessarily be embedded; digest + locator + relevant normalized fields are sufficient where reproducibility allows re-fetching.

## Normalization metadata

```yaml
normalization:
  schema_version: v0
  normalizer_version: 0.0.x
  semantic_digest: sha256:...
```

The digest is calculated from normalized semantic content excluding timestamps and other non-semantic observation noise.

## Determinism requirements

For identical observed semantics:

- maps use canonical key ordering in serialized fixtures
- set-like collections are sorted
- API-generated timestamps/URLs/IDs do not affect semantic digest unless identity/enforcement depends on them
- unknown/inaccessible states are stable explicit values

## Redaction

Observed state must be safe to save locally by default.

Never include:

- token values
- secrets
- private keys
- secret environment values

Potentially sensitive repository topology may still exist; upload/sharing is always user-controlled.

## DG-02 acceptance tests

The first implementation must prove:

1. the same repository observed twice without material changes normalizes to the same semantic digest
2. API list ordering changes do not change digest
3. inaccessible bypass actors do not become empty bypass lists
4. a required check with multiple providers is represented as ambiguous
5. a workflow filename rename with equivalent semantic provider can preserve logical provider identity after reconciliation
6. Bastion and a standard Avkroken repo can both be represented without schema exceptions

## Relationship to other contracts

```text
DG-02 Observed State
       +
User Intent / Config
       +
Template
       -> Resolved Desired Graph
       -> Plan
       -> Apply
       -> Verification Evidence
```

DG-02 remains read-only factual input throughout this lifecycle.

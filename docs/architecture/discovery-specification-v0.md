# Discovery Specification v0

Status: architecture draft
Date: 2026-08-27

## Purpose

`detect` is a first-class analysis phase. It must build an evidence-backed observed-state graph, not just prefill a config file.

Core principle:

> Discovery may report unknown or inaccessible. It must never convert lack of visibility into absence.

## Discovery outputs

A detection run should produce at least:

- normalized observed repository graph
- capability probe results
- workflow dependency graph
- policy/effective-rule observations
- project classification signals
- conflicts and ambiguities
- unanswered user-intent questions
- evidence/provenance records

The persisted human-readable representation may be exposed as `observed-state.yaml`, but the internal graph remains authoritative.

## Probe classes

### Repository metadata

Detect:

- repository identity
- owner/account type
- public/private/internal visibility
- default branch
- archived/template/fork state where relevant
- allowed merge methods
- auto-merge/update-branch settings where observable
- repository topics/languages as weak classification signals

### Refs

Detect:

- branches and tags relevant to governance
- default branch
- open PR head/base refs
- stale/active branch signals where useful
- naming patterns
- branches that appear to form pools/classes

Do not classify semantic purpose from name alone if stronger evidence is unavailable.

### Rules and protection

Detect:

- repository rulesets
- organization/enterprise rules affecting the repository where observable
- classic branch protection
- ruleset targets and conditions
- bypass actors/modes when visible
- effective rules for representative concrete refs
- ruleset enforcement mode
- rule suites/insights where supported and useful

### Workflows

Detect and parse every runnable workflow directly under `.github/workflows/`.

Extract:

- workflow display name
- path
- triggers/events
- branches/branches-ignore
- paths/paths-ignore
- permissions
- concurrency
- jobs
- job display names and IDs
- matrix definitions
- `needs`
- environments
- reusable workflow calls
- local/external actions
- scripts/commands where safely analyzable
- outputs and check-producing behavior where inferable

### Reusable workflows and local actions

Follow local references and build graph edges. Record unresolved dynamic references as unknown rather than guessing.

### CI/check observations

Where API access permits, observe recent workflow runs/check runs/statuses to map configured jobs to actual check identities and source Apps.

Static analysis and runtime observation must remain separate evidence types.

### Dependency automation

Detect signals for:

- Dependabot
- Renovate or similar bots when identifiable
- scheduled security/dependency workflows
- GitHub security features where observable

### Documentation

Detect common documentation structure, docs-specific workflows/builds, and branches only as project signals. Documentation presence should not by itself imply a branch model.

### Deployment

Detect:

- GitHub environments
- deployment-related workflows
- deployment gates
- common provider config when a detector module supports it

Provider-specific analysis should be modular.

### Ownership/review

Detect CODEOWNERS and review-related policy/workflow signals. Team structure and intended review culture remain user intent unless proven elsewhere.

## Evidence levels

Each observation receives provenance and confidence.

Suggested confidence classes:

- authoritative_api
- authoritative_file
- runtime_observed
- strongly_inferred
- weakly_inferred
- user_declared

Inferences must retain the evidence they were based on.

## Access/error states

A probe result can be:

- observed
- absent
- inaccessible
- unsupported
- rate_limited
- transient_error
- malformed
- ambiguous
- not_probed

Only `absent` means evidence positively established absence.

## Representative-ref strategy

Effective policy cannot be inferred safely by reading only stored rulesets. Discovery should choose representative refs for every relevant RefClass and query effective rules where GitHub exposes that capability.

Example set:

- default branch
- one ref per detected work/documentation/release class
- refs involved in open PRs
- template-requested future names where preflight evaluation is possible

## Detection modes

### quick

Low-cost facts sufficient for initial user questions and template filtering.

### full

Repository-wide analysis including workflow dependency parsing, effective-policy probes, check-provider observations, and classification.

### targeted

Re-run only detectors affected by a stale plan, changed files, or selected template capability.

The CLI can decide defaults later; the architecture must allow incremental detection.

## Modular detector contract

Each detector declares:

```text
id
inputs
required_permissions
cost/rate profile
produced node/edge types
confidence policy
dependencies
failure semantics
cacheability
```

Detectors must not mutate the repository.

## User questions after discovery

Questions should only request information discovery cannot reliably derive, for example:

- solo vs team intent
- expected parallel work
- budget/free-only policy
- willingness to move to organization/paid plan
- desired review strictness
- tolerance for generated files
- whether existing resources may be adopted/modified
- deployment/release goals when not evident

Questions irrelevant to the detected capability envelope should not be shown.

## Safety gates

Discovery must mark planning as unsafe for destructive adaptation when material policy facts are inaccessible or conflicting.

Example:

```text
Organization-level policy may affect main, but current token cannot read enough information to establish enforcement.
Result: replacement migration blocked; augment-only planning may still be possible.
```

## Idempotence and snapshots

Discovery itself is read-only and repeatable. A plan should record the observation snapshot/preconditions it was based on so apply can detect staleness.

## v0 success criteria

Discovery v0 is sufficient when it can accurately construct the inputs needed to:

1. render a repository inventory
2. identify obvious existing capabilities for reuse
3. flag ambiguous required-check providers
4. identify effective protection for representative refs
5. classify material unknown/inaccessible state
6. reduce the template catalog to plausible candidates
7. produce the minimum remaining user questions

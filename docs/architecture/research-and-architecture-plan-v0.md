# Research & Architecture Plan v0

Status: initial planning baseline.

This document defines what must be researched and modeled before the new Regelverket implementation is allowed to harden around assumptions.

## Goal

Build enough verified knowledge to design Regelverket as a generic GitHub repository-governance planner/compiler with safe template application, repository adaptation, idempotent reconciliation and effective-policy verification.

The research phase must separate four evidence classes:

- `documented`: explicitly stated by GitHub primary documentation/API docs,
- `observed`: reproduced in controlled tests,
- `inferred`: derived from documented/observed behavior but not directly confirmed,
- `unknown`: unresolved or inaccessible.

No inferred behavior should silently become a hard architectural fact.

## Phase 1 — GitHub factual platform model

Map current primary documentation and APIs for:

1. repository rulesets,
2. organization rulesets,
3. enterprise rulesets where relevant,
4. legacy branch protection and interactions with rulesets,
5. branch, tag, push and repository rule targets,
6. all available rule types and parameters,
7. include/exclude/ref matching semantics,
8. bypass actors and bypass modes,
9. merge strategies and repository merge settings,
10. merge queue,
11. status/check APIs and required status checks,
12. GitHub Actions workflow semantics,
13. reusable workflows and local actions,
14. environments/deployments,
15. CODEOWNERS and required reviews,
16. GitHub Apps and permission models,
17. Dependabot and common automation identities,
18. repository/organization permissions,
19. rule suites/insights/history,
20. plan/account/repository visibility feature availability,
21. pricing/cost constraints relevant to capabilities,
22. API versions, pagination, rate limiting and partial visibility.

### Deliverables

- machine-readable capability records,
- rule-type catalog,
- API surface map,
- permission matrix,
- plan/account/visibility capability matrix,
- list of unresolved questions that require live experiments.

## Phase 2 — Rule semantics and interaction model

For every supported rule or policy feature, document:

- target scope,
- parameters,
- prerequisites,
- exact behavior,
- what it does not guarantee,
- interactions with other rules,
- workflow/check dependencies,
- bypass behavior,
- required permissions,
- plan/account requirements,
- known failure modes,
- API representation,
- effective-policy observation method,
- verification strategy.

### Deliverable

A normalized rule knowledge base suitable for use by a constraint engine.

## Phase 3 — Workflow semantics and dependency analysis

Map GitHub Actions behavior required to safely reuse/adapt workflows.

Research and model:

- events/triggers,
- branch/path filters,
- `merge_group`,
- jobs and check names,
- matrix jobs,
- reusable workflows,
- local actions,
- `uses`, `needs`, `working-directory`,
- permissions,
- secrets,
- environments,
- concurrency,
- scripts/config files called by workflows,
- conditions/skipped jobs,
- provider ambiguity for required checks,
- naming/path constraints.

### Deliverable

Workflow capability model and dependency-graph schema.

## Phase 4 — Behavioral experiments

Build disposable test repositories and reproduce behavior where documentation/API shape is insufficient.

Initial experiment backlog:

- overlapping repository rulesets,
- organization + repository ruleset layering,
- legacy branch protection + rulesets,
- branch creation/update/deletion behavior,
- bypass actors and modes,
- inaccessible policy fields,
- strict vs loose required checks,
- check provider/App pinning,
- skipped workflow behavior,
- duplicate job/check names,
- matrix job naming,
- `merge_group` and merge queue,
- reusable workflow check behavior,
- bot/App/Dependabot identities,
- rule-suite observations,
- API partial failure,
- concurrent state changes during plan/apply.

### Deliverables

- reproducible experiment definitions,
- captured expected results,
- gotcha/constraint records linked back to evidence.

## Phase 5 — Real-world workflow research

Study how real projects structure repository governance.

Sources should include:

- GitHub's own ruleset recipes/examples,
- established open-source repositories,
- small team repositories,
- solo-maintained projects,
- monorepos,
- libraries/packages,
- web applications,
- deployment-heavy repositories,
- bot/AI-heavy repositories.

The objective is not popularity ranking alone. Capture:

```text
project characteristics
→ working model
→ branch topology
→ rules
→ workflows
→ merge/release behavior
→ trade-offs/failure modes
```

### Deliverables

- archetype candidates,
- evidence-backed recommendation signals,
- anti-pattern catalog.

## Phase 6 — Repository classification model

Define what `detect` can observe safely.

Candidate signals:

- languages/frameworks,
- package managers,
- monorepo structure,
- build/test/lint commands,
- documentation layout,
- deployment targets,
- release automation,
- branches/tags,
- workflows,
- Dependabot/Renovate,
- AI/bot automation,
- CODEOWNERS,
- contributor/team indicators where available,
- existing governance.

Every field must specify confidence/provenance. Detection must not turn guesses into facts.

### Deliverable

Observed repository profile schema.

## Phase 7 — User/environment intent model

Define the non-observable questions needed to select/adapt policy.

Candidate fields:

- individual/organization/enterprise,
- solo vs team,
- human/AI/bot composition,
- desired parallel work level,
- review strictness,
- security posture,
- release/deployment model,
- cost policy (`free-only`, paid acceptable, etc.),
- willingness to upgrade GitHub plan/account structure,
- tolerance for repository restructuring,
- preservation requirements for existing branches/workflows.

Questions should be lazy-loaded based on detected state and relevant capabilities.

### Deliverable

User/environment profile schema and question-routing rules.

## Phase 8 — Capability engine

Combine:

- GitHub plan/account/repository type,
- repository visibility,
- observed permissions,
- feature availability,
- cost policy,
- user intent.

Capability states must be richer than boolean. At minimum support:

- `available`,
- `requires-upgrade`,
- `missing-permission`,
- `unsupported`,
- `inaccessible`,
- `unknown`.

### Deliverable

Capability resolution model used by templates and recommendation.

## Phase 9 — Internal policy/dependency graph

Design a normalized graph for:

- repositories,
- refs/branch classes,
- rulesets/rules,
- checks,
- workflows/jobs,
- Apps/actors,
- environments/deployments,
- files/scripts/actions,
- dependency edges,
- include/exclude relations,
- ownership state.

### Deliverable

Versioned internal schema with serialization format for testing/debugging.

## Phase 10 — Template and archetype specification

Templates are required from the beginning.

Conceptual layers:

```text
Archetype
  → Template
    → Resolved Policy
```

Every template must declare:

- intent/purpose,
- suitable project/team types,
- assumptions,
- required/optional capabilities,
- cost/plan constraints,
- branch model,
- governance rules,
- workflow capabilities,
- adaptation rules,
- security properties,
- trade-offs,
- failure modes,
- verification probes,
- E2E tests.

Initial candidate archetypes should be researched rather than prematurely fixed. Sequential Slots/Avkroken is one evidence-backed starting point.

### Deliverable

Template schema + first validated archetype catalog.

## Phase 11 — Adaptation and conflict model

Define the order:

```text
REUSE → COMPOSE/ADAPT → GENERATE → CONFLICT
```

For every resource type specify:

- semantic compatibility rules,
- ownership rules,
- allowed safe edits,
- conflicts requiring explicit approval,
- naming fallback strategy,
- dependency updates required by moves/renames,
- rollback behavior.

Existing workflows with identical names must never be overwritten merely because a template uses that name.

### Deliverable

Adaptation decision matrix.

## Phase 12 — Resource identity, ownership and idempotence

Design stable resource IDs and classifications:

- unmanaged,
- shared,
- adopted,
- managed.

Define manifest/state strategy and drift behavior.

Required invariant:

```text
apply(desired)
apply(same desired)
→ second run produces no changes
```

### Deliverables

- resource identity scheme,
- manifest schema,
- drift policy,
- idempotence test suite design.

## Phase 13 — Planner and migration model

Candidate migration modes:

- `adopt`,
- `augment`,
- `replace`.

Plans must be semantic and show:

- creates,
- updates,
- reuses,
- adoptions,
- deletes,
- dependent resources,
- blockers,
- warnings,
- capability changes/upgrades required,
- destructive operations requiring explicit approval.

### Deliverable

Plan schema and migration safety model.

## Phase 14 — Constraint engine

Encode documented and observed relations as constraints rather than enumerating every combination.

Constraint classes:

- hard-invalid,
- hard-prerequisite,
- capability requirement,
- ambiguity/conflict,
- safety blocker,
- soft warning/recommendation.

Every constraint should link to evidence and tests where possible.

### Deliverable

Constraint representation + initial constraint knowledge base.

## Phase 15 — Verify and explain

Verification should use both desired-resource comparison and effective-policy probes.

`explain` should be generated from the same graph/constraints so users can understand why a branch/check/workflow behaves as it does.

### Deliverable

Verification probe model and explainability contract.

## Phase 16 — Security/threat model

Threat-model at least:

- token/App compromise,
- malicious PR/workflow mutation,
- workflow self-modification,
- action supply chain,
- bypass abuse,
- config/template injection,
- TOCTOU between plan/apply,
- concurrent repository mutation,
- force-push races,
- API partial failure,
- rollback failure,
- unsafe adoption of unmanaged resources.

### Deliverable

Threat model + security invariants + privilege matrix.

## Phase 17 — CLI and interactive UX

Only after the domain model is stable, define the CLI.

Candidate core surface:

```text
regelverk detect
regelverk recommend
regelverk init/configure
regelverk plan
regelverk apply
regelverk verify
regelverk explain
```

The interactive path should ask only questions made relevant by detection/capabilities.

### Deliverable

CLI contract and interaction flow.

## Phase 18 — Implementation roadmap

Only after the above is sufficiently grounded, choose language/runtime and break implementation into milestones.

Preliminary milestone idea:

- `0.0.x`: research, schemas, fixtures, architecture,
- `0.1`: generic config → deterministic renderer + ownership/idempotence,
- `0.2`: detection + observed-state,
- `0.3`: planner/apply/verify + safe migration,
- `0.4`: interactive setup/adaptation,
- `0.5`: first externally validated archetypes/recommendations,
- later: richer constraint knowledge base and template catalog.

These version boundaries are provisional until architecture/research establishes realistic implementation slices.

## Reference implementation role

Avkroken `avkroken-rulesets-workflows-v24.2.0` should be preserved as:

- a fixture describing one successful policy archetype,
- a regression source for hard-won operational behavior,
- input for extracting reusable semantics,
- a test target for `config → resolved policy` equivalence.

The new code does not need to preserve its file layout or implementation approach.

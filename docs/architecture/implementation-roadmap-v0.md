# Implementation Roadmap v0

Status: planning draft
Date: 2026-08-27

## Guiding rule

Build vertical slices that prove the architecture in reality. Do not implement every subsystem in parallel.

## Milestone 0.0.x — Research and contracts

Goal: stabilize factual knowledge and internal contracts before production implementation.

Deliverables:

- GitHub rule/capability matrix
- Actions/check semantics research
- live experiment suite design
- normalized graph contracts
- template schema draft
- ownership/manifest model
- planner/constraint contracts
- migration/verification/security architecture
- technology spike decision

Exit criteria:

- material P0 research gaps identified
- internal model can represent Avkroken v24.2 semantics without file-per-repo duplication
- no implementation-critical ambiguity hidden as an assumption

## Milestone 0.1 — Deterministic compiler core

Goal: prove config/template -> desired semantic graph -> deterministic rendered artifacts.

Scope:

- selected implementation language
- versioned config schema
- versioned template schema
- core graph model
- normalization/semantic digest
- first template: Sequential Slots reference family
- deterministic workflow/ruleset rendering
- local validation
- Avkroken semantic fixture

No live mutation required yet.

Exit criteria:

- same inputs produce same desired graph/artifacts
- Avkroken fixture can be represented through data/template instead of duplicated files
- unit/golden tests are stable

## Milestone 0.2 — Read-only discovery

Goal: `detect` safely understands existing repositories.

Scope:

- repository metadata/refs/settings discovery
- ruleset and effective-policy discovery
- workflow dependency analyzer
- required-check provider model
- capability/permission probes
- observed-state serialization
- initial project classification
- explain for observed facts

Exit criteria:

- no repository mutations
- unknown/inaccessible is preserved
- common existing workflow structures are modeled
- detection handles unmanaged collisions safely

## Milestone 0.3 — Planner and reconciliation

Goal: compare observed and desired state and produce safe, explainable plans.

Scope:

- ownership manifest
- REUSE/COMPOSE/ADAPT/GENERATE resolution
- semantic diff
- constraint engine
- drift detection
- stale-plan preconditions
- risk/rollback classification
- plan output

Exit criteria:

- repeated plan against unchanged state is stable
- unmanaged resources are never proposed for overwrite without explicit adaptation/adoption
- dangerous unknowns become blockers
- simulation tests cover partial and drifted states

## Milestone 0.4 — Safe apply and verify

Goal: controlled repository mutation with idempotent convergence.

Scope:

- GitHub/file adapters
- transaction journal/checkpoints
- dependency-before-enforcement ordering
- snapshots/compensation where possible
- resource verification
- effective-policy verification
- `apply` rerun NOOP behavior

Exit criteria:

- empty/new test repo apply succeeds
- second apply is NOOP
- interrupted/partial operations converge or clearly report FAILED_PARTIAL
- required-check deadlock regression tests pass

## Milestone 0.5 — Adaptive templates and recommendation

Goal: public-use smart setup.

Scope:

- template catalog
- `recommend`
- user/environment profile
- cost constraints
- apply vs adapt UX
- compatibility/degradation variants
- migration-risk scoring
- first set of externally useful certified templates

Exit criteria:

- recommendations include reasons and disqualifiers
- templates have capability support matrices
- existing compatible CI can be reused without duplicate generated workflows

## Milestone 0.6 — Multi-repo and organization-aware workflows

Goal: safely operate over multiple repositories without pretending cross-repo atomicity.

Scope:

- batch planning
- per-repo apply/verification
- organization-level policy awareness
- shared configuration profiles
- aggregate reporting

Exit criteria:

- failure in one repo does not corrupt plans/state for others
- organization-level effects are represented as effective policy/provenance

## Milestone 0.7+ — Expanded governance archetypes

Goal: broaden tested workflow models based on completed rule research and real-world usage research.

Potential families must be research-backed before naming/support promises.

Scope may include:

- protected trunk
- release-oriented models
- deployment promotion models
- AI/bot-heavy variants
- monorepo-specific variants

## Workstream ordering

The critical path is:

```text
Research facts
  -> internal graph/contracts
  -> technology decision
  -> deterministic compiler
  -> discovery
  -> planner
  -> apply/verify
  -> recommendation/adaptation breadth
```

Do not build interactive wizard polish before discovery and capability resolution are trustworthy.

## Technology spike gate

Before 0.1 implementation, build the same narrow vertical spike in the top two candidate languages.

Spike should include:

- parse config YAML
- parse/edit a representative workflow YAML without destructive formatting/semantic loss beyond agreed policy
- model a ruleset and required check
- query/mock GitHub adapter
- build desired graph
- emit a small plan
- produce a static binary/release artifact

Decision criteria:

- correctness and maintainability
- YAML transformation quality
- type/model ergonomics
- GitHub API ergonomics
- testability
- binary distribution/cross-platform support
- dependency/supply-chain surface
- contributor accessibility

Record final choice in an ADR.

## Test repository strategy

Maintain dedicated disposable test repositories/accounts or organization resources for live experiments. Tests that modify policy must never target production repositories by default.

Use fixture classes:

- blank repository
- existing CI repository
- complex reusable-workflow repository
- overlapping ruleset repository
- organization-policy repository
- bot/Dependabot repository
- merge-queue-capable repository

## Documentation alongside implementation

Each implemented capability updates:

- user docs
- internal capability matrix
- constraints
- verification strategy
- template support matrix
- gotchas/research provenance

Documentation drift is treated as a product defect.

## Release discipline

Early releases should state support boundaries precisely. A feature is not “supported” merely because the API call exists; support requires validation and appropriate E2E evidence for the declared profile.

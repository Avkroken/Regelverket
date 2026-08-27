# Implementation Technology Evaluation v0

Status: architecture evaluation draft
Date: 2026-08-27

## Goal

Select an implementation stack only after evaluating it against Regelverket's actual requirements. This document narrows candidates without making the final decision yet.

## Required characteristics

The core implementation needs:

- strong YAML/JSON parsing and schema validation
- precise typed domain model for policy graph, uncertainty and provenance
- reliable GitHub REST/GraphQL integration
- deterministic rendering
- filesystem-safe edits
- Git operations and process execution without shell injection
- graph/dependency analysis
- concurrency with bounded API calls
- cross-platform CLI distribution
- unit/golden/property/integration tests
- good error/context handling
- cryptographic hashing/provenance verification
- low-friction installation for public users
- ability to run without a language runtime if practical

The project does **not** require:

- browser/UI runtime in the core
- shell as the orchestration language
- arbitrary template code execution
- a server for basic operation

## Architectural split independent of language

```text
CLI/UI adapter
    -> application services
       -> domain graph + planner + constraints
          -> GitHub adapter
          -> Git adapter
          -> filesystem adapter
          -> template/catalog adapter
```

Domain/planner code must not import CLI presentation logic or direct HTTP details.

## Candidate: Go

### Strengths

- single static-ish executable distribution is straightforward
- strong standard library for HTTP, concurrency, filesystem and CLI utilities
- mature GitHub API ecosystem plus direct REST support
- structs/interfaces fit explicit domain models well
- fast startup and good cross-platform builds
- easy embedding of built-in templates/schemas
- generally low operational dependency burden for end users

### Risks

- YAML AST-preserving edits require deliberate library selection/design
- algebraic/variant-heavy domain states such as known/unknown/inaccessible are less expressive than Rust enums
- error handling can become repetitive without conventions

### Fit

Very strong for a portable public CLI and likely the baseline candidate.

## Candidate: Rust

### Strengths

- excellent type system for explicit state machines, ownership states and capability result variants
- strong memory safety
- single binary distribution
- strong serde/schema/CLI ecosystem
- well suited to correctness-heavy planners and parsers
- explicit error and concurrency models

### Risks

- higher implementation complexity and contributor learning curve
- compile times/tooling complexity greater than Go for many contributors
- some GitHub-specific SDK coverage may require more direct REST implementation

### Fit

Excellent for correctness and safety; potentially best technical model, but contributor/velocity cost must be justified.

## Candidate: TypeScript / Node.js

### Strengths

- excellent GitHub ecosystem; Octokit is first-class and GitHub-adjacent
- rich YAML/JSON/schema tooling
- discriminated unions can model domain variants cleanly
- broad contributor familiarity
- easy future shared types with a web/TUI/config editor if one appears

### Risks

- runtime/package distribution introduces dependency/supply-chain surface
- packaging a truly simple single executable across platforms is less native than Go/Rust
- npm dependency tree may conflict with the project's supply-chain goals unless aggressively controlled/bundled
- filesystem/process safety needs discipline

### Fit

Strong development ergonomics and GitHub integration; weaker default distribution/supply-chain characteristics for this particular CLI.

## Candidate: Python

### Strengths

- very fast prototyping
- excellent YAML/HTTP/data tooling
- approachable for research scripts and fixtures
- useful for migration/analyzer prototypes

### Risks

- runtime/environment/package management burden for public CLI users
- packaging reproducible single-binary distributions is possible but not as natural
- type guarantees are weaker unless strict typing is enforced throughout
- dependency/environment variance can complicate support

### Fit

Excellent research/test tooling; less attractive as the primary distributed core unless installation simplicity is solved convincingly.

## Shell

Shell remains useful only for tiny bootstrap/wrapper tasks.

It is rejected as the core architecture because Regelverket requires typed graph manipulation, structured API error classification, YAML semantics, transactional planning, concurrency, provenance and cross-platform behavior.

## Evaluation dimensions

Before final selection, build the same small vertical prototype in the two strongest candidates.

Prototype scope:

1. parse a sample Regelverket config
2. parse two existing workflows preserving enough source information for a safe plan
3. build normalized workflow/check graph
4. call a read-only GitHub endpoint through an adapter
5. render deterministic JSON/YAML
6. emit semantic diff and structured diagnostics
7. package executable for Linux/macOS/Windows

Score:

- domain-model clarity
- parser/edit fidelity
- GitHub integration effort
- binary size/startup
- dependency count/supply-chain surface
- cross-platform packaging
- test ergonomics
- contributor readability
- performance on large repository fixtures

## Provisional ranking

Based on architecture requirements before prototype evidence:

1. **Go** — strongest balance of distribution simplicity, ecosystem and implementation complexity.
2. **Rust** — strongest correctness model; higher engineering/contributor cost.
3. **TypeScript** — strongest GitHub/web ecosystem; runtime/dependency distribution cost.
4. **Python** — strong research/prototype language; weaker default public CLI packaging.

This ranking is provisional and must not be treated as the language decision.

## Possible hybrid

Even with Go/Rust core, auxiliary research tooling can remain Python. A future visual/web configuration experience can be separate from the core and communicate through versioned config/plan schemas.

The core should not choose TypeScript merely because a future UI might use TypeScript.

## GitHub client strategy

Regardless of language:

- hide SDK behind a narrow adapter
- preserve raw response/evidence where needed for provenance
- set explicit GitHub API version
- classify 403/404/422 by operation context rather than generic failure
- support REST first where rulesets are REST-centric
- use GraphQL only where it materially improves data availability
- avoid coupling domain types to generated SDK response types

## YAML strategy

There are two distinct YAML use cases:

1. Regelverket-owned config/templates: canonical deterministic serialization is acceptable.
2. User-owned workflow files: adaptation should preserve comments/format/order where practical, or avoid direct edits and generate a dedicated resource when fidelity cannot be guaranteed.

Language/library choice must be tested specifically against case 2.

## Release strategy implications

Go/Rust make a signed/attested multi-platform binary release straightforward. GitHub artifact attestations can provide build provenance for released binaries; consumers still need a trust policy that checks expected repository/workflow identity.

## Decision gate

Do not select the language until:

- the vertical prototype is complete in Go and Rust at minimum
- YAML-preserving adaptation feasibility is measured
- GitHub adapter permissions/errors are exercised
- release/install UX is compared
- contributor expectations are decided

The decision should then be recorded as an ADR with benchmark/prototype evidence.

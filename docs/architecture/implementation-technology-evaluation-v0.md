# Implementation Technology Evaluation v0

Status: closed by implementation-language-decision-v0
Date: 2026-08-27

## Goal

Select an implementation stack only after evaluating it against Regelverket's actual requirements.

The evaluation is now complete. The production core language is **Rust**. The full decision and evidence weighting are recorded in `implementation-language-decision-v0.md`.

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

### Strengths observed

- simple single-binary CLI distribution
- strong standard library for HTTP, concurrency and filesystem work
- minimal dependency surface in the initial compiler and GitHub-adapter spikes
- lower expected contributor learning curve
- successful deterministic compiler/planner parity
- successful Linux/macOS/Windows packaging

### Measured limitation

In the shared user-owned workflow adaptation experiment, the Go `yamsplice` prototype preserved untouched bytes, quotes and inline comments, but normalized spacing on the edited line. That was weaker edited-node fidelity than the Rust prototype.

## Candidate: Rust

### Strengths observed

- `BTreeSet` and enums map naturally to deterministic collections and explicit uncertainty/state variants
- successful deterministic compiler/planner parity
- exact edited-line fidelity in the measured user-owned YAML adaptation case with `yaml-rt`
- successful GitHub error/capability adapter including live REST verification
- successful Linux/macOS/Windows packaging
- no runtime requirement for end users

### Costs accepted

- larger dependency graph in the measured GitHub adapter
- higher compiler/tooling complexity
- steeper contributor learning curve

These costs are accepted because the product's safety-critical work is deterministic planning and conservative adaptation of user-owned resources, where the Rust prototype produced stronger direct evidence.

## Prototype evidence

### Compiler/planner

PR #3 established a shared local/CI verifier and semantic parity for Go and Rust across no-op/update fixture cases.

### YAML adaptation

PR #4 measured a difficult comment/layout-heavy workflow edit. Rust preserved the entire edited line except for the intended scalar change; Go introduced spacing normalization on that line. Both preserved untouched lines.

### GitHub adapter

PR #5 implemented the same narrow read-only GitHub adapter in both languages and verified explicit headers, API versioning, authentication behavior and classified `401`, `403`, `404`, `422`, `429` and `5xx` outcomes. Both also succeeded against the live public repository endpoint.

### Packaging

PR #6 verified host-native release construction on Linux, macOS and Windows. Both candidates passed after shared CI portability fixes.

## Final ranking after prototype evidence

1. **Rust** — selected for the production core because it combines stronger domain-state modelling with the best measured user-owned YAML adaptation fidelity.
2. **Go** — remains a strong alternative with lower dependency and contributor cost, but did not win the highest-weight preservation experiment.
3. **TypeScript** — strong GitHub/web ecosystem, weaker default runtime/supply-chain profile for this CLI.
4. **Python** — retained as useful research/test tooling, not the primary distributed core.

## GitHub client strategy

Regardless of language:

- hide SDK/client details behind a narrow adapter
- preserve raw response/evidence where needed for provenance
- set explicit GitHub API version
- classify 403/404/422 by operation context rather than generic failure
- support REST first where rulesets are REST-centric
- use GraphQL only where it materially improves data availability
- avoid coupling domain types to generated SDK response types

The Rust spike deliberately used a narrow direct REST adapter rather than binding the domain model to a GitHub SDK.

## YAML strategy

There are two distinct YAML use cases:

1. Regelverket-owned config/templates: canonical deterministic serialization is acceptable.
2. User-owned workflow files: adaptation must preserve comments/format/order where practical and avoid broad serialization rewrites.

Rust's measured fidelity is a reason for the language selection, not a waiver of this preservation requirement. Golden and byte-preservation tests remain mandatory.

## Release strategy implications

The packaging spike demonstrated viable host-native Rust binaries on Linux, macOS and Windows. Production release work should add signing/attestations and release publishing separately from this technology gate.

## Decision

The gate is closed. See `implementation-language-decision-v0.md` for the accepted Rust decision, contributor/maintenance tradeoff and revisit conditions.

# Implementation Language Decision v0

Status: accepted
Date: 2026-08-27
Decision: Rust for the Regelverket core implementation

## Context

The architecture evaluation intentionally deferred the implementation language until Go and Rust had been exercised against Regelverket-specific failure modes rather than compared on syntax or general ecosystem preference.

The decision gate required evidence for:

1. deterministic config/observed-state/resolved-policy/plan behavior;
2. adaptation of user-owned GitHub Actions YAML with formatting preservation;
3. a narrow GitHub API adapter with explicit permission/error classification;
4. Linux, macOS and Windows binary packaging;
5. contributor and maintenance cost.

## Decision

Use **Rust** for the production core.

Python may remain useful for research/fixture tooling, and future UI surfaces may use other languages, but the compiler/planner, GitHub adapter, filesystem/Git mutation layer and primary CLI should share the Rust core unless a later ADR supersedes this decision.

## Evidence

### Deterministic compiler/planner slice

Go and Rust both passed the same fixture-driven contract chain and emitted semantically identical deterministic plans. Neither candidate gained a decisive advantage here.

Evidence: PR #3 and `Technology Spikes` successful verification.

### User-owned workflow YAML fidelity

This was the most discriminating safety-critical experiment.

Both implementations preserved untouched workflow bytes. On the edited scalar line:

- Rust `yaml-rt` v0.2.3 preserved the line exactly except for the intended `"20"` -> `"22"` scalar change.
- Go `yamsplice` v0.3.0 preserved quotes and the inline comment, but normalized spacing before the inline comment.

Regelverket's architecture treats user-owned resources conservatively. Exact preservation where a narrowly-scoped edit is possible therefore carries more weight than cosmetic convenience: unnecessary rewrites create review noise, increase merge-conflict probability and weaken confidence in idempotent adaptation.

Evidence: PR #4, `Technology Spikes` run `33099531138`.

### GitHub adapter

Both candidates successfully implemented the same read-only adapter behavior:

- explicit GitHub REST media type and API version;
- optional bearer authentication;
- explicit user-agent;
- classified `401`, `403`, `404`, `422`, `429` and `5xx` responses;
- preservation of GitHub's ambiguous `404` meaning as `not_found_or_inaccessible`;
- parity against a local validating mock and the live public `Avkroken/Regelverket` endpoint.

Go required only the standard HTTP library for the prototype. Rust used pinned `ureq` and consequently has the larger dependency graph. This is a real maintenance and supply-chain cost in Go's favor, but the adapter abstraction keeps that cost localized.

Evidence: PR #5, `GitHub Adapter Spike` successful locked verification.

### Cross-platform packaging

Both candidates built and packaged successfully on GitHub-hosted Linux, macOS and Windows runners after making toolchain provisioning explicit and fixing a Windows archive-path portability issue in the shared packaging verifier.

The packaging experiment found no product-level blocker for either language.

Evidence: PR #6, `Packaging Spike` run `33109986935`.

## Contributor and maintenance comparison

| Dimension | Go | Rust | Decision impact |
| --- | --- | --- | --- |
| Initial contributor readability | stronger | steeper learning curve | Go advantage |
| Dependency surface in current spikes | smaller | larger | Go advantage |
| HTTP/GitHub adapter implementation | standard library sufficient | external HTTP client used | Go advantage |
| Deterministic collections | explicit sorting/conventions | ordered collection types fit naturally | Rust advantage |
| Explicit uncertainty/state modelling | conventions/wrappers required | enums/result types fit directly | Rust advantage |
| User-owned YAML edited-node fidelity | near-lossless in measured case | exact in measured case | Rust advantage, high weight |
| Cross-platform binary distribution | proven | proven | tie |
| Runtime requirement for users | none | none | tie |
| Compiler/planner correctness ergonomics | adequate | stronger type-level modelling | Rust advantage |

The contributor-cost disadvantage is accepted rather than ignored. The production repository should compensate with narrow modules, explicit domain types, small adapters, fixture-heavy tests, conventional error types and documentation aimed at contributors who are not Rust specialists.

## Why Rust wins

Go remains the simpler operational and contributor choice, and before direct prototypes it was the provisional leader. The prototypes changed that ranking.

Regelverket is not primarily a generic GitHub API client. Its high-risk work is compiling uncertain repository state into deterministic plans and adapting user-owned automation without collateral edits. Rust showed the stronger fit on both explicit state modelling and the most difficult measured mutation case. Those properties align directly with the project's correctness and preservation invariants.

The dependency and learning-curve costs are visible and manageable. Loss of adaptation fidelity is closer to the product's core safety boundary and is therefore weighted more heavily.

## Consequences

- Milestone 0.1 production compiler work should start in Rust rather than promoting the Go spike.
- Go spikes remain useful comparative evidence but are not the production implementation path.
- Rust dependency additions should be conservative and justified at adapter boundaries.
- User-owned YAML mutations must retain golden/byte-preservation tests; the language choice does not remove the need for explicit preservation policy.
- Release CI must continue to prove Linux/macOS/Windows builds.
- Domain code must remain independent of direct GitHub SDK/HTTP response types.

## Revisit conditions

Revisit this decision only with new evidence that materially changes one of the weighted constraints, for example:

- the selected Rust YAML strategy cannot handle required real-world workflow constructs safely;
- contributor/tooling cost becomes a demonstrated delivery blocker rather than a predicted cost;
- release portability or supply-chain constraints materially deteriorate;
- a Go implementation demonstrates equal or better preservation and state-model safety with materially lower maintenance cost.

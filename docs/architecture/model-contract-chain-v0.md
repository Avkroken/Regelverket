# Model Contract Chain v0

Status: architecture summary
Date: 2026-08-27

## Purpose

This document links the four principal data contracts that define Regelverket's pre-implementation architecture.

```text
Observed State (DG-02)
        ↓
Desired State / Config
        ↓
Resolved Policy
        ↓
Plan
```

Each contract has one responsibility and must not absorb the semantics of another.

## Observed State

Represents facts, evidence, inferred relationships and uncertainty discovered from GitHub and repository contents.

It answers:

- what exists?
- what is currently enforced?
- what can be proven?
- what is inaccessible or unknown?
- which workflows/check providers/refs/rulesets depend on each other?

Observed State is read-only evidence.

## Desired State / Config

Represents user intent and constraints.

It answers:

- what working model does the user want?
- which template/archetype is selected?
- what may be preserved, changed or destroyed?
- what cost/plan constraints apply?
- what review/merge/security behavior is desired?

Config should not contain transient GitHub IDs or copied discovery state unless explicitly overridden.

## Resolved Policy

Represents the exact semantic target after adapting intent to repository facts and available capabilities.

It answers:

- which semantic resources are required?
- which existing resources are reused/shared/adopted?
- which new resources must be generated?
- which template variants or fallbacks were selected?
- which constraints passed, warned or blocked?
- what effective policy should be true after convergence?

Resolved Policy is deterministic for identical inputs.

## Plan

Represents the safe transition from observed state to resolved policy.

It answers:

- what changes?
- in what order?
- why?
- with what preconditions?
- what is destructive or privilege-sensitive?
- how is each operation verified?
- what rollback/compensation class applies?

The plan is approval-scoped and stale-state-aware.

## Non-overlap rules

1. Observed State never expresses desired mutation.
2. Config never claims that inferred discovery facts are user intent.
3. Resolved Policy never contains execution ordering as its primary semantics.
4. Plan never becomes the durable source of policy intent.
5. GitHub REST payloads are renderer/adapter artifacts, not any of the four core contracts.

## Digest chain

For reproducibility, each downstream contract records the semantic digest of its material upstream inputs.

Conceptually:

```text
observed_digest
config_digest
     ↓
resolved_policy_digest
     ↓
plan_digest
```

A changed upstream digest invalidates stale downstream artifacts unless the change is explicitly proven non-semantic by versioned normalization.

## First implementation acceptance path

The first vertical implementation spike should prove this sequence entirely in memory and from fixtures before any GitHub mutation:

1. Parse `fixtures/observed-state/avkroken-dumpen-v0.yaml`.
2. Parse a minimal Sequential Slots config.
3. Resolve a deterministic policy.
4. Produce a `no_changes` plan for matching state.
5. Change one desired required-check capability.
6. Produce one deterministic update plan with reason, dependencies and verification expectations.
7. Serialize, reload and reproduce the same semantic digests.

The same code path must later work against Bastion without repository-name conditionals.

## Technology-spike implication

Go and Rust prototypes should be judged primarily on their ability to implement this contract chain cleanly, with deterministic serialization/normalization, strong typing, YAML handling, graph traversal, diagnostics and test ergonomics.

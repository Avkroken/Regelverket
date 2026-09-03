# REGELVERKET.md

This is the repository governance document for `Avkroken/Regelverket`. Binding AI coding-agent policy is defined only in `Avkroken/.github/AGENTS.md`. This document records repository-specific technical contracts, invariants, validation requirements, and operational context required by that policy; it must not define, supplement, narrow, or override agent policy.

## Repository status

Regelverket is retired as an application project. Do not build new product functionality here. Shared GitHub automation belongs in `Avkroken/.github`.

There is no permanent `dev` working branch.

## Workflow inventory

The repository is intentionally limited to the transition checks enforced by `.github/workflows/scope-policy.yml`:

- `.github/workflows/required-ci.yml`: verifies that retired product code is not reintroduced.
- `.github/workflows/scope-policy.yml`: verifies the allowed workflow inventory and repository scope.

The organization-level required OSV workflow is now owned by `Avkroken/.github`; Regelverket no longer provides a central OSV workflow source.

New product code or a new independent workflow responsibility requires a separate explicit repository-owner decision. Shared metadata, dependency and organization policy automation belongs in `Avkroken/.github` rather than Regelverket.

## Security

Never commit or expose secrets, tokens, private keys or other credentials. Transition workflows should remain deterministic and narrowly scoped to their documented responsibility.

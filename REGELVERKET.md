# REGELVERKET.md

This is the repository governance document for `Avkroken/Regelverket`. Binding AI coding-agent policy is defined only in `Avkroken/.github/AGENTS.md`. This document records repository-specific technical contracts, invariants, validation requirements, and operational context required by that policy; it must not define, supplement, narrow, or override agent policy.

## Repository status

Regelverket is retired as an application project. It is a temporary transition repository while Avkroken organization policy still depends on repository-owned transition workflows.

Do not build new product functionality here. Shared GitHub automation belongs in `Avkroken/.github`.

There is no permanent `dev` working branch.

## Transition workflow inventory

The repository is intentionally limited to transition/governance workflows. Preserve the exact inventory enforced by `.github/workflows/scope-policy.yml` and inspect that policy before adding, renaming or deleting workflow files.

The current responsibilities are:

- `.github/workflows/required-ci.yml`: minimal transition gate that verifies retired product code is not reintroduced.
- `.github/workflows/scope-policy.yml`: verifies the allowed workflow inventory and repository scope.
- `.github/workflows/osv-scanner.yml`: legacy central OSV required-workflow source while an organization ruleset still references it.

New product code or a new independent workflow responsibility requires a separate explicit repository-owner decision. Shared metadata, dependency and organization policy automation belongs in `Avkroken/.github` rather than Regelverket.

Keep `osv-scanner.yml` available until live organization rules no longer reference it. Once Regelverket is no longer required as a policy/workflow source, archive the repository instead of assigning it a new responsibility for convenience.

## Security

Never commit or expose secrets, tokens, private keys or other credentials. Transition workflows should remain deterministic and narrowly scoped to their documented responsibility.

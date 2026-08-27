# Migration & Transaction Model v0

Status: architecture draft
Date: 2026-08-27

## Goal

Regelverket must be able to change repository governance without leaving a repository half-configured, silently destroying user resources, or locking maintainers out.

GitHub does not provide one atomic transaction spanning repository files, refs, rulesets, settings and external App state. Regelverket therefore implements a compensating transaction model with explicit preconditions, checkpoints and verification boundaries.

## Migration strategies

### adopt

Model existing compatible resources and transfer selected resources into Regelverket ownership only after explicit approval.

### augment

Preserve existing governance and add the minimum resources required for the selected policy.

### replace

Move toward the selected policy as the authoritative model. Replace is not `delete everything first`; it is a planned migration with dependency analysis, snapshots and explicit destructive approval.

## Transaction phases

1. **Observe** — create a fresh observed-state graph and capability snapshot.
2. **Preflight** — evaluate blocking unknowns, permissions, plan/features, open PRs, branch dependencies, workflow dependencies and required-check providers.
3. **Snapshot** — persist enough previous state to explain and, where possible, compensate for mutations.
4. **Stage dependencies** — create or update non-enforcing resources needed by later policy, such as workflows/check providers and branches.
5. **Validate staged resources** — syntax/schema/static validation and, where possible, live evidence that required checks can exist.
6. **Apply enforcement** — create/update rulesets and repository settings in an order that does not require resources that are not ready.
7. **Verify effective behavior** — verify both resources and representative effective policy.
8. **Cleanup** — remove obsolete managed resources only after replacements are verified.
9. **Commit manifest** — record ownership, bindings, semantic digests, template/config version and verification evidence.

## Ordering rule

Dependencies before enforcement; replacements before removals.

Example:

```text
create workflow provider
  -> verify provider definition
  -> add required check rule
  -> verify effective rule
  -> remove obsolete provider
```

Never invert this sequence when inversion can make merging impossible.

## Plan preconditions

A saved plan records material observations such as:

- repository identity
- default branch SHA
- relevant file blob SHAs
- ruleset IDs + normalized semantic digests
- repository setting digest
- relevant ref SHAs
- manifest digest
- capability/permission observations
- plan creation time

Before apply, material preconditions are rechecked. Stale plans are rejected or replanned; `--force` must not mean “ignore arbitrary stale safety assumptions”.

## Checkpoints

Each transaction phase emits a durable checkpoint in the operation journal. A checkpoint records completed operations, observed external IDs and verification results.

This permits safe diagnosis after interruption and supports resume/replan behavior without guessing what completed.

## Compensation / rollback

Rollback is best-effort and capability-specific, not a promise of universal atomicity.

Operations are classified:

- **reversible** — previous payload can normally be restored
- **compensatable** — equivalent previous semantics can normally be recreated, possibly with a new external ID
- **irreversible/high-risk** — cannot be reliably restored exactly

Examples requiring explicit research/tests include branch deletion, force updates, settings that affect queued work, and externally generated workflow/check history.

The plan must state rollback class before approval.

## Destructive gates

The following require explicit destructive approval unless proven harmless by a future policy setting:

- delete unmanaged/adopted refs
- delete or replace unmanaged rulesets
- overwrite user-modified managed resources after drift
- remove workflows used by required checks
- change default branch
- disable merge methods relied upon by open PRs/policy
- ownership takeover

## Open work protection

Before destructive ref or workflow changes, inspect relevant open PRs and dependency edges. A plan is blocked when deletion or renaming would strand active work unless the migration includes a verified transition.

## Multi-repository transactions

There is no implied cross-repository atomicity.

A multi-repo run produces:

- one plan per repository
- a batch summary
- explicit ordering if repositories depend on each other
- independent verification and failure status

Default behavior should stop scheduling new repository applies after a failure, while preserving completed repositories and reporting exact state.

## Lockout prevention

Before enforcement changes, planner must prove a viable maintenance path according to selected policy. Examples:

- required checks have identifiable providers
- bypass assumptions are known where bypass is required
- branch creation needed by the model remains possible
- generated automation has necessary permissions
- effective parent/org rules are not materially unknown

If proof cannot be established, operation is blocked rather than attempted.

## Idempotent recovery

After interruption, a new `detect + plan` must be able to classify already-completed mutations and converge rather than duplicate them. Operation journals help explain history but current GitHub state remains authoritative.

## Transaction output

Every apply ends in one of:

- VERIFIED
- VERIFIED_WITH_WARNINGS
- FAILED_NO_MUTATION
- FAILED_COMPENSATED
- FAILED_PARTIAL
- BLOCKED

`success` is reserved for verified desired/effective state, not merely HTTP 2xx writes.

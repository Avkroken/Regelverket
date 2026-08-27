# P0 Production Observations — Avkroken

Date: 2026-08-27
Status: observed production evidence, read-only
Scope: Avkroken/bastion as a live reference implementation

## Purpose

These observations complement documentation research with real GitHub state from an active repository. No repository mutation was performed.

## Ruleset topology observed

`Avkroken/bastion` currently exposes 11 active repository-level branch rulesets.

Representative rulesets:

### Protect main

Observed properties:

- target: branch
- include: `~DEFAULT_BRANCH`
- deletion blocked
- non-fast-forward blocked
- linear history required
- required status checks enabled
- pull-request rule enabled
- merge queue enabled
- CodeQL code-scanning rule enabled
- no bypass actors

The required status checks are bound to `integration_id: 15368`, which is GitHub Actions. The observed required contexts include build/test/package checks plus `osv` and `scope-policy`.

Observed merge queue parameters include squash merge, one entry built/merged at a time, ALLGREEN grouping, and a 60-minute check timeout.

Architecture consequence:

Required check identity must retain both context and expected provider/App identity. Merge queue configuration is part of effective policy, not merely a repository-level toggle.

### Protect work branches

Observed targets:

- `refs/heads/work/feature`
- `refs/heads/work/fix`
- `refs/heads/work/chore`

Rules:

- deletion blocked
- non-fast-forward blocked
- linear history required

Observed bypass actor:

- Integration actor ID `4594645`, bypass mode `always`

Architecture consequence:

The internal Actor model must preserve opaque GitHub actor IDs plus actor type and resolved human/app identity separately. Never infer identity from an integer label remembered from historical notes.

### Protect docs content

Observed target:

- `refs/heads/docs/content`

Rules match the work-slot protection body and use the same Integration bypass actor.

This is direct production evidence that documentation belongs naturally in the same semantic slot family while still having its own RefClass and content-scope policy.

### Protect security reports

Observed target:

- `refs/heads/automation/security-reports`

Rules include `update` in addition to deletion, non-fast-forward and linear-history rules. It uses the same Integration bypass actor as slot rules.

Architecture consequence:

Semantic normalization must not collapse resources merely because most of their rule body matches. Small rule differences can encode a materially different actor model.

### Lock branches

Observed condition:

- include `~ALL`
- exclude the explicit approved branch pool

Observed rule:

- creation blocked

Observed bypass actors:

- Integration actor ID `29110`
- Integration actor ID `262318`

This confirms the closed-pool design is implemented by a broad deny-creation rule plus exact exclusions rather than one allow-list primitive.

Architecture consequence:

Template rendering must derive this ruleset from the resolved RefClass/slot list. Manual duplication of branch names in separate policy files is not acceptable in the new design.

## Workflow/check-provider observations

### Check runs vs legacy commit statuses

For a recent merged commit, the legacy combined-status endpoint returned no status contexts while the Checks API exposed 50 check runs.

Architecture consequence:

Discovery must query Checks/Actions data, not treat the legacy Status API as authoritative for GitHub Actions required checks.

### Provider identity

Observed check runs such as `build-rpm`, `xcodegen-and-build` and `swiftpm-macos` identify App ID `15368` with slug `github-actions`.

This matches the `integration_id: 15368` stored in the main ruleset required-check entries.

Architecture consequence:

The provider resolver can correlate ruleset expected integration IDs against live check-run App IDs. This is stronger evidence than filename/job-name matching alone.

### Merge-group execution is real, not theoretical

The Actions API currently reports 94 `merge_group` workflow runs in Bastion.

A representative successful run:

- event: `merge_group`
- branch: `gh-readonly-queue/main/pr-372-f7a28c85a3ae1b41e4d539a749edb3c43787fa07`
- workflow: `.github/workflows/xcode.yml`
- conclusion: success

The run produced jobs including:

- impact
- swiftpm-macos
- ios-screenshots
- xcodegen-and-build

Architecture consequence:

The analyzer must model the merge-queue synthetic ref namespace and event separately from ordinary pull-request and push execution. The current observed ref embeds a `pr-<number>` segment, but code must not rely solely on that format because it is an implementation detail that may change.

### Scope policy

Observed `.github/workflows/scope-policy.yml`:

- triggers on `pull_request` to main
- triggers on `merge_group` / `checks_requested`
- publishes stable job/check name `scope-policy`
- derives originating PR/branch during merge-group execution
- first attempts parsing `pr-<number>` from merge-group ref
- falls back to commit-to-PR API lookup
- fails closed if the source PR cannot be determined unambiguously
- validates documentation and platform branch path scopes

Architecture consequence:

This provides a concrete example for a future `derived-context` workflow capability. Regelverket's analyzer should distinguish stable required-check identity from event-specific context derivation.

### Stable wrapper-check pattern

Observed `osv-scanner.yml` intentionally exposes one stable required check named `osv` for both pull-request and merge-group lifecycles. Event-specific reusable scanners feed that wrapper job.

Architecture consequence:

This is a strong reusable template pattern:

- event-specific/internal jobs may vary
- the externally required governance check remains stable

The Template/Constraint model should support a `stable_gate` abstraction rather than forcing every implementation job to become required.

### Conditional job behavior

Observed workflows use job-level `if:` conditions and impact-detection jobs rather than top-level path filtering for some required checks.

This aligns with the documented risk that a whole required workflow skipped by trigger/path filtering can fail to produce a required check. A wrapper or always-present gate job is safer when a stable check is required.

## Slot synchronization observations

Observed `sync-pool.yml` defines semantic slot invariants directly in executable automation:

- free slot must equal default-branch SHA exactly
- active slot retains only active PR work rebased above current default
- App token is used for privileged push
- destructive updates use `--force-with-lease`
- inability to determine open PR state causes no destructive mutation
- missing slot can be recreated
- post-reset remote SHA is verified

Architecture consequence:

These are policy requirements and transaction constraints, not merely implementation details. They belong in the Sequential Slots archetype/constraint knowledge base and should be rendered into whatever implementation best satisfies them.

## Historical production evidence from PR data

A recent Bastion PR documents a concrete failure mode where squash-merged slot history later conflicted during rebase and a protected slot could not simply be reset manually because of non-fast-forward protection. The repaired workflow distinguishes open-PR state and fails closed when that state cannot be read.

Architecture consequence:

Sequential slot management requires explicit lifecycle state and conflict recovery. `slot exists` is insufficient state modeling.

## P0 conclusions strengthened by production evidence

1. Required-check provider identity must include the producing GitHub App/integration, not only the check string.
2. Checks API and Actions workflow runs are required discovery inputs; legacy commit statuses alone are insufficient.
3. Merge-group analysis is mandatory for templates using merge queue.
4. Stable wrapper gates are a useful pattern for event-dependent/reusable workflow implementations.
5. Actor/bypass IDs must be resolved authoritatively and stored with provenance.
6. Closed branch pools should be derived from semantic RefClasses rather than duplicated explicit lists.
7. Destructive slot synchronization must fail closed when PR state or concurrency preconditions are unknown.
8. Production policy contains small rule-body differences that must survive normalization.

## Still blocked from controlled live mutation tests

The currently connected GitHub capability supports read access to rulesets and write access to files/refs, but does not expose ruleset create/update/delete or disposable repository creation. Plugin discovery found no additional installable GitHub REST/ruleset-write connector.

Therefore the following P0 tests remain pending rather than guessed:

- create/update/delete ruleset round trips
- exact API validation errors for invalid rule combinations
- controlled overlap/classic-protection experiments
- lower-privilege bypass-field visibility experiments
- effective-rule endpoint mutation before/after comparisons
- lockout/deadlock reproduction in a disposable repository

These tests must run in a disposable repository once a suitable write-capable API path is available.

## Evidence sources

Observed directly through GitHub API on 2026-08-27:

- `GET /repos/Avkroken/bastion/rulesets`
- individual ruleset reads for Protect main, Protect work branches, Protect docs content, Protect security reports and Lock branches
- `.github/workflows/scope-policy.yml`
- `.github/workflows/osv-scanner.yml`
- `.github/workflows/sync-pool.yml`
- Actions workflow runs filtered by `event=merge_group`
- workflow-run jobs for a representative merge-group run
- Checks API on recent PR/main commits
- recent merged PR metadata

Status of conclusions: observed production evidence. Where a conclusion generalizes beyond Bastion, it remains an architectural inference until confirmed by controlled experiments or GitHub documentation.

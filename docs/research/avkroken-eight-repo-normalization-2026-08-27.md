# Avkroken eight-repository normalization

Status: observed production research
Date: 2026-08-27

## Scope

This document compares the eight repositories that were managed by the v24.2 policy package before Regelverket was created:

- `docker-idempotent-update`
- `produkter`
- `pastebinit`
- `routines-relay`
- `politiker`
- `bastion`
- `klarsprak`
- `dumpen`

The purpose is not to treat Avkroken as the universal model. It is to extract the smallest semantic representation capable of reproducing this real production family without duplicating repository-specific JSON.

## Repository-level ruleset topology

Observed on 2026-08-27:

Seven repositories have exactly five active repository-level branch rulesets:

1. `Protect main`
2. `Protect work branches`
3. `Protect docs content`
4. `Protect security reports`
5. `Lock branches`

`bastion` has those same five plus six scope-specific rulesets:

- `Protect Bastion android scope`
- `Protect Bastion apple scope`
- `Protect Bastion cli scope`
- `Protect Bastion linux scope`
- `Protect Bastion swift-core scope`
- `Protect Bastion windows scope`

Therefore the empirical model is not “five files per repository”. It is:

```text
base policy
  + N additional protected ref classes
```

where `N = 0` for seven repositories and `N = 6` for Bastion.

## Protect main — common semantic body

All eight observed `Protect main` rulesets share the same conditions and rule structure:

Target:

```text
branch
```

Condition:

```text
include: ~DEFAULT_BRANCH
exclude: none
```

Rules:

```text
deletion
non_fast_forward
required_linear_history
required_status_checks
pull_request
merge_queue
code_scanning
```

The following `required_status_checks` parameters are common:

```text
strict_required_status_checks_policy = false
do_not_enforce_on_create = true
integration_id = 15368 for every observed required check
```

The observed integration ID `15368` is GitHub Actions.

The following pull-request parameters are common:

```text
required_approving_review_count = 0
dismiss_stale_reviews_on_push = true
require_code_owner_review = false
require_last_push_approval = false
required_review_thread_resolution = true
require_extra_approval_for_unattributed_changes = true
allowed_merge_methods = [squash]
```

The merge queue configuration is also common:

```text
merge_method = SQUASH
max_entries_to_build = 1
min_entries_to_merge = 1
max_entries_to_merge = 1
min_entries_to_merge_wait_minutes = 0
grouping_strategy = ALLGREEN
check_response_timeout_minutes = 60
```

Code scanning is common:

```text
tool = CodeQL
security_alerts_threshold = all
alerts_threshold = all
```

### Only observed repository-specific field in the main body

The required-check list.

| Repository | Required checks |
|---|---|
| docker-idempotent-update | `lint`, `python`, `docker`, `osv`, `scope-policy` |
| produkter | `python`, `node`, `docker`, `dependency-review`, `osv`, `scope-policy` |
| pastebinit | `python`, `osv`, `scope-policy` |
| routines-relay | `repository-checks`, `osv`, `scope-policy` |
| politiker | `python`, `typecheck`, `docker`, `osv`, `scope-policy` |
| bastion | `xcodegen-and-build`, `swiftpm-macos`, `ios-screenshots`, `swiftpm-linux`, `linuxapp-build`, `linuxapp-msrv`, `build-deb`, `build-rpm`, `build-deb-linuxapp`, `build-rpm-linuxapp`, `android-build`, `windowsapp-core-tests`, `windowsapp-build`, `osv`, `scope-policy` |
| klarsprak | `validate`, `osv`, `scope-policy` |
| dumpen | `test`, `osv`, `scope-policy` |

This confirms a strong architecture conclusion:

> `Protect main` should be represented as one policy implementation parameterized by a resolved set of required CheckProvider identities.

The renderer must not maintain one main ruleset template per repository.

## Work branch class

Observed standard rule body:

Refs:

```text
refs/heads/work/feature
refs/heads/work/fix
refs/heads/work/chore
```

Rules:

```text
deletion
non_fast_forward
required_linear_history
```

Observed bypass:

```text
actor_type = Integration
actor_id = 4594645
bypass_mode = always
```

The same semantic body is observed in Bastion and in a standard repository.

Architecture consequence:

```text
RefClass(work)
  refs = configurable list
  protections = slot-protection profile
  maintenance_actor = resolved actor capability
```

The literal integration ID must not be embedded in a public template. It is an observed binding that must be discovered/resolved for each installation.

## Documentation ref class

Observed ref:

```text
refs/heads/docs/content
```

Rules:

```text
deletion
non_fast_forward
required_linear_history
```

Bypass is the same maintenance Integration observed for work branches.

This validates documentation as a first-class RefClass rather than a one-off branch exception.

A public template may suggest `docs/content`, but adaptation must be able to bind another existing branch that satisfies the same semantic role.

## Automation/security-report ref class

Observed ref:

```text
refs/heads/automation/security-reports
```

Rules:

```text
update
deletion
non_fast_forward
required_linear_history
```

This differs from work/docs by exactly one observed additional rule: `update`.

Therefore it should not be copied as a wholly separate policy body. A normalized representation can express it as:

```text
slot-protection
  + restrict_update
```

or an equivalent capability-level modifier once `update` semantics are fully frozen by P0 research.

## Closed branch pool / Lock branches

Observed standard repository behavior:

```text
include: ~ALL
exclude:
  automation/security-reports
  docs/content
  work/chore
  work/feature
  work/fix
rule: creation
```

Observed Bastion behavior uses the same rule and pattern, but its exclusion set is expanded with:

```text
core/swift
platform/android
platform/apple
platform/cli
platform/linux
platform/windows
```

Observed bypass actors for branch creation lock:

```text
Integration 29110, always
Integration 262318, always
```

This yields a second strong architecture conclusion:

> The branch lock exclusion set must be derived from the resolved RefClass graph. It must never be maintained as an independent user-edited list.

Conceptually:

```text
allowed_ref_pool = union(all template/config managed ref classes)
lock.exclude = allowed_ref_pool
lock.include = ~ALL
```

Adding or removing a slot/ref class must therefore cause the lock ruleset to update automatically.

The bypass actor IDs are installation bindings, not template data. Their semantic roles need to be represented and resolved independently.

## Bastion specialization

Bastion demonstrates that a generic Sequential Slots family cannot model only `work/*` plus docs.

Bastion has six additional persistent work scopes that share the protected-slot concept but also participate in scope-policy file restrictions:

```text
platform/windows -> WindowsApp/
platform/android -> Android/
platform/linux   -> LinuxApp/
platform/apple   -> App/
platform/cli     -> Sources/bastion-cli/
core/swift       -> Sources/SSHCore/, Tests/SSHCoreTests/, Package.swift, Package.resolved
```

The current `scope-policy.yml` confirms that these are semantic branch-to-path bindings, not arbitrary extra branch names.

A normalized model should therefore support something equivalent to:

```yaml
ref_classes:
  - id: platform.windows
    ref: platform/windows
    role: work-slot
    scope:
      allow_paths:
        - WindowsApp/**
```

The exact public schema is not frozen yet.

## Required common capabilities discovered empirically

Across the family, two checks appear as common policy infrastructure rather than project-specific CI:

```text
osv
scope-policy
```

Project/build checks vary by repository.

This suggests that desired-state construction should classify checks into at least:

```text
project-provided
policy-provided
shared/reused
```

A template can require the capability `scope validation` without requiring that the check necessarily comes from a newly generated file if an existing compatible provider is detected.

## What is data vs policy vs binding

The eight-repository observation supports the following separation.

### Policy logic

Common reusable semantics:

- default branch protections
- squash-only PR/queue flow
- queue serialization parameters
- CodeQL enforcement
- protected persistent ref classes
- closed branch pool
- scope gate concept
- synchronization concept

### Repository data

- required project checks
- concrete additional ref classes
- branch-to-path scope mappings
- project-specific workflow providers

### Installation/environment bindings

- GitHub App / Integration actor IDs
- GitHub Actions integration identity
- repository IDs
- ruleset numeric IDs
- actual workflow paths
- current branch SHAs

These three categories must remain separate in Regelverket's internal model and manifest.

## Minimal configuration hypothesis from production data

Avkroken's current eight-repository family could be represented approximately by common template data plus per-repository deltas:

```yaml
archetype: sequential-slots

work_slots:
  - work/feature
  - work/fix
  - work/chore

documentation:
  ref: docs/content

automation:
  security_reports_ref: automation/security-reports

required_checks: <detected/resolved per repo>

additional_scopes: <empty except Bastion>
```

The final public schema must not be inferred solely from this fixture, but the fixture proves that duplicated repository-specific policy files are unnecessary.

## Normalization invariants for the implementation

The future core should satisfy these tests:

1. Eight different configs/resolved repository graphs can share one default-branch policy renderer.
2. Required check order differences do not create semantic drift if GitHub treats the field as set-like.
3. Adding a RefClass automatically expands the lock exclusion set.
4. Removing a RefClass automatically contracts the lock exclusion set, subject to migration safety checks.
5. Standard work/docs classes share one protection profile.
6. Security-report class composes the standard profile with its additional update restriction.
7. External numeric actor/ruleset IDs do not appear in template source.
8. Bastion's six scopes are represented as data, not six hard-coded policy implementations.
9. Rendering the Avkroken fixture produces semantically equivalent effective policy, not necessarily byte-identical historical files.

## Evidence classification

Status of conclusions in this document:

- ruleset counts/names: **observed via GitHub API**
- detailed main rule bodies: **observed via GitHub API for all eight repositories**
- work/docs/security/lock bodies: **observed directly in representative standard repository and Bastion; consistent with installed v24.2 family**
- check provider App identity 15368: **observed via live Check Runs as GitHub Actions**
- Bastion branch/path scope map: **observed from current production workflow**

Where a conclusion generalizes beyond these repositories, it remains architecture inference rather than GitHub platform fact.

## Next empirical pass

The next pass should normalize the workflow layer across all eight repositories:

- inventory workflow paths
- identify common policy-provided workflows
- map each required check to observed provider workflow/job/App
- extract event support (`pull_request`, `merge_group`, `push`, schedule)
- map reusable workflows/local actions/scripts
- classify project-provided vs policy-provided capabilities
- detect repository-specific variations such as cron and scope maps

That pass will turn the current ruleset normalization into a complete `PolicyRequirement -> CheckProvider -> Workflow` fixture for DG-02.

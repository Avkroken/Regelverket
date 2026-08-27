# Live GitHub Experiment Plan

Status: planned
Date: 2026-08-27

Purpose: convert important GitHub behavior from documentation/inference into reproducible observations before Regelverket's planner or adaptation engine depends on it.

## Rules

1. Experiments run only in disposable test repositories/organizations designed for the test.
2. Each experiment records GitHub product/plan, repository visibility, API version, timestamp, actor identity/permissions, exact configuration, operation, API responses and observed UI/runtime result.
3. A documented fact is not silently upgraded to an observed fact.
4. Unexpected behavior becomes a regression fixture, not an undocumented workaround.
5. Destructive experiments require a snapshot and teardown procedure.

## Test record schema

```yaml
id: EXP-000
status: planned # planned|running|passed|failed|inconclusive
question: ""
environment:
  github_product: github.com
  plan: unknown
  owner_type: organization
  visibility: public
  api_version: "2026-03-10"
actors: []
setup: []
actions: []
expected_from_docs: ""
observed: ""
evidence: []
architecture_impact: []
cleanup: []
```

## Priority P0 — correctness blockers

### EXP-001 Effective rules and overlapping rulesets
Create overlapping repository rulesets with different includes/excludes and verify structural enumeration versus `GET /repos/{owner}/{repo}/rules/branches/{branch}` for existing and nonexistent branches.

Validates: effective-policy oracle, targeting normalization, proposed-branch preflight.

### EXP-002 Required check skipped by path/branch filter
Make a check required while its provider workflow can be skipped. Test PRs that do and do not match filters.

Validates: provider-safety constraint and adaptation reuse rules.

### EXP-003 Duplicate required-check context providers
Create two workflows/jobs capable of publishing the same context and require that context.

Validates: whether GitHub ambiguity is operationally safe and how detect should classify providers.

### EXP-004 Merge queue without merge_group
Require a GitHub Actions check, enable merge queue, omit `merge_group`, enqueue a PR and capture behavior.

Validates: merge-queue constraint and diagnostics.

### EXP-005 Merge queue with merge_group
Repeat EXP-004 with correct trigger and record check names, SHAs, workflow events and queue transitions.

Validates: positive queue archetype fixture.

### EXP-006 Matrix check names
Run matrix jobs with stable and dynamic job names; inspect emitted check runs/status contexts and ruleset requirements.

Validates: check-provider identity model.

### EXP-007 Required workflow invocation
Configure a ruleset workflow with pull_request/merge_group filters and record which filters GitHub ignores, event payloads, check identities and behavior for already-open PRs.

Validates: required-workflow capability model.

### EXP-008 Bypass visibility under permissions
Read the same ruleset using actors/tokens with different permissions and compare `bypass_actors` presence.

Validates: unknown/inaccessible state model.

## Priority P1 — reconciliation and migration

### EXP-009 Ruleset create/update/delete idempotence
Apply equivalent payloads repeatedly, reorder semantically unordered arrays where possible and record API/resource changes.

Validates: canonicalization and reconciliation strategy.

### EXP-010 Ruleset history and rollback
Create several ruleset versions, query history/version endpoints and test restoration strategy.

Validates: rollback architecture.

### EXP-011 Required checks on branch creation
Test required checks with `do_not_enforce_on_create` true/false against new branches.

Validates: safe branch-slot bootstrap.

### EXP-012 Classic branch protection + ruleset
Apply both mechanisms to the same branch with partially overlapping requirements.

Validates: legacy-policy discovery and effective-policy explanation.

### EXP-013 Ref pattern semantics
Test exact refs, `*`, `**`, slash boundaries, `~DEFAULT_BRANCH`, `~ALL`, includes and excludes.

Validates: targeting solver.

### EXP-014 Signed commits and automation
Test human signed commits, GitHub-generated squash/rebase merges and selected bot/App flows where available.

Validates: signed-commit recommendation constraints.

### EXP-015 Required deployment
Create an environment/deployment gate and test success, failure, missing deployment and renamed environment.

Validates: deployment dependency graph.

## Priority P2 — advanced/product-specific

### EXP-016 Push ruleset and fork network
Where plan/capability permits, verify root/fork enforcement and bypass behavior.

### EXP-017 Evaluate mode and rule suites
Exercise evaluate mode and capture Rule Insights/rule-suite API behavior.

### EXP-018 Organization parent rulesets
Where available, combine organization and repository rulesets and compare parent enumeration/effective rules.

### EXP-019 Required reviewers
Where available, test team/file-pattern reviewer semantics, negation and zero-approval visibility behavior.

### EXP-020 Preview capabilities
Isolate Code Quality/coverage, Copilot review and license compliance tests behind capability flags. Preview behavior must never be assumed stable.

## Acceptance rule for architecture dependencies

A behavior may be encoded as a hard safety constraint when either:

- GitHub explicitly documents it and the constraint does not depend on ambiguous runtime behavior; or
- it has a reproducible live experiment with recorded evidence.

Recommendations may use weaker evidence, but must expose confidence/provenance. Destructive adaptation must use the stronger standard.

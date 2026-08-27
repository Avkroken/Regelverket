# GitHub Rule & Capability Matrix v0

Status: research baseline, 2026-08-27

This document is a factual input to Regelverket's capability, constraint, detection and template engines. It is not yet a template catalog and does not claim that every accepted API combination is operationally safe.

## Evidence labels

- **DOC** — explicitly documented by current GitHub documentation/API schema.
- **OBS** — observed in a controlled live repository experiment.
- **INF** — inferred from multiple documented facts; must not be treated as platform guarantee.
- **OPEN** — requires further research or live experiment.

## Platform scope

Repository rulesets can target `branch`, `tag`, or `push`. Current repository REST creation accepts these targets. Enterprise-level schemas additionally expose a `repository` target. Rulesets may be `active`, `disabled`, or `evaluate`; effective-rule queries exclude evaluate and disabled rulesets. [DOC]

Repository ruleset listing supports `includes_parents=true`, allowing higher-level rulesets that apply to a repository to be returned. The branch-effective-rules endpoint returns all active rules applying to a branch regardless of repository/organization source, and the branch need not exist. [DOC]

`bypass_actors` is permission-sensitive: GitHub omits it from a ruleset response when the caller lacks write access to that ruleset. Detection must therefore represent unknown/inaccessible bypass state rather than interpreting absence as an empty bypass list. [DOC]

## Availability baseline

GitHub documents repository rulesets for public repositories on Free/Free for organizations, and public/private repositories on Pro, Team and Enterprise Cloud. Push-ruleset availability differs by plan and product context; this must be resolved dynamically rather than encoded as a single global boolean. [DOC]

Enterprise Cloud exposes additional rules/capabilities not present in the general repository-rules page. Therefore Regelverket must version capability knowledge by GitHub product/account/plan rather than assume one universal ruleset vocabulary. [DOC]

## Branch/tag rules

| Rule | API type | Target | Core semantics | Important constraints / dependencies | Evidence |
|---|---|---|---|---|---|
| Restrict creations | `creation` | branch/tag | Only bypass actors may create matching refs | Interacts with branch bootstrap and required checks | DOC |
| Restrict updates | `update` | branch/tag | Only bypass actors may update matching refs | API has `update_allows_fetch_and_merge` | DOC |
| Restrict deletions | `deletion` | branch/tag | Only bypass actors may delete matching refs | Default-selected in GitHub UI documentation | DOC |
| Linear history | `required_linear_history` | branch/tag | Prevent merge commits on matching refs | Repo must allow squash or rebase merge | DOC |
| Merge queue | `merge_queue` | branch | Merges must use queue | Repository-level only in current Enterprise Cloud docs; workflows providing required checks need merge-group compatibility | DOC |
| Required deployments | `required_deployments` | branch | Selected environments must deploy successfully before merge | Current Enterprise Cloud docs say not available on org-level rulesets | DOC |
| Signed commits | `required_signatures` | branch/tag | Commits must have verified signatures | Branch creation semantics differ from classic branch protection; bot/signing implications | DOC |
| Pull request | `pull_request` | branch | Changes must be associated with PR before target update | Rich review/merge parameters; see below | DOC |
| Required status checks | `required_status_checks` | branch/tag | Named checks/statuses must pass | Check provider identity, strictness and skip behavior are critical | DOC |
| Block force pushes | `non_fast_forward` | branch/tag | Prevent non-fast-forward updates | Does not override other rules | DOC |
| Code scanning | `code_scanning` | branch | Required tools must meet alert thresholds | Code scanning must be configured and produce results | DOC |
| Required workflows | `workflows` | branch | Selected workflow files must pass | Capability/product scope needs deeper mapping; event semantics need testing | DOC |

## Pull-request rule parameter matrix

Current REST schema exposes:

- `allowed_merge_methods`: merge/squash/rebase; at least one must be enabled.
- `dismiss_stale_reviews_on_push`.
- `dismissal_restriction`: allowed User, Team, IntegrationInstallation or RepositoryRole actors.
- `require_code_owner_review`.
- `require_last_push_approval`.
- `required_approving_review_count`.
- `required_review_thread_resolution`.
- `required_reviewers`: beta, team + file-pattern based reviewers with minimum approvals.

Current GitHub documentation additionally notes that stale-review dismissal and last-push approval can be invalidated by merge-base changes. Required reviewers are not available for user-owned repositories because they rely on teams. [DOC]

A public-preview behavior currently adds an extra approval by default for unattributed Copilot-created pull requests when approval count is nonzero. This is volatile capability data and must not be hard-coded as permanent semantics. [DOC/PREVIEW]

## Required-status-check matrix

REST parameters:

- `required_status_checks[].context`: required context name.
- `required_status_checks[].integration_id`: optional GitHub App/provider restriction.
- `strict_required_status_checks_policy`: require testing against latest base state.
- `do_not_enforce_on_create`: allow branch/repository creation despite missing check.

GitHub documents strict checks as requiring the topic branch to be current with base; loose checks do not. GitHub also documents that a required check can be restricted to a specific GitHub App. [DOC]

### Constraint candidates

1. A required check must have at least one discoverable provider before Regelverket enables it. [INF -> test]
2. Multiple providers emitting the same required context are ambiguous and must be surfaced to the user even if GitHub accepts the configuration. [INF -> test]
3. A workflow that can be skipped by branch/path/commit filtering must not be naively selected as a required status-check provider: GitHub documents skipped required workflows/checks remaining pending and blocking merge. [DOC]
4. With merge queue, any GitHub Actions provider needed by the queue must support the `merge_group` event. [DOC]

## Merge queue parameters

REST currently exposes:

- `check_response_timeout_minutes`
- `grouping_strategy`: `ALLGREEN` or `HEADGREEN`
- `max_entries_to_build`
- `max_entries_to_merge`
- `merge_method`: MERGE/SQUASH/REBASE
- `min_entries_to_merge`
- `min_entries_to_merge_wait_minutes`

Enterprise Cloud documentation states the rule is repository-level, not organization-level. Build concurrency controls dispatch of `merge_group.checks_requested`. [DOC]

Regelverket must model merge queue as a capability bundle, not a boolean: account/plan availability + repository setting/rule + compatible merge method + compatible required-check providers + workflow triggers. [INF]

## Required workflow rule

REST representation contains a list of workflow references with:

- repository ID
- workflow path
- optional ref
- optional SHA
- `do_not_enforce_on_create`

This differs materially from a required status check: the policy identifies a workflow resource rather than merely a status context. Regelverket must keep these as distinct capability types. [DOC]

Current GitHub troubleshooting documentation says ruleset workflows support `pull_request`, `pull_request_target`, and `merge_group`; event filters on those supported events are ignored when invoked as ruleset workflows. This requires dedicated live tests before adaptation logic edits existing workflows. [DOC -> OBS required]

## Push rules

| Rule | API type | Semantics | Parameters | Notes | Evidence |
|---|---|---|---|---|---|
| Restrict file paths | `file_path_restriction` | Block pushes containing matching paths | restricted paths | GitHub docs specify entry/length limits; Enterprise docs expose allowed exceptions in some contexts | DOC |
| Max path length | `max_file_path_length` | Block paths longer than threshold | character limit | Push rule | DOC |
| Restrict extensions | `file_extension_restriction` | Block listed extensions | extensions | Push rule | DOC |
| Max file size | `max_file_size` | Block files above MB threshold | MB limit | Does not apply to Git LFS | DOC |

Push rulesets apply to the repository's fork network, and bypass permissions for forks derive from the root repository. This is a high-impact semantic difference from ordinary branch rules and must be shown prominently in plans. [DOC]

## Metadata/pattern rules exposed by REST

Current repository REST schema exposes at least:

- `commit_message_pattern`
- `commit_author_email_pattern`
- `committer_email_pattern`
- `branch_name_pattern`
- `tag_name_pattern`

Pattern parameters support `starts_with`, `ends_with`, `contains`, or `regex`, plus `negate`. [DOC]

Enterprise Cloud documentation contains additional metadata and product-specific rules. These require a second matrix scoped to organization/enterprise capabilities before generic template recommendations use them. [OPEN]

## New/volatile capabilities observed in current schema

The current Enterprise Cloud repository-rules schema also exposes `copilot_code_review` and `license_compliance_scanning`. These were not assumptions in the original Regelverket design and demonstrate why the knowledge base must be versioned and refreshable. [DOC]

`copilot_code_review` includes options for draft PR review and review on push. License-compliance scanning enforces organization license policy for added/changed dependencies. Exact plan/product prerequisites require dedicated capability research. [OPEN]

## Conditions and targeting

Repository ref conditions support `include` and `exclude`, with special selectors including `~DEFAULT_BRANCH` and `~ALL`. [DOC]

Organization/enterprise rulesets add repository selectors/properties and higher-level source semantics. Regelverket's normalized graph must retain the source level of every rule; flattening rules without provenance would make safe editing impossible. [DOC/INF]

## Bypass model

Current REST actor types include Integration, OrganizationAdmin, RepositoryRole, Team, DeployKey, EnterpriseOwner, EnterpriseRole and User, subject to context. Bypass modes include `always`, `pull_request`, and `exempt`; `pull_request` applies only to branch rulesets and not DeployKey, while `exempt` means rules are not run and no bypass audit entry is created. [DOC]

Required design consequence: bypass is not simply an allow-list. Internal representation needs actor type, stable actor identity, mode, source ruleset and visibility/knowledge state. [INF]

## Effective-policy detection

Regelverket should use two complementary reads:

1. **Structural read** — enumerate rulesets with parents included, preserving source, enforcement, conditions, rule parameters and visible bypass actors.
2. **Effective read** — query active rules for representative or proposed branch names.

The effective branch endpoint can evaluate a branch name even if the branch does not yet exist. This is particularly useful for `plan`, because Regelverket can ask GitHub what rules would hit a proposed slot before creating it. [DOC/ARCHITECTURE]

Neither view alone is sufficient: the effective endpoint excludes evaluate/disabled rulesets, while structural reads may contain permission-redacted data. [DOC]

## Initial normalized capability states

Every capability result should use more than true/false. Proposed states:

- `available`
- `unavailable_plan`
- `unavailable_account_type`
- `unavailable_visibility`
- `permission_missing`
- `configured`
- `misconfigured`
- `unknown_inaccessible`
- `unknown_unresearched`
- `preview`

A capability record should include evidence source, observed timestamp, GitHub product/API version, prerequisites and blockers.

## Immediate constraint rules justified by documentation

- `required_linear_history` => repository must allow squash or rebase.
- `merge_queue` => repository-level capability in current Enterprise Cloud ruleset docs.
- `merge_queue + required GitHub Actions check` => provider workflow must handle merge-group execution.
- `required_status_check + skip-capable provider` => unsafe until provider semantics prove the check always resolves.
- `required_deployments` => referenced environments must exist and successfully deploy.
- `required_signatures` => bot/automation signing compatibility must be assessed before recommendation.
- `pull_request.allowed_merge_methods` must intersect repository-enabled merge methods.
- `required_reviewers` => organization-owned repository/team context required.
- `push ruleset` => impact includes fork network; plan must disclose this scope.
- inaccessible `bypass_actors` => never normalize to empty list.

## Live experiment backlog

The following should become reproducible test cases in disposable repositories before implementation depends on them:

1. Layering of multiple repository rulesets with overlapping include/exclude patterns.
2. Repository + organization ruleset layering and effective-rule response.
3. `evaluate` behavior and Rule Insights/rule-suite APIs.
4. Branch creation under required checks with/without `do_not_enforce_on_create`.
5. Required check provider ambiguity when two workflows/jobs emit identical contexts.
6. Matrix job context naming and stability.
7. Required check + path-filter skip behavior.
8. Merge queue + missing `merge_group` trigger.
9. Merge queue grouping strategies and status timeout behavior.
10. Required workflow invocation semantics and ignored filters.
11. Reusable workflow/check naming across `workflow_call`.
12. Ruleset update atomicity and partial-failure behavior.
13. Permission-redacted bypass detection using lower-privilege credentials.
14. Signed commits from bots/GitHub Apps and squash/rebase combinations.
15. Push-ruleset fork-network effects.
16. Ruleset history/version endpoints for rollback support.
17. Ref `fnmatch` edge cases, including `/`, `**`, special selectors and exclusions.
18. Interaction between classic branch protection and rulesets targeting the same branch.

## Research still required before template recommendations

- Exact current plan/account/visibility matrix for every rule type.
- Organization and enterprise ruleset conditions, repository properties and policy precedence.
- Repository-target rulesets.
- Code Quality and code coverage preview semantics/costs.
- Copilot code review rule prerequisites/costs.
- License compliance scanning prerequisites.
- Environments/deployment protection semantics.
- GitHub Apps and automation identity/bypass design.
- Checks API versus commit-status API naming/identity behavior.
- CODEOWNERS and required-reviewer interaction.
- Dependabot/Renovate and other bot compatibility.
- Actions permissions, reusable workflows and supply-chain controls.
- API rate limits, pagination and concurrency behavior for multi-repo detection.
- GitHub Enterprise Server version differences.

## Architectural conclusion from v0

The data already rules out a file-template-only implementation. Regelverket needs a versioned knowledge base plus a normalized graph representing source level, target, conditions, rules, dependencies, actors, workflows/check providers and uncertainty. Detection must be permission-aware; planning must reason about effective policy; verification must compare both managed resources and GitHub's effective active rules.

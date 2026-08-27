# Verified GitHub research baseline — 2026-08-27

Evidence class for this file: `documented`, from current GitHub primary documentation/API documentation. This is an initial baseline, not the complete capability matrix.

## Ruleset layering

GitHub documents that multiple rulesets may apply to the same branch or tag at the same time. Rules are aggregated; when the same rule is defined differently across applicable rulesets, the most restrictive version applies. Rulesets also layer with legacy branch protection rules.

Architecture consequence:

- repository-local rulesets alone are not sufficient to describe effective policy,
- detection and verification need to model overlapping policy sources,
- adaptation must avoid assuming an exclusion/removal in one ruleset removes protection supplied by another ruleset/branch-protection source.

Source:
https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-rulesets/about-rulesets

## Effective rules for a branch API

GitHub's REST endpoint `Get rules for a branch` returns all active rules that apply to the specified branch, including rules configured at repository or organization level. The branch name does not need to exist. Rulesets in `evaluate` or `disabled` enforcement status are not returned by this endpoint.

Architecture consequence:

- this endpoint is a candidate effective-policy probe for `detect`/`verify`,
- desired-resource verification and effective-policy verification should remain separate,
- evaluate/disabled policy still needs separate inventory if Regelverket wants a complete policy picture.

Source:
https://docs.github.com/en/rest/repos/rules?apiVersion=2022-11-28

## Partial visibility of bypass actors

GitHub documents for organization repository rulesets that `bypass_actors` is returned only when the caller has write access to the ruleset, to avoid leaking sensitive information.

Architecture consequence:

- absence of `bypass_actors` in an API response cannot always be interpreted as an empty bypass list,
- the observed-state model needs states such as `unknown`/`inaccessible`, not just present/absent,
- destructive adaptation must fail closed when relevant higher-level policy cannot be observed safely.

Source:
https://docs.github.com/en/rest/orgs/rules?apiVersion=2022-11-28

## Merge queue and GitHub Actions required checks

GitHub documents that workflows providing required checks for a merge queue must trigger on the `merge_group` event. Without it, required checks are not reported for the queued merge group and the merge fails.

Architecture consequence:

- `merge_queue + required GitHub Actions check` is a hard companion constraint,
- workflow adaptation must inspect event triggers before declaring an existing CI workflow reusable,
- template validation must include merge-group behavior in E2E tests.

Sources:
https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/configuring-pull-request-merges/managing-a-merge-queue?apiVersion=2022-11-28
https://docs.github.com/en/actions/reference/workflows-and-actions/events-that-trigger-workflows

## Required checks and skipped workflows

GitHub documents that when an entire required workflow is skipped due to path/branch filtering, the required check can remain pending and block merging.

Architecture consequence:

- semantic workflow compatibility must include trigger/filter behavior, not only job/check names,
- Regelverket should detect required checks whose provider can be skipped for relevant changes,
- this belongs in the constraint/anti-pattern knowledge base.

Source:
https://docs.github.com/en/pull-requests/how-tos/merge-and-close-pull-requests/troubleshooting-required-status-checks?apiVersion=2022-11-28

## Required check provider identity

GitHub documents that a protected branch can require a status check from a specific GitHub App.

Architecture consequence:

- a required check is not always identified by name alone,
- normalized check identity should support expected/provider App identity where configured,
- adaptation/reuse must verify that an existing provider is compatible with the required check policy.

Source:
https://docs.github.com/en/pull-requests/how-tos/merge-and-close-pull-requests/troubleshooting-required-status-checks?apiVersion=2022-11-28

## Workflow directory constraint

GitHub documents that reusable workflows, like normal workflows, must be located in `.github/workflows`; subdirectories of the workflows directory are not supported.

Architecture consequence:

- executable Regelverket workflow entrypoints cannot live under `.github/workflows/regelverk/`,
- dedicated Regelverket metadata/configuration can still live elsewhere such as `.github/regelverk/`,
- generated workflow naming must resolve collisions within the flat workflow directory namespace.

Source:
https://docs.github.com/en/enterprise-cloud@latest/actions/how-tos/reuse-automations/reuse-workflows

## Ruleset ref pattern semantics

GitHub documents organization ruleset targeting with `fnmatch` and notes that `*` does not match directory separators (`/`) because GitHub uses `File::FNM_PATHNAME`; patterns such as `qa/*` and `qa/**/*` therefore have materially different scope.

Architecture consequence:

- include/exclude/ref matching must use GitHub-compatible semantics rather than generic glob assumptions,
- the policy graph/constraint engine needs a normalized matcher that can explain which refs are targeted,
- template tests should include multi-level branch names.

Source:
https://docs.github.com/en/organizations/managing-organization-settings/creating-rulesets-for-repositories-in-your-organization

## Next research tranche

Priority areas still to map:

1. complete current repository/organization rule-type and parameter catalogs,
2. plan/account/visibility availability and pricing constraints,
3. branch protection compatibility and migration behavior,
4. ruleset evaluate/history/rule-suite APIs,
5. GitHub App/bypass actor identity and permissions,
6. merge methods, linear history and merge queue interactions,
7. check/job identity including matrix/reusable workflows,
8. repository settings and environment/deployment constraints,
9. Dependabot and automation identities,
10. controlled live experiments for undocumented or ambiguous behavior.

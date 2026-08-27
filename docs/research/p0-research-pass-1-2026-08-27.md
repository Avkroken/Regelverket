# P0 Research Pass 1 — GitHub Semantics

Date: 2026-08-27
Status: primary-source research; live mutation experiments still pending
API documentation context: GitHub REST version shown by current docs: `2026-03-10`

## Purpose

Resolve as much of the P0 decision-gate research as possible from current GitHub primary sources before live mutation experiments. Claims below distinguish documented behavior from observed connector/API behavior.

## 1. Ruleset availability and targets

### Documented

Repository rulesets are available for public repositories on GitHub Free/Free for organizations, and public/private repositories on Pro, Team and Enterprise Cloud.

Push rulesets are separately gated. Current general GitHub documentation states availability for Team in internal/private repositories and forks of repositories with push rules enabled; Enterprise Cloud documentation exposes the corresponding Enterprise capability.

Repository REST ruleset target values are `branch`, `tag`, and `push`. Organization REST rulesets additionally expose `repository` targeting.

Organization-level rulesets require Team or Enterprise-class organization capability according to current GitHub documentation.

Architecture consequence: target and availability are capability-scoped values, not universal enum assumptions.

Sources:
- https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-rulesets/available-rules-for-rulesets
- https://docs.github.com/en/organizations/managing-organization-settings/creating-rulesets-for-repositories-in-your-organization
- https://docs.github.com/en/rest/repos/rules?apiVersion=2026-03-10
- https://docs.github.com/en/rest/orgs/rules?apiVersion=2026-03-10

## 2. Rule layering vs classic branch protection

### Documented

Rulesets are layered: multiple active rulesets can apply to the same ref, including rulesets from repository and organization scope. GitHub's effective-rules endpoint returns all active rules that apply to a named branch regardless of source level. `evaluate` and `disabled` rulesets are excluded from that endpoint.

Classic branch protection has different resolution semantics: only one branch protection rule applies at a time. Exact-name rules have higher priority; wildcard rules have creation-order priority under documented conditions.

Architecture consequence: classic branch protection and rulesets must be modeled as separate policy sources. A normalizer must not flatten classic branch protection into ruleset layering semantics.

Sources:
- https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-rulesets/about-rulesets
- https://docs.github.com/en/rest/repos/rules?apiVersion=2026-03-10
- https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-protected-branches/managing-a-branch-protection-rule

## 3. Effective policy oracle

### Documented

`GET /repos/{owner}/{repo}/rules/branches/{branch}` returns all active rules applying to a branch. The branch does not have to exist. The response attributes each rule with ruleset source type/source/ID. Public resources can be queried without authentication; fine-grained authenticated access needs Metadata read.

Architecture consequence: discovery/verification can query hypothetical branch names to test template/ref-class boundary cases without first creating those branches. This is stronger than only inspecting existing refs.

Source:
- https://docs.github.com/en/rest/repos/rules?apiVersion=2026-03-10

## 4. Bypass identity and partial visibility

### Documented

Current REST schemas expose bypass actor types including Integration, RepositoryRole, Team, User and scope-dependent administrative/enterprise/deploy-key actor forms. Bypass modes include `always`, `pull_request`, and in current organization schema `exempt` under applicable conditions.

GitHub explicitly states that `bypass_actors` is returned only when the caller has write access to the ruleset, to avoid leaking sensitive information.

Architecture consequence: absence of `bypass_actors` cannot be normalized to an empty bypass list unless sufficient access has been proven. `inaccessible` is a required value state.

Sources:
- https://docs.github.com/en/rest/orgs/rules?apiVersion=2026-03-10
- https://docs.github.com/en/rest/repos/rules?apiVersion=2026-03-10

## 5. Permissions split

### Documented

The effective branch-rules endpoint requires only repository Metadata read for authenticated private access and can be anonymous for public resources.

Creating repository rulesets requires repository Administration write for fine-grained tokens/App tokens. Current organization ruleset management endpoints require organization Administration write.

Rule-suite/insight APIs have stronger administration requirements than effective branch-rule reading.

Architecture consequence: `detect` must be decomposed into probes with separate permission requirements. A user should receive useful read-only analysis without granting mutation permissions.

Sources:
- https://docs.github.com/en/rest/repos/rules?apiVersion=2026-03-10
- https://docs.github.com/en/rest/orgs/rules?apiVersion=2026-03-10
- https://docs.github.com/en/rest/repos/rule-suites?apiVersion=2026-03-10

## 6. `fnmatch` targeting semantics

### Documented

GitHub documents use of Ruby-style `File.fnmatch` semantics with `File::FNM_PATHNAME` for ruleset/branch targeting. Therefore `*` does not match `/`. Example: `qa/*` matches one path/name segment after `qa/` but not `qa/foo/bar`. GitHub documents `qa/**/*` as a form that can match deeper slash-separated names.

Character sets in `[]` are supported. GitHub currently documents that complementing a character set with `^` is not supported. `File::FNM_EXTGLOB` is not supported.

Architecture consequence: do not use shell glob, minimatch, doublestar or regex libraries as an assumed equivalent. Regelverket needs a compatibility-tested matcher or must delegate authoritative boundary verification to GitHub.

Sources:
- https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-rulesets/creating-rulesets-for-a-repository
- https://docs.github.com/en/organizations/managing-organization-settings/creating-rulesets-for-repositories-in-your-organization

## 7. Required status-check identity

### Documented

Current GitHub troubleshooting documentation states:

- workflow status-check name format is the job name;
- reusable workflow format is `<job name> / <reusable job name>`;
- other checks use the check name;
- required status checks do not take workflow, matrix, or event trigger type into account;
- if a check run and a commit status have the same required name, both must pass;
- a required status check must have completed successfully in the repository during the previous seven days;
- duplicate job names across workflows can create ambiguous status-check results and block merging.

Ruleset REST parameters can bind a required status check to an optional `integration_id`.

Architecture consequence: check identity cannot be represented as workflow filename. The provider graph must model context/name, check-vs-status origin, integration/App identity, workflow/job source, reusable-workflow expansion, and event coverage separately.

Sources:
- https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-rulesets/troubleshooting-rules
- https://docs.github.com/en/pull-requests/how-tos/merge-and-close-pull-requests/troubleshooting-required-status-checks
- https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-protected-branches/about-protected-branches
- https://docs.github.com/en/rest/repos/rules?apiVersion=2026-03-10

## 8. Required-check creation and strictness

### Documented

The REST ruleset schema includes:

- `required_status_checks[]`
- `context`
- optional `integration_id`
- `strict_required_status_checks_policy`
- `do_not_enforce_on_create`

GitHub documents that strict/up-to-date behavior only takes effect when at least one status check is configured.

Architecture consequence: creation behavior and strictness are independent parameters in the normalized rule model.

Source:
- https://docs.github.com/en/rest/repos/rules?apiVersion=2026-03-10

## 9. Merge queue capability

### Documented

Current GitHub documentation states that pull-request merge queues are available in any public repository owned by an organization, and in private organization-owned repositories using GitHub Enterprise Cloud.

For GitHub Actions required checks, workflows must include the `merge_group` event for checks to execute when a pull request enters a merge queue. Without this event, required checks are not reported for the merge group and the merge cannot complete.

Architecture consequence: merge queue is not a simple boolean template option. Capability resolution must include owner type, visibility/plan, merge-queue rule availability and lifecycle coverage for every required check provider.

Sources:
- https://docs.github.com/en/pull-requests/how-tos/merge-and-close-pull-requests/merging-a-pull-request-with-a-merge-queue
- https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/configuring-pull-request-merges/managing-a-merge-queue
- https://docs.github.com/en/pull-requests/how-tos/merge-and-close-pull-requests/troubleshooting-required-status-checks

## 10. Rule inventory — current schema families

### Documented

Current REST/GraphQL documentation exposes at least the following normalized rule families across supported scopes/targets. Availability is not assumed identical for every target/plan:

- creation
- update
- deletion
- required_linear_history
- required_deployments
- required_signatures
- pull_request
- required_status_checks
- non_fast_forward
- commit_message_pattern
- commit_author_email_pattern
- committer_email_pattern
- branch_name_pattern
- tag_name_pattern
- file_path_restriction
- max_file_path_length
- file_extension_restriction
- max_file_size
- workflows
- code_scanning
- copilot_code_review
- license_compliance_scanning (current Enterprise Cloud schema)

The GraphQL schema also exposes rule-type enumeration and typed parameter inputs. REST remains the current primary rendering/API reference for this research pass.

Important: this is an inventory, not yet a target-by-target certification matrix. P0 remains open until each family has target, scope, plan and parameter constraints recorded.

Sources:
- https://docs.github.com/en/rest/repos/rules?apiVersion=2026-03-10
- https://docs.github.com/en/rest/orgs/rules?apiVersion=2026-03-10
- https://docs.github.com/en/graphql/reference/repos

## 11. Push-rule REST behavior

### Documented

GitHub states push rulesets also apply to REST operations that create blobs, trees, and create/update repository file contents. GitHub also documents a maximum of 1000 reference updates per push when push rulesets are in use.

Architecture consequence: a migration executor using GitHub Contents/Git Data APIs is not outside push-policy enforcement. Planner error classification must recognize policy rejection from API writes as enforcement, not generic transport failure.

Source:
- https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-rulesets/troubleshooting-rules

## 12. Repository-level observation performed in this pass

### Observed

Using the connected GitHub API against `Avkroken/Regelverket`, `GET /repos/Avkroken/Regelverket/rulesets` returned an empty array on 2026-08-27. This is only evidence that the repository-level list visible through that request was empty at observation time. It is not evidence that organization-level effective policy is absent.

The connected GitHub fetch surface did not permit the effective branch-rules endpoint in this session, so no claim about `main` effective policy is recorded from that failed probe.

Architecture consequence: the research process itself follows the uncertainty rule — failed/inaccessible probes are not converted into absence.

## P0 status after pass 1

### Substantially source-resolved

- ruleset vs classic branch-protection resolution model
- effective branch-rule endpoint semantics
- core bypass partial-visibility hazard
- core read-vs-write permission split
- documented fnmatch baseline
- required-check naming baseline
- seven-day required-check eligibility statement
- merge-queue account/visibility baseline
- `merge_group` requirement

### Still requires live experiments / deeper matrix work

- exact aggregation outcome for overlapping rules of the same type and conflicting parameters
- ruleset + classic branch protection combined effective behavior
- duplicate check providers in real runs, including check-run + commit-status collision
- matrix job runtime naming across representative matrix definitions
- skipped workflow/job combinations and required-check outcomes
- expected App/integration binding behavior under provider changes
- merge queue E2E with multiple required providers
- lower-privilege API field/error behavior across repo/org scopes
- fnmatch edge corpus verified against GitHub rather than documentation alone
- target/scope/plan matrix for every current rule family

## Decision-gate impact

DG-02 (internal schema) is not yet closed. The evidence is sufficient to continue graph/schema prototyping, but not to freeze the rule/check contracts.

DG-06 (first certified template) remains blocked on live E2E.

## Live experiment tooling note

The currently connected GitHub tool can read repository rulesets and mutate repository files/refs, but this session does not expose ruleset create/update/delete or repository-creation operations. Therefore destructive/live ruleset experiments must not be improvised against `Avkroken/Regelverket` or another production repository.

A disposable GitHub repository plus an API surface/credential capable of ruleset mutation is required for the P0 live suite. Until that exists, experiments remain explicitly pending rather than simulated and mislabeled as observations.

# GitHub capability and cost matrix

Status: research baseline
Date: 2026-08-27

This document records product/account/visibility constraints that Regelverket must treat as data. It intentionally distinguishes documented availability from capabilities that still need live verification.

## Core dimensions

Capability resolution must consider at least:

- owner kind: individual / organization / enterprise
- GitHub plan: Free / Pro / Free for organizations / Team / Enterprise Cloud
- repository visibility: public / private / internal
- caller permissions and token/App permissions
- optional paid products such as GitHub Code Security / Code Quality
- Actions runner model: GitHub-hosted / self-hosted
- user cost policy: free-only / budget-capped / paid-ok

A capability must never be represented as a single boolean. Suggested states:

- available
- available-with-metered-cost
- available-with-addon
- requires-plan-upgrade
- requires-owner-type-change
- permission-missing
- unavailable
- unknown
- inaccessible
- preview

## Repository rulesets

Documented baseline:

- Public repositories: rulesets are available with GitHub Free, GitHub Free for organizations, GitHub Pro, GitHub Team, and GitHub Enterprise Cloud.
- Private repositories: repository rulesets are documented for GitHub Pro, GitHub Team, and GitHub Enterprise Cloud.
- A repository can have up to 75 rulesets.
- Read access can view repository rulesets; admin access or a custom role with `edit repository rules` can create/edit/delete them.

Implication: a public individual Free repository can be a valid target for substantial Regelverket functionality without requiring an organization.

## Organization-level rulesets

GitHub documents organization rulesets for GitHub Team and GitHub Enterprise plans. Organization owners can create them, and current documentation also describes a dedicated organization permission for managing organization ref-update rules/rulesets.

Organization rulesets can target multiple repositories and can select repositories using mechanisms including explicit selection, naming convention, deployment context, and custom properties.

Implication: organization governance is a separate capability module. A repository admin may be able to add repository rulesets while being unable to modify an organization ruleset already affecting the repository.

## Push rulesets

Current general GitHub documentation states that push rulesets are available on GitHub Team for internal/private repositories and forks of repositories where push rulesets are enabled. Enterprise documentation describes Enterprise Cloud support as well.

Push rulesets protect the entire fork network from the root repository. Their bypass model therefore has wider consequences than an ordinary branch ruleset.

Research requirement: live-test exact plan/owner combinations and API behavior before encoding a final matrix because GitHub documentation has historically differed between general and Enterprise-specific pages.

## GitHub Actions cost model

Documented baseline:

- Standard GitHub-hosted runners are free for public repositories.
- Self-hosted runner usage is not billed as GitHub-hosted Actions minutes.
- Private repositories receive an included quota of GitHub-hosted minutes/storage according to plan; usage beyond included allowance can be billed.

Published included monthly GitHub-hosted Actions minutes at the time of research:

| Plan | Included minutes/month |
| --- | ---: |
| GitHub Free | 2,000 |
| GitHub Pro | 3,000 |
| GitHub Free for organizations | 2,000 |
| GitHub Team | 3,000 |
| GitHub Enterprise Cloud | 50,000 |

Published Actions storage allowances:

| Plan | Storage |
| --- | ---: |
| GitHub Free | 500 MB |
| GitHub Pro | 1 GB |
| GitHub Free for organizations | 500 MB |
| GitHub Team | 2 GB |
| GitHub Enterprise Cloud | 50 GB |

These numbers are time-sensitive and must not be hard-coded into the executable without versioned capability data and an update mechanism.

## Code scanning / GitHub Code Security

GitHub documents code scanning as enabled by default for public repositories. For private/internal repositories, GitHub Free/Pro do not provide the equivalent private-repository capability; GitHub documentation directs users to Team/Enterprise plus GitHub Code Security for private/internal code scanning.

Therefore `require code scanning results` must have prerequisites in the knowledge model. Regelverket must not offer the rule merely because the Rulesets API schema contains it.

## GitHub Code Quality and coverage rules

GitHub Code Quality is documented for GitHub Team and GitHub Enterprise Cloud. It depends on GitHub Actions. Rulesets can enforce Code Quality findings and, while the coverage feature remains preview, line-coverage thresholds when Code Quality and coverage uploads are configured.

Consequences:

- `require code quality results` is capability-gated.
- `restrict code coverage` is capability-gated and currently preview-sensitive.
- enabling the rule without successful analysis can block merging, so capability detection must include operational readiness, not only subscription entitlement.

## Merge queue

Merge queue must be modeled independently from ordinary pull-request protection. Its availability and configuration depend on repository/account context, and its Actions integration imposes the `merge_group` requirement for required GitHub Actions checks.

Regelverket must represent at least:

- entitlement/availability
- whether merge queue is configured
- merge method and queue parameters
- whether every required Actions check has a merge-group execution path

Exact current plan/visibility entitlement is retained as a dedicated research/live-test item rather than inferred from UI assumptions.

## Required workflows / ruleset workflows

Current GitHub documentation describes ruleset workflows at organization/enterprise scope. The Rules API exposes a `workflows` rule containing workflow repository ID, path and optional ref/SHA.

Ruleset workflows support `pull_request`, `pull_request_target`, and `merge_group`. GitHub documents that event filters on these supported events are ignored when the workflow is invoked by a ruleset.

This is semantically different from merely requiring a named status check and must be a separate capability in Regelverket.

## Workflow execution protections

GitHub currently documents workflow execution protections as public preview. They are ruleset-backed and can restrict actors/events that may trigger Actions workflows at enterprise, organization and repository levels.

Because this is preview functionality, Regelverket should initially discover and explain it but avoid making it a default template dependency until its stability and plan availability are established.

## Cost policy model

Suggested user intent:

```yaml
cost_policy:
  mode: free-only # free-only | budget-capped | paid-ok
  monthly_limit_usd: null
  allow_plan_upgrade: false
  allow_paid_addons: false
  prefer_self_hosted: false
```

Planner behavior:

1. resolve required template capabilities;
2. resolve entitlement for current owner/plan/visibility;
3. estimate whether the design can create metered Actions usage;
4. reject options violating `free-only`;
5. for paid alternatives, explain what upgrade/add-on enables them rather than silently degrading the policy;
6. recommend a capability-equivalent free template variant when one exists.

## Open research items

- Exact merge-queue entitlement matrix in current GitHub plans.
- Exact push-ruleset behavior across Team vs Enterprise and organization/root-fork cases.
- API fields that expose plan/account capabilities without requiring billing permissions.
- Reliable detection of optional Code Security/Code Quality entitlement versus merely disabled configuration.
- Actions larger-runner and macOS multiplier implications for cost estimation.
- Enterprise Managed Users and data-residency differences.
- GitHub Enterprise Server compatibility/version matrix (separate from github.com / Enterprise Cloud).

## Sources

Primary sources used for this baseline are current GitHub Docs pages for About/Available Rules for Rulesets, organization rulesets, Actions billing and included product usage, Code Security/code scanning, GitHub Code Quality, ruleset workflows, and workflow execution protections. URLs are intentionally not treated as immutable facts: each fact in the eventual machine-readable knowledge base must carry a checked date and source identifier.
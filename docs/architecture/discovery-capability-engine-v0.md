# Discovery and Capability Engine v0

Status: architecture draft
Date: 2026-08-27

## Goal

Turn an unknown repository/account context into an evidence-backed model that can answer:

1. What exists?
2. What effective policy applies?
3. What capabilities are available?
4. What capabilities are operationally ready?
5. What is unknown because Regelverket lacks access?
6. Which template variants are compatible with the user's constraints?

## Discovery phases

### Phase A: repository facts

Collect non-destructive facts:

- owner kind
- visibility
- default branch
- branches/tags
- merge settings
- rulesets
- classic branch protection
- effective rules for representative branches
- Actions workflows
- environments/deployments where accessible
- CODEOWNERS
- Dependabot/Renovate indicators
- project manifests and language/build metadata
- documentation structure
- open PR/branch relationships when needed for migration planning

### Phase B: policy graph

Normalize repository and organization-visible protections into a policy graph. Preserve provenance for every node and edge.

Do not collapse `not returned` into `absent`.

### Phase C: workflow graph

Parse workflows and local dependencies. Build check-provider candidates and identify generated/shared/unmanaged resources.

### Phase D: capability probes

Resolve entitlements/configuration for features relevant to the detected repository and selected templates. Avoid querying irrelevant enterprise-only surfaces for a simple individual repository.

### Phase E: questions

Only ask questions that cannot be safely inferred, such as:

- solo vs team intent
- AI/bot concurrency expectations
- willingness to change branch topology
- cost policy
- desired review strictness
- whether existing resources may be adopted

## Evidence model

Every detected fact should carry:

```text
value
state: observed | inferred | user-declared | unknown | inaccessible
source
observed_at
confidence
```

Examples:

```text
merge_queue.available = unknown
reason = entitlement endpoint not accessible
```

is categorically different from:

```text
merge_queue.available = false
reason = documented owner/visibility/plan incompatibility
```

## Capability resolution

Suggested structure:

```yaml
capability: merge_queue
entitlement:
  state: available | unavailable | requires-upgrade | unknown
operational:
  state: ready | misconfigured | not-configured | unknown
requirements:
  - organization-owned repository
  - compatible plan/visibility
constraints:
  - required Actions providers must support merge_group
cost:
  class: included | metered | addon | upgrade | unknown
provenance: []
```

A template may require `entitlement=available` and `operational=ready`, or may contain an installation step that can move `not-configured -> ready`.

## Project classification

Detection should produce observations, not a single magical project label.

Possible signals:

- languages
- frameworks
- package/build managers
- monorepo markers
- test systems
- deployment targets
- docs generators
- package publishing
- release automation
- number/type of workflows
- bot configuration
- branch topology

Classification modules can then emit scored traits:

```text
traits:
  library: 0.9
  web_application: 0.2
  monorepo: 1.0
  deployment_heavy: 0.1
  bot_heavy: unknown
```

User intent remains separate.

## Smart template adaptation

Inputs:

```text
Observed Repository Graph
+ Effective Policy
+ Capability Model
+ User Intent
+ Cost Policy
+ Template Requirements
```

Output:

```text
Resolved Policy
+ Semantic Plan
+ Warnings/Blockers
+ Explanation
```

For each template requirement:

1. search for an existing semantic provider;
2. verify it is compatible and unambiguous;
3. mark as SHARED when reused without ownership;
4. propose ADOPT when management is beneficial and user permits it;
5. generate a new managed provider only when necessary;
6. stop on unresolved collision or unknown destructive consequence.

## Effective-policy verification

Where supported, use GitHub's effective branch-rule endpoint as an oracle in addition to comparing raw resources.

Verification has two layers:

- resource verification: did Regelverket create/update what it intended?
- semantic verification: does GitHub report the expected effective rules for representative refs?

This protects against errors in local layering calculations and hidden organization policy.

## Cost-aware recommendation

Recommendation must run after capability resolution.

Example:

```text
Template: Sequential Queue
Fit: high
Blocked: merge queue entitlement unavailable
Upgrade path: organization/plan capability required
Free alternative: Sequential Slots Lite
```

A free-only user should never receive a plan whose successful execution requires a paid upgrade.

## Module interface concept

A module can declare:

```text
id
applicability predicate
detectors
capabilities produced
questions
constraints
template contributions
verification probes
```

Examples:

- github-actions
- merge-queue
- code-security
- code-quality
- dependabot
- node
- python
- monorepo
- cloudflare-deployment
- organization-governance

Modules should be loaded from observed state/template requirements, not all enabled globally.

## Safety invariants

- Detection is read-only.
- Unknown is never silently treated as absent.
- Existing unmanaged files are never overwritten because of a filename collision.
- Shared resources are never modified without adoption/approval.
- Destructive plans require complete dependency analysis or must stop.
- Plan/apply is idempotent.
- Apply is followed by resource and semantic verification.
- Cost constraints are enforced before apply.
- Preview GitHub capabilities are opt-in for templates unless explicitly accepted.

## Next design work

- Define normalized repository graph schema.
- Define workflow/check provider schema.
- Define capability database schema and versioning/update process.
- Define template requirement language.
- Define reconciliation states and resource manifest schema.
- Define plan operation model and transaction/rollback semantics.
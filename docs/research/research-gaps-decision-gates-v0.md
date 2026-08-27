# Research Gaps & Decision Gates v0

Status: planning draft
Date: 2026-08-27

## Purpose

This document prevents implementation from silently treating unresolved questions as facts. Each gap has a decision gate describing what must be known before the related architecture can be frozen.

## P0 — must resolve before core implementation

### GitHub rule semantics coverage

Need:

- authoritative inventory of current ruleset rule types and parameters
- branch/tag/push/repository target differences
- condition/include/exclude semantics
- bypass actor types and access visibility
- interaction with classic branch protection
- organization/repository layering behavior

Gate:

Core normalized Rule model and renderer cannot be frozen until P0 rule fields are representable with provenance and uncertainty.

### Required checks / check identity

Need:

- exact check identity semantics across statuses/check runs
- expected App/source behavior
- duplicate check-name ambiguity
- matrix naming behavior
- skipped workflow/check behavior
- freshness/eligibility behavior relevant to required checks
- merge-group behavior

Gate:

No template may require generated/existing CI until provider analysis and deadlock constraints are tested.

### Merge queue capability matrix

Need:

- current plan/account/visibility availability
- repository prerequisites
- interaction with merge methods and required checks/workflows
- API support and permissions

Gate:

Merge queue cannot be a certified template capability until live E2E passes for declared profiles.

### API partial visibility and permissions

Need:

- which fields disappear versus return errors under lower privilege
- permission requirements for detect vs apply endpoints
- organization/enterprise policy visibility boundaries

Gate:

Destructive/adaptive planning cannot be enabled where absence might mean inaccessible.

### GitHub matching semantics

Need:

- verified `fnmatch` behavior used by rulesets and branch/path conditions
- edge cases involving slash, star, double-star and near matches

Gate:

Pattern normalizer and boundary verifier cannot be frozen until tests match GitHub behavior.

## P1 — resolve before safe apply/public 0.4

### Transaction compensation behavior

Need experiments for:

- ruleset create/update/delete restoration
- branch create/delete/force-update compensation
- repository settings restoration
- failures between workflow deployment and enforcement
- interrupted manifest update

Gate:

Each mutation adapter must declare reversible/compensatable/high-risk behavior before production use.

### Authentication choice

Need spike and threat review for:

- existing `gh` credential/token use
- GitHub App installation flow
- fine-grained PAT fallback
- credential discovery/storage UX

Gate:

Select supported auth paths and permission documentation before public apply release.

### YAML adaptation fidelity

Need representative corpus of real workflows testing:

- comments
- anchors/aliases where relevant
- expressions
- quoting
- unusual keys
- multiline shell
- reusable workflows

Gate:

Automatic ADAPT of user-owned workflows remains disabled unless transformation quality meets agreed safety bar. REUSE/GENERATE can ship earlier.

### Effective policy verification

Need:

- API behavior for effective rules on representative refs
- parent organization rules attribution
- inaccessible parent-policy behavior
- pattern boundary sampling strategy

Gate:

Apply cannot claim VERIFIED effective policy without authoritative or clearly qualified evidence.

## P2 — resolve before recommendation/template breadth

### Real-world workflow archetypes

Research:

- major open-source repositories
- solo maintainers
- small teams
- organization/enterprise examples
- monorepos
- release-heavy projects
- deployment-heavy projects
- bot/AI-heavy development

Capture:

- branch model
- review model
- merge strategy
- automation
- rules/protections
- pain points/trade-offs

Gate:

New public archetype names and recommendations require evidence beyond one internal implementation.

### Project classification quality

Need evaluation corpus to determine which signals actually help template selection.

Gate:

Do not claim recommendation accuracy from language/framework detection alone.

### Cost/plan recommendations

Need current, source-backed capability/cost matrix and update strategy for changing GitHub pricing/features.

Gate:

`free-only` recommendations require current verified capability data or explicit uncertainty.

### Template trust/distribution

Need final choices for:

- catalog location
- release/digest scheme
- provenance/attestation
- offline/cache behavior
- revocation/deprecation

Gate:

Third-party template consumption is not enabled until trust boundaries are explicit.

## P3 — future expansion

### Organization-wide orchestration

Research organization rulesets, repository custom properties, inherited governance and cross-repo rollout patterns in depth.

### External module/plugin ecosystem

Requires sandboxing, compatibility API, signing/trust and privilege separation. Not a v0.x assumption.

### Enterprise variants

Enterprise policy features, required workflows and governance need dedicated test profiles before support claims.

## Decision gates

### DG-01 — language/runtime

Input:

- Go spike
- Rust spike
- dependency/supply-chain review
- distribution tests
- YAML transformation tests

Output: ADR selecting implementation language/runtime.

Required before: milestone 0.1 implementation.

### DG-02 — internal schema v0

Input:

- P0 rule/check research
- Avkroken fixture representation
- graph prototype

Output: versioned internal node/edge/value contracts.

Required before: renderer and planner implementation stabilize.

### DG-03 — config/template schema v0

Input:

- internal schema
- first template requirements
- user intent model

Output: public versioned config/template schemas.

Required before: public 0.1.

### DG-04 — workflow adaptation safety

Input:

- YAML corpus
- transformation spike
- dependency analyzer tests

Output: one of:

- automatic ADAPT enabled for supported transformations
- ADAPT proposal-only
- generate/reuse only

Required before: public smart-adapt claims.

### DG-05 — authentication/public apply

Input:

- permissions map
- auth threat review
- local/App UX spike

Output: supported authentication modes and least-privilege matrix.

Required before: public 0.4 apply.

### DG-06 — first certified template

Input:

- deterministic rendering
- discovery
- planner
- apply/verify
- live E2E matrix

Output: first certified template support profile, expected to derive from Sequential Slots research fixture but not pre-decided as universally recommended.

### DG-07 — recommendation engine release

Input:

- multiple certified templates
- real-world archetype research
- classification evaluation
- cost/capability data

Output: recommendation scoring/ranking policy with explainable evidence.

Required before: recommendation promoted beyond experimental.

## Research artifact standard

Every material research conclusion should record:

- claim
- source URL/API endpoint/test
- observation date
- GitHub version/context if relevant
- status: documented / observed / inferred
- confidence
- affected architecture/constraint/template
- retest trigger

## Retest triggers

Research should be revalidated when:

- GitHub changelog announces relevant ruleset/Actions/merge changes
- REST API version changes
- template E2E fails unexpectedly
- user reports contradict documented behavior
- capability/pricing pages change
- normalization or constraint rules are modified

## Immediate next research sequence

1. Finish the P0 rule/check/API matrix with primary sources.
2. Execute highest-risk live experiments in disposable GitHub repositories.
3. Build representative workflow YAML corpus.
4. Run Go/Rust technology spikes.
5. Freeze DG-01 and begin milestone 0.1 only after the above evidence is sufficient.

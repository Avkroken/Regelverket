# Project Classification & Recommendation Model v0

Status: architecture draft
Date: 2026-08-27

## Purpose

Regelverket should recommend workflow archetypes/templates based on repository characteristics, available GitHub capabilities and user intent. Recommendations are advisory and must expose their reasoning.

## Separation of concerns

Three evidence groups must remain distinct:

1. observed repository facts
2. user-declared intent/constraints
3. heuristic recommendation rules

A heuristic must never be reported as a GitHub platform fact.

## Observed project signals

Potential signals include:

- language/runtime mix
- package/build systems
- mono/polyrepo indicators
- application/library/docs/infrastructure signals
- presence and shape of CI
- number/type of deployment workflows
- release/tag automation
- documentation pipelines
- dependency bots
- code owners
- branch/ref topology
- recent contributor/activity signals where safely available
- AI/bot automation indicators where identifiable

Each signal carries confidence and evidence.

## User/environment profile

Important non-observable inputs include:

- solo / team / organization workflow
- approximate team size
- human-only vs AI/bot-heavy development
- expected concurrent workstreams
- preference for trunk/branches/slots
- required review strictness
- release strategy
- deployment criticality
- desired automation level
- tolerance for complexity
- willingness to modify existing workflows
- cost policy: free-only / paid allowed / explicit budget
- willingness to move repository into an organization or upgrade plan
- security/compliance expectations

## Capability envelope

Recommendation filtering happens only after capability resolution.

A template can be:

- compatible now
- compatible with adaptation
- compatible with degraded variant
- requires permission change
- requires organization/account migration
- requires plan upgrade/payment
- incompatible
- unknown due to inaccessible facts

## Recommendation pipeline

```text
Observed project signals
 + User profile
 + Capability envelope
 + Template metadata
 + Research-backed heuristics
 -> candidate filter
 -> suitability scoring
 -> risk/complexity scoring
 -> explainable ranking
```

The exact scoring mechanism is intentionally not fixed in v0.

## Hard filters before scoring

Do not rank a template normally if a required capability cannot be satisfied.

Examples:

- required merge queue unavailable and no validated degradation exists
- template requires organization-level rule feature but repository cannot access it
- user has `free-only` and required feature is paid
- adaptation would require a destructive operation the user prohibited

Such templates may still be shown under `Unavailable / requires change` with explanation.

## Suitability dimensions

Candidate dimensions for future research:

- concurrency fit
- team collaboration fit
- AI/bot isolation fit
- release fit
- deployment fit
- review/governance fit
- operational complexity
- maintenance burden
- cost
- failure containment
- onboarding complexity

Weights should be explainable and user-adjustable later if useful.

## Example recommendation output

```text
Repository profile
  Type: TypeScript application
  Team: solo
  Automation: Dependabot + AI coding agent
  Parallel work: medium
  Repository: public organization repo
  Budget: free-only

Recommended
1. Sequential Slots Lite
   Why:
   - isolates concurrent AI work
   - current CI can be reused
   - no paid capability required
   Trade-off:
   - more branch management than protected trunk

2. Protected Trunk
   Why:
   - simplest maintenance
   Trade-off:
   - weaker isolation for simultaneous agents

Unavailable
3. Sequential Slots + Merge Queue
   Requires:
   - capability not available under detected account/repo envelope
```

Claims like "isolates concurrent AI work" must be backed by the template's documented properties. Claims like "best for TypeScript" require research evidence or must be phrased as a heuristic.

## Project type classification

Classification should be multi-label rather than one fixed type.

Example:

```yaml
project_signals:
  web_application: 0.95
  node_typescript: 1.0
  documentation_heavy: 0.55
  library: 0.15
  infrastructure: 0.10
```

Confidence numbers are conceptual; implementation may use categorical confidence rather than pseudo-precision.

## AI/bot-heavy profile

Regelverket should explicitly research and model repositories where automated agents contribute code.

Potential characteristics:

- multiple simultaneous bot branches/PRs
- need for narrow write scopes
- need to continuously rebase/reconcile against default
- bot-specific bypass or permissions
- stronger safeguards against workflow/policy modification
- high CI frequency/cost implications

The existing Avkroken model is useful evidence but must not be generalized as universally optimal without comparative research.

## Team/organization profile

Signals that may favor different governance:

- multiple reviewers/teams
- CODEOWNERS
- protected deployment environments
- organization-wide rulesets
- merge queues under high PR throughput
- release trains
- compliance/security requirements

Templates should declare which of these they exploit versus require.

## Repository adaptation fit

Recommendation must account for migration cost, not just end-state quality.

A theoretically strong template can rank lower when:

- it conflicts with mature existing CI
- it requires disruptive branch replacement
- it cannot reuse existing check identities
- open PRs make migration risky

The planner can estimate adaptation complexity using operation counts and dependency impact.

## Recommendation provenance

Every recommendation reason should be traceable to one of:

- GitHub documented capability
- live experiment result
- observed repository fact
- user-declared preference
- template invariant/property
- external workflow research
- heuristic hypothesis

The UI/CLI need not show full provenance by default, but `explain` must be able to.

## Research needed before stable scoring

Before ranking becomes a stable public feature, gather evidence from:

- GitHub's official ruleset recipes
- major open-source repositories across project types
- solo-maintainer projects
- small/medium teams
- monorepos
- release-heavy libraries/packages
- deployment-heavy applications
- repositories with high automated dependency activity
- AI-assisted/agent-heavy workflows where public evidence exists

Research should identify patterns and trade-offs, not blindly copy popular configurations.

## Cold-start behavior

If evidence is insufficient, Regelverket should return a small set of plausible templates plus discriminating questions rather than invent a confident winner.

Example:

```text
Two templates remain equally plausible.
Question: Do you normally have more than one code change in progress at the same time?
```

## Recommendation safety

A recommendation never mutates anything. The user selects a template/variant, then normal planning and constraint checks apply.

## v0 success criteria

The first recommendation engine is successful if it can:

1. eliminate impossible templates correctly
2. separate free vs paid/upgrade-dependent choices
3. explain why each surviving template fits
4. expose uncertainty and assumptions
5. ask fewer, more relevant questions after detection
6. incorporate adaptation/migration cost into the choice

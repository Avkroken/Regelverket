# Verification Model v0

Status: architecture draft
Date: 2026-08-27

## Goal

Verification answers a stronger question than “did GitHub accept our API request?” It must establish that the intended resources exist, their dependencies are coherent, and GitHub's effective enforcement matches the policy for representative refs.

## Verification layers

### V0 — Local validity

Before writes:

- config/template schema validity
- generated YAML/JSON parseability
- workflow structural analysis
- reference resolution
- constraint evaluation
- deterministic rendering

### V1 — Resource state

After writes, re-read managed resources from GitHub and compare normalized semantics to desired graph.

Covers:

- repository settings
- refs
- files/workflows
- rulesets
- manifest bindings

Raw payload equality is not required when GitHub normalizes/defaults/reorders fields.

### V2 — Dependency coherence

Verify semantic bindings:

- every required check has a valid provider model
- reusable workflow/local action references resolve
- required workflow/check events are compatible with merge model
- ruleset include/exclude bindings resolve as intended
- managed resource dependencies exist
- no unexpected duplicate provider ambiguity exists for critical checks

### V3 — Effective policy

For representative refs, query GitHub's effective rules where supported and compare against expected enforcement after repository/organization layering.

Representative set should include at least:

- default branch
- one ref from each managed RefClass
- boundary examples for include/exclude patterns
- refs expected to be excluded

Unknown/inaccessible parent policy must be represented explicitly.

### V4 — Behavioral verification

Where policy risk justifies it, execute or observe real behavior in controlled E2E scenarios. Examples:

- prohibited ref creation is rejected
- allowed slot update succeeds for intended actor
- direct default-branch mutation is rejected
- required checks block merge when absent
- merge queue produces required checks via `merge_group`
- slot synchronization converges correctly

Behavioral verification may be mandatory for template certification even when not run on every user repository apply.

## Evidence

Verification produces structured evidence, not only console text.

Conceptual record:

```yaml
verification:
  resource: ruleset.default
  requirement: default.no_force_push
  result: pass
  method: effective_rules_api
  observed_at: ...
  evidence_digest: ...
```

Evidence can expire. Time-sensitive capability observations and check-provider evidence need freshness policies.

## Result states

- PASS
- PASS_WITH_WARNING
- FAIL
- UNKNOWN
- INACCESSIBLE
- NOT_APPLICABLE

For security/enforcement invariants, UNKNOWN and INACCESSIBLE are not equivalent to PASS.

## Semantic digest

Each normalized resource can produce a semantic digest after removing known non-semantic API noise while retaining enforcement-relevant distinctions.

Digest uses:

- drift detection
- stale-plan preconditions
- manifest state
- reproducibility tests

Normalization rules themselves are versioned because changing normalization can change digests.

## Effective-policy expectations

Templates define expected policy requirements, not expected raw ruleset payloads. Verification maps effective GitHub rules back to those requirements.

This permits implementation changes without invalidating the higher-level policy contract.

## Include/exclude boundary verification

Pattern-based policy requires generated test cases around boundaries. For a pattern/class, verification should sample:

- known included ref
- known excluded ref
- near-match that must not match
- nested path/name where GitHub `fnmatch` semantics are relevant

The exact generator depends on verified GitHub matching semantics.

## Required-check verification

A configured required check is not considered fully verified merely because its name appears in a ruleset.

Verification should distinguish:

1. rule references check name/context
2. provider can be identified
3. provider trigger can run in required lifecycle
4. provider has emitted compatible evidence recently, when available
5. merge-queue lifecycle is supported when required

## Template certification

A public template version receives a support matrix based on E2E evidence. Example states:

- certified
- experimental
- documented-only
- unsupported

Certification is capability-profile-specific. A template can be certified for public organization repos but experimental for another profile.

## Continuous compatibility research

GitHub behavior can change independently of Regelverket releases. The research suite should periodically rerun selected live experiments and compare results to the knowledge base.

Behavioral changes become explicit research updates before assumptions are silently changed in templates.

## Explain integration

`regelverk explain` should consume verification evidence. It can answer:

- why is this branch protected?
- which rules currently apply?
- where does this required check come from?
- is this fact documented, observed, inferred or inaccessible?
- why did verification fail?

Verification therefore doubles as operational diagnostics.

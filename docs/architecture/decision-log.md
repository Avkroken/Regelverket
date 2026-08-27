# Architecture decision log

This log records decisions and working assumptions separately from research facts.

## Accepted direction

### D-001 — New implementation may be written from scratch

The new public Regelverket is not required to patch or preserve the Avkroken v24.2 implementation. v24.2 is a reference, regression fixture and source of operational lessons.

### D-002 — Templates are mandatory core functionality

Users must be able to choose ready-built, documented policy/workflow templates.

At minimum support two experiences:

- apply a template substantially as-is to one or more repositories,
- adapt a template to an existing repository while preserving/reusing compatible resources.

### D-003 — Smart detection precedes smart adaptation

Adaptation decisions must be based on detected repository structure, workflows, rules, capabilities and dependencies rather than filename matching alone.

### D-004 — Reuse before generation

Compatible existing workflows/resources should be reused where possible. Regelverket should avoid unnecessary duplicate automation.

### D-005 — Existing resources are not overwritten on name collision

If a generated workflow/resource name collides with an unmanaged existing resource, Regelverket must preserve the existing resource and resolve the conflict through reuse/adaptation or a deterministic alternate name.

A short stable hash may be used as a conflict suffix, but content hashes should not be the default filename identity.

### D-006 — Idempotence is a core invariant

Repeated application of unchanged desired state must not create duplicates or repeated rewrites. A second successful run should normally produce no changes.

### D-007 — Resource ownership is explicit

The architecture will distinguish unmanaged, shared, adopted and managed resources.

### D-008 — GitHub effective policy matters

Verification must account for overlapping/organization-level policy and should not assume repository-local rulesets alone describe the actual rules experienced by a branch.

### D-009 — Cost and account capabilities affect recommendations

The user's willingness to pay, account/repository type and feature availability are constraints. A `free-only` user must not be recommended a solution requiring paid capabilities without clearly presenting it as unavailable/upgrade-required.

### D-010 — Detection asks less, not more

The system should infer observable facts from the repository and only ask users for intent or information that cannot be reliably observed.

### D-011 — Modules should load based on relevance

Enterprise/organization/deployment/specialized modules should be activated when detection/profile/capabilities make them relevant rather than forcing every user through every question.

### D-012 — Workflows need semantic dependency analysis

Regelverket must understand meaningful references among workflows, reusable workflows, local actions, scripts/configs, jobs/checks and policy dependencies so adaptation can update or preserve linked resources safely.

### D-013 — Runtime-generated GitHub Actions workflows cannot rely on workflow subdirectories

Executable workflow entrypoints must be compatible with GitHub's workflow path rules. Regelverket-specific metadata/configuration may use its own `.github/regelverk/` directory.

### D-014 — Research precedes implementation hardening

GitHub documentation, API semantics, rule behavior and real-world patterns are mapped before CLI/file-format implementation decisions become difficult to reverse.

### D-015 — Rust is the production core implementation language

The Go/Rust decision gate is closed in favor of Rust.

Both candidates passed deterministic compiler/planner parity, the read-only GitHub adapter experiment and Linux/macOS/Windows packaging. Rust is selected because the most safety-critical discriminating prototype — minimal adaptation of a comment/layout-heavy user-owned workflow — preserved the edited YAML line exactly except for the intended scalar change, while the Go prototype normalized spacing on that line. Rust also fits the project's explicit uncertainty/state modelling more directly.

The higher contributor learning curve and larger dependency graph are accepted costs and must be mitigated through narrow modules, conservative dependencies, fixture-heavy tests and explicit architecture boundaries.

See `implementation-language-decision-v0.md` for evidence and revisit conditions.

## Working hypotheses — not yet architectural facts

These require research or prototyping before being promoted to decisions:

- final config/DSL shape,
- exact manifest format and storage path,
- final CLI command names,
- initial public archetype catalog,
- how much of existing workflow YAML can be safely auto-adapted versus requiring explicit approval,
- how repository/team/AI-bot characteristics should influence template recommendation,
- exact fallback naming algorithm for collisions,
- whether any persistent state beyond repository-contained manifest data is needed.

## Known reference archetype

The Avkroken v24.2 model is a useful starting archetype candidate, provisionally described as **Sequential Slots**:

- closed pool of named work branches,
- branch lock preventing arbitrary refs outside the pool,
- scope validation tied to slot purpose,
- synchronization of unused slots to current default-branch tip,
- PRs/checks evaluated against current code,
- explicit automation/bypass behavior,
- documentation as a standard branch/slot role.

The name, abstraction and public template behavior remain subject to research and redesign.

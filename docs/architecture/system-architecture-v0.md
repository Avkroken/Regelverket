# System Architecture v0

Status: consolidated architecture draft
Date: 2026-08-27

## Mission

Regelverket is a declarative planner/compiler for GitHub repository governance. It detects current state, understands capabilities and constraints, recommends or resolves a tested governance template, plans safe changes, applies them idempotently, and verifies effective enforcement.

It is not primarily a shell installer and it does not model governance as copied JSON/YAML files.

## Architectural principles

1. GitHub documentation and observed behavior are the factual base.
2. Repository files and API payloads are adapters around an internal semantic model.
3. Templates express intent and tested workflow models, not file-copy recipes.
4. Detection must preserve uncertainty and partial visibility.
5. Existing compatible resources are reused before new resources are generated.
6. Resource identity is stable and independent of path, content hash and external numeric IDs.
7. Same desired state applied repeatedly converges to NOOP.
8. Dependencies are installed before enforcement; replacements before removals.
9. Verification checks effective behavior, not only API write success.
10. Destructive or privilege-sensitive uncertainty fails closed.

## High-level architecture

```text
                     User / Config / Template Choice
                                |
                                v
                    +------------------------+
                    | Intent + Profile Layer |
                    +-----------+------------+
                                |
                                v
GitHub API/Git ---> +------------------------+
Repository files ->| Discovery Engine       |
Account context --->| + Capability Probes   |
                    +-----------+------------+
                                |
                                v
                    +------------------------+
                    | Normalized Repository  |
                    | / Policy / Workflow    |
                    | Dependency Graph       |
                    +-----------+------------+
                                |
             Knowledge Base ----+---- Template Catalog
             Constraints -------+---- Cost/Plan Model
                                |
                                v
                    +------------------------+
                    | Resolver / Adaptation  |
                    | + Constraint Engine    |
                    +-----------+------------+
                                |
                                v
                    +------------------------+
                    | Desired Policy Graph   |
                    +-----------+------------+
                                |
                                v
                    +------------------------+
                    | Reconciliation Planner |
                    +-----------+------------+
                                |
                         plan / approval
                                |
                                v
                    +------------------------+
                    | Transaction Executor   |
                    | GitHub/File Adapters   |
                    +-----------+------------+
                                |
                                v
                    +------------------------+
                    | Verification Engine    |
                    +-----------+------------+
                                |
                                v
                    Manifest / Evidence / Explain
```

## Major subsystems

### 1. Discovery Engine

Collects repository/account facts without executing untrusted repository code.

Outputs:

- observed repository state
- policy graph
- workflow dependency graph
- actor/check-provider observations
- capability probes
- classification signals
- unknown/inaccessible facts
- unresolved questions

Detection modules are loaded based on relevance so simple repositories do not require enterprise-only analysis.

### 2. Capability Engine

Resolves whether a feature is actually usable given:

- owner type
- repository visibility
- GitHub plan
- permissions
- feature state
- API visibility
- user cost constraints

States include available, metered, addon, requires-upgrade, permission-missing, inaccessible, unsupported and unknown.

### 3. Knowledge Base

Versioned factual/behavioral model of GitHub rules, API semantics, workflow behavior, capability availability, known dangerous interactions and research provenance.

Knowledge entries distinguish documented, observed, inferred and unknown.

### 4. Repository / Policy Graph

Shared semantic representation for all later stages.

Core node families include repository, ref/refclass, ruleset/rule, workflow/job/check provider, actor, capability, file resource, environment/deployment target, policy requirement and evidence.

Observed, desired and effective state are graph overlays rather than unrelated representations.

### 5. Template Catalog

Three levels:

- Archetype: development/governance model
- Template: tested implementation for a capability envelope
- Resolved Policy: concrete desired graph for one repository

Templates declare intent, requirements, variants, degradation paths, supported profiles, risks and test/certification evidence.

### 6. Project Classification & Recommendation

Combines detected facts and user-supplied intent to rank templates.

Recommendations must expose:

- evidence/signals
- assumptions
- capability/cost disqualifiers
- migration cost/risk
- confidence

Heuristics never masquerade as GitHub platform facts.

### 7. Adaptation Resolver

For each template requirement, resolution preference is:

1. REUSE
2. COMPOSE
3. ADAPT
4. GENERATE
5. CONFLICT

The resolver binds existing semantic capabilities before creating files.

### 8. Constraint Engine

Evaluates hard and soft relationships before planning.

Constraint classes:

- platform constraints
- template invariants
- security constraints
- compatibility constraints
- capability/cost constraints
- recommendations

Outcomes include PASS, WARN, FAIL and UNKNOWN_BLOCKING.

### 9. Ownership & Reconciliation

Resources are classified unmanaged, shared, adopted or managed.

Stable logical IDs map to paths/API IDs through a manifest under `.github/regelverket/`.

Reconciliation compares normalized observed and desired semantics and produces NOOP/REUSE/CREATE/UPDATE/ADOPT/MOVE/DETACH/DELETE/CONFLICT/BLOCKED operations.

### 10. Planner

Produces a semantic plan with reasons, dependencies, risk, rollback class, stale-state preconditions and affected resources.

A plan is read-only and safe to generate repeatedly.

### 11. Transaction Executor

Implements compensating transactions because GitHub has no cross-resource atomic transaction.

Phases:

observe -> preflight -> snapshot -> stage dependencies -> validate -> enforcement -> verify -> cleanup -> manifest commit

Destructive operations have separate approval boundaries.

### 12. Verification Engine

Verification layers:

- local/schema validity
- resource state
- dependency coherence
- effective GitHub enforcement
- behavioral E2E where required

HTTP success is not sufficient.

### 13. Authentication / Permission Layer

Supports simple local authentication and a least-privilege GitHub App path for repeatable/public use.

Detection should require less privilege than mutation. Permission needs are derived per operation.

### 14. Template Distribution & Trust

Templates and releases are supply-chain artifacts.

Official distribution requires version/digest identity, provenance, immutable release references where possible, schema validation and security policy for executable dependencies.

### 15. Explain / Diagnostics

Consumes graph, provenance, constraints and verification evidence to answer why a policy applies, where checks come from, why a plan is blocked and whether a conclusion is documented/observed/inferred/unknown.

## CLI lifecycle

Provisional public concepts:

```text
regelverk detect
regelverk recommend
regelverk init/configure
regelverk plan
regelverk apply
regelverk verify
regelverk explain
```

CLI syntax is not yet frozen. The lifecycle is.

## Config lifecycle

Config describes desired intent and constraints, not raw GitHub REST payloads.

Important categories:

- target repositories
- selected archetype/template
- ref/slot preferences
- review/merge intent
- cost constraints
- adaptation strategy
- security posture
- advanced escape hatches when needed

Config schema is versioned and migration-aware.

## Repository-local state

Provisionally:

```text
.github/regelverket/
  manifest.yaml
  config.yaml          # optional, depending on config strategy
  evidence/            # optional/local policy TBD
```

Runnable Actions workflows remain directly under `.github/workflows/` because GitHub does not discover nested workflow directories.

## Generated resource naming

Readable deterministic names are preferred. Stable collision suffixes are only used when an unmanaged path conflicts.

Content hashes are not filenames because content changes must update the same logical resource.

## Security posture

- detect does not execute repository code
- least privilege
- explicit ownership
- no silent overwrite on drift
- no destructive planning from material inaccessible facts
- no enforcement before required dependencies exist
- external Actions pinned according to project security policy
- stale plans rejected

## Testing architecture

1. unit tests for domain model/constraints
2. golden tests for rendering/normalization
3. simulation tests for observed+desired -> plan
4. live GitHub E2E tests
5. template certification matrix
6. regression fixture derived from Avkroken v24.2 intended semantics
7. periodic behavioral compatibility experiments against GitHub

## Extension model

Architecture is modular but a third-party executable plugin ecosystem is not assumed for v0.x. Modules should initially be compiled/trusted project components with explicit detectors, capabilities, questions, constraints and renderers.

A future external module system requires separate sandbox/trust architecture.

## Non-goals for early releases

- enumerating every possible rule combination
- fully automatic destructive migration without review
- arbitrary executable template hooks
- enterprise-wide orchestration before single-repository correctness
- recommendations based on undocumented intuition without evidence

## Success definition

Regelverket is successful when a user can point it at an existing or new repository, receive an explainable analysis and compatible template recommendation, inspect a safe deterministic plan, apply it without overwriting unrelated work, rerun with NOOP convergence, and verify that GitHub enforces the intended policy.

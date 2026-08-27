# Threat Model v0

Status: security architecture draft
Date: 2026-08-27

## Scope

Regelverket is a governance tool with potentially administrative authority over repositories. Its failure modes include repository lockout, policy bypass, arbitrary workflow execution, source loss, privilege escalation and supply-chain compromise.

This document identifies threat classes and required architectural controls. It is not yet a completed security review.

## Assets

- source code and Git history
- repository availability and maintainer access
- branch/ruleset integrity
- Actions workflows and required-check integrity
- GitHub App/token credentials
- organization/repository permissions
- template/catalog integrity
- Regelverket config, manifest and operation journal
- verification evidence
- user intent and approval boundaries

## Trust boundaries

1. local Regelverket process
2. GitHub API
3. Git transport
4. repository-controlled files
5. pull-request-controlled content
6. GitHub Actions runtime
7. third-party Actions
8. GitHub Apps/bots
9. template/catalog distribution
10. organization-level policy outside repository control

## Threat classes

### Credential compromise

A PAT, installation token or App private key can grant broad mutation capability.

Controls:

- least privilege per operation/capability
- short-lived credentials where available
- never persist secrets in config/manifest/logs
- redact diagnostic output
- separate read/detect permission requirements from apply permissions
- document exact required permissions

### Malicious repository content

A repository being analyzed may contain malicious YAML, scripts, symlinks, unusual paths or expressions.

Controls:

- detection parses; it does not execute repository code
- no shell sourcing/eval of repository content
- safe YAML parser/schema
- explicit filesystem boundary checks
- treat workflow expressions as data

### Workflow privilege escalation

Generated or adapted workflows can execute with repository credentials.

Controls:

- minimal workflow `permissions`
- explicit permissions per generated workflow
- no privileged workflow should execute untrusted PR-controlled code before trust boundary checks
- separate privileged maintenance automation from ordinary CI
- security review for `pull_request_target` and equivalent dangerous patterns

### Supply-chain compromise

Third-party Actions or template dependencies may change or become malicious.

Controls:

- pin external Actions according to security policy, preferably immutable commit SHA
- record provenance
- inventory external dependencies during detect
- template certification includes dependency review
- update mechanism must surface dependency changes

### Ruleset bypass / wrong actor identity

Misidentified Apps, users or teams can accidentally receive bypass rights or required-check trust.

Controls:

- resolve actor identity from authoritative API data
- distinguish GitHub-owned actors from installed Apps
- no label/name-only trust decisions
- verify bypass behavior where material
- bind required checks to expected source/App when appropriate and supported

### Repository lockout

A policy can require a check that cannot run, deny creation of needed refs, or remove all viable maintenance paths.

Controls:

- dependency-before-enforcement transaction ordering
- constraint engine
- required-check provider analysis
- effective-policy verification
- preflight lockout proof
- staged enforcement where necessary

### TOCTOU / stale plans

Repository changes after planning can make a previously safe plan destructive.

Controls:

- material preconditions
- optimistic concurrency
- re-observe before destructive operations
- reject stale plans
- force-with-lease for relevant Git operations

### Drift overwrite

A user edits a managed file and later apply silently destroys the change.

Controls:

- three-way semantic drift detection
- explicit restore/adopt decision
- never overwrite unmanaged collisions

### Template/config injection

A template or config attempts arbitrary code execution or unsafe renderer behavior.

Controls:

- declarative schema
- no arbitrary executable hooks in core template format
- strict renderer escaping/validation
- separate executable artifacts from declarative policy
- catalog trust/provenance model

### Destructive migration

Replace mode deletes branches/rulesets/workflows needed by active work.

Controls:

- dependency graph
- open PR analysis
- snapshot
- destructive approval
- replacements before removals
- compensation classification

### Partial API failure

Some writes succeed and later operations fail.

Controls:

- operation journal/checkpoints
- idempotent reconciliation
- post-write re-read
- compensating transaction model
- explicit FAILED_PARTIAL state

### Partial visibility

Insufficient permissions hide organization policy or bypass actors and detector interprets absence as none.

Controls:

- known/unknown/inaccessible state model
- destructive operations block on material unknowns
- capability probes distinguish unsupported from unauthorized

### Denial of service / API exhaustion

Large organizations or repositories can trigger expensive scans and rate limits.

Controls:

- bounded discovery
- caching with freshness metadata
- incremental scanning
- API budget/rate-limit awareness
- module loading based on relevance

### Untrusted fork PRs and bots

Forks/bots have different secret/token behavior and can affect check availability.

Controls:

- model actor/event context
- template E2E includes fork/bot scenarios where supported
- do not assume a workflow that works on same-repo PR works identically for forks

## Approval boundaries

User approval must be scoped. One confirmation must not silently authorize unrelated destructive actions discovered later.

Plan operations should be grouped into:

- safe additive
- policy enforcement
- ownership transfer
- destructive
- privilege/security-sensitive

The UI/CLI must surface these classes before apply.

## Logging and privacy

Logs should contain enough provenance for diagnosis without leaking secrets. Public bug reports should be producible from a redacted diagnostic bundle.

Potentially sensitive repository topology should not be uploaded anywhere by default.

## Security testing

Required test families:

- malicious YAML/paths/parser inputs
- stale plan race
- managed drift
- required-check deadlock
- bypass actor mismatch
- third-party Action pin validation
- partial API failure injection
- interrupted apply/resume
- permission-restricted detection
- fork/bot workflow behavior
- template tampering/provenance tests once distribution design exists

## Security posture

Default posture is fail closed for enforcement/destructive uncertainty, but not for harmless discovery. Regelverket should still produce useful partial analysis when access is incomplete while clearly identifying which conclusions cannot be trusted.

## Open research

- optimal authentication model for public use: `gh` credentials, GitHub App, PAT, or combinations
- signing/provenance for template catalog and releases
- GitHub artifact attestations applicability
- safe update channel
- secret scanning of generated artifacts
- enterprise/organization permission edge cases
- sandbox strategy for any future plugin/module ecosystem

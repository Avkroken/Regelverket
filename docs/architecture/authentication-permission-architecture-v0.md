# Authentication & Permission Architecture v0

Status: architecture draft
Date: 2026-08-27

## Goal

Regelverket must support a low-friction local experience while preserving least privilege for repository and organization mutations. Authentication method, authorization capability and user intent are separate concerns.

## Principles

1. Detect should require less privilege than apply.
2. No credential is stored in Regelverket config, manifest or operation journal.
3. Missing permission is represented as `permission-missing` or `inaccessible`, never as resource absence.
4. Permissions are requested/explained per capability rather than as one broad "admin required" statement.
5. Long-lived public use should prefer scoped GitHub App installations where that model can satisfy required operations.
6. Existing GitHub CLI credentials can provide a practical local bootstrap path.

## Authentication providers

### GitHub CLI session

Regelverket may consume an authenticated `gh` environment/session for local interactive use. GitHub CLI supports browser login, environment-token/headless use and credential storage via the system credential store where available.

Benefits:

- minimal onboarding for developers already using `gh`
- user identity naturally matches interactive operator
- good bootstrap/detect path

Risks/limitations:

- token scopes/permissions may be broader or narrower than Regelverket expects
- fine-grained token resource scoping can make behavior non-obvious
- user credentials are unsuitable as the default identity for persistent privileged automation

Regelverket should inspect/probe effective permissions rather than infer them solely from token type.

### Fine-grained personal access token

Useful for headless/local scenarios where GitHub App installation is not desired.

Requirements:

- explicit repository scope
- minimum repository/organization permissions needed by selected operation
- passed through environment/secure input, never config
- capability probing before planning mutations

Classic PAT support, if provided, should be compatibility-oriented rather than preferred because broad OAuth scopes weaken least-privilege guarantees.

### GitHub App

Preferred strategic model for repeatable/public deployment where users want Regelverket to operate across selected repositories or an organization.

GitHub Apps start with no explicit permissions; permissions are selected during registration and installations can be scoped to repositories. Regelverket should define an App permission manifest derived from its capability catalog rather than request a monolithic maximum permission set.

Potential architecture:

```text
Regelverket CLI
   -> user/session identity for interactive approval
   -> GitHub App installation identity for scoped mutations
```

The exact OAuth/device/install UX remains a later product-design decision.

### GitHub Actions token

A workflow-local `GITHUB_TOKEN` can be useful for verification or self-maintenance capabilities, but must not be assumed to have administration powers needed for initial governance installation. Workflow permissions are explicitly minimized per generated workflow.

## Permission capability catalog

Each GitHub operation maps to a named Regelverket capability, for example:

```text
github.repo.read
github.contents.read
github.contents.write
github.rulesets.read
github.rulesets.write
github.actions.read
github.pull_requests.read
github.repository_settings.write
github.organization_rules.read
github.organization_rules.write
```

Exact mapping to GitHub permission names/endpoints is maintained in the versioned knowledge base and verified against current GitHub documentation/API behavior.

## Split detect/apply authorization

A recommended lifecycle:

```text
regelverk detect     -> read-only credentials sufficient where possible
regelverk recommend  -> no additional write authority
regelverk plan       -> read-only plus enough visibility to prove safety
regelverk apply      -> acquire/prove required write capabilities
regelverk verify     -> primarily read; behavioral tests may require scoped writes
```

A user should be able to explore a repository and obtain partial recommendations without granting administrative mutation rights.

## Permission probing

Capability Engine records:

```yaml
capability: github.rulesets.write
state: available | permission-missing | inaccessible | unsupported | unknown
identity:
  kind: github_app | user | actions
source: api_probe
```

Where a harmless API probe cannot distinguish plan limitation from authorization limitation, state stays ambiguous and the planner must not invent a conclusion.

## Elevation model

Regelverket should not silently reuse a broad credential for every task merely because it is available.

For a future interactive UX, when a selected template requires additional authority, the product should explain:

- capability needed
- exact operation needing it
- affected repositories
- whether permission is temporary/session-based or App installation based
- whether a lower-privilege alternative/template exists

## Actor identity

Ruleset bypass and required-check source bindings make actor identity security-critical.

Requirements:

- store authoritative GitHub node/database/App identities where appropriate as observed bindings
- display human names only as labels
- never infer privileged identity from a name such as "merge bot"
- re-resolve stale identities before privileged policy changes

## Credential handling

Must:

- avoid command-line token arguments where process listings/history can expose them
- use environment/stdin/credential stores/provider SDK mechanisms
- redact Authorization headers and tokens from logs
- zero/release secret buffers where implementation/runtime reasonably permits
- exclude credentials from diagnostics and crash reports

## Multi-account / enterprise hosts

The internal provider interface must not hardcode github.com. GitHub CLI and API adapters should preserve host identity so future GitHub Enterprise Server/data-residency scenarios can be represented explicitly.

Support level for GHES is not assumed until its ruleset/API capability matrix is researched and tested.

## Recommended initial product path

Provisional, subject to implementation research:

1. local CLI can start with existing `gh` authentication or explicitly supplied token
2. detect/plan works with the minimum available read permissions and reports visibility gaps
3. public multi-repo/organization workflow introduces a Regelverket GitHub App
4. generated repository automation uses its own least-privilege runtime identity, not the installer's user token

## Open decisions

- whether v0.1 supports only `gh`/token and introduces App installation later, or ships both
- GitHub App ownership: public app operated by project vs user-created app/bootstrap helper
- OAuth/user-token need in addition to installation tokens
- exact permission bundles for each capability
- GHES support boundary
- credential-helper abstraction and OS support

## Primary-source notes

GitHub recommends selecting the minimum permissions required for a GitHub App. GitHub CLI's interactive login uses a browser flow by default and stores credentials in the system credential store when available; environment tokens are supported for headless use. These facts should be periodically reverified as GitHub evolves.

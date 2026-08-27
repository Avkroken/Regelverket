# Required Check Provider Graph — empirical v0

Date: 2026-08-27
Status: empirical model from live Avkroken repositories

## Why this exists

Required status checks are policy identities, not workflow filenames. GitHub rulesets in the eight reference repositories bind checks to GitHub Actions via `integration_id: 15368`, but the actual emitting provider can be a job in any workflow that produces the matching check name.

Regelverket therefore needs a runtime-backed provider graph.

## Observed chain

```text
ruleset required_status_checks
  -> check context/name
  -> expected integration/App
  -> observed check-run
  -> Actions job
  -> workflow run
  -> workflow file
  -> event
  -> dependencies/reusable workflows
```

Each edge can have independent confidence/evidence.

## Shared provider facts

Across the reference rulesets, required checks are bound to `integration_id: 15368`, observed in live check-runs as GitHub Actions (`github-actions`). This means provider matching should distinguish:

- context/name
- App/integration identity
- observed job/workflow relationship

A filename alone is not sufficient identity.

## Per-repository required contexts

### docker-idempotent-update

- lint
- python
- docker
- osv
- scope-policy

### produkter

- python
- node
- docker
- dependency-review
- osv
- scope-policy

### pastebinit

- python
- osv
- scope-policy

### routines-relay

- repository-checks
- osv
- scope-policy

### politiker

- python
- typecheck
- docker
- osv
- scope-policy

### klarsprak

- validate
- osv
- scope-policy

### dumpen

- test
- osv
- scope-policy

### Bastion

- xcodegen-and-build
- swiftpm-macos
- ios-screenshots
- swiftpm-linux
- linuxapp-build
- linuxapp-msrv
- build-deb
- build-rpm
- build-deb-linuxapp
- build-rpm-linuxapp
- android-build
- windowsapp-core-tests
- windowsapp-build
- osv
- scope-policy

## Bastion runtime evidence

A live `merge_group` run on Bastion produced jobs including:

- impact
- swiftpm-macos
- ios-screenshots
- xcodegen-and-build

The workflow run event was `merge_group`, on a `gh-readonly-queue/main/pr-...` branch. This is direct production evidence that required providers must be event-compatible with merge queue and that provider discovery can correlate runtime job names to workflow files.

## Stable wrapper pattern

The OSV workflow demonstrates a useful generic pattern:

```text
required context: osv
  -> wrapper job: osv
     -> needs scan-pr + scan-merge
        -> reusable OSV workflows
```

The provider used by policy is stable even though the implementation differs by event.

Regelverket should recognize this as a high-quality provider topology rather than treating the internal scan jobs as separate required contexts.

## Provider states

A required context should resolve to one of:

- `resolved_single` — one compatible provider with supporting runtime evidence
- `resolved_static` — one plausible static provider, no runtime evidence yet
- `ambiguous` — multiple potential providers can emit same identity
- `missing` — no provider found
- `event_incomplete` — provider exists but cannot run in all required lifecycle events
- `app_mismatch` — matching context from wrong App/integration
- `unknown` — insufficient visibility/evidence

## Provider record

Conceptual normalized record:

```yaml
check:
  context: osv
  integration:
    id: 15368
    slug: github-actions
  state: resolved_single
  providers:
    - workflow_path: .github/workflows/osv-scanner.yml
      job_id: osv
      effective_name: osv
      events:
        pull_request: supported
        merge_group: supported
      runtime_evidence:
        observed: true
```

## Static analysis inputs

The workflow analyzer should extract:

- workflow path
- workflow `name`
- `on` events
- job key
- job `name`
- matrix dimensions
- job-level `if`
- reusable workflow `uses`
- `needs`
- branch/path filters

Static analysis produces candidates, not certainty.

## Runtime analysis inputs

Where permissions and recent runs permit:

- workflow runs by event
- jobs per run
- check-runs for representative SHAs
- check App/integration identity
- job/check names after matrix/reusable expansion

Runtime evidence should upgrade confidence rather than overwrite contradictory static evidence silently.

## Merge queue compatibility

If a required check applies to a branch using merge queue, provider compatibility requires evidence that the required context can be produced for `merge_group`.

State transition example:

```text
provider exists on pull_request only
  => event_incomplete
  => blocking constraint when merge_queue is required
```

## Duplicate context handling

If two workflow/job paths can emit the same required context under overlapping events, Regelverket must not pick one arbitrarily. Mark `ambiguous`, show both providers, and block destructive/adaptive changes until disambiguated or empirically proven safe.

## Reuse algorithm consequence

When adapting a template that requires capability `ci`:

1. discover required/observed check contexts
2. map them to providers
3. determine event compatibility
4. score whether existing providers satisfy template requirements
5. REUSE provider if sufficient
6. ADAPT/COMPOSE only with explicit semantic delta
7. GENERATE a new provider only when no safe existing provider exists

## Detect schema consequence

Observed state must store check-provider evidence separately from workflow source. A future schema should permit multiple evidence records per logical check and preserve timestamps because provider evidence can become stale.

## Verification consequence

Verification of a required check has at least four levels:

1. ruleset references the intended context/App
2. static provider candidate exists
3. provider can run for required events
4. runtime evidence confirms the expected check identity

A successful ruleset write only proves level 1.

## Conclusion

The eight-repository production data confirms the architecture decision that `CheckProvider` is a first-class graph node. Required checks must never be modeled as strings attached directly to workflow filenames.

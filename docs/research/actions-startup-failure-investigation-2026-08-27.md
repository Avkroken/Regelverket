# Actions startup failure investigation — 2026-08-27

Status: externally blocked after isolation

## Symptom

`Technology Spikes` runs associated with PR #1 concluded `startup_failure` with zero jobs created. GitHub documents `startup_failure` as a check-suite startup failure rather than a check-run conclusion.

## Evidence

- `Technology Spikes` pull-request run `33091854292` concluded `startup_failure` with zero jobs.
- A second `Technology Spikes` pull-request run `33092062891` reproduced the same result.
- A minimal `Startup Smoke` pull-request workflow with no external Actions and only `echo startup-ok` produced run `33093502358`, also `startup_failure` with zero jobs.
- The same minimal workflow was then given a `push` trigger on `work/tech-spikes`. Push run `33093567357` also concluded `startup_failure` immediately with zero jobs.
- Therefore the failure is not caused by Go, Rust, YAML dependencies, `actions/checkout`, a third-party Action, or the `pull_request` event path.
- Dependabot's dynamic dependency-graph run `33092174673` succeeded on `main`; this is GitHub-owned dynamic automation and does not prove that ordinary repository workflows can currently instantiate jobs.

## Current classification

Failure boundary: GitHub Actions startup/control-plane before job instantiation.

Remaining plausible classes:

1. repository/organization Actions configuration or policy outside workflow content;
2. repository/organization backend state left inconsistent by the recent GitHub Actions incidents;
3. an active GitHub Actions control-plane/service defect not yet reflected as a current public incident.

The available GitHub connector exposes workflow runs/jobs but not repository/organization Actions settings, so the remaining distinction cannot be made authoritatively from this tool surface.

## External incident context

GitHub Status reported multiple Actions incidents immediately preceding this investigation, including jobs failing to start and pull-request-triggered workflows failing to trigger. Community reports on 2026-08-26/27 also show `startup_failure`/zero-job symptoms after those incidents. This is supporting context, not proof that Regelverket has the same backend defect.

## Consequence for technology spikes

Go/Rust implementation comparison remains **not CI-verified**. The test harness is present, but no runner has executed it. Language selection must not use these failed runs as evidence against either implementation.

## Required next evidence

Authoritatively inspect repository and organization Actions settings, especially:

- whether Actions are enabled for `Avkroken/Regelverket`;
- allowed-actions policy;
- GitHub-hosted runner availability/policy;
- organization-level Actions restrictions inherited by the repo;
- any billing/usage/account restriction affecting ordinary workflows.

If those are normal and a one-step `ubuntu-latest` workflow still produces `startup_failure` with zero jobs, treat this as GitHub backend/service state and escalate with the four run IDs above.

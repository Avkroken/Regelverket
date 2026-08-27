# Actions startup failure investigation — 2026-08-27

Status: active investigation

## Symptom

`Technology Spikes` runs associated with PR #1 concluded `startup_failure` with zero jobs created. GitHub documents `startup_failure` as a check-suite startup failure rather than a check-run conclusion.

## Evidence so far

- Workflow file parses as ordinary GitHub Actions YAML by inspection.
- Repository Actions infrastructure is not globally absent: Dependabot's dynamic dependency graph run succeeded on `main` after PR #1 merged.
- Two `Technology Spikes` pull-request runs failed before job creation.
- No job logs exist because no jobs were instantiated.

## Isolation experiment

A minimal workflow named `Startup Smoke` is added on a temporary work branch. It has:

- `pull_request` trigger scoped to its own file
- `ubuntu-latest`
- no external Actions
- one shell `echo` step

Interpretation:

- if Startup Smoke also gets `startup_failure`, investigate repository/organization Actions startup policy/service state;
- if Startup Smoke starts, isolate `actions/checkout` / allowed-action policy / action runtime version next.

No production policy is changed by this experiment.

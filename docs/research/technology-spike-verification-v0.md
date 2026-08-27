# Technology spike verification v0

Status: implementation evidence boundary
Date: 2026-08-27

## Purpose

This note defines what the current Go/Rust technology spikes prove and what still remains before the implementation-language ADR may be written.

The comparison must remain evidence-driven. GitHub Actions `startup_failure` is an infrastructure blocker, not evidence for or against either candidate language.

## Reproducible local verification

Run from the repository root:

```bash
bash scripts/verify-technology-spikes.sh
```

The verifier uses the same observed-state and desired-config fixtures for both implementations and checks:

1. each implementation's unit tests;
2. a host-platform binary build for each implementation;
3. semantic parity for the baseline no-op case;
4. semantic parity for the required-check update case;
5. byte-stable repeated JSON output for the update case.

The GitHub Actions `Technology Spikes` workflow delegates to this same script so local and CI verification cannot silently diverge.

## Proven by the current spike shape

The current Go and Rust implementations both exercise the narrow contract chain:

```text
Observed State fixture
  -> Desired Config fixture
  -> normalized required-check target
  -> semantic digest
  -> deterministic Plan
```

The comparison therefore provides useful evidence for basic model ergonomics, deterministic normalization/serialization, dependency footprint, test ergonomics and ordinary host binary construction.

## Not yet proven

The language decision gate remains open until the following implementation-critical dimensions have direct prototype evidence:

- YAML-preserving adaptation of a user-owned GitHub Actions workflow, including comments/order/format policy;
- a narrow read-only GitHub adapter with classified permission/error behavior rather than domain types coupled to SDK payloads;
- multi-platform release/install packaging evidence for Linux, macOS and Windows;
- a recorded contributor/maintenance comparison based on the completed prototype rather than the provisional architecture ranking.

These are not optional polish items. They are explicit decision criteria in `implementation-technology-evaluation-v0.md` and `implementation-roadmap-v0.md`.

## Actions infrastructure status

Repository Actions currently fail before job instantiation, including minimal workflows with no external actions. Until that external condition changes, absence of a green GitHub-hosted run must not block local spike progress and must not be interpreted as a failure of Go or Rust.

When Actions can instantiate jobs again, the same `scripts/verify-technology-spikes.sh` command is the first verification target.

## Next vertical slice

The next spike should compare YAML adaptation fidelity in Go and Rust against the same workflow fixture. It should make the preservation policy explicit and produce a machine-checkable result before either implementation is promoted to the production core.

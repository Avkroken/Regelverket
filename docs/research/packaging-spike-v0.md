# Cross-platform packaging spike v0

Status: implementation evidence
Date: 2026-08-27

## Purpose

Close the distribution portion of the Go/Rust technology gate with host-native builds on the three desktop platforms targeted by the first public CLI release.

## Platforms

The `Packaging Spike` workflow runs the same verifier on:

- Linux (`ubuntu-latest`)
- macOS (`macos-latest`)
- Windows (`windows-latest`)

Each runner executes unit tests and a release build for both existing model-chain implementations.

## Package contract

Each implementation must produce one executable named `regelverket` (`regelverket.exe` on Windows) plus a tiny manifest inside a platform-native archive:

- Linux/macOS: `regelverket-{go|rust}-{platform}.tar.gz`
- Windows: `regelverket-{go|rust}-windows.zip`

The verifier checks that the executable and archive are non-empty. This slice intentionally measures build/distribution mechanics rather than signing, attestations, installer UX or release publishing.

## Decision evidence

The comparison records whether either candidate requires additional runtime installation or target-specific build machinery for ordinary host-native releases. Binary size and compile time may be collected later, but they are secondary to reliable reproducible packaging for the initial technology decision.

## Remaining language gate

After YAML fidelity, GitHub adapter/error classification and this packaging slice have direct CI evidence, the remaining gate is the contributor/maintenance comparison and the final ADR. No language is selected by this document alone.

# Go vs Rust Implementation Spike v0

Status: active experiment
Date: 2026-08-27

## Scope

Both prototypes implement the same deliberately narrow vertical path:

1. parse the empirical Dumpen observed-state fixture
2. parse a Regelverket config fixture
3. resolve the desired required-check set
4. compute an order-independent semantic digest
5. compare observed vs desired required checks
6. emit a deterministic no-op or update plan

Two configs are exercised:

- baseline: `test`, `osv`, `scope-policy` -> expected no-op
- changed: adds `dependency-review` -> expected one update operation adding that context

The spike does not claim to implement the full DG-02, resolved-policy or plan contracts. It tests language fit for the contract chain with real fixture shapes.

## Go implementation

Location: `spikes/go/`

Dependencies:

- Go standard library
- `gopkg.in/yaml.v3`

Observed characteristics:

- concise single-binary CLI shape
- deterministic set normalization requires explicit sorting
- structs are readable for the current slice
- optional/uncertain state modelling will require explicit wrapper types or tagged interfaces as the model expands
- YAML dependency is small, but safe comment-preserving adaptation remains untested

Local environment result:

- Go 1.23.2 is installed
- local execution could not fetch `gopkg.in/yaml.v3` because the sandbox has no outbound DNS/network access
- therefore local compile/test is not recorded as PASS

## Rust implementation

Location: `spikes/rust/`

Dependencies:

- serde
- serde_json
- serde_yaml
- sha2
- hex

Observed characteristics from implementation:

- `BTreeSet` naturally expresses deterministic set semantics
- serde data modelling is direct
- enums are expected to fit DG-02 uncertainty/capability states better than ad-hoc string wrappers
- dependency count is larger in the current minimal spike than Go
- contributor complexity and compile-time cost remain to be measured

Local environment result:

- Rust/Cargo are not installed in the sandbox
- package installation timed out
- therefore local compile/test is not recorded as PASS

## GitHub CI verification

`.github/workflows/technology-spikes.yml` verifies both prototypes on a GitHub-hosted runner.

The workflow:

- pins `actions/checkout` to immutable commit `3d3c42e5aac5ba805825da76410c181273ba90b1` (v7.0.1)
- runs Go tests and both fixture cases
- runs Rust tests and both fixture cases
- parses the produced JSON and requires Go/Rust output equality
- asserts the baseline is a no-op
- asserts the changed case adds exactly `dependency-review`

CI result must be observed before either implementation is considered verified.

## What this spike already tells us

The language decision should not be made on syntax alone. Both candidates express this narrow path cleanly. The decisive tests remain:

1. full uncertainty/provenance model
2. YAML AST/comment-preserving analysis and edits
3. GitHub REST error/capability adapter
4. dependency graph operations
5. binary/release footprint
6. contributor readability
7. performance on large fixtures

## Current score — provisional

| Dimension | Go | Rust |
| --- | --- | --- |
| Small CLI/distribution model | strong | strong |
| Minimal dependency surface | stronger in this spike | weaker in this spike |
| Deterministic collections | explicit sorting | natural with ordered collections |
| Rich state modelling | adequate, more conventions needed | strong |
| Contributor simplicity | likely stronger | likely weaker |
| Compile verification in local sandbox | blocked by dependency network | blocked by missing toolchain |
| GitHub CI verification | pending | pending |

No language decision is made by this document.

## Next spike after CI passes

The next comparison must use a difficult case rather than more boilerplate: parse a real workflow while retaining source locations/comments, identify `merge_group`, jobs and check-provider candidates, then propose a minimal safe edit without destroying unrelated formatting. That experiment is expected to be more discriminating than this first contract-chain slice.

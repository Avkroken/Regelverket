# Template Distribution & Trust v0

Status: architecture draft
Date: 2026-08-27

## Goal

Public templates are governance supply-chain artifacts. Regelverket must let users consume, inspect, pin, verify and upgrade them without granting arbitrary code execution to a template catalog.

## Core rule

The declarative template format is data, not code.

Templates may cause Regelverket to render executable GitHub workflows, but template parsing itself must not execute shell, JavaScript, Python, hooks, plugins or repository-provided code.

## Distribution layers

### Built-in catalog

A small set of project-maintained templates shipped with a Regelverket release.

Properties:

- versioned with Regelverket
- fully tested against supported capability profiles
- available offline once installed
- trusted to the same level as the Regelverket binary/source release

### Official remote catalog

A separately versioned repository/catalog controlled by the project. This allows template updates without forcing a core release while preserving independent signatures/provenance.

The client must pin an exact catalog/template version for apply. `latest` can be used for discovery, never as the implicit identity of an already managed installation.

### Third-party catalog

Potential future feature. Third-party catalogs are untrusted by default and must require explicit opt-in plus trust metadata. They must not expand the execution model beyond the declarative schema.

## Template package

Conceptual package contents:

```text
template.yaml
README.md
constraints.yaml
variants/
fixtures/
checksums.json
provenance.json
```

Generated workflow source can be represented as versioned renderer inputs/fragments, but executable artifacts must remain inspectable in the final plan.

## Trust identity

A selected template is bound by:

- catalog identity
- template stable ID
- semantic version
- content digest
- schema version
- provenance/attestation state

The manifest records all of these so future `plan` can distinguish:

- same template, no change
- same version, unexpected content change
- intentional version upgrade
- catalog substitution

A released immutable template version must never silently mutate.

## Versioning

Semantic versioning is appropriate for public template contracts:

- patch: bug/security fix preserving intended governance semantics
- minor: backward-compatible capabilities/variants or optional behavior
- major: changed defaults/invariants/migration expectations

Because governance changes can be operationally significant even within semantic compatibility, every template upgrade still produces a plan.

## Verification / provenance

GitHub artifact attestations can establish where and how release artifacts were built. GitHub documents that attestations use Sigstore-backed provenance and can be verified by consumers. Attestations provide build provenance/integrity evidence; they do not prove that the artifact is safe.

Potential release model:

1. release workflow builds Regelverket binary/package and canonical catalog bundle
2. generate cryptographic digest
3. generate GitHub artifact attestation for distributable artifacts where supported
4. publish release-specific immutable version
5. client optionally/ultimately requires provenance matching the official repository and release policy

Exact enforcement level depends on packaging technology selected later.

## Source dependency security

Generated workflows that reference third-party Actions are executable supply-chain dependencies.

GitHub's current secure-use guidance states that a full-length commit SHA is the only immutable way to reference an Action release. Official templates therefore should default to full SHA pins for third-party Actions and record the human-readable upstream version as metadata/comment where useful.

A template update that changes a third-party dependency SHA is security-significant and must appear explicitly in the plan/changelog.

## Catalog update model

Commands conceptually separate:

```text
regelverk catalog refresh
regelverk template show <id>@<version>
regelverk template diff <old> <new>
regelverk plan --template <id>@<version>
```

No background automatic application of new template versions.

Catalog metadata may be cached with freshness/provenance metadata.

## Template upgrade lifecycle

```text
fetch candidate version
  -> verify catalog identity/provenance
  -> validate schema
  -> compare template invariants
  -> resolve against current repo/capabilities
  -> semantic plan
  -> user approval
  -> transactional apply
  -> verification
  -> manifest records new version/digest
```

## Revocation / compromised release

The design must support marking a template/core release as compromised or deprecated.

A revocation advisory must not silently modify repositories. Instead `detect/plan` should surface:

- installed affected version
- severity/reason
- safe replacement versions
- migration plan

Research is still needed on the best signed advisory channel.

## Offline behavior

A pinned built-in or cached verified template should remain usable offline for render/simulation. Operations requiring GitHub capability detection/apply obviously remain online-dependent.

Offline use must display freshness of capability knowledge and avoid claiming current GitHub support when the knowledge base is stale.

## Community contribution model

A future official catalog contribution should require:

- template schema validation
- documentation
- security review
- fixtures
- constraint tests
- adaptation tests
- E2E evidence for claimed support profiles
- provenance generated by the official release process

Popularity alone does not promote a template to official/certified status.

## Separation of knowledge and template

GitHub capability/rule knowledge is shared infrastructure, not copied into every template. Templates reference stable knowledge/constraint IDs.

This lets GitHub behavior corrections improve validation without rewriting all template definitions.

## Open decisions

- built-in-only through v0.x vs early remote catalog
- artifact format: archive, OCI artifact, GitHub Release asset, package registry, or combination
- exact attestation/signature enforcement policy
- catalog/release key continuity and project governance
- third-party catalogs timeline
- security advisory/revocation channel

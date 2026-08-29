# Regelverket

Regelverket är ett publikt verktyg under utveckling för att analysera, planera, applicera och verifiera GitHub repository-governance.

Projektet utgår från erfarenheter i Avkrokens tidigare ruleset/workflow-installation, men implementationen byggs som ett generiskt system från grunden. Den gamla implementationen används som referens, regression fixture och källa till dokumenterade driftlärdomar — inte som arkitektur som måste bevaras.

## Vision

Regelverket ska kunna:

- analysera ett befintligt eller nytt GitHub-repository,
- identifiera repositorytyp, workflows, branchstruktur, automation och tillgängliga GitHub-capabilities,
- rekommendera färdigbyggda och testade policy-/workflowmallar,
- applicera en mall rakt av eller anpassa den säkert till ett befintligt repository,
- återanvända existerande resurser när de redan uppfyller önskad capability,
- undvika namnkonflikter, oavsiktliga överskrivningar och dubbla resurser,
- generera en semantisk plan före förändringar,
- vara idempotent: samma desired state applicerad flera gånger ska bli en no-op efter första lyckade körningen,
- upptäcka drift och verifiera både deklarerad och effektiv GitHub-policy,
- förklara varför en regel, workflow eller branchpolicy finns och vilka andra resurser den beror på.

## Arkitekturriktning

Regelverket är i första hand en compiler/planner för GitHub repository-governance, inte ett växande installationsscript.

```text
User intent + template + repository state + GitHub capabilities
                         ↓
               normalized policy model
                         ↓
         constraints + adaptation + planning
                         ↓
       rulesets + workflows + branch topology
                         ↓
                     verify
```

## Status

Rust är vald för produktionskärnan och den första deterministiska compiler/planner-implementationen finns i `src/`. Go- och Rust-spikesen ligger kvar som reproducerbar beslutsevidens för YAML-fidelitet, GitHub-adapters och cross-platform packaging.

Projektet är fortfarande tidigt. Nästa implementation ska byggas vertikalt ovanpå den deterministiska kärnan och bevara idempotens, konservativ anpassning och verifierbar semantik.

Se:

- `docs/architecture/research-and-architecture-plan-v0.md`
- `docs/architecture/core-design-principles.md`
- `docs/architecture/decision-log.md`

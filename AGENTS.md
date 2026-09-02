# AGENTS.md

Den här filen är repositoryts auktoritativa arbetsinstruktion. Live GitHub-konfiguration är verkställande sanning om dokumentation och faktisk enforcement skiljer sig.

## Arbetsprincip

Leverera fungerande, verifierade och avgränsade ändringar. Läs relevant implementation, tester, konfiguration och dokumentation innan en lösning bestäms. Inför inte breaking changes utan uttryckligt beslut.

## Brancher och pull requests

- Pusha aldrig direkt till `main`.
- Använd `dev` som arbetsbranch och öppna PR från `dev` till `main`.
- Skapa inte ytterligare arbetsbrancher för normalt agentarbete.
- Squash är enda tillåtna merge-metod.
- Använd inte direkt merge om det inte uttryckligen begärts.
- Repositoryts workflows får inte uppdatera PR-brancher, armera auto-merge eller utföra annan PR-maintenance. Sådan automation är ett separat ansvar utanför Regelverkets verifieringsworkflows.

## Live merge-policy för `main`

`main` skyddas av aktiva organization-rulesets från `Avkroken` utan bypass actors.

Pull request-policyn kräver:

- 1 approval
- stale approvals avvisas efter ny push
- approval från någon annan än den som gjorde senaste pushen
- resolved review-trådar
- squash merge

Required status checks är exakt:

- `CI / required`
- `scope-policy`
- `scan-pr / osv-scan`

Required status checks använder strict latest-base enforcement. Resultat från en äldre base eller HEAD är inte mergebevis.

## Workflow-arkitektur

Varje workflow har ett enda ansvar. Den godkända workflow-inventeringen är:

- `.github/workflows/required-ci.yml` — build-, test- och verifierings-CI för repositoryts implementation och spikes. Den får endast läsa källkod och producera verifieringsresultat.
- `.github/workflows/scope-policy.yml` — regression-gate för den godkända workflow-inventeringen. Den får endast verifiera policy.
- `.github/workflows/osv-scanner.yml` — OSV dependency scanning. Den får endast skanna och rapportera dependency/security-resultat.

Ett workflow får inte få ett andra operativt ansvar för att det råkar vara praktiskt att lägga koden där. PR-maintenance, auto-merge, branchmutation, remediation och deployment ska inte gömmas i CI- eller security-workflows.

De pensionerade workflowsen `compiler-core.yml`, `github-adapter-spike.yml`, `packaging-spike.yml`, `technology-spikes.yml`, `sync-pool.yml`, `pr-watchdog.yml`, `auto-fix-review.yml` och `startup-smoke.yml` ska inte återinföras som parallella vägar. Pensionerade remediation- och security-reporting-flöden ska inte heller återinföras.

GitHub Actions ska använda minsta nödvändiga behörighet och actions ska pinnas till full commit-SHA när praktiskt möjligt.

## Required CI

`CI / required` är den stabila aggregate-gaten. `required-ci.yml` klassificerar ändrade paths och kör endast relevanta verifieringar:

- Rust core: `cargo fmt`, `cargo clippy`, `cargo test`
- GitHub adapter spike: `scripts/verify-github-adapter-spike.sh`
- technology spikes: `scripts/verify-technology-spikes.sh`
- cross-platform packaging: `scripts/verify-packaging-spike.sh` på Linux, macOS och Windows

Okänd påverkan ska fail-safe till hela CI-matrisen. Aggregate-jobbet ska alltid skapas och faila om en vald obligatorisk verifiering inte slutar i `success`.

## Scope policy

`scope-policy` verifierar att `.github/workflows/` exakt motsvarar den godkända workflow-inventeringen ovan. Ett nytt workflow kräver därför ett uttryckligt arkitekturbeslut och en samtidig uppdatering av policyn.

## OSV

`osv-scanner.yml` är repositoryts källa för organisationens centrala OSV required workflow. På PR är merge-gaten `scan-pr / osv-scan`. På `main`, schemalagd körning och manuell körning skannas dependencies för rapportering utan att workflowen får PR- eller branchbehörigheter.

## Code Scanning och review

Code Scanning merge protection gäller `CodeQL`:

- security alerts från `medium` och uppåt blockerar merge
- CodeQL errors och warnings blockerar merge

Copilot Code Review är aktiverat via organization-ruleset och är rådgivande, men relevanta review-trådar måste hanteras eftersom resolved conversations krävs före merge.

CodeRabbit är best effort och inte en required status check. Faktiska relevanta findings ska ändå verifieras och åtgärdas.

## Pre-PR quality gate

Före en ready PR ska hela diffen mot `main` granskas. Kontrollera korrekthet, säkerhet, felhantering, kompatibilitet, relevanta edge cases, secrets/debugrester/oavsiktliga filer och kör relevant test/lint/typecheck/build. Efter varje ny commit ska påverkad verifiering och live merge-status kontrolleras igen för exakt aktuell HEAD.

## Credentials

Committa eller exponera aldrig secrets, tokens, privata nycklar eller andra credentials. Ett lyckat API- eller workflow-anrop är inte i sig bevis på att en live-ändring är aktiv; verifiera resulterande state.

## Definition of done

En PR-baserad uppgift är klar först när diffen är självgranskad, relevant validering är genomförd, aktuell HEAD har passerat `CI / required`, `scope-policy`, `scan-pr / osv-scan` och CodeQL merge protection, nödvändiga approvals finns, relevanta review-trådar är resolved och ändringen har mergats enligt normal repositorypolicy.

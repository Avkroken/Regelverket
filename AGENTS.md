# AGENTS.md

Den här filen är repositoryts auktoritativa arbetsinstruktion. Live GitHub-konfiguration är verkställande sanning när dokumentation och faktisk enforcement skiljer sig.

## Arbetsprincip

Leverera fungerande, verifierade och avgränsade ändringar. Läs relevant implementation, tester, konfiguration och dokumentation innan lösningen bestäms. Bevara befintlig arkitektur och inför inte breaking changes om de inte uttryckligen krävs.

## Brancher och pull requests

- Pusha aldrig direkt till `main`.
- Använd en kortlivad arbetsgren och öppna en ready PR till `main`.
- Aktivera auto-merge först när live-rulesetet motsvarar mergekontraktet nedan. Under ruleset-migration lämnas PR:n öppen tills den nya policyn är aktiv och verifierad.
- Använd inte direkt merge om det inte uttryckligen begärts.
- Repositoryt ska inte vara beroende av en synkroniserad branchpool, PR-watchdog, review-router eller kopierad Codex-remediation för normalt agentarbete.
- Squash är enda tillåtna merge-metod.
- Ingen merge queue används.

## Merge-gates

När det nya rulesetet är importerat ska senaste PR-HEAD minst ha:

- `CI / required`
- `scope-policy`
- `scan-pr / osv-scan`
- native `CodeRabbit` commit-status
- Code Scanning merge protection för CodeQL
- lösta review-trådar

`CI / required` är den stabila aggregate-gaten för compiler- och spike-verifiering. `.github/workflows/required-ci.yml` routar påverkan och kör Rust core, GitHub-adapter-spike, technology-spikes och cross-platform packaging endast när de är relevanta. Vid okänd påverkan körs hela matrisen.

`scope-policy` är en separat regression-gate som verifierar att pensionerad repositoryautomation inte återinförs. `CodeRabbit` ska vara pending under review och success först när aktuell HEAD är färdiggranskad. Copilot Code Review är rådgivande, ska köras om efter push, men är inte en hard gate.

Alla review-kommentarer och trådar ska läsas och utvärderas. Relevanta findings åtgärdas i samma PR. En tråd markeras resolved först när eventuell nödvändig fix är genomförd och verifierad.

Efter varje ny commit ska CI och review-status kontrolleras igen. Kringgå aldrig rulesets, required checks eller review-thread-resolution.

## Pre-PR quality gate

Innan en ready PR skapas eller uppdateras ska hela diffen mot base branch granskas. Kontrollera korrekthet, säkerhet, felhantering, kompatibilitet, relevanta edge cases, secrets/debugrester/oavsiktliga filer och kör relevanta tester/lint/typecheck/build. Efter senare commits ska påverkad validering köras igen.

## Review-signal

Prioritera funktionell och teknisk signal framför redaktionell puts. Rapportera inte rena stavnings-, grammatik-, interpunktions-, wording- eller stilfel i dokumentation, Markdown, README, kodkommentarer eller docstrings. Rapportera däremot textfel som materiellt kan ändra teknisk betydelse, säkerhet, korrekthet, användarbeteende eller bokstavliga instruktioner samt typos i maskin- eller semantikbärande innehåll.

## Repository-specifikt

Regelverket utvecklar ett deterministiskt compiler/planner-flöde och research-spikes. Repositoryts CI ska verifiera faktisk implementation och evidens, inte mutera PR-grenen. Genererade filer som krävs av verifieringen ska committas av den som gör ändringen.

De tidigare path-filtrerade workflowsen `compiler-core.yml`, `github-adapter-spike.yml`, `packaging-spike.yml` och `technology-spikes.yml` är ersatta av `.github/workflows/required-ci.yml` och ska inte återinföras som parallella CI-vägar.

Pensionerad automation som `.github/workflows/sync-pool.yml`, `.github/workflows/pr-watchdog.yml`, `.github/workflows/auto-fix-review.yml`, `.github/workflows/codex-issue-remediation.yml`, `.github/workflows/startup-smoke.yml` och `.github/workflows/security-alert-snapshot.yml` får inte återinföras utan ett nytt uttryckligt repositorybeslut.

GitHub Actions ska använda minsta nödvändiga behörighet och pinnas till full commit-SHA när praktiskt möjligt.

## Credentials och verifiering

Committa eller exponera aldrig secrets, tokens, privata nycklar eller andra credentials. Ett lyckat API- eller workflow-anrop är inte i sig bevis på att en live-ändring är aktiv; verifiera resulterande state när uppgiften ändrar GitHub-konfiguration eller annan runtime/infrastruktur.

## Definition of done

En PR-baserad uppgift är klar först när diffen är självgranskad, relevant validering är genomförd, all review-feedback är utvärderad, senaste HEAD har passerat merge-gates ovan, relevanta review-trådar är resolved och live-rulesetet faktiskt har verkställt policyn före merge.

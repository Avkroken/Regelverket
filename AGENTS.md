# AGENTS.md

Den här filen är repositoryts auktoritativa arbetsinstruktion. Live GitHub-konfiguration är verkställande sanning när dokumentation och faktisk enforcement skiljer sig.

## Arbetsprincip

Leverera fungerande, verifierade och avgränsade ändringar. Läs relevant implementation, tester, konfiguration och dokumentation innan lösningen bestäms. Bevara befintlig arkitektur och inför inte breaking changes om de inte uttryckligen krävs.

## Brancher och pull requests

- Pusha aldrig direkt till `main`.
- Använd en kortlivad arbetsgren och öppna en ready PR till `main`.
- **Aktivera auto-merge omedelbart när PR:n skapats**, även medan CI eller review pågår.
- Använd inte direkt merge om det inte uttryckligen begärts.
- Repositoryt ska inte vara beroende av en synkroniserad branchpool, PR-watchdog, review-router eller kopierad Codex-remediation för normalt agentarbete.
- Live-rulesetet tillåter för närvarande endast squash merge.
- Ingen merge queue är live-enforced.

## Merge-gates

För `main` gäller för närvarande:

- required status context: `scope-policy`
- olösta review-trådar blockerar merge
- Copilot Code Review körs vid push till PR-grenen
- squash är enda tillåtna merge-metod

`scope-policy` är en liten regression-gate som verifierar att pensionerad repositoryautomation inte återinförs. Den ersätter inte projektets övriga compiler-, spike-, packaging- eller säkerhetsverifiering.

Alla review-kommentarer och trådar ska läsas och utvärderas. Relevanta findings åtgärdas i samma PR. En tråd markeras resolved först när eventuell nödvändig fix är genomförd och verifierad.

Efter varje ny commit ska CI och review-status kontrolleras igen. När required `scope-policy` är grön och alla relevanta review-trådar är resolved ska den redan armerade auto-merge-funktionen föra PR:n till `main`.

Om auto-merge inte sker ska den konkreta live-blockeraren identifieras. Kringgå aldrig rulesets, required checks eller review-thread-resolution.

## Pre-PR quality gate

Innan en ready PR skapas eller uppdateras ska hela diffen mot base branch granskas. Kontrollera korrekthet, säkerhet, felhantering, kompatibilitet, relevanta edge cases, secrets/debugrester/oavsiktliga filer och kör relevanta tester/lint/typecheck/build. Efter senare commits ska påverkad validering köras igen.

## Review-signal

Prioritera funktionell och teknisk signal framför redaktionell puts. Rapportera inte rena stavnings-, grammatik-, interpunktions-, wording- eller stilfel i dokumentation, Markdown, README, kodkommentarer eller docstrings. Rapportera däremot textfel som materiellt kan ändra teknisk betydelse, säkerhet, korrekthet, användarbeteende eller bokstavliga instruktioner samt typos i maskin- eller semantikbärande innehåll.

## Repository-specifikt

Regelverket utvecklar ett deterministiskt compiler/planner-flöde och research-spikes. Repositoryts CI ska verifiera faktisk implementation och evidens, inte mutera PR-grenen. Genererade filer som krävs av verifieringen ska committas av den som gör ändringen.

Pensionerad automation som `.github/workflows/sync-pool.yml`, `.github/workflows/pr-watchdog.yml`, `.github/workflows/auto-fix-review.yml`, `.github/workflows/codex-issue-remediation.yml`, `.github/workflows/startup-smoke.yml` och `.github/workflows/security-alert-snapshot.yml` får inte återinföras utan ett nytt uttryckligt repositorybeslut.

GitHub Actions ska använda minsta nödvändiga behörighet och pinnas till full commit-SHA när praktiskt möjligt.

## Credentials och verifiering

Committa eller exponera aldrig secrets, tokens, privata nycklar eller andra credentials. Ett lyckat API- eller workflow-anrop är inte i sig bevis på att en live-ändring är aktiv; verifiera resulterande state när uppgiften ändrar GitHub-konfiguration eller annan runtime/infrastruktur.

## Definition of done

En PR-baserad uppgift är klar först när diffen är självgranskad, relevant validering är genomförd, all review-feedback är utvärderad, required `scope-policy` är grön, relevanta review-trådar är resolved och auto-merge har mergat PR:n eller är armerad medan en verifierad extern gate fortfarande väntar.

# AGENTS.md

Den här filen är repositoryts auktoritativa arbetsinstruktion. Live GitHub-konfiguration är verkställande sanning när dokumentation och faktisk enforcement skiljer sig.

## Arbetsprincip

Leverera fungerande, verifierade och avgränsade ändringar. Läs relevant implementation, tester, konfiguration och dokumentation innan lösningen bestäms. Bevara befintlig arkitektur och inför inte breaking changes om de inte uttryckligen krävs.

## Brancher och pull requests

- Pusha aldrig direkt till `main`.
- Använd en kortlivad arbetsgren och öppna en ready PR till `main`.
- Aktivera auto-merge först när live-rulesetet är verifierat, required checks för aktuell HEAD är gröna och relevanta review-trådar är resolved.
- Använd inte direkt merge om det inte uttryckligen begärts.
- Repositoryt ska inte vara beroende av en synkroniserad branchpool, PR-watchdog, review-router eller kopierad Codex-remediation för normalt agentarbete.
- Squash är enda tillåtna merge-metod.
- Ingen merge queue används.

## Live merge-policy för `main`

Det aktiva repository-rulesetet gäller default branch och har inga bypass actors. Det blockerar deletion och non-fast-forward/force push och kräver pull request före merge.

Pull request-policyn är:

- 0 generella approvals
- ingen last-push approval
- olösta review-trådar blockerar merge
- endast squash merge

Required status checks är exakt:

- `CI / required`
- `scope-policy`
- `osv`

Required status checks använder strict latest-base enforcement. En PR måste därför verifieras mot aktuell `main`; gamla resultat från en äldre base eller HEAD får inte användas som mergebevis.

`CI / required` är den stabila aggregate-gaten för compiler- och spike-verifiering. `.github/workflows/required-ci.yml` routar påverkan och kör Rust core, GitHub-adapter-spike, technology-spikes och cross-platform packaging när de är relevanta. Vid okänd påverkan körs hela matrisen. Aggregate-jobbet ska alltid skapas och faila om en obligatorisk underliggande verifiering inte slutar i `success`.

`scope-policy` är en separat regression-gate som verifierar att pensionerad repositoryautomation inte återinförs.

`osv` är den stabila dependency/security-gaten för PR. Den failar om OSV:s PR-skanning inte slutar i `success`.

## Code Scanning

Code Scanning merge protection är aktiv för verktyget `CodeQL`.

- security alerts från `medium` och uppåt blockerar merge
- CodeQL error/warning-alerts blockerar merge

Trivy är inte konfigurerat i repositoryt och det finns därför ingen Trivy merge-gate eller Trivy-threshold att dokumentera som aktiv policy.

## CodeRabbit och Copilot

CodeRabbit är best effort och är **inte** en required status check. `.coderabbit.yaml` får publicera sanningsenlig commit-status, reviewfel och incremental review-signal, men saknad, pending, rate-limited eller misslyckad CodeRabbit-status blockerar inte ensam merge.

Om CodeRabbit faktiskt lämnar relevanta findings ska de verifieras och åtgärdas. Relevanta review-trådar måste vara resolved före merge eftersom GitHub-rulesetet kräver review-thread-resolution.

Copilot Code Review är rådgivande och är inte en hard merge-gate. Rulesetet har `review_on_push` aktiverat och draft-PR:er undantas. Om Copilot faktiskt lämnar relevanta findings ska de utvärderas och eventuella relevanta review-trådar hanteras som annan review-feedback.

## Review-feedback

Alla review-kommentarer och trådar ska läsas och utvärderas. Relevanta findings åtgärdas i samma PR. En tråd markeras resolved först när eventuell nödvändig fix är genomförd och verifierad.

Efter varje ny commit ska CI, Code Scanning och review-status kontrolleras igen för exakt aktuell HEAD. Kringgå aldrig rulesets, required checks, Code Scanning merge protection eller review-thread-resolution.

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

En PR-baserad uppgift är klar först när diffen är självgranskad, relevant validering är genomförd, all review-feedback är utvärderad, exakt aktuell HEAD har passerat `CI / required`, `scope-policy`, `osv` och CodeQL merge protection, relevanta review-trådar är resolved, live-rulesetet fortfarande motsvarar policyn ovan och PR:n har mergats enligt normal repositorypolicy.

## PR-scope efter öppning

Den här sektionen förtydligar tidigare formuleringar om att relevanta findings ska åtgärdas i samma PR.

- När en PR har öppnats är dess avsedda scope, så som det beskrivs i PR:n, fryst. Fortsatta commits får endast slutföra eller korrigera det scopet.
- Om CI, Code Scanning, tester eller review hittar ett fel som orsakas av PR:ns befintliga ändringar ska just det felet rättas på samma branch/PR. Det är en korrigering inom scope, inte ny scope.
- Ny funktionalitet, opportunistiska refactors, städning eller separata förbättringar som upptäcks efter att PR:n öppnats ska få en ny kortlivad branch och en ny PR från aktuell `main`; återanvänd inte den öppna PR-grenen för nästa uppgift.
- Försök inte hinna lägga commits före eller under en pågående CI-/reviewkörning av tidsskäl. Gör en komplett ändring, pusha den, låt gates utvärdera den HEAD:en och reagera därefter.
- Efter varje korrigerande commit ska relevanta tester köras om och hela tillämpliga gate- och review-state verifieras på den nya HEAD:en före merge.

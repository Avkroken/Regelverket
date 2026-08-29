# Regelverket — AI Agent Guide

## Pre-PR quality gate

Before opening a pull request, an AI coding agent must treat its own work as production code and perform a first-pass review itself. Read the applicable repository instructions and documentation, inspect the existing implementation and nearby tests before changing code, and preserve established architecture and conventions.

Before the PR is created:

- inspect the complete diff against `main` for correctness, unintended changes, security issues, compatibility problems, and repository conventions;
- run the relevant tests, linting, type checks, builds, and other validation defined by the repository;
- add or update tests when behavior changes and add a regression test for bug fixes when practical;
- fix failures and defects found during self-review instead of delegating first-pass debugging to CI or external reviewers; and
- only open the PR when the agent reasonably believes the change is complete and merge-ready, subject to independent CI and review gates.

CI and AI/human reviewers are independent verification, not substitutes for understanding the code, testing the implementation, or reviewing the diff before submission. After every subsequent commit, repeat the relevant validation and inspect the new diff before relying on downstream review.

## GitHub-arbetsflöde

Arbete sker via pull requests till `main`. `main` är den skyddade integrationsgrenen; arbetsgrenar är tillfälliga och får använda repo- eller agentvalda namn som `claude/*`, `codex/*`, `feature/*`, `fix/*` eller motsvarande.

- Skicka aldrig direkt till `main`. Öppna en ready PR till `main` och aktivera auto-merge omedelbart, även medan CI eller review fortfarande pågår.
- Required CI-checkar och olösta review-trådar är merge-blockerare. Läs och utvärdera alltid alla review-kommentarer; relevanta problem åtgärdas i samma PR innan tråden markeras resolved.
- Efter varje ny commit ska både CI och review-status kontrolleras igen. När required CI är grönt och alla review-trådar är resolved ska den redan armerade auto-merge-funktionen eller merge-kön föra PR:n till `main`.
- Om auto-merge inte sker trots gröna checkar och lösta review-trådar, identifiera exakt vilken repository-regel eller blockerare som återstår. Direkt merge får endast användas på uttrycklig instruktion.
- Kringgå aldrig branch protection, rulesets, required checks, review resolution eller merge queue.

`.github/workflows/pr-watchdog.yml` bevakar alla lokala branches utom `main`, merge-köns `gh-readonly-queue/*`, den interna permanenta state-branchen `automation/pr-watchdog-state` och uttryckliga permanenta undantag. När en branch med unika commits först observeras utan öppen PR sparas `firstSeen` beständigt på state-branchen. Perioden fortsätter även om HEAD ändras och nollställs först när en öppen PR finns eller branchen inte längre har unika commits mot `main`. Efter mer än 60 minuter skapas en ready PR till `main` och squash auto-merge armeras. Watchdoggen ska inte återöppna exakt samma HEAD som redan har behandlats i en stängd PR och ska inte själv avgöra om arbetet är önskvärt eller mergebart; det beslutet lämnas till CI, review och repositoryts merge-gates.

Befintliga `work/feature`, `work/fix` och `work/chore` får fortsätta användas som återanvändbara slots där repot har sync-pool, men de är inte de enda tillåtna arbetsgrenarna. `.github/workflows/sync-pool.yml` får endast återställa eller synka de uttryckligen konfigurerade poolslotsen och får aldrig resetta godtyckliga agent- eller arbetsgrenar.

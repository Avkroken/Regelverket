# AGENTS.md

Den här filen är repositoryts auktoritativa arbetsinstruktion. Live GitHub-konfiguration är verkställande sanning när dokumentation och faktisk enforcement skiljer sig.

## Repository status

Regelverket är pensionerat som applikationsprojekt. Repositoryt är ett tillfälligt övergångsrepo medan Avkrokens organization-ruleset fortfarande refererar `.github/workflows/osv-scanner.yml` som central required workflow.

Bygg inte ny produktfunktionalitet här. Gemensam GitHub-automation hör hemma i `Avkroken/.github`.

## Brancher och pull requests

- Pusha aldrig direkt till `main`.
- Skapa en kortlivad branch från aktuell `main` för varje avgränsad ändring.
- Öppna en ready pull request till `main`.
- Squash är enda tillåtna merge-metod.
- Kringgå aldrig rulesets, required checks, reviewkrav eller thread resolution.
- Det finns ingen permanent `dev`-arbetsbranch.

## Tillåten workflow-inventering

Repository-owned workflows är exakt:

- `.github/workflows/required-ci.yml` — minimal övergångsgate som verifierar att pensionerad produktkod inte återinförs.
- `.github/workflows/scope-policy.yml` — verifierar den exakta workflow-inventeringen.
- `.github/workflows/osv-scanner.yml` — central OSV required workflow som fortfarande refereras av Avkrokens organization-ruleset.

Nya workflows eller återinförd produktkod kräver ett nytt uttryckligt beslut från repositoryägaren.

## Säkerhet och automation

- Committa eller exponera aldrig secrets, tokens, privata nycklar eller andra credentials.
- Workflows får inte skapa eller uppdatera branches/PR:er, automatisera review eller merge, deploya, delegera remediation eller starta coding agents.
- Metadata- och AI-automation för Avkroken ska utvecklas centralt i `Avkroken/.github`, inte här.
- `osv-scanner.yml` ska behållas oförändrad tills organization-rulesetet har flyttats till en annan källa.

## Definition of done

En ändring är klar först när diffen är avgränsad, aktuell HEAD har passerat repositoryts live required gates, relevanta review-trådar är resolved och ändringen har mergats genom normal enforcement.

När organization-rulesetet inte längre refererar Regelverkets `osv-scanner.yml` ska repositoryt arkiveras i stället för att få ett nytt ansvar av bekvämlighetsskäl.

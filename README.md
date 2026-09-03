# Regelverket

Regelverket är pensionerat som applikationsprojekt och repositoryt är avsiktligt nedbantat.

Den tidigare compiler/planner-idén, Rust-kärnan, spikes, fixtures och arkitekturdokumentationen är borttagna. Gemensam GitHub-automation för Avkroken hör hemma i `Avkroken/.github`.

## Kvarvarande ansvar

Repositoryt innehåller endast sina egna övergångskontroller:

- `.github/workflows/required-ci.yml` — verifierar att pensionerat produktinnehåll inte återinförs.
- `.github/workflows/scope-policy.yml` — verifierar repositoryts tillåtna workflow-inventarium och scope.

Organisationens required OSV-workflow ägs nu av `Avkroken/.github`; Regelverket är inte längre central OSV-källa.

## Slutläge

Repositoryt kan arkiveras när dess kvarvarande övergångsberoenden inte längre behövs av organisationens GitHub-konfiguration. Ingen ny produktfunktionalitet eller gemensam automation ska byggas här.

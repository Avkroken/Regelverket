# Regelverket

Regelverket är pensionerat som applikationsprojekt och repositoryt är avsiktligt nedbantat.

Den tidigare compiler/planner-idén, Rust-kärnan, spikes, fixtures och arkitekturdokumentationen är borttagna. Gemensam GitHub-automation för Avkroken hör hemma i `Avkroken/.github`.

## Tillfälligt kvarvarande ansvar

Repositoryt finns kvar tills organisationens GitHub-ruleset har flyttats bort från denna referens:

- `.github/workflows/osv-scanner.yml` — används fortfarande som central required workflow av Avkroken.

`CI / required` och `scope-policy` finns endast för att nuvarande merge-policy för detta repository ska fortsätta fungera medan övergången slutförs.

## Slutläge

När organization-rulesetet i GitHub UI inte längre refererar Regelverkets OSV-workflow kan repositoryt arkiveras. Ingen ny produktfunktionalitet ska byggas här.

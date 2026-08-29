# AGENTS.md

Den här filen innehåller instruktioner för AI-agenter som arbetar i repositoryt.

Root-`AGENTS.md` är den auktoritativa källan för repositoryövergripande agentpolicy. En mer specifik `AGENTS.md` längre ned i katalogträdet får lägga till regler för sitt subtree, men ska inte duplicera eller motsäga den repositoryövergripande policyn.

Följ dessutom de repository-specifika instruktionerna längre ned i denna fil.

<!-- AVKROKEN-COMMON:START -->

## Arbetsprincip

Leverera fungerande, verifierade och avgränsade ändringar. CI, GitHub Copilot Code Review och mänskliga reviewers är oberoende verifieringslager och ska inte vara den första debuggern för fel som agenten rimligen kan upptäcka själv före en pull request. Ändra inte mer än uppgiften kräver och bevara befintlig arkitektur och repository-specifika konventioner om det inte finns ett konkret skäl att ändra dem.

## Innan implementation

1. Läs denna fil och eventuell närmare `AGENTS.md` för de filer som berörs.
2. Läs relevant implementation, tester, konfiguration och närliggande dokumentation innan lösningen bestäms.
3. Identifiera repositoryts faktiska build-, test-, lint-, typecheck- och CI-kommandon från befintlig konfiguration.
4. Följ repositoryts branchmodell. Skapa inte egna branchkonventioner och anta inte att en policy är ruleset-enforced utan att den faktiskt är det.
5. Gör minsta kompletta ändring som löser problemet.

## Pre-PR quality gate

Innan en ready pull request skapas eller uppdateras ska agenten:

- granska hela den egna diffen mot PR:ns base branch;
- kontrollera korrekthet, säkerhet, felhantering, kompatibilitet och relevanta edge cases;
- köra alla relevanta lokala tester samt tillämplig lint, typecheck och build;
- lägga till eller uppdatera tester när beteende ändras och detta är praktiskt testbart;
- kontrollera att inga secrets, credentials, debugrester eller oavsiktliga filer har lagts till;
- fixa legitima egna findings innan PR:n lämnas för extern review.

Efter en senare commit eller push ska validering som påverkas av den nya ändringen köras igen. Om full lokal validering inte är möjlig ska detta redovisas konkret i PR:n; hitta inte på ett grönt resultat.

## Review-signal

Vid code review ska funktionell och teknisk signal prioriteras framför redaktionell puts.

Rapportera inte rena stavnings-, grammatik-, interpunktions-, wording- eller stilfel i README och annan dokumentation, Markdown eller annan mänskligt läsbar prosa, kodkommentarer eller docstrings.

Undantag: rapportera ett textfel när det materiellt kan ändra teknisk betydelse, säkerhet, korrekthet, användarbeteende eller en instruktion som förväntas köras eller kopieras bokstavligt.

Rapportera typos och andra textfel i maskin- eller semantikbärande innehåll när de kan påverka beteendet, inklusive identifierare och symbolnamn, strängkonstanter med programbetydelse, paths och filnamn, konfigurationsnycklar, environment-variabler, API-fält, kommandon och flaggor, selectors samt protokoll- och enumvärden.

Prioritera korrekthet, säkerhet, tillförlitlighet, kompatibilitet, tester och underhållbarhet.

## Reviewnivå och eskalering

Använd lägsta reviewnivå som ger tillräcklig säkerhet.

- **Low:** GitHub Copilot Code Review Lite för rutinmässiga, lokala och väl avgränsade ändringar.
- **Medium:** Copilot Balanced för icke-trivial logik, flera sammanhängande komponenter eller API-/kompatibilitetsbeteende.
- **High:** minst Balanced för auth/access control, credentials/secrets, persistent data/schema/migrationer, concurrency/retries/idempotency, distributed/cross-service state, protokoll/integrationskontrakt, releaseflöden, privilegierad infrastruktur eller stora/riskfyllda refactors. Om frågan fortfarande kräver en separat djup implementation eller ett oberoende andra pass, delegera via den installerade OpenAI Codex-agentens faktiska GitHub-`@handle`.
- **Critical:** Balanced + Codex när ett fel trovärdigt kan innebära auth bypass, secret exposure, dataförlust/-korruption, destruktiv/irreversibel migration eller annan exceptionell produktionspåverkan. Om frågan fortfarande är olöst eller väsentligt tvetydig, begär ett separat andra pass från den installerade Anthropic Claude-agentens faktiska GitHub-`@handle`.

Bygg inte ett nytt router-workflow enbart för denna eskalering. Native GitHub-delegering är standardvägen så länge inget organisationsbeslut säger annat.

## Pull request och merge

Pusha aldrig direkt till `main`. Följ repositoryts specifika branchmodell och skapa en ready PR först när pre-PR-gaten är genomförd.

Efter varje ny commit eller push ska den aktuella PR-statusen verifieras igen. Verifiera åtminstone aktuell HEAD, required checks/CI, mergeability, mergekonflikter och obligatoriska review-trådar/blockers.

När GitHub bedömer PR:n som direkt mergebar och alla tillämpliga repository-gates är uppfyllda — required checks/CI är klara och godkända, inga mergekonflikter finns och inga relevanta obligatoriskt olösta review-trådar eller andra blockers återstår — ska PR:n mergas omedelbart.

Försök inte aktivera auto-merge på en PR som redan är direkt mergebar. Använd auto-merge när PR:n ännu inte kan mergas enbart därför att obligatoriska gates fortfarande väntar och repositoryt stöder auto-merge. Repositoryts aktuella ruleset, merge queue och repositoryinställningar bestämmer vilka merge-metoder som är tillåtna. Om GitHub inte tillåter merge trots att PR:n ser grön ut ska den konkreta blockeraren identifieras; forcera eller kringgå inte repositoryskydd.

## Credentials och AI-infrastruktur

Committa eller exponera aldrig secrets, tokens, privata nycklar eller andra credentials. Lägg inte till `OPENAI_API_KEY`, `ANTHROPIC_API_KEY` eller annan extern AI-provider-credential i repository, Actions secrets eller organisationskonfiguration utan uttryckligt godkännande från repository- eller organisationsägaren. Ändra inte billing, Copilot-policy, repository secrets eller organisationsinställningar enbart för att möjliggöra AI-routing utan uttryckligt godkännande. Föredra befintliga GitHub/Copilot-native mekanismer framför nya workflows, botar eller dispatchers när de redan löser uppgiften.

## Definition of done

En agentuppgift är inte klar förrän implementationen är färdig och avgränsad, relevanta tester/checks har körts eller en konkret begränsning dokumenterats, den slutliga diffen självgranskats, legitima review-findings åtgärdats, PR-status verifierats mot aktuell HEAD, PR:n antingen mergats därför att alla gates är uppfyllda eller har auto-merge aktiverat därför att endast väntande obligatoriska gates återstår, och ingen repositoryregel har kringgåtts.

<!-- AVKROKEN-COMMON:END -->

## Repository-specifika instruktioner

Arbete sker via pull requests till `main`. `main` är den skyddade integrationsgrenen; arbetsgrenar är tillfälliga och får använda repo- eller agentvalda namn som `claude/*`, `codex/*`, `feature/*`, `fix/*` eller motsvarande.

`.github/workflows/pr-watchdog.yml` bevakar alla lokala branches utom `main`, merge-köns `gh-readonly-queue/*`, state-branchen `automation/pr-watchdog-state` och uttryckliga permanenta undantag. När en branch med unika commits först observeras utan öppen PR sparas `firstSeen` beständigt. Perioden fortsätter även om HEAD ändras och nollställs först när en öppen PR finns eller branchen inte längre har unika commits mot `main`. Efter mer än 60 minuter skapas en ready PR till `main`. Watchdoggen ska inte återöppna exakt samma HEAD som redan behandlats i en stängd PR och ska inte själv avgöra om arbetet är önskvärt eller mergebart; det beslutet lämnas till CI, review och repositoryts merge-gates.

Befintliga `work/feature`, `work/fix` och `work/chore` får fortsätta användas som återanvändbara slots där repot har sync-pool, men de är inte de enda tillåtna arbetsgrenarna. `.github/workflows/sync-pool.yml` får endast återställa eller synka uttryckligen konfigurerade poolslots och får aldrig resetta godtyckliga agent- eller arbetsgrenar.

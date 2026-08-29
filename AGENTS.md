# AGENTS.md

Den här filen innehåller instruktioner för AI-agenter som arbetar i repositoryt. Live repository configuration är verkställande sanning när faktisk enforcement skiljer sig från dokumentation.

Root-`AGENTS.md` är den auktoritativa källan för repositoryövergripande agentpolicy. En mer specifik `AGENTS.md` längre ned i katalogträdet får lägga till regler för sitt subtree, men ska inte duplicera eller motsäga den repositoryövergripande policyn.

Följ dessutom de repository-specifika instruktionerna längre ned i denna fil.

<!-- AVKROKEN-COMMON:START -->

## Arbetsprincip

Leverera fungerande, verifierade och avgränsade ändringar. CI, GitHub Copilot Code Review och mänskliga reviewers är oberoende verifieringslager och ska inte vara den första debuggern för fel som agenten rimligen kan upptäcka själv före en pull request. Ändra inte mer än uppgiften kräver, bevara befintlig arkitektur och repository-specifika konventioner och inför inte breaking changes om de inte uttryckligen krävs av uppgiften.

## Innan implementation

1. Läs denna fil och eventuell närmare `AGENTS.md` för de filer som berörs.
2. Läs relevant implementation, tester, konfiguration och närliggande dokumentation innan lösningen bestäms.
3. Identifiera repositoryts faktiska build-, test-, lint-, typecheck- och CI-kommandon från befintlig konfiguration.
4. Följ repositoryts branchmodell. Skapa inte egna branchkonventioner och anta inte att en policy är ruleset-enforced utan att den faktiskt är det.
5. Gör minsta kompletta ändring som löser problemet.

## Pre-PR quality gate

Innan en ready PR skapas eller uppdateras ska agenten granska hela diffen mot base branch, kontrollera korrekthet, säkerhet, felhantering, kompatibilitet och relevanta edge cases, köra relevanta tester/lint/typecheck/build, uppdatera tester när beteende ändras och detta är praktiskt testbart, kontrollera att inga secrets/debugrester/oavsiktliga filer lagts till och fixa legitima egna findings före extern review. Efter senare commits ska påverkad validering köras igen. Om full lokal validering inte är möjlig ska begränsningen redovisas konkret.

## Review-signal

Vid code review ska funktionell och teknisk signal prioriteras framför redaktionell puts. Rapportera inte rena stavnings-, grammatik-, interpunktions-, wording- eller stilfel i mänskligt läsbar prosa. Rapportera däremot textfel som materiellt kan ändra teknisk betydelse, säkerhet, korrekthet, användarbeteende eller bokstavliga instruktioner samt typos i maskin- eller semantikbärande innehåll såsom identifierare, strängkonstanter, paths, config keys, environment-variabler, API-fält, kommandon, flags, selectors, protokoll- och enumvärden.

## Reviewnivå och eskalering

Low använder Copilot Lite; Medium använder Balanced; High använder minst Balanced och vid behov installerad OpenAI Codex-agent via dess faktiska GitHub-handle; Critical använder Balanced + Codex och vid kvarstående kritisk tvetydighet installerad Anthropic Claude-agent via dess faktiska GitHub-handle. Gissa eller hårdkoda inte mention-namn. Bygg inte ett nytt router-workflow enbart för eskaleringen.

## Pull request och merge

Pusha aldrig direkt till `main`. Följ repositoryts branchmodell och skapa en ready PR först när pre-PR-gaten är genomförd.

Efter varje ny commit eller push ska aktuell HEAD, required checks/CI, mergeability och mergekonflikter verifieras igen. Läs och utvärdera dessutom alla review-kommentarer och alla nya, öppna eller återöppnade review-trådar; relevanta findings ska åtgärdas innan PR:n betraktas som klar.

När GitHub bedömer PR:n som direkt mergebar och alla tillämpliga live gates är uppfyllda — required checks är godkända, inga konflikter eller relevanta olösta reviewtrådar/blockers återstår och ingen relevant review-feedback är outhanterad — ska PR:n mergas omedelbart.

Försök inte aktivera auto-merge på en PR som redan är direkt mergebar. Använd auto-merge när PR:n ännu inte kan mergas enbart därför att obligatoriska gates fortfarande väntar och repositoryt stöder auto-merge. Live ruleset, merge queue och GitHub-inställningar bestämmer tillåten merge-metod. Forcera eller kringgå inte repositoryskydd.

## Credentials och AI-infrastruktur

Committa eller exponera aldrig secrets, tokens, privata nycklar eller andra credentials. Lägg inte till externa AI-provider-credentials eller ändra billing/Copilot-policy/secrets/organisationsinställningar enbart för AI-routing utan uttryckligt ägargodkännande. Föredra GitHub/Copilot-native mekanismer.

## Verifiering efter ändringar

Ett lyckat API-svar, workflow-anrop eller deployment-request är inte i sig bevis på att ändringen är aktiv. När uppgiften ändrar GitHub-inställningar, permissions, deployments, routes, bindings eller annan runtime-/live-konfiguration ska relevant resulterande state verifieras.

## Definition of done

För en uppgift som skapar eller uppdaterar en PR är arbetet inte klart förrän ändringen är färdig och avgränsad, relevanta checks är körda eller begränsningen dokumenterad, diffen självgranskad, all review-feedback läst och utvärderad, legitima findings åtgärdade, PR-status verifierad mot aktuell HEAD, eventuell live-state verifierad när tillämpligt och PR:n antingen mergad när alla gates är uppfyllda eller har auto-merge aktiverat när endast väntande obligatoriska gates återstår.

För read-only reviews, investigations, frågor eller live-konfigurationsuppgifter utan PR gäller inte PR-/mergekraven ovan; uppgiften är klar när efterfrågat arbete är genomfört och relevant resulterande status verifierats.

<!-- AVKROKEN-COMMON:END -->

## Repository-specifika instruktioner

Arbete sker via pull requests till `main`. `main` är den skyddade integrationsgrenen; arbetsgrenar är tillfälliga och får använda repo- eller agentvalda namn som `claude/*`, `codex/*`, `feature/*`, `fix/*` eller motsvarande.

`.github/workflows/pr-watchdog.yml` bevakar alla lokala branches utom `main`, merge-köns `gh-readonly-queue/*`, state-branchen `automation/pr-watchdog-state` och uttryckliga permanenta undantag. När en branch med unika commits först observeras utan öppen PR sparas `firstSeenAt` beständigt. Perioden fortsätter även om HEAD ändras och nollställs först när en öppen PR finns eller branchen inte längre har unika commits mot `main`. Efter mer än 60 minuter skapas en ready PR till `main` och workflowen aktiverar auto-merge enligt sin konfigurerade/tillåtna metod; required CI, review och övriga live gates fortsätter att blockera faktisk merge tills de är uppfyllda. Watchdoggen ska inte återöppna exakt samma HEAD som redan behandlats i en stängd PR och ska inte själv avgöra om arbetet är önskvärt eller mergebart; det beslutet lämnas till CI, review och repositoryts merge-gates.

Befintliga `work/feature`, `work/fix` och `work/chore` får fortsätta användas som återanvändbara slots där repot har sync-pool, men de är inte de enda tillåtna arbetsgrenarna. `.github/workflows/sync-pool.yml` får endast återställa eller synka uttryckligen konfigurerade poolslots och får aldrig resetta godtyckliga agent- eller arbetsgrenar.

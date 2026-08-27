use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{collections::BTreeSet, env, fs};

#[derive(Debug, Deserialize)]
struct Config {
    repository: ConfigRepository,
    intent: Intent,
}

#[derive(Debug, Deserialize)]
struct ConfigRepository {
    target: String,
}

#[derive(Debug, Deserialize)]
struct Intent {
    checks: Checks,
}

#[derive(Debug, Deserialize)]
struct Checks {
    required_contexts: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct Observed {
    repository: ObservedRepository,
    rulesets: Vec<Ruleset>,
}

#[derive(Debug, Deserialize)]
struct ObservedRepository {
    github: GithubRepo,
}

#[derive(Debug, Deserialize)]
struct GithubRepo {
    full_name: String,
}

#[derive(Debug, Deserialize)]
struct Ruleset {
    id: String,
    rules: Option<Vec<Rule>>,
}

#[derive(Debug, Deserialize)]
struct Rule {
    #[serde(rename = "type")]
    kind: String,
    parameters: Option<RuleParameters>,
}

#[derive(Debug, Deserialize)]
struct RuleParameters {
    checks: Option<Vec<Check>>,
}

#[derive(Debug, Deserialize)]
struct Check {
    context: String,
}

#[derive(Debug, Serialize)]
struct ResolvedPolicy {
    schema: &'static str,
    repository: String,
    required_checks: Vec<String>,
    semantic_digest: String,
}

#[derive(Debug, Serialize)]
struct Operation {
    action: &'static str,
    resource: &'static str,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    add: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    remove: Vec<String>,
}

#[derive(Debug, Serialize)]
struct Plan {
    schema: &'static str,
    repository: String,
    no_changes: bool,
    operations: Vec<Operation>,
}

#[derive(Debug, Serialize)]
struct Output {
    resolved_policy: ResolvedPolicy,
    plan: Plan,
}

fn normalized(values: &[String]) -> Vec<String> {
    values
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

fn observed_checks(observed: &Observed) -> Result<Vec<String>, String> {
    let ruleset = observed
        .rulesets
        .iter()
        .find(|ruleset| ruleset.id == "ruleset.main")
        .ok_or_else(|| "ruleset.main missing".to_string())?;
    let rules = ruleset
        .rules
        .as_ref()
        .ok_or_else(|| "ruleset.main rules missing".to_string())?;
    let rule = rules
        .iter()
        .find(|rule| rule.kind == "required_status_checks")
        .ok_or_else(|| "required_status_checks missing".to_string())?;
    let parameters = rule
        .parameters
        .as_ref()
        .ok_or_else(|| "required_status_checks parameters missing".to_string())?;
    let checks = parameters
        .checks
        .as_ref()
        .ok_or_else(|| "required_status_checks checks missing".to_string())?;
    Ok(normalized(
        &checks
            .iter()
            .map(|check| check.context.clone())
            .collect::<Vec<_>>(),
    ))
}

fn semantic_digest(repository: &str, checks: &[String]) -> String {
    #[derive(Serialize)]
    struct Payload<'a> {
        repository: &'a str,
        required_checks: Vec<String>,
    }

    let payload = Payload {
        repository,
        required_checks: normalized(checks),
    };
    let bytes = serde_json::to_vec(&payload).expect("serialize digest payload");
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    format!("sha256:{}", hex::encode(hasher.finalize()))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        return Err("usage: regelverket-rust-spike OBSERVED CONFIG".into());
    }

    let observed: Observed = serde_yaml::from_str(&fs::read_to_string(&args[1])?)?;
    let config: Config = serde_yaml::from_str(&fs::read_to_string(&args[2])?)?;

    if observed.repository.github.full_name != config.repository.target {
        return Err("repository target mismatch".into());
    }

    let current = observed_checks(&observed)?;
    let desired = normalized(&config.intent.checks.required_contexts);
    let current_set: BTreeSet<_> = current.iter().cloned().collect();
    let desired_set: BTreeSet<_> = desired.iter().cloned().collect();
    let add: Vec<_> = desired_set.difference(&current_set).cloned().collect();
    let remove: Vec<_> = current_set.difference(&desired_set).cloned().collect();
    let no_changes = add.is_empty() && remove.is_empty();
    let operations = if no_changes {
        vec![]
    } else {
        vec![Operation {
            action: "update",
            resource: "ruleset.main.required_status_checks",
            add,
            remove,
        }]
    };

    let output = Output {
        resolved_policy: ResolvedPolicy {
            schema: "regelverket.resolved-policy/v0-spike",
            repository: config.repository.target.clone(),
            required_checks: desired.clone(),
            semantic_digest: semantic_digest(&config.repository.target, &desired),
        },
        plan: Plan {
            schema: "regelverket.plan/v0-spike",
            repository: config.repository.target,
            no_changes,
            operations,
        },
    };

    println!("{}", serde_json::to_string_pretty(&output)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn digest_is_order_independent() {
        let a = semantic_digest(
            "Avkroken/dumpen",
            &["test".into(), "osv".into(), "scope-policy".into()],
        );
        let b = semantic_digest(
            "Avkroken/dumpen",
            &["scope-policy".into(), "test".into(), "osv".into()],
        );
        assert_eq!(a, b);
    }
}

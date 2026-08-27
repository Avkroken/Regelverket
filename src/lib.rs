use serde::Serialize;
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ResolvedRequiredChecks {
    pub repository: String,
    pub required_checks: Vec<String>,
    pub semantic_digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RequiredCheckPlan {
    pub repository: String,
    pub no_changes: bool,
    pub add: Vec<String>,
    pub remove: Vec<String>,
}

pub fn normalize_checks<I, S>(values: I) -> Vec<String>
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    values
        .into_iter()
        .map(Into::into)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

pub fn semantic_digest(repository: &str, checks: &[String]) -> String {
    #[derive(Serialize)]
    struct Payload<'a> {
        repository: &'a str,
        required_checks: Vec<String>,
    }

    let payload = Payload {
        repository,
        required_checks: normalize_checks(checks.iter().cloned()),
    };
    let bytes = serde_json::to_vec(&payload).expect("semantic digest payload must serialize");
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    format!("sha256:{}", hex::encode(hasher.finalize()))
}

pub fn resolve_required_checks(
    repository: impl Into<String>,
    desired: impl IntoIterator<Item = impl Into<String>>,
) -> ResolvedRequiredChecks {
    let repository = repository.into();
    let required_checks = normalize_checks(desired);
    let semantic_digest = semantic_digest(&repository, &required_checks);

    ResolvedRequiredChecks {
        repository,
        required_checks,
        semantic_digest,
    }
}

pub fn plan_required_checks(
    repository: impl Into<String>,
    observed: impl IntoIterator<Item = impl Into<String>>,
    desired: impl IntoIterator<Item = impl Into<String>>,
) -> RequiredCheckPlan {
    let repository = repository.into();
    let observed: BTreeSet<_> = normalize_checks(observed).into_iter().collect();
    let desired: BTreeSet<_> = normalize_checks(desired).into_iter().collect();
    let add = desired.difference(&observed).cloned().collect::<Vec<_>>();
    let remove = observed.difference(&desired).cloned().collect::<Vec<_>>();

    RequiredCheckPlan {
        repository,
        no_changes: add.is_empty() && remove.is_empty(),
        add,
        remove,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalization_is_sorted_and_deduplicated() {
        assert_eq!(
            normalize_checks(["test", "osv", "test", "scope-policy"]),
            vec!["osv", "scope-policy", "test"]
        );
    }

    #[test]
    fn semantic_digest_is_order_independent() {
        let a = resolve_required_checks("Avkroken/dumpen", ["test", "osv", "scope-policy"]);
        let b = resolve_required_checks("Avkroken/dumpen", ["scope-policy", "test", "osv"]);
        assert_eq!(a.semantic_digest, b.semantic_digest);
        assert_eq!(a.required_checks, b.required_checks);
    }

    #[test]
    fn unchanged_state_produces_noop() {
        let plan = plan_required_checks(
            "Avkroken/dumpen",
            ["test", "osv", "scope-policy"],
            ["scope-policy", "test", "osv"],
        );
        assert!(plan.no_changes);
        assert!(plan.add.is_empty());
        assert!(plan.remove.is_empty());
    }

    #[test]
    fn changes_are_deterministic() {
        let plan = plan_required_checks(
            "Avkroken/dumpen",
            ["legacy", "test", "osv"],
            ["dependency-review", "osv", "scope-policy", "test"],
        );
        assert!(!plan.no_changes);
        assert_eq!(plan.add, vec!["dependency-review", "scope-policy"]);
        assert_eq!(plan.remove, vec!["legacy"]);
    }
}

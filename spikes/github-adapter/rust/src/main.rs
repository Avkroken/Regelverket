use std::{env, process};

const API_VERSION: &str = "2026-03-10";

fn classify(status: u16) -> &'static str {
    match status {
        200 => "ok",
        401 => "authentication_failed",
        403 => "permission_denied",
        404 => "not_found_or_inaccessible",
        422 => "validation_failed",
        429 => "rate_limited",
        500..=599 => "github_service_error",
        400..=499 => "request_rejected",
        _ => "unexpected_status",
    }
}

fn emit(status: u16, class: &str, repository: Option<&str>) {
    match repository {
        Some(repo) => println!(
            "{{\"status\":{status},\"class\":\"{class}\",\"repository\":\"{repo}\"}}"
        ),
        None => println!("{{\"status\":{status},\"class\":\"{class}\"}}"),
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return Err("usage: github-adapter-rust URL".into());
    }

    let mut request = ureq::get(&args[1])
        .header("Accept", "application/vnd.github+json")
        .header("X-GitHub-Api-Version", API_VERSION)
        .header("User-Agent", "regelverket-rust-adapter-spike");
    if let Ok(token) = env::var("GITHUB_TOKEN") {
        if !token.is_empty() {
            request = request.header("Authorization", &format!("Bearer {token}"));
        }
    }

    match request.call() {
        Ok(mut response) => {
            let status = response.status().as_u16();
            let body = response.body_mut().read_to_string()?;
            let repository = body
                .contains("Avkroken/Regelverket")
                .then_some("Avkroken/Regelverket");
            emit(status, classify(status), repository);
        }
        Err(ureq::Error::StatusCode(status)) => emit(status, classify(status), None),
        Err(err) => {
            eprintln!("{err}");
            emit(0, "transport_error", None);
        }
    }
    Ok(())
}

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::classify;

    #[test]
    fn preserves_github_error_ambiguity() {
        assert_eq!(classify(403), "permission_denied");
        assert_eq!(classify(404), "not_found_or_inaccessible");
        assert_eq!(classify(422), "validation_failed");
    }
}

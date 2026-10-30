use std::collections::HashSet;

use anyhow::{Result, bail};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct App {
    pub name: String,
    pub repo: String,
    #[serde(default)]
    pub description: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Catalog {
    apps: Vec<App>,
}

pub fn load() -> Result<Vec<App>> {
    parse(include_str!("../catalogue.json"))
}

fn parse(json: &str) -> Result<Vec<App>> {
    let catalog: Catalog = serde_json::from_str(json)?;
    let mut repos = HashSet::new();
    for app in &catalog.apps {
        let Some((owner, repo)) = app.repo.split_once('/') else {
            bail!("invalid repository for {}: expected owner/repo", app.name);
        };
        if app.name.trim().is_empty()
            || !valid_component(owner, false)
            || !valid_component(repo, true)
            || !repos.insert(app.repo.to_ascii_lowercase())
        {
            bail!("invalid or duplicate catalog entry: {}", app.repo);
        }
    }
    Ok(catalog.apps)
}

fn valid_component(value: &str, repository: bool) -> bool {
    !value.is_empty()
        && value != "."
        && value != ".."
        && value.bytes().all(|b| {
            b.is_ascii_alphanumeric() || b == b'-' || (repository && (b == b'_' || b == b'.'))
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_catalog_is_valid() {
        assert!(parse(r#"{"apps":[]}"#).unwrap().is_empty());
    }

    #[test]
    fn parses_hica_app() {
        let apps = parse(
            r#"{"apps":[{"name":"Example","repo":"cladam/example","description":"Example app"}]}"#,
        )
        .unwrap();
        assert_eq!(apps[0].repo, "cladam/example");
    }

    #[test]
    fn rejects_invalid_and_duplicate_repositories() {
        for repo in [
            "",
            "owner",
            "owner/repo/extra",
            "../repo",
            "owner/..",
            "owner/repo?x=1",
            "owner\\repo",
        ] {
            let json = format!(r#"{{"apps":[{{"name":"Example","repo":"{repo}"}}]}}"#);
            assert!(parse(&json).is_err(), "{repo}");
        }
        assert!(parse(r#"{"apps":[{"name":"One","repo":"cladam/app"},{"name":"Two","repo":"CLADAM/App"}]}"#).is_err());
    }

    #[test]
    fn rejects_missing_or_empty_fields() {
        assert!(parse(r#"{"apps":[{"name":"","repo":"cladam/app"}]}"#).is_err());
        assert!(parse(r#"{"apps":[{"name":"Example"}]}"#).is_err());
        assert!(parse(r#"{"apps":[{"name":"Example","repository":"cladam/app"}]}"#).is_err());
    }
}

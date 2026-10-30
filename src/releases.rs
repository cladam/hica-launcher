use std::time::Duration;

use anyhow::{Context, Result};
use serde::Deserialize;

#[derive(Deserialize)]
struct GithubRelease {
    tag_name: String,
}

pub fn latest(repo: &str) -> Result<String> {
    let url = format!("https://api.github.com/repos/{repo}/releases/latest");
    let agent = ureq::Agent::config_builder()
        .timeout_global(Some(Duration::from_secs(15)))
        .tls_config(
            ureq::tls::TlsConfig::builder()
                .root_certs(ureq::tls::RootCerts::PlatformVerifier)
                .build(),
        )
        .build()
        .new_agent();
    let mut response = agent
        .get(&url)
        .header(
            "User-Agent",
            concat!("hica-launcher/", env!("CARGO_PKG_VERSION")),
        )
        .header("Accept", "application/vnd.github+json")
        .call()
        .with_context(|| format!("could not fetch the latest release for {repo}"))?;
    let release: GithubRelease = response
        .body_mut()
        .read_json()
        .with_context(|| format!("invalid release response for {repo}"))?;
    version(release).with_context(|| format!("invalid release response for {repo}"))
}

fn version(release: GithubRelease) -> Result<String> {
    if release.tag_name.trim().is_empty() {
        anyhow::bail!("release has no version tag");
    }
    Ok(release.tag_name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_release_tag() {
        let release: GithubRelease =
            serde_json::from_str(r#"{"tag_name":"v1.2.3","assets":[]}"#).unwrap();
        assert_eq!(version(release).unwrap(), "v1.2.3");
    }

    #[test]
    fn rejects_missing_or_blank_tag() {
        assert!(serde_json::from_str::<GithubRelease>(r#"{"assets":[]}"#).is_err());
        let release: GithubRelease = serde_json::from_str(r#"{"tag_name":" "}"#).unwrap();
        assert!(version(release).is_err());
    }
}

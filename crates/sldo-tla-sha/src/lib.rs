//! Library surface for `sldo-tla-sha`: read `tools.toml`, compute SHA-256 of
//! pinned artifacts with redirect + size protections, produce TOML patches.
//!
//! The library is split from `main.rs` so unit tests can exercise the hash
//! and host-allow-list logic without touching the network.

use anyhow::{bail, Context, Result};
use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::fs;
use std::io::Read;
use std::path::Path;
use url::Url;

/// Default ceiling on response size for streamed hashing. 500 MB is more than
/// 50× the current TLC and Apalache release sizes; exceeding it means the
/// upstream changed in a way that warrants maintainer attention, or the
/// response is tampered.
pub const DEFAULT_MAX_BYTES: u64 = 500 * 1024 * 1024;

/// Hosts the fetcher will accept as final URLs after redirects. GitHub
/// release URLs resolve to `objects.githubusercontent.com` via signed-URL
/// redirect; we accept that and the parent domain only.
pub const ALLOWED_HOSTS: &[&str] = &[
    "github.com",
    "objects.githubusercontent.com",
    "release-assets.githubusercontent.com",
    "codeload.github.com",
];

/// A single pinned artifact from `tools.toml`.
#[derive(Debug, Clone, Deserialize)]
pub struct ToolEntry {
    pub version: String,
    pub url: String,
    pub sha256: String,
}

/// Parsed `tools.toml`. Preserves arbitrary `[section]` names as a
/// BTreeMap so new artifacts can be added without touching this file.
#[derive(Debug, Clone, Deserialize)]
pub struct ToolsToml {
    #[serde(flatten)]
    pub sections: BTreeMap<String, ToolEntry>,
}

impl ToolsToml {
    pub fn load(path: &Path) -> Result<Self> {
        let content = fs::read_to_string(path)
            .with_context(|| format!("failed to read tools file: {}", path.display()))?;
        let parsed: Self = toml::from_str(&content)
            .with_context(|| format!("failed to parse tools.toml at {}", path.display()))?;
        Ok(parsed)
    }

    /// Sections whose `sha256` is the UNSET sentinel. Those need populating.
    pub fn unset_sections(&self) -> Vec<(&str, &ToolEntry)> {
        self.sections
            .iter()
            .filter(|(_, e)| e.sha256 == "UNSET")
            .map(|(n, e)| (n.as_str(), e))
            .collect()
    }

    /// Sections with a real-looking SHA (64 lowercase hex chars).
    pub fn populated_sections(&self) -> Vec<(&str, &ToolEntry)> {
        self.sections
            .iter()
            .filter(|(_, e)| e.sha256 != "UNSET")
            .map(|(n, e)| (n.as_str(), e))
            .collect()
    }
}

/// Stream `reader` into a SHA-256 hasher, aborting if `max_bytes` is
/// exceeded. Returns lowercase hex digest.
pub fn hash_reader<R: Read>(mut reader: R, max_bytes: u64) -> Result<String> {
    let mut hasher = Sha256::new();
    let mut total = 0u64;
    let mut buf = [0u8; 64 * 1024];
    loop {
        let n = reader.read(&mut buf).context("reading response body")?;
        if n == 0 {
            break;
        }
        total = total
            .checked_add(n as u64)
            .ok_or_else(|| anyhow::anyhow!("byte counter overflow"))?;
        if total > max_bytes {
            bail!(
                "response exceeded size cap of {} bytes (read {}); aborting",
                max_bytes,
                total
            );
        }
        hasher.update(&buf[..n]);
    }
    let digest = hasher.finalize();
    let mut hex = String::with_capacity(64);
    for b in digest {
        use std::fmt::Write;
        write!(hex, "{:02x}", b).unwrap();
    }
    Ok(hex)
}

/// Return true if the host is on the GitHub-release allow-list. Case-insensitive.
pub fn is_host_allowed(host: &str) -> bool {
    let host = host.to_ascii_lowercase();
    ALLOWED_HOSTS.iter().any(|h| host == *h)
}

/// Parse a URL and return the host portion, or an error if the URL is
/// malformed or has no host.
pub fn host_of(url_str: &str) -> Result<String> {
    let u = Url::parse(url_str).with_context(|| format!("invalid URL: {url_str}"))?;
    u.host_str()
        .map(|s| s.to_string())
        .ok_or_else(|| anyhow::anyhow!("URL has no host: {url_str}"))
}

/// Fetch the URL with redirects followed, then verify the FINAL URL's host
/// is on the allow-list, then stream-hash the body. Errors out before
/// hashing if the final host is disallowed.
pub fn fetch_and_hash(url: &str, max_bytes: u64) -> Result<String> {
    let client = reqwest::blocking::Client::builder()
        .redirect(reqwest::redirect::Policy::limited(10))
        .timeout(std::time::Duration::from_secs(120))
        .build()
        .context("failed to build HTTP client")?;
    let resp = client
        .get(url)
        .send()
        .with_context(|| format!("HTTP request failed for {url}"))?
        .error_for_status()
        .with_context(|| format!("HTTP error from {url}"))?;

    let final_url = resp.url().clone();
    let final_host = final_url.host_str().unwrap_or("");
    if !is_host_allowed(final_host) {
        bail!(
            "final URL host `{}` is not on the allow-list ({}). \
             Pinned URL: {}. Final URL after redirects: {}. \
             Refusing to compute SHA of content from an unexpected host.",
            final_host,
            ALLOWED_HOSTS.join(", "),
            url,
            final_url
        );
    }

    hash_reader(resp, max_bytes)
}

/// Format the TOML patch stdout for a set of (section_name, new_sha) pairs.
pub fn format_patch(updates: &[(String, String)]) -> String {
    let mut out = String::new();
    out.push_str("# sldo-tla-sha patch — apply these edits to skills/slo-tla/tools.toml\n");
    out.push_str("# computed at runtime by reading each [section].url, streaming, and SHA-256-hashing\n\n");
    for (section, sha) in updates {
        out.push_str(&format!("# [{section}]\n"));
        out.push_str(&format!("# replace:  sha256 = \"UNSET\"\n"));
        out.push_str(&format!("# with:     sha256 = \"{sha}\"\n\n"));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn hash_reader_matches_known_vector() {
        // SHA-256("hello") = 2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824
        let out = hash_reader(Cursor::new(b"hello"), 1024).unwrap();
        assert_eq!(
            out,
            "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
        );
    }

    #[test]
    fn hash_reader_aborts_at_size_cap() {
        // 11 bytes, cap at 10 → should abort.
        let data = b"hello world";
        let err = hash_reader(Cursor::new(data), 10).unwrap_err();
        assert!(err.to_string().contains("exceeded size cap"));
    }

    #[test]
    fn hash_reader_empty_is_well_defined() {
        // SHA-256("") = e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
        let out = hash_reader(Cursor::new(b""), 1024).unwrap();
        assert_eq!(
            out,
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }

    #[test]
    fn allowed_hosts_cover_github_releases() {
        assert!(is_host_allowed("github.com"));
        assert!(is_host_allowed("GITHUB.COM"));
        assert!(is_host_allowed("objects.githubusercontent.com"));
        assert!(is_host_allowed("release-assets.githubusercontent.com"));
    }

    #[test]
    fn allowed_hosts_rejects_foreign_hosts() {
        assert!(!is_host_allowed("evil.example.com"));
        assert!(!is_host_allowed("mirror.not-github.net"));
        assert!(!is_host_allowed(""));
        // A lookalike that contains "github.com" as a substring must NOT pass.
        assert!(!is_host_allowed("github.com.attacker.tld"));
        assert!(!is_host_allowed("fake-github.com"));
    }

    #[test]
    fn tools_toml_parses_tlc_apalache_sections() {
        let src = r#"
[tlc]
version = "1.8.0"
url = "https://github.com/tlaplus/tlaplus/releases/download/v1.8.0/tla2tools.jar"
sha256 = "UNSET"

[apalache]
version = "0.44.11"
url = "https://github.com/apalache-mc/apalache/releases/download/v0.44.11/apalache.tgz"
sha256 = "abcd"
"#;
        let parsed: ToolsToml = toml::from_str(src).unwrap();
        assert_eq!(parsed.sections.len(), 2);
        let unset = parsed.unset_sections();
        assert_eq!(unset.len(), 1);
        assert_eq!(unset[0].0, "tlc");
        let pop = parsed.populated_sections();
        assert_eq!(pop.len(), 1);
        assert_eq!(pop[0].0, "apalache");
    }

    #[test]
    fn format_patch_empty_lists_no_updates() {
        let out = format_patch(&[]);
        assert!(out.contains("sldo-tla-sha patch"));
        assert!(!out.contains("replace"));
    }

    #[test]
    fn format_patch_shows_each_section_replacement() {
        let updates = vec![
            ("tlc".to_string(), "a".repeat(64)),
            ("apalache".to_string(), "b".repeat(64)),
        ];
        let out = format_patch(&updates);
        assert!(out.contains("[tlc]"));
        assert!(out.contains("[apalache]"));
        assert!(out.contains(&"a".repeat(64)));
        assert!(out.contains(&"b".repeat(64)));
        // Must always show the replace+with pattern for each.
        assert_eq!(out.matches("replace:").count(), 2);
        assert_eq!(out.matches("with:").count(), 2);
    }

    #[test]
    fn host_of_parses_https() {
        assert_eq!(host_of("https://github.com/foo/bar").unwrap(), "github.com");
    }

    #[test]
    fn host_of_rejects_malformed() {
        assert!(host_of("not-a-url").is_err());
    }
}

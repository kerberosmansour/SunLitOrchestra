// xtasks/sast-verify/tests/fixtures/clean_subset/example.rs
//
// Known-clean Rust source for `cargo xtask sast-verify check-clean`'s
// default scan target. Per /slo-critique eng-1, the gate's `check-clean`
// step runs every authored rule against this directory and rejects any
// rule that produces a match. The directory is intentionally small and
// curated to keep the false-positive bar tight.
//
// What this file IS:
// - A handful of typical safe-Rust patterns that should match no security rule.
// - Used to catch overly-broad rules at gate-write time, before they land
//   in the rule pack and start blocking PRs in CI.
//
// What this file is NOT:
// - The host crate's `src/`. Scanning host `src/` is opt-in via
//   `cargo xtask sast-verify check-clean <rule> --clean-dir src/` for the
//   "find actual unfixed bugs" use case.
// - A full Rust feature exhibition. If a future rule fires on a benign
//   shape not represented here, the rule should be tightened — NOT this
//   directory widened (per AUTHORING.md).

use std::collections::HashMap;

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct CreateUser {
    name: String,
    email: String,
}

pub fn parse_user(body: &str) -> Result<CreateUser, serde_json::Error> {
    let user: CreateUser = serde_json::from_str(body)?;
    Ok(user)
}

pub fn safe_index<T>(v: &[T], i: usize) -> Option<&T> {
    v.get(i)
}

pub fn safe_capacity(header: usize, body: usize) -> Option<Vec<u8>> {
    let cap = header.checked_add(body)?;
    Some(Vec::with_capacity(cap))
}

pub fn safe_renew(now: u64, ttl: u64) -> u64 {
    now.saturating_add(ttl)
}

pub fn safe_path_join(base: &str, file: &str) -> std::io::Result<std::path::PathBuf> {
    if file.contains('/') || file.contains("..") {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "bad name",
        ));
    }
    Ok(std::path::PathBuf::from(base).join(file))
}

pub fn safe_constant_time_compare(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut diff = 0u8;
    for (x, y) in a.iter().zip(b.iter()) {
        diff |= x ^ y;
    }
    diff == 0
}

pub fn safe_session_check(session_expires_at: u64, now: u64) -> bool {
    if session_expires_at < now {
        return false;
    }
    true
}

pub fn safe_html_render(name: &str) -> String {
    let escaped = name
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;");
    format!("<h1>Welcome, {escaped}!</h1>")
}

pub fn safe_owned_buf(len: usize) -> Vec<u8> {
    vec![0u8; len]
}

pub fn safe_lookup(map: &HashMap<String, i64>, key: &str) -> Option<i64> {
    map.get(key).copied()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn safe_index_in_range() {
        let v = vec![1, 2, 3];
        assert_eq!(safe_index(&v, 1), Some(&2));
    }

    #[test]
    fn safe_index_out_of_range() {
        let v: Vec<i32> = vec![];
        assert_eq!(safe_index(&v, 0), None);
    }
}

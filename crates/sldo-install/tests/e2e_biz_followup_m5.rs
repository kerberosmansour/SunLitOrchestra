//! Follow-up M5 — biz-pack-pii-scan-runtime-test.
//!
//! Combined critique f2 (B1 M1): "no automated runtime test exists for the
//! Pass 4 PII-pattern scan." This follow-up implements the regex set in Rust
//! + tests against tempdir fixtures.
//!
//! The PII patterns are documented in skills/slo-verify/SKILL.md Pass 4
//! "Biz-pack PII-pattern scan" subsection. This test extracts those patterns
//! into Rust regexes and exercises them against tempdir fixtures containing
//! known matches and known non-matches.
//!
//! The implementation here lives in the test file (no sldo-pii-scan crate
//! exists yet). A future runbook may elevate this into a crate that
//! `/slo-verify` Pass 4 shells out to; until then, the Rust implementation
//! is the source of truth + the test is the gate.

use std::fs;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf()
}

// ---------------------------------------------------------------------------
// PII regex implementations.
//
// Patterns mirror skills/slo-verify/SKILL.md Pass 4 documentation. Stdlib-only
// (no `regex` crate dependency added) — the patterns are simple enough.
// ---------------------------------------------------------------------------

/// Email regex (RFC 5321 simplified): `[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}`
fn matches_email(s: &str) -> bool {
    for token in s.split_whitespace() {
        // Strip leading/trailing punctuation that's not part of an email.
        let token =
            token.trim_matches(|c: char| !c.is_ascii_alphanumeric() && !".-_+%@".contains(c));
        let Some(at_idx) = token.find('@') else {
            continue;
        };
        let (local, after_at) = token.split_at(at_idx);
        let domain = &after_at[1..];
        if local.is_empty() {
            continue;
        }
        // Local part chars: alphanumeric + . _ - + %
        if !local
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || ".-_+%".contains(c))
        {
            continue;
        }
        // Domain must contain a '.'
        let Some(dot_idx) = domain.find('.') else {
            continue;
        };
        if dot_idx == 0 || dot_idx == domain.len() - 1 {
            continue;
        }
        // TLD ≥ 2 alpha chars.
        let tld_start = domain.rfind('.').unwrap() + 1;
        let tld = &domain[tld_start..];
        if tld.len() < 2 {
            continue;
        }
        if !tld.chars().all(|c| c.is_ascii_alphabetic()) {
            continue;
        }
        // Domain pre-TLD chars: alphanumeric + . -
        let domain_pre = &domain[..tld_start - 1];
        if !domain_pre
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || ".-".contains(c))
        {
            continue;
        }
        return true;
    }
    false
}

/// UK National Insurance number regex: `\b[A-CEGHJ-PR-TW-Z][A-CEGHJ-NPR-TW-Z]\d{6}[A-D]\b`
/// Char-aware: iterate over chars, not bytes — avoids panics on multi-byte content.
fn matches_uk_ni(s: &str) -> bool {
    let valid_first = "ABCEGHJKLMNOPRSTWXYZ";
    let valid_second = "ABCEGHJKLMNPRSTWXYZ";
    let valid_suffix = "ABCD";
    let chars: Vec<char> = s.chars().collect();
    for i in 0..chars.len().saturating_sub(8) {
        // Word boundary before (ignore alphanumeric-following).
        if i > 0 && chars[i - 1].is_ascii_alphanumeric() {
            continue;
        }
        // 9 chars: 2 letters, 6 digits, 1 letter.
        if !valid_first.contains(chars[i]) {
            continue;
        }
        if !valid_second.contains(chars[i + 1]) {
            continue;
        }
        if !chars[i + 2..i + 8].iter().all(|c| c.is_ascii_digit()) {
            continue;
        }
        if !valid_suffix.contains(chars[i + 8]) {
            continue;
        }
        // Word boundary after.
        if i + 9 < chars.len() && chars[i + 9].is_ascii_alphanumeric() {
            continue;
        }
        return true;
    }
    false
}

/// UK sort code regex: `\b\d{2}-\d{2}-\d{2}\b` paired with the literal token "sort code" within ±3 lines.
/// Char-aware iteration to avoid byte-boundary issues with multi-byte chars (em-dashes etc.).
fn matches_uk_sort_code(s: &str) -> bool {
    let lines: Vec<&str> = s.lines().collect();
    for (i, line) in lines.iter().enumerate() {
        // Char-by-char scan for the 2-2-2 digit pattern.
        let chars: Vec<char> = line.chars().collect();
        for j in 0..chars.len().saturating_sub(7) {
            if chars[j].is_ascii_digit()
                && chars[j + 1].is_ascii_digit()
                && chars[j + 2] == '-'
                && chars[j + 3].is_ascii_digit()
                && chars[j + 4].is_ascii_digit()
                && chars[j + 5] == '-'
                && chars[j + 6].is_ascii_digit()
                && chars[j + 7].is_ascii_digit()
            {
                // Word boundary before (ignore digit-following chars).
                if j > 0 && chars[j - 1].is_ascii_alphanumeric() {
                    continue;
                }
                // Word boundary after.
                if j + 8 < chars.len() && chars[j + 8].is_ascii_alphanumeric() {
                    continue;
                }
                // Check ±3 lines for "sort code" literal.
                let lo = i.saturating_sub(3);
                let hi = (i + 3).min(lines.len() - 1);
                let context = lines[lo..=hi].join(" ").to_lowercase();
                if context.contains("sort code") {
                    return true;
                }
            }
        }
    }
    false
}

/// Capitalised-bigram named-person heuristic: line beginning with `name:` (case-insensitive)
/// followed by a `[A-Z][a-z]+ [A-Z][a-z]+` pattern.
fn matches_named_person(s: &str) -> bool {
    for line in s.lines() {
        let lower = line.trim_start().to_lowercase();
        if lower.starts_with("name:") || lower.starts_with("- name:") {
            let after_colon = line.split(':').nth(1).unwrap_or("").trim();
            // Must be at least two capitalised words.
            let words: Vec<&str> = after_colon.split_whitespace().take(3).collect();
            if words.len() >= 2 {
                let bigram_match = words[..2].iter().all(|w| {
                    let chars: Vec<char> = w.chars().collect();
                    if chars.is_empty() {
                        return false;
                    }
                    chars[0].is_ascii_uppercase()
                        && chars
                            .iter()
                            .skip(1)
                            .all(|c| c.is_ascii_lowercase() || *c == '-' || *c == '\'')
                        && chars.len() >= 2
                });
                if bigram_match {
                    return true;
                }
            }
        }
    }
    false
}

/// Combined PII scan — returns vector of (pattern_name, line_number) for each match.
fn scan_for_pii(content: &str) -> Vec<(&'static str, usize)> {
    let mut hits = Vec::new();
    for (i, line) in content.lines().enumerate() {
        if matches_email(line) {
            hits.push(("email", i + 1));
        }
        if matches_uk_ni(line) {
            hits.push(("uk_ni", i + 1));
        }
    }
    if matches_uk_sort_code(content) {
        hits.push(("uk_sort_code", 0));
    }
    if matches_named_person(content) {
        hits.push(("named_person", 0));
    }
    hits
}

/// Override check: returns true if the artifact has `pii_scan_override: true`
/// AND a `tier_override_reason: <one-line>` in its frontmatter.
fn has_valid_override(content: &str) -> bool {
    let has_override = content.contains("pii_scan_override: true");
    if !has_override {
        return false;
    }
    // tier_override_reason must be present and non-empty.
    let reason_line = content
        .lines()
        .find(|l| l.trim_start().starts_with("tier_override_reason:"));
    if let Some(line) = reason_line {
        let value = line
            .split("tier_override_reason:")
            .nth(1)
            .unwrap_or("")
            .trim();
        !value.is_empty() && value != "<one-line rationale>"
    } else {
        false
    }
}

// ---------------------------------------------------------------------------
// Test 1 — email pattern fires on a simple match.
// ---------------------------------------------------------------------------

#[test]
fn pii_scan_detects_email() {
    let dir = TempDir::new().unwrap();
    let file = dir.path().join("artifact.md");
    fs::write(&file, "name: Alice Smith\nemail: alice@example.com\n").unwrap();
    let content = fs::read_to_string(&file).unwrap();
    let hits = scan_for_pii(&content);
    assert!(
        hits.iter().any(|(p, _)| *p == "email"),
        "email pattern should fire on `alice@example.com`; got hits: {hits:?}"
    );
}

// ---------------------------------------------------------------------------
// Test 2 — UK NI number pattern fires.
// ---------------------------------------------------------------------------

#[test]
fn pii_scan_detects_uk_ni_number() {
    let dir = TempDir::new().unwrap();
    let file = dir.path().join("artifact.md");
    fs::write(&file, "Founder NI: AB123456C for tax records\n").unwrap();
    let content = fs::read_to_string(&file).unwrap();
    let hits = scan_for_pii(&content);
    assert!(
        hits.iter().any(|(p, _)| *p == "uk_ni"),
        "UK NI pattern should fire on `AB123456C`; got hits: {hits:?}"
    );
}

// ---------------------------------------------------------------------------
// Test 3 — UK sort code with "sort code" context fires.
// ---------------------------------------------------------------------------

#[test]
fn pii_scan_detects_uk_sort_code_with_context() {
    let content = "Bank details for the contractor\nSort code: 12-34-56\nAccount: 12345678\n";
    let hits = scan_for_pii(content);
    assert!(
        hits.iter().any(|(p, _)| *p == "uk_sort_code"),
        "UK sort code with `sort code` context should fire; got hits: {hits:?}"
    );
}

// ---------------------------------------------------------------------------
// Test 4 — UK sort code without "sort code" context does NOT fire (avoids false positives on dates).
// ---------------------------------------------------------------------------

#[test]
fn pii_scan_skips_uk_sort_code_without_context() {
    // Content has a 2-2-2 digit pattern but NO "sort code" literal anywhere
    // in ±3 lines (the avoidance-of-false-positive-on-dates property).
    let content = "Effective date: 12-34-56\nThis is actually a fake date format used in an example fixture.\n";
    let hits = scan_for_pii(content);
    assert!(
        !hits.iter().any(|(p, _)| *p == "uk_sort_code"),
        "UK sort code without context literal should NOT fire; got hits: {hits:?}"
    );
}

// ---------------------------------------------------------------------------
// Test 5 — capitalised-bigram named-person fires.
// ---------------------------------------------------------------------------

#[test]
fn pii_scan_detects_named_person() {
    let content = "name: Alice Smith\nrole: Head of Operations\n";
    let hits = scan_for_pii(content);
    assert!(
        hits.iter().any(|(p, _)| *p == "named_person"),
        "named_person should fire on `name: Alice Smith`; got hits: {hits:?}"
    );
}

// ---------------------------------------------------------------------------
// Test 6 — generic placeholder text does NOT fire.
// ---------------------------------------------------------------------------

#[test]
fn pii_scan_does_not_fire_on_placeholders() {
    let content = "name: <founder name>\nemail: <founder email>\nNI: <NI number>\n";
    let hits = scan_for_pii(content);
    assert!(
        hits.is_empty(),
        "placeholders should not trigger PII matches; got hits: {hits:?}"
    );
}

// ---------------------------------------------------------------------------
// Test 7 — override frontmatter accepted as documented in spec.
// ---------------------------------------------------------------------------

#[test]
fn pii_scan_override_frontmatter_recognised() {
    let content = r#"---
name: case-study-anonymised
tier: public
pii_scan_override: true
tier_override_reason: anonymised pseudonyms — Alice / Bob / Carol — used in case study; no real persons
---

# Case study with pseudonyms

name: Alice Smith
email: alice@example.com
"#;
    let hits = scan_for_pii(content);
    assert!(!hits.is_empty(), "PII scan should still detect patterns even when override is set; override applies to milestone-fail decision, not to detection");
    assert!(
        has_valid_override(content),
        "override frontmatter must be recognised as valid"
    );
}

// ---------------------------------------------------------------------------
// Test 8 — override without reason rejected.
// ---------------------------------------------------------------------------

#[test]
fn pii_scan_override_without_reason_rejected() {
    let content_no_reason = "---\npii_scan_override: true\n---\n\nname: Bob Jones\n";
    assert!(
        !has_valid_override(content_no_reason),
        "override without tier_override_reason must be rejected"
    );

    let content_placeholder_reason =
        "---\npii_scan_override: true\ntier_override_reason: <one-line rationale>\n---\n";
    assert!(
        !has_valid_override(content_placeholder_reason),
        "override with placeholder-template reason text must be rejected"
    );
}

// ---------------------------------------------------------------------------
// Test 9 — full milestone simulation: tempdir with mixed files, scan walks, classifies correctly.
// ---------------------------------------------------------------------------

#[test]
fn pii_scan_walks_tempdir_classifies_files() {
    let dir = TempDir::new().unwrap();
    let public_dir = dir.path().join("docs").join("biz-public").join("users");
    fs::create_dir_all(&public_dir).unwrap();

    // File 1: clean public artifact (placeholder names only, no real PII).
    fs::write(public_dir.join("clean.md"), "---\ntier: public\n---\n# Interview template\n\nname: <founder name>\nemail: <founder email>\n").unwrap();

    // File 2: leaked PII (real-looking content; should fire scan).
    fs::write(public_dir.join("leaked.md"), "---\ntier: public\n---\n# Interview with Sarah Patel\n\nname: Sarah Patel\nemail: sarah@acmelogistics.example.com\n").unwrap();

    // File 3: anonymised case study with valid override.
    fs::write(public_dir.join("anonymised.md"), "---\ntier: public\npii_scan_override: true\ntier_override_reason: anonymised pseudonyms used in published case study; Alice / Bob / Carol are not real persons\n---\n\nname: Alice Smith\n").unwrap();

    let mut violators: Vec<String> = Vec::new();
    for entry in fs::read_dir(&public_dir).unwrap() {
        let path = entry.unwrap().path();
        let content = fs::read_to_string(&path).unwrap();
        let hits = scan_for_pii(&content);
        if !hits.is_empty() && !has_valid_override(&content) {
            violators.push(path.file_name().unwrap().to_string_lossy().into_owned());
        }
    }

    // Expected violators: only `leaked.md`.
    assert_eq!(
        violators.len(),
        1,
        "expected exactly 1 violator; got {violators:?}"
    );
    assert_eq!(violators[0], "leaked.md");
}

// ---------------------------------------------------------------------------
// Test 10 — slo-verify SKILL.md Pass 4 documents all four pattern types
// (regression on Runbook B1 M1 contract).
// ---------------------------------------------------------------------------

#[test]
fn slo_verify_pass_4_documents_all_four_pattern_types() {
    let path = repo_root().join("skills/slo-verify/SKILL.md");
    let body = fs::read_to_string(&path).unwrap();
    let pattern_signals = [
        "Email addresses",
        "National Insurance",
        "sort code",
        "Capitalised-bigram",
    ];
    for sig in &pattern_signals {
        assert!(body.contains(sig), "slo-verify SKILL.md Pass 4 must document pattern type `{sig}` (regression on B1 M1 contract)");
    }
}

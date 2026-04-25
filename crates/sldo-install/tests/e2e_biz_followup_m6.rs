//! Follow-up M6 — biz-pack-pecr-runtime-check.
//!
//! Combined critique f7: direct-marketing channels in /slo-marketing,
//! /slo-sales-funnel, /slo-launch outputs route to /slo-legal triage via
//! TEXT directive, but no enforcement mechanism prevents the founder from
//! skipping the triage and proceeding to send. This follow-up adds a
//! `pecr_triage_completed: bool` frontmatter field + runtime check that
//! gates direct-marketing artifacts on triage completion.
//!
//! The check function lives here (no separate crate); future tooling can
//! invoke it as a library. The check walks `docs/biz-public/marketing/` +
//! `docs/biz-public/sales/`, identifies artifacts referencing direct
//! marketing, and asserts each carries `pecr_triage_completed: true` (with
//! a provenance pointer) OR `pecr_triage_completed: false` with an explicit
//! `pecr_triage_blocker:` reason.

use std::fs;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).parent().unwrap().parent().unwrap().to_path_buf()
}

// ---------------------------------------------------------------------------
// PECR triage detection + check.
// ---------------------------------------------------------------------------

const DIRECT_MARKETING_KEYWORDS: &[&str] = &[
    "cold email",
    "Cold email",
    "cold-email",
    "Cold-email",
    "outbound email",
    "Outbound email",
    "SMS marketing",
    "push notification",
    "direct marketing",
    "Direct marketing",
];

#[derive(Debug, Clone)]
enum PecrCheckOutcome {
    NotApplicable,                    // Artifact doesn't reference direct marketing.
    TriageCompleted(String),          // pecr_triage_completed: true; doc path provenance.
    TriagePending(String),            // pecr_triage_completed: false; blocker reason.
    MissingTriageField,               // Artifact references direct marketing but lacks the field.
    InconsistentField(String),        // pecr_triage_completed: true but no pecr_triage_doc; or false with no blocker.
}

/// Check a single artifact for PECR-triage compliance. The artifact qualifies
/// for the check if its body contains any direct-marketing keyword.
fn check_pecr_triage(content: &str) -> PecrCheckOutcome {
    let mentions_direct_marketing = DIRECT_MARKETING_KEYWORDS.iter().any(|kw| content.contains(kw));
    if !mentions_direct_marketing {
        return PecrCheckOutcome::NotApplicable;
    }

    let line_completed = content
        .lines()
        .find(|l| l.trim_start().starts_with("pecr_triage_completed:"))
        .map(|l| l.split("pecr_triage_completed:").nth(1).unwrap_or("").trim().to_string());

    let line_doc = content
        .lines()
        .find(|l| l.trim_start().starts_with("pecr_triage_doc:"))
        .map(|l| l.split("pecr_triage_doc:").nth(1).unwrap_or("").trim().to_string());

    let line_blocker = content
        .lines()
        .find(|l| l.trim_start().starts_with("pecr_triage_blocker:"))
        .map(|l| l.split("pecr_triage_blocker:").nth(1).unwrap_or("").trim().to_string());

    let Some(completed_value) = line_completed else {
        return PecrCheckOutcome::MissingTriageField;
    };

    if completed_value == "true" {
        let Some(doc) = line_doc else {
            return PecrCheckOutcome::InconsistentField(
                "pecr_triage_completed: true but pecr_triage_doc: missing — provenance pointer required when completed".to_string()
            );
        };
        if doc.is_empty() || doc.starts_with('<') {
            return PecrCheckOutcome::InconsistentField(format!("pecr_triage_doc value `{doc}` is placeholder — must be a real path"));
        }
        PecrCheckOutcome::TriageCompleted(doc)
    } else if completed_value == "false" {
        let Some(blocker) = line_blocker else {
            return PecrCheckOutcome::InconsistentField(
                "pecr_triage_completed: false but pecr_triage_blocker: missing — blocker reason required when not completed".to_string()
            );
        };
        if blocker.is_empty() || blocker.starts_with('<') {
            return PecrCheckOutcome::InconsistentField(format!("pecr_triage_blocker value `{blocker}` is placeholder — must be a real reason"));
        }
        PecrCheckOutcome::TriagePending(blocker)
    } else {
        PecrCheckOutcome::InconsistentField(format!("pecr_triage_completed value `{completed_value}` must be `true` or `false`"))
    }
}

/// Walk a target dir tree (docs/biz-public/marketing/ + docs/biz-public/sales/),
/// returning a vector of (path, outcome) for every .md file.
fn walk_pecr_check(root: &Path) -> Vec<(PathBuf, PecrCheckOutcome)> {
    let mut results = Vec::new();
    let target_subdirs = ["marketing", "sales"];
    for sub in &target_subdirs {
        let dir = root.join("docs").join("biz-public").join(sub);
        if !dir.exists() { continue; }
        for entry in walk(&dir) {
            if entry.extension().map(|e| e == "md").unwrap_or(false) {
                let content = fs::read_to_string(&entry).unwrap();
                results.push((entry, check_pecr_triage(&content)));
            }
        }
    }
    results
}

fn walk(dir: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                out.extend(walk(&path));
            } else {
                out.push(path);
            }
        }
    }
    out
}

// ---------------------------------------------------------------------------
// Test 1 — artifact without direct-marketing reference passes (NotApplicable).
// ---------------------------------------------------------------------------

#[test]
fn pecr_check_skips_non_direct_marketing_artifact() {
    let content = "---\nname: gtm-strategy\ntier: public\n---\n\n# GTM strategy\n\nFocus on community-led distribution. No outbound channels.\n";
    let outcome = check_pecr_triage(content);
    assert!(matches!(outcome, PecrCheckOutcome::NotApplicable), "expected NotApplicable; got {outcome:?}");
}

// ---------------------------------------------------------------------------
// Test 2 — artifact with cold-email reference + completed triage passes.
// ---------------------------------------------------------------------------

#[test]
fn pecr_check_accepts_completed_triage() {
    let content = r#"---
name: marketing-b2b-plan
tier: public
pecr_triage_completed: true
pecr_triage_doc: docs/biz-public/legal/triage-pecr-2026-04-25.md
---

# B2B marketing plan

Channels include cold email outbound.
"#;
    let outcome = check_pecr_triage(content);
    match outcome {
        PecrCheckOutcome::TriageCompleted(doc) => {
            assert!(doc.contains("triage-pecr"), "expected triage doc reference; got {doc}");
        }
        other => panic!("expected TriageCompleted; got {other:?}"),
    }
}

// ---------------------------------------------------------------------------
// Test 3 — artifact with cold-email reference + pending triage flagged.
// ---------------------------------------------------------------------------

#[test]
fn pecr_check_flags_pending_triage_with_blocker() {
    let content = r#"---
name: marketing-b2c-plan
tier: public
pecr_triage_completed: false
pecr_triage_blocker: cold email channel proposed; /slo-legal triage not yet run; channel BLOCKED until triage resolves
---

# B2C marketing plan

Cold email channel for prospect outreach.
"#;
    let outcome = check_pecr_triage(content);
    match outcome {
        PecrCheckOutcome::TriagePending(blocker) => {
            assert!(blocker.contains("BLOCKED"), "expected blocker reason; got {blocker}");
        }
        other => panic!("expected TriagePending; got {other:?}"),
    }
}

// ---------------------------------------------------------------------------
// Test 4 — artifact with cold-email reference + missing triage field fails.
// ---------------------------------------------------------------------------

#[test]
fn pecr_check_fails_missing_triage_field() {
    let content = "---\nname: bad-artifact\ntier: public\n---\n\n# B2B marketing plan\n\nWe will run cold email at scale.\n";
    let outcome = check_pecr_triage(content);
    assert!(matches!(outcome, PecrCheckOutcome::MissingTriageField), "expected MissingTriageField; got {outcome:?}");
}

// ---------------------------------------------------------------------------
// Test 5 — completed triage without doc path is inconsistent.
// ---------------------------------------------------------------------------

#[test]
fn pecr_check_fails_inconsistent_completed_without_doc() {
    let content = "---\nname: bad-artifact\ntier: public\npecr_triage_completed: true\n---\n\n# Marketing plan\n\nCold email.\n";
    let outcome = check_pecr_triage(content);
    match outcome {
        PecrCheckOutcome::InconsistentField(reason) => {
            assert!(reason.contains("pecr_triage_doc"), "expected reason mentions missing doc; got {reason}");
        }
        other => panic!("expected InconsistentField; got {other:?}"),
    }
}

// ---------------------------------------------------------------------------
// Test 6 — pending triage without blocker is inconsistent.
// ---------------------------------------------------------------------------

#[test]
fn pecr_check_fails_inconsistent_pending_without_blocker() {
    let content = "---\nname: bad-artifact\ntier: public\npecr_triage_completed: false\n---\n\n# Marketing plan\n\nCold email.\n";
    let outcome = check_pecr_triage(content);
    match outcome {
        PecrCheckOutcome::InconsistentField(reason) => {
            assert!(reason.contains("blocker"), "expected reason mentions missing blocker; got {reason}");
        }
        other => panic!("expected InconsistentField; got {other:?}"),
    }
}

// ---------------------------------------------------------------------------
// Test 7 — full milestone simulation: tempdir walk classifies multiple artifacts.
// ---------------------------------------------------------------------------

#[test]
fn pecr_check_walks_tempdir_classifies_artifacts() {
    let dir = TempDir::new().unwrap();
    let marketing = dir.path().join("docs/biz-public/marketing");
    let sales = dir.path().join("docs/biz-public/sales");
    fs::create_dir_all(&marketing).unwrap();
    fs::create_dir_all(&sales).unwrap();

    // Artifact 1: no direct marketing → NotApplicable.
    fs::write(marketing.join("brand-guide.md"), "---\nname: brand-guide\ntier: public\n---\n\n# Brand voice\n\nCommunity-focused; no outbound.\n").unwrap();

    // Artifact 2: completed triage → TriageCompleted.
    fs::write(marketing.join("b2b-plan.md"), "---\nname: b2b-plan\ntier: public\npecr_triage_completed: true\npecr_triage_doc: docs/biz-public/legal/triage-pecr.md\n---\n\nCold email outbound.\n").unwrap();

    // Artifact 3: pending triage → TriagePending.
    fs::write(sales.join("funnel-enterprise.md"), "---\nname: funnel-enterprise\ntier: public\npecr_triage_completed: false\npecr_triage_blocker: cold email proposed; triage pending\n---\n\nCold email at scale to enterprise prospects.\n").unwrap();

    // Artifact 4: violator (cold email + missing field).
    fs::write(sales.join("funnel-bad.md"), "---\nname: funnel-bad\ntier: public\n---\n\nCold email channel.\n").unwrap();

    let results = walk_pecr_check(dir.path());
    assert_eq!(results.len(), 4, "should find 4 artifacts; got {}", results.len());

    // Tally by outcome.
    let mut na = 0;
    let mut completed = 0;
    let mut pending = 0;
    let mut missing = 0;
    let mut inconsistent = 0;
    for (_, outcome) in &results {
        match outcome {
            PecrCheckOutcome::NotApplicable => na += 1,
            PecrCheckOutcome::TriageCompleted(_) => completed += 1,
            PecrCheckOutcome::TriagePending(_) => pending += 1,
            PecrCheckOutcome::MissingTriageField => missing += 1,
            PecrCheckOutcome::InconsistentField(_) => inconsistent += 1,
        }
    }
    assert_eq!(na, 1, "expected 1 NotApplicable; got {na}");
    assert_eq!(completed, 1, "expected 1 TriageCompleted; got {completed}");
    assert_eq!(pending, 1, "expected 1 TriagePending; got {pending}");
    assert_eq!(missing, 1, "expected 1 MissingTriageField (the violator); got {missing}");
    assert_eq!(inconsistent, 0, "expected 0 InconsistentField; got {inconsistent}");
}

// ---------------------------------------------------------------------------
// Test 8 — artifact-schema documents the three new PECR frontmatter keys.
// ---------------------------------------------------------------------------

#[test]
fn artifact_schema_documents_pecr_triage_fields() {
    let body = fs::read_to_string(repo_root().join("references/biz/artifact-schema.md")).unwrap();
    let required = ["pecr_triage_completed", "pecr_triage_doc", "pecr_triage_blocker"];
    for key in &required {
        assert!(body.contains(key), "artifact-schema.md must document PECR field `{key}`");
    }
}

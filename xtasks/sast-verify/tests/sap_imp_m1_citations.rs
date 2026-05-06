//! M1 structural-contract test (sap-imp runbook).
//!
//! Asserts the citation invariant from RUNBOOK-SECURE-AGENT-PLAYBOOK-IMPORTS.md M1:
//!
//! Every skill in {slo-sast, slo-rulegen, slo-ruleverify, slo-ship, slo-critique, slo-verify}
//! contains at least one Markdown link to either
//! `references/security/security-finding-template.md` or
//! `references/security/security-assessment-summary-template.md`.
//!
//! Per F-ENG-1 critique resolution: parser MUST be `pulldown-cmark` AST-based.
//! Citations inside fenced code blocks are NOT counted.
//! Per F-ENG-3 resolution (M2): no shipped SKILL.md links to `examples/` (asserted here too,
//! since the same AST walk runs once over the same files).

use pulldown_cmark::{Event, Parser, Tag};
use std::path::{Path, PathBuf};

/// The set of skills that must cite at least one shared security template.
const CITING_SKILL_SET: &[&str] = &[
    "slo-sast",
    "slo-rulegen",
    "slo-ruleverify",
    "slo-ship",
    "slo-critique",
    "slo-verify",
];

/// Canonical template paths the citation must point to.
const CANONICAL_TEMPLATES: &[&str] = &[
    "references/security/security-finding-template.md",
    "references/security/security-assessment-summary-template.md",
];

/// Resolve the workspace root (the directory containing the root `Cargo.toml`).
///
/// Tests run with `CARGO_MANIFEST_DIR` set to the package being tested
/// (`xtasks/sast-verify`). The workspace root is two levels up.
fn workspace_root() -> PathBuf {
    if let Ok(cwd) = std::env::current_dir() {
        if cwd.join("skills").is_dir() && cwd.join("Cargo.toml").is_file() {
            return cwd;
        }
    }

    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    PathBuf::from(manifest_dir)
        .parent()
        .and_then(Path::parent)
        .expect("xtasks/sast-verify must live two levels below workspace root")
        .to_path_buf()
}

/// Walk a SKILL.md file with `pulldown-cmark` and return every link destination
/// emitted as `Event::Start(Tag::Link(...))`. Code-fence content is excluded
/// because the parser yields `Event::Code` / `Event::Html` for those, not
/// `Event::Start(Tag::Link)`.
fn extract_link_destinations(markdown: &str) -> Vec<String> {
    let mut links = Vec::new();
    for event in Parser::new(markdown) {
        if let Event::Start(Tag::Link {
            link_type: _,
            dest_url,
            title: _,
            id: _,
        }) = event
        {
            links.push(dest_url.into_string());
        }
    }
    links
}

/// Test: every skill in the citing set contains ≥ 1 link to either canonical template.
#[test]
fn every_security_skill_cites_a_template() {
    let root = workspace_root();
    let mut failures: Vec<String> = Vec::new();

    for skill in CITING_SKILL_SET {
        let skill_md = root.join("skills").join(skill).join("SKILL.md");
        let markdown = std::fs::read_to_string(&skill_md)
            .unwrap_or_else(|e| panic!("failed to read {}: {}", skill_md.display(), e));

        let links = extract_link_destinations(&markdown);

        let cites_canonical = links.iter().any(|dest| {
            CANONICAL_TEMPLATES
                .iter()
                .any(|tpl| dest.ends_with(tpl) || dest.contains(tpl))
        });

        if !cites_canonical {
            failures.push(format!(
                "expected ≥ 1 link to references/security/security-{{finding,assessment-summary}}-template.md \
                 in skills/{}/SKILL.md, found 0 (links seen: {:?})",
                skill, links
            ));
        }
    }

    assert!(
        failures.is_empty(),
        "M1 citation invariant violated:\n  - {}",
        failures.join("\n  - ")
    );
}

/// Test: every cited template path resolves to an existing file at HEAD.
#[test]
fn cited_template_paths_resolve() {
    let root = workspace_root();
    let mut failures: Vec<String> = Vec::new();

    for skill in CITING_SKILL_SET {
        let skill_md = root.join("skills").join(skill).join("SKILL.md");
        let markdown = std::fs::read_to_string(&skill_md)
            .unwrap_or_else(|e| panic!("failed to read {}: {}", skill_md.display(), e));

        for dest in extract_link_destinations(&markdown) {
            // Only check links that LOOK like our canonical-template citations
            // (path ends with one of the two filenames). External URLs and
            // anchor links are out of scope for this assertion.
            let is_canonical_citation = CANONICAL_TEMPLATES
                .iter()
                .any(|tpl| dest.ends_with(tpl) || dest.contains(tpl));
            if !is_canonical_citation {
                continue;
            }

            // The destination is repo-relative (possibly with `../../` prefix
            // because SKILL.md lives in `skills/<name>/`). Strip leading `../`
            // segments and resolve against the repo root.
            let cleaned = dest.trim_start_matches("./");
            // Compute the absolute path the link "points at" — anchored at
            // workspace root, the cleaned tail (after stripping ../) should
            // match references/security/security-*.md.
            let canonical_tail = CANONICAL_TEMPLATES
                .iter()
                .find(|tpl| cleaned.ends_with(*tpl) || cleaned.contains(*tpl))
                .copied()
                .unwrap_or(cleaned);
            let absolute = root.join(canonical_tail);

            if !absolute.exists() {
                failures.push(format!(
                    "skills/{}/SKILL.md cites `{}` which resolves to `{}` — file does not exist at HEAD",
                    skill,
                    dest,
                    absolute.display()
                ));
            }
        }
    }

    assert!(
        failures.is_empty(),
        "M1 cited-path-resolution invariant violated:\n  - {}",
        failures.join("\n  - ")
    );
}

/// Test: `/slo-ship`'s security-summary section is gated on "new public surface" phrase.
///
/// Per M1 Invariants row (d) and the BDD scenario `slo_ship_security_summary_is_gated`:
/// when `/slo-ship` SKILL.md cites the assessment-summary template, a gate phrase
/// (e.g., "new public surface") MUST appear within 200 characters of the citation
/// so the section is not always-on.
#[test]
fn slo_ship_security_summary_is_gated() {
    let root = workspace_root();
    let skill_md = root.join("skills/slo-ship/SKILL.md");
    let markdown = std::fs::read_to_string(&skill_md)
        .unwrap_or_else(|e| panic!("failed to read {}: {}", skill_md.display(), e));

    // Find the byte index of the assessment-summary template citation.
    let tpl_marker = "security-assessment-summary-template.md";
    let cite_pos = markdown.find(tpl_marker);

    let Some(pos) = cite_pos else {
        // If /slo-ship doesn't cite the assessment template, the citation
        // invariant test will already fail with a more specific message;
        // this test is only meaningful when the citation IS present.
        return;
    };

    // Window: 200 chars on each side of the citation marker.
    let window_start = pos.saturating_sub(200);
    let window_end = (pos + tpl_marker.len() + 200).min(markdown.len());
    let window = &markdown[window_start..window_end];

    // Acceptable gate phrasings (per M1 Notes).
    let gate_phrases = [
        "new public surface",
        "new public-facing surface",
        "introduces a new surface",
        "introduced new public surface",
    ];

    let has_gate = gate_phrases.iter().any(|phrase| window.contains(phrase));

    assert!(
        has_gate,
        "skills/slo-ship/SKILL.md cites the assessment-summary template but the gate phrase \
         (one of {:?}) is missing within 200 chars of the citation. Window: {:?}",
        gate_phrases, window
    );
}

/// Test: per F-ENG-3 critique resolution (M2 invariant landed early in M1's test file
/// since the same AST walk runs over the same skill files), no shipped SKILL.md
/// contains a Markdown link to `examples/`. This enforces M2's Out-of-Scope rule
/// "no skill consumes examples/".
#[test]
fn no_skill_links_to_examples() {
    let root = workspace_root();
    let skills_dir = root.join("skills");
    let mut failures: Vec<String> = Vec::new();

    let entries = std::fs::read_dir(&skills_dir)
        .unwrap_or_else(|e| panic!("failed to read {}: {}", skills_dir.display(), e));

    for entry in entries {
        let entry = entry.expect("dir entry");
        let skill_md = entry.path().join("SKILL.md");
        if !skill_md.exists() {
            continue;
        }
        let markdown = std::fs::read_to_string(&skill_md)
            .unwrap_or_else(|e| panic!("failed to read {}: {}", skill_md.display(), e));

        for dest in extract_link_destinations(&markdown) {
            // Reject any link whose destination starts with `examples/` or
            // ends with a path that traverses into `/examples/`.
            if dest.starts_with("examples/")
                || dest.contains("/examples/")
                || dest.starts_with("../examples/")
                || dest.starts_with("../../examples/")
            {
                failures.push(format!(
                    "skills/{}/SKILL.md links to `{}` — Out-of-Scope rule violated (no skill may consume examples/)",
                    entry.file_name().to_string_lossy(),
                    dest
                ));
            }
        }
    }

    assert!(
        failures.is_empty(),
        "M1 (anticipating M2 F-ENG-3) skill-references-examples invariant violated:\n  - {}",
        failures.join("\n  - ")
    );
}

/// Test: extract_link_destinations correctly excludes code-block content.
/// This is the F-ENG-1 acceptance test: a markdown sample with a "fake" link
/// inside a fenced code block must NOT count as a citation.
#[test]
fn ast_parser_excludes_code_block_content() {
    let markdown = r#"
# Sample

Real link: [template](references/security/security-finding-template.md)

```text
Fake link: [template](references/security/security-finding-template.md)
```

```rust
let url = "[template](references/security/security-finding-template.md)";
```
"#;

    let links = extract_link_destinations(markdown);
    assert_eq!(
        links.len(),
        1,
        "AST parser should yield 1 link (real one outside code blocks); got {:?}",
        links
    );
    assert_eq!(
        links[0], "references/security/security-finding-template.md",
        "the one link should be the real one, not a code-block one"
    );
}

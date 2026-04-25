//! Follow-up M3 — biz-pack-onenda-canonical.
//!
//! The original Runbook A plan was to render the canonical oneNDA body
//! byte-for-byte unmodified into a Markdown artifact. This was fragile:
//! - oneNDA's canonical format is .docx, not Markdown.
//! - A Markdown rendering of the .docx is arguably a CC BY-ND 4.0 derivative
//!   (forbidden).
//! - Automated retrieval of license-protected legal templates is supply-chain-risky.
//!
//! This follow-up replaces the "render verbatim" plan with a "cover-only,
//! .docx-fetched-manually-by-project-owner" flow, hash-pinned via frontmatter
//! field `pinned_canonical_sha256`. Until the project owner runs the manual-
//! fetch procedure (documented in references/biz/templates/onenda-uk.md and
//! in skills/slo-legal/SKILL.md), the field reads `pending-user-fetch` and
//! the placeholder marker `ONENDA-UK-PLACEHOLDER` is required.
//!
//! Once the project owner pins the SHA-256, this test transitions to
//! enforcing hash-format and accepting `ONENDA-UK-CANONICAL-PINNED` marker.

use std::fs;
use std::path::{Path, PathBuf};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).parent().unwrap().parent().unwrap().to_path_buf()
}

fn read(path: &Path) -> String {
    fs::read_to_string(path).unwrap_or_else(|e| panic!("cannot read {}: {e}", path.display()))
}

#[test]
fn onenda_placeholder_documents_pinned_sha256_field() {
    let body = read(&repo_root().join("references/biz/templates/onenda-uk.md"));

    // The frontmatter must declare the `pinned_canonical_sha256:` field.
    assert!(
        body.contains("pinned_canonical_sha256:"),
        "onenda-uk.md must declare a `pinned_canonical_sha256:` frontmatter field (added by follow-up biz-pack-onenda-canonical). Either `pending-user-fetch` or a hex digest is acceptable; absence is not."
    );

    // The value must be either `pending-user-fetch` OR a 64-char hex digest.
    // Find the FRONTMATTER occurrence (inside a yaml code fence) — not the
    // procedure-comment occurrence. Search for the yaml block and parse from
    // there.
    let yaml_start = body
        .find("```yaml\ncanonical_source:")
        .or_else(|| body.find("```yaml\n").map(|i| {
            // Find the first yaml block that contains pinned_canonical_sha256:.
            let after = &body[i..];
            if after.contains("pinned_canonical_sha256:") { i } else { body.len() }
        }))
        .expect("frontmatter yaml block not found");
    let after_yaml_start = &body[yaml_start..];
    let yaml_end = after_yaml_start[8..].find("```").map(|i| i + 8).unwrap_or(after_yaml_start.len());
    let yaml_block = &after_yaml_start[..yaml_end];

    let line = yaml_block
        .lines()
        .find(|l| l.trim_start().starts_with("pinned_canonical_sha256:"))
        .expect("pinned_canonical_sha256 line not found in yaml block");
    let value = line.split("pinned_canonical_sha256:").nth(1).unwrap_or("").trim();
    let is_pending = value == "pending-user-fetch";
    let is_hex = value.len() == 64 && value.chars().all(|c| c.is_ascii_hexdigit());
    assert!(
        is_pending || is_hex,
        "pinned_canonical_sha256 must be `pending-user-fetch` or a 64-char hex SHA-256 digest; got `{value}` from yaml block"
    );
}

#[test]
fn onenda_placeholder_documents_canonical_format_and_url() {
    let body = read(&repo_root().join("references/biz/templates/onenda-uk.md"));

    // Format must be docx (canonical-format awareness is the load-bearing change).
    assert!(
        body.contains("canonical_format: docx"),
        "onenda-uk.md must declare `canonical_format: docx` — the canonical artifact is a .docx file, not a Markdown rendering"
    );

    // Canonical source URL must be onenda.org (project home).
    assert!(
        body.contains("https://www.onenda.org/"),
        "onenda-uk.md must cite https://www.onenda.org/ as the canonical source"
    );

    // The discovered .docx URL must be documented (so the project owner has a
    // starting point for the manual-fetch procedure, even if the URL was
    // tool-discovered and requires verification).
    assert!(
        body.contains("canonical_url_discovered:") || body.contains("storage.googleapis.com"),
        "onenda-uk.md must document a starting-point .docx URL (clearly labelled as discovered / needs-verification)"
    );

    // License obligation must remain documented.
    assert!(
        body.contains("CC BY-ND 4.0"),
        "onenda-uk.md must continue to document the CC BY-ND 4.0 license"
    );
}

#[test]
fn onenda_placeholder_marker_or_canonical_pinned_marker_present() {
    let body = read(&repo_root().join("references/biz/templates/onenda-uk.md"));

    // Until the project owner runs the manual-fetch procedure, the marker
    // ONENDA-UK-PLACEHOLDER must be present. After pinning, the marker
    // becomes ONENDA-UK-CANONICAL-PINNED.
    let placeholder = body.contains("ONENDA-UK-PLACEHOLDER");
    let pinned = body.contains("ONENDA-UK-CANONICAL-PINNED");
    assert!(
        placeholder || pinned,
        "onenda-uk.md must contain ONE OF: `ONENDA-UK-PLACEHOLDER` (pre-fetch state) or `ONENDA-UK-CANONICAL-PINNED` (post-pinning state). Neither is present — the file is in an unknown state."
    );

    // Cross-check: marker state matches the pinned_canonical_sha256 field.
    // Find the frontmatter yaml block (same logic as the field-format test).
    let yaml_start = body.find("```yaml\n").unwrap_or(body.len());
    let after_yaml_start = &body[yaml_start..];
    let yaml_end = after_yaml_start.find("\n```").unwrap_or(after_yaml_start.len());
    let yaml_block = &after_yaml_start[..yaml_end];
    let line = yaml_block
        .lines()
        .find(|l| l.trim_start().starts_with("pinned_canonical_sha256:"))
        .unwrap_or("");
    let value = line.split("pinned_canonical_sha256:").nth(1).unwrap_or("").trim();
    let is_pending = value == "pending-user-fetch";

    if is_pending && !placeholder {
        panic!(
            "Inconsistent state: pinned_canonical_sha256 is `pending-user-fetch` but `ONENDA-UK-PLACEHOLDER` marker is absent. Either the project owner has pinned the hash (then update pinned_canonical_sha256 with the hex digest AND change marker to ONENDA-UK-CANONICAL-PINNED) or the marker was removed prematurely (restore it)."
        );
    }
    if !is_pending && !pinned {
        panic!(
            "Inconsistent state: pinned_canonical_sha256 is set to a hex digest but `ONENDA-UK-CANONICAL-PINNED` marker is absent. After pinning the hash, update the marker to signal canonical state."
        );
    }
}

#[test]
fn slo_legal_skill_md_documents_cover_only_flow() {
    let skill = read(&repo_root().join("skills/slo-legal/SKILL.md"));

    // The cover-only flow must be documented.
    let signals = ["cover-only", "Cover-only", "cover artifact", "Cover artifact"];
    let count = signals.iter().filter(|s| skill.contains(**s)).count();
    assert!(
        count >= 2,
        "slo-legal SKILL.md must document the cover-only flow (found {count} of {signals:?})"
    );

    // The ".docx body fetched manually" framing must be explicit.
    assert!(
        skill.contains(".docx"),
        "slo-legal SKILL.md must reference the .docx canonical format"
    );
    assert!(
        skill.contains("https://www.onenda.org/"),
        "slo-legal SKILL.md must cite onenda.org as the canonical source"
    );

    // Hash-verification step must be documented.
    let hash_signals = ["shasum", "SHA-256", "pinned_canonical_sha256"];
    let any = hash_signals.iter().any(|s| skill.contains(s));
    assert!(any, "slo-legal SKILL.md must document the SHA-256 verification step");
}

#[test]
fn slo_legal_no_longer_inlines_canonical_body() {
    let skill = read(&repo_root().join("skills/slo-legal/SKILL.md"));

    // The skill must NOT promise to render canonical body bytes inline. The
    // updated flow is cover-only with the founder fetching the .docx
    // separately. We approximate the negative check by asserting the skill
    // EXPLICITLY says it never modifies the .docx body.
    let explicit_signals = ["never modifies the .docx", "NEVER modifies the .docx", "skill NEVER modifies", "do not edit the body"];
    let any = explicit_signals.iter().any(|s| skill.contains(s));
    assert!(
        any,
        "slo-legal SKILL.md must explicitly state the skill NEVER modifies the .docx body (CC BY-ND 4.0 verbatim is the load-bearing rule)"
    );
}

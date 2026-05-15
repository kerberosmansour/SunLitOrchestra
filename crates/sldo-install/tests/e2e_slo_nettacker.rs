//! Structural-contract tests for the /slo-nettacker skill.

use std::fs;
use std::path::{Path, PathBuf};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf()
}

fn read(path: impl AsRef<Path>) -> String {
    let path = path.as_ref();
    fs::read_to_string(path).unwrap_or_else(|e| panic!("cannot read {}: {e}", path.display()))
}

fn skill_md() -> String {
    read(repo_root().join("skills/slo-nettacker/SKILL.md"))
}

#[test]
fn nettacker_skill_is_discoverable_and_cataloged() {
    let skill = skill_md();
    assert!(skill.starts_with("---\n"));
    assert!(skill.contains("name: slo-nettacker"));
    assert!(skill.contains("OWASP Nettacker"));
    assert!(skill.contains("custom Nettacker"));

    let catalog = read(repo_root().join("docs/skill-pack-catalog.md"));
    assert!(catalog.contains("Shipped skills at HEAD: 40"));
    assert!(catalog.contains("/slo-nettacker"));
    assert!(catalog.contains("9 power tools"));
}

#[test]
fn nettacker_skill_hard_gates_live_scanning() {
    let skill = skill_md().to_ascii_lowercase();
    for needle in [
        "authorization gate",
        "asset owner",
        "written authorization",
        "in-scope target list",
        "out-of-scope assets",
        "time window",
        "rate/concurrency limits",
        "credential-testing permission",
        "refuse live scan",
    ] {
        assert!(skill.contains(needle), "SKILL.md missing gate `{needle}`");
    }
}

#[test]
fn nettacker_skill_documents_safe_auto_mode() {
    let skill = skill_md();
    for needle in [
        "## Auto Mode",
        "No `-m all`",
        "No brute/default-credential modules",
        "URL-probe modules",
        "request-volume",
        "--retries 1",
        "modest `-t` and `-M`",
        "JSON/CSV evidence",
    ] {
        assert!(
            skill.contains(needle),
            "SKILL.md missing auto-mode rule `{needle}`"
        );
    }
}

#[test]
fn nettacker_skill_guards_confidential_assessment_outputs() {
    let skill = skill_md();
    for needle in [
        "Before writing assessment artifacts under `.sldo/nettacker/`",
        "git check-ignore",
        "git ls-files .sldo/nettacker",
        "git status --porcelain -- .sldo/nettacker",
        "refuse to write assessment artifacts",
        "repo has a remote",
    ] {
        assert!(
            skill.contains(needle),
            "SKILL.md missing confidential-output guard `{needle}`"
        );
    }
}

#[test]
fn nettacker_skill_resolves_install_location_before_commands() {
    let skill = skill_md();
    let location = read(repo_root().join("skills/slo-nettacker/references/nettacker-location.md"));

    for needle in [
        "references/nettacker-location.md",
        "Do not assume the tool is in the current repo or on `PATH`",
        "Record the resolved runner form",
        "If no runner is found",
    ] {
        assert!(
            skill.contains(needle),
            "SKILL.md missing locator rule `{needle}`"
        );
    }

    for needle in [
        "[tool.poetry.scripts]",
        "nettacker = \"nettacker.main:run\"",
        "pipx install nettacker",
        "pip3 install nettacker",
        "python3 nettacker.py --help",
        "poetry run nettacker --help",
        "owasp/nettacker",
        "command -v nettacker",
        "kind: checkout | path-cli | poetry | docker | api",
        "PATH-only or Docker-only install",
    ] {
        assert!(
            location.contains(needle),
            "location reference missing `{needle}`"
        );
    }
}

#[test]
fn nettacker_references_cover_assessment_and_module_authoring() {
    let root = repo_root();
    let assessment = read(root.join("skills/slo-nettacker/references/assessment-workflow.md"));
    let authoring = read(root.join("skills/slo-nettacker/references/custom-module-authoring.md"));

    for needle in [
        "subdomain_scan",
        "port_scan",
        "http_status_scan",
        "http_html_title_scan",
        "web_technologies_scan",
        "server_version_vuln",
        "x_powered_by_vuln",
        "pqc_scan",
        "Credential testing is separate permission",
        "Continuous Monitoring",
        "nettacker-location.md",
    ] {
        assert!(
            assessment.contains(needle),
            "assessment reference missing `{needle}`"
        );
    }

    for needle in [
        "nettacker/modules/scan/",
        "nettacker/modules/vuln/",
        "nettacker/modules/brute/",
        "split on the final underscore",
        "tests/test_yaml_schema_and_regex.py",
        "Non-destructive and idempotent",
        "positive test proves detection",
        "negative test proves it stays quiet",
        "PATH-only and Docker-only installs",
    ] {
        assert!(
            authoring.contains(needle),
            "authoring reference missing `{needle}`"
        );
    }
}

#[test]
fn nettacker_assessment_workflow_encodes_lab_run_hardening() {
    let assessment =
        read(repo_root().join("skills/slo-nettacker/references/assessment-workflow.md"));

    for needle in [
        "## 1.5 Baseline And Wildcard Detection",
        "curl -sI <target>",
        "curl -sI -X OPTIONS <target>",
        "body length, body hash, and title",
        "SPA, wildcard route, soft-404, or catch-all",
        "Observed Noisy Modes",
        "waf_scan",
        "dir_scan`, `admin_scan`, `pma_scan",
        "Header no-hit cross-check",
        "A no-hit from a Nettacker header module is not proof",
        "Use `-d/--skip-service-discovery` as a diagnostic fallback",
        "## 7. Teardown",
        "do not auto-run",
    ] {
        assert!(
            assessment.contains(needle),
            "assessment workflow missing hardening text `{needle}`"
        );
    }
}

#[test]
fn nettacker_location_records_docker_platform_metadata() {
    let location = read(repo_root().join("skills/slo-nettacker/references/nettacker-location.md"));

    for needle in [
        "docker image inspect owasp/nettacker --format '{{.Architecture}}'",
        "host architecture",
        "image architecture",
        "emulation risk",
        "image architecture: <architecture>",
        "host architecture: <architecture>",
    ] {
        assert!(
            location.contains(needle),
            "location reference missing Docker platform metadata `{needle}`"
        );
    }
}

#[test]
fn nettacker_skill_is_marked_high_risk_with_evals() {
    let tests = read(repo_root().join("crates/sldo-install/tests/e2e_eng_imp_m5.rs"));
    assert!(
        tests.contains("\"slo-nettacker\""),
        "slo-nettacker must be in the high-risk eval list"
    );

    for case in [
        "happy-path",
        "missing-context",
        "ambiguous-input",
        "adversarial",
        "outdated-information",
        "tool-failure",
        "high-risk-case",
    ] {
        let path = repo_root()
            .join("skills/slo-nettacker/evals")
            .join(format!("{case}.md"));
        assert!(path.is_file(), "missing eval case {}", path.display());
        let body = read(&path);
        assert!(body.contains("skill: slo-nettacker"));
        assert!(body.contains("## Input"));
        assert!(body.contains("## Expected Behavior"));
        assert!(body.contains("## Must Not"));
    }
}

//! M5 structural-contract tests for the engineering skill-improvements runbook.
//!
//! M5 seeds documented eval expectations for the high-risk skills, adds the
//! opt-in project freeze hook, and tightens several cross-skill polish items.

use std::fs;
use std::path::{Path, PathBuf};

const HIGH_RISK_SKILLS: &[&str] = &[
    "slo-legal",
    "slo-accounting",
    "slo-equity",
    "slo-fundraise",
    "slo-hire",
    "slo-sast",
    "slo-tla",
    "slo-execute",
    "slo-verify",
    "slo-rulegen",
    "slo-ruleverify",
    "slo-research",
    "slo-architect",
    "slo-plan",
    "slo-talk-to-users",
    "slo-founder-check",
];

const FRONTMATTER_FIELDS: &[&str] = &["skill", "case-name", "category", "expected-behavior"];

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

fn skill_md(skill: &str) -> String {
    read(repo_root().join("skills").join(skill).join("SKILL.md"))
}

fn assert_valid_json(input: &str) {
    let mut parser = JsonParser::new(input);
    parser.parse_value();
    parser.skip_ws();
    assert_eq!(
        parser.pos,
        parser.bytes.len(),
        "settings.json has trailing data"
    );
}

struct JsonParser<'a> {
    bytes: &'a [u8],
    pos: usize,
}

impl<'a> JsonParser<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            bytes: input.as_bytes(),
            pos: 0,
        }
    }

    fn parse_value(&mut self) {
        self.skip_ws();
        match self.peek() {
            Some(b'{') => self.parse_object(),
            Some(b'[') => self.parse_array(),
            Some(b'"') => self.parse_string(),
            Some(b't') => self.expect_literal(b"true"),
            Some(b'f') => self.expect_literal(b"false"),
            Some(b'n') => self.expect_literal(b"null"),
            Some(b'-' | b'0'..=b'9') => self.parse_number(),
            other => panic!("invalid JSON value at byte {}: {other:?}", self.pos),
        }
    }

    fn parse_object(&mut self) {
        self.expect(b'{');
        self.skip_ws();
        if self.consume(b'}') {
            return;
        }

        loop {
            self.skip_ws();
            self.parse_string();
            self.skip_ws();
            self.expect(b':');
            self.parse_value();
            self.skip_ws();

            if self.consume(b'}') {
                break;
            }
            self.expect(b',');
        }
    }

    fn parse_array(&mut self) {
        self.expect(b'[');
        self.skip_ws();
        if self.consume(b']') {
            return;
        }

        loop {
            self.parse_value();
            self.skip_ws();

            if self.consume(b']') {
                break;
            }
            self.expect(b',');
        }
    }

    fn parse_string(&mut self) {
        self.expect(b'"');
        while let Some(byte) = self.next() {
            match byte {
                b'"' => return,
                b'\\' => match self.next() {
                    Some(b'"' | b'\\' | b'/' | b'b' | b'f' | b'n' | b'r' | b't') => {}
                    Some(b'u') => {
                        for _ in 0..4 {
                            assert!(
                                self.next().is_some_and(|b| b.is_ascii_hexdigit()),
                                "invalid unicode escape at byte {}",
                                self.pos
                            );
                        }
                    }
                    other => panic!("invalid escape at byte {}: {other:?}", self.pos),
                },
                0x00..=0x1f => panic!("control character in string at byte {}", self.pos),
                _ => {}
            }
        }

        panic!("unterminated string");
    }

    fn parse_number(&mut self) {
        self.consume(b'-');
        match self.peek() {
            Some(b'0') => {
                self.pos += 1;
            }
            Some(b'1'..=b'9') => {
                self.pos += 1;
                while self.peek().is_some_and(|b| b.is_ascii_digit()) {
                    self.pos += 1;
                }
            }
            other => panic!("invalid number at byte {}: {other:?}", self.pos),
        }

        if self.consume(b'.') {
            assert!(
                self.peek().is_some_and(|b| b.is_ascii_digit()),
                "fraction requires a digit at byte {}",
                self.pos
            );
            while self.peek().is_some_and(|b| b.is_ascii_digit()) {
                self.pos += 1;
            }
        }

        if self.consume(b'e') || self.consume(b'E') {
            let _ = self.consume(b'+') || self.consume(b'-');
            assert!(
                self.peek().is_some_and(|b| b.is_ascii_digit()),
                "exponent requires a digit at byte {}",
                self.pos
            );
            while self.peek().is_some_and(|b| b.is_ascii_digit()) {
                self.pos += 1;
            }
        }
    }

    fn expect_literal(&mut self, literal: &[u8]) {
        for expected in literal {
            self.expect(*expected);
        }
    }

    fn skip_ws(&mut self) {
        while self
            .peek()
            .is_some_and(|b| matches!(b, b' ' | b'\n' | b'\r' | b'\t'))
        {
            self.pos += 1;
        }
    }

    fn expect(&mut self, expected: u8) {
        let actual = self.next();
        assert_eq!(
            actual,
            Some(expected),
            "expected byte {:?} at {}, saw {actual:?}",
            expected as char,
            self.pos
        );
    }

    fn consume(&mut self, expected: u8) -> bool {
        if self.peek() == Some(expected) {
            self.pos += 1;
            true
        } else {
            false
        }
    }

    fn next(&mut self) -> Option<u8> {
        let byte = self.peek()?;
        self.pos += 1;
        Some(byte)
    }

    fn peek(&self) -> Option<u8> {
        self.bytes.get(self.pos).copied()
    }
}

#[test]
fn every_high_risk_skill_has_evals_dir() {
    for skill in HIGH_RISK_SKILLS {
        let evals = repo_root().join("skills").join(skill).join("evals");
        assert!(evals.is_dir(), "{skill} missing evals/ directory");

        let markdown_count = fs::read_dir(&evals)
            .unwrap_or_else(|e| panic!("cannot read {}: {e}", evals.display()))
            .filter_map(Result::ok)
            .filter(|entry| entry.path().extension().and_then(|e| e.to_str()) == Some("md"))
            .count();
        assert!(
            markdown_count >= 1,
            "{skill} evals/ must contain at least one markdown case"
        );
    }
}

#[test]
fn eval_files_have_required_frontmatter() {
    for skill in HIGH_RISK_SKILLS {
        let evals = repo_root().join("skills").join(skill).join("evals");
        let entries =
            fs::read_dir(&evals).unwrap_or_else(|e| panic!("cannot read {}: {e}", evals.display()));

        for entry in entries {
            let path = entry.expect("cannot read eval entry").path();
            if path.extension().and_then(|e| e.to_str()) != Some("md") {
                continue;
            }

            let body = read(&path);
            assert!(
                body.starts_with("---\n"),
                "{} must start with frontmatter",
                path.display()
            );
            let Some(rest) = body.strip_prefix("---\n") else {
                unreachable!();
            };
            let Some((frontmatter, markdown)) = rest.split_once("\n---\n") else {
                panic!("{} missing closing frontmatter delimiter", path.display());
            };

            for field in FRONTMATTER_FIELDS {
                assert!(
                    frontmatter
                        .lines()
                        .any(|line| line.starts_with(&format!("{field}:"))),
                    "{} missing frontmatter field `{field}`",
                    path.display()
                );
            }
            assert!(
                frontmatter.contains(&format!("skill: {skill}")),
                "{} frontmatter skill must match directory",
                path.display()
            );
            assert!(
                markdown.contains("## Input")
                    && markdown.contains("## Expected Behavior")
                    && markdown.contains("## Must Not"),
                "{} must use the shared eval body shape",
                path.display()
            );
        }
    }
}

#[test]
fn claude_settings_pretooluse_hook_present() {
    let settings = read(repo_root().join(".claude/settings.json"));
    assert_valid_json(&settings);

    assert!(settings.contains("\"PreToolUse\""));
    assert!(settings.contains("Edit"));
    assert!(settings.contains("Write"));
    assert!(settings.contains("NotebookEdit"));
    assert!(settings.contains("~/.sldo/freeze-scope.txt"));
    assert!(settings.contains("freeze: cannot edit"));

    let lowered = settings.to_ascii_lowercase();
    for forbidden in ["bash -c", "sh -c", "eval"] {
        assert!(
            !lowered.contains(forbidden),
            "hook must not use shell-string pattern `{forbidden}`"
        );
    }
}

#[test]
fn freeze_hook_setup_doc_exists() {
    let path = repo_root().join("references/freeze/hook-setup.md");
    let body = read(&path);

    assert!(body.starts_with("---\n"));
    for needle in [
        ".claude/settings.json",
        "PreToolUse",
        "opt-in",
        "update-config",
        "additive",
        "~/.sldo/freeze-scope.txt",
        "not a security boundary",
    ] {
        assert!(body.contains(needle), "hook setup missing `{needle}`");
    }
}

#[test]
fn cross_skill_polish_landed() {
    let freeze = skill_md("slo-freeze");
    assert!(freeze.contains("not a security boundary"));
    assert!(freeze.contains("references/freeze/hook-setup.md"));
    assert!(freeze.contains("missing `~/.sldo/freeze-scope.txt`"));

    let second_opinion = skill_md("slo-second-opinion");
    assert!(second_opinion.contains("neither response is verified"));
    assert!(second_opinion.contains("Minimum CLI versions"));

    let get_api_docs = skill_md("get-api-docs");
    assert!(get_api_docs.contains("do NOT fall back to training memory"));
    assert!(get_api_docs.contains("If `chub get` fails"));
    assert!(get_api_docs.contains("If `chub search` returns nothing"));

    let research = skill_md("slo-research");
    assert!(research.contains("sldo-research --help"));
    assert!(research.contains("references/templates/tool-safety-section.md"));

    let talk_to_users = skill_md("slo-talk-to-users");
    assert!(talk_to_users.contains("git rev-parse --git-dir"));
    assert!(talk_to_users.contains("git remote -v"));
    assert!(talk_to_users.contains("references/biz/consent-script-uk.md"));

    let verify = skill_md("slo-verify");
    assert!(verify.contains("Capitalised-bigram"));
    assert!(verify.contains("tier_override_reason"));
    assert!(verify.contains("pass/fail/skipped/N/A"));
}

#[test]
fn consent_script_exists_with_uk_gdpr_framing() {
    let body = read(repo_root().join("references/biz/consent-script-uk.md"));
    assert!(body.starts_with("---\n"));
    for needle in [
        "UK GDPR",
        "legitimate interest",
        "recording",
        "withdraw",
        "confidential",
    ] {
        assert!(body.contains(needle), "consent script missing `{needle}`");
    }
}

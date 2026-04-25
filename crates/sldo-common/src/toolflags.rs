//! Tool-permission flag definitions for Claude Code CLI invocations.

/// Allow flags for `sldo-plan`.
pub fn plan_allow_flags() -> Vec<String> {
    vec!["--allowedTools=Read,Write,Edit,Bash,Glob,Grep".to_string()]
}

/// Deny flags for `sldo-plan`.
pub fn plan_deny_flags() -> Vec<String> {
    vec![]
}

/// Allow flags for `sldo-run`.
pub fn run_allow_flags() -> Vec<String> {
    vec!["--allowedTools=Read,Write,Edit,Bash,Glob,Grep,WebFetch".to_string()]
}

/// Deny flags for `sldo-run`.
pub fn run_deny_flags() -> Vec<String> {
    vec![]
}

/// Allow flags for `sldo-research`.
pub fn research_allow_flags() -> Vec<String> {
    vec!["--allowedTools=Read,Write,Edit,Bash,Glob,Grep,WebFetch,WebSearch".to_string()]
}

/// Deny flags for `sldo-research`.
pub fn research_deny_flags() -> Vec<String> {
    vec![]
}

/// Allow flags for `/slo-rulegen` (bootstrap and extend modes).
///
/// Per SECURITY.md "SAST rule-gen skill pack — additional rules": this list
/// EXPLICITLY excludes `WebFetch` and `WebSearch`. The CWE map is pre-baked in
/// `references/sast/`; rule generation does not need network access.
/// The denial is the primary control against prompt-injection-via-bug-summary
/// (threat-model row tm-sast-rulegen-skill-pack-abuse-1).
///
/// `Bash` is allowed so the skill can shell out to `cargo xtask sast-verify gate`.
pub fn rulegen_allow_flags() -> Vec<String> {
    vec!["--allowedTools=Read,Write,Edit,Bash,Glob,Grep".to_string()]
}

/// Deny flags for `/slo-rulegen` — defense-in-depth.
///
/// Some Claude Code versions check both lists; explicitly listing the denials
/// here ensures WebFetch/WebSearch are blocked even if the allow-list is
/// edited in error.
pub fn rulegen_deny_flags() -> Vec<String> {
    vec!["--disallowedTools=WebFetch,WebSearch".to_string()]
}

/// Allow flags for `/slo-ruleverify` — read-only verifier.
///
/// Excludes `Write` and `Edit` (verify is read-only), `WebFetch`, and `WebSearch`.
/// Only `Bash` (for `cargo xtask sast-verify gate`), `Read`, `Glob`, `Grep`.
pub fn ruleverify_allow_flags() -> Vec<String> {
    vec!["--allowedTools=Read,Bash,Glob,Grep".to_string()]
}

/// Deny flags for `/slo-ruleverify` — defense-in-depth.
pub fn ruleverify_deny_flags() -> Vec<String> {
    vec!["--disallowedTools=Write,Edit,WebFetch,WebSearch".to_string()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plan_allow_flags_contains_write() {
        let flags = plan_allow_flags();
        assert!(flags.iter().any(|f| f.contains("Write")));
    }

    #[test]
    fn plan_allow_flags_contains_read() {
        let flags = plan_allow_flags();
        assert!(flags.iter().any(|f| f.contains("Read")));
    }

    #[test]
    fn plan_deny_flags_is_empty() {
        let flags = plan_deny_flags();
        assert!(flags.is_empty());
    }

    #[test]
    fn run_allow_flags_contains_bash() {
        let flags = run_allow_flags();
        assert!(flags.iter().any(|f| f.contains("Bash")));
    }

    #[test]
    fn run_allow_flags_contains_write() {
        let flags = run_allow_flags();
        assert!(flags.iter().any(|f| f.contains("Write")));
    }

    #[test]
    fn run_deny_flags_is_empty() {
        let flags = run_deny_flags();
        assert!(flags.is_empty());
    }

    #[test]
    fn research_allow_flags_contains_web_search() {
        let flags = research_allow_flags();
        assert!(flags.iter().any(|f| f.contains("WebSearch")));
    }

    #[test]
    fn research_allow_flags_contains_web_fetch() {
        let flags = research_allow_flags();
        assert!(flags.iter().any(|f| f.contains("WebFetch")));
    }

    #[test]
    fn research_deny_flags_is_empty() {
        let flags = research_deny_flags();
        assert!(flags.is_empty());
    }

    // /slo-rulegen toolflags — primary control for tm-sast-rulegen-skill-pack-abuse-1.

    #[test]
    fn rulegen_allow_flags_excludes_webfetch() {
        let flags = rulegen_allow_flags();
        assert!(
            !flags.iter().any(|f| f.contains("WebFetch")),
            "rulegen allow flags must NOT include WebFetch (prompt-injection mitigation)"
        );
    }

    #[test]
    fn rulegen_allow_flags_excludes_websearch() {
        let flags = rulegen_allow_flags();
        assert!(
            !flags.iter().any(|f| f.contains("WebSearch")),
            "rulegen allow flags must NOT include WebSearch (prompt-injection mitigation)"
        );
    }

    #[test]
    fn rulegen_allow_flags_includes_bash_for_xtask_shellout() {
        let flags = rulegen_allow_flags();
        assert!(flags.iter().any(|f| f.contains("Bash")));
    }

    #[test]
    fn rulegen_deny_flags_explicitly_lists_webfetch_and_websearch() {
        let flags = rulegen_deny_flags();
        let combined = flags.join(",");
        assert!(combined.contains("WebFetch"));
        assert!(combined.contains("WebSearch"));
    }

    // /slo-ruleverify toolflags — read-only.

    #[test]
    fn ruleverify_allow_flags_excludes_write_and_edit() {
        let flags = ruleverify_allow_flags();
        assert!(
            !flags.iter().any(|f| f.contains("Write")),
            "ruleverify allow flags must NOT include Write (read-only verifier)"
        );
        assert!(
            !flags.iter().any(|f| f.contains("Edit")),
            "ruleverify allow flags must NOT include Edit (read-only verifier)"
        );
    }

    #[test]
    fn ruleverify_allow_flags_excludes_webfetch_and_websearch() {
        let flags = ruleverify_allow_flags();
        assert!(!flags.iter().any(|f| f.contains("WebFetch")));
        assert!(!flags.iter().any(|f| f.contains("WebSearch")));
    }

    #[test]
    fn ruleverify_deny_flags_lists_write_edit_webfetch_websearch() {
        let flags = ruleverify_deny_flags();
        let combined = flags.join(",");
        assert!(combined.contains("Write"));
        assert!(combined.contains("Edit"));
        assert!(combined.contains("WebFetch"));
        assert!(combined.contains("WebSearch"));
    }
}

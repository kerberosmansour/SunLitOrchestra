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
}

use clap::ValueEnum;

#[derive(Copy, Clone, Debug, Eq, PartialEq, ValueEnum)]
pub enum Host {
    ClaudeCode,
    GithubCopilot,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct HostDescriptor {
    pub id: &'static str,
    pub display_name: &'static str,
    pub config_dir: &'static str,
}

const CLAUDE_CODE: HostDescriptor = HostDescriptor {
    id: "claude-code",
    display_name: "Claude Code",
    config_dir: ".claude",
};

const GITHUB_COPILOT: HostDescriptor = HostDescriptor {
    id: "github-copilot",
    display_name: "GitHub Copilot",
    config_dir: ".copilot",
};

impl Host {
    pub fn descriptor(self) -> &'static HostDescriptor {
        match self {
            Self::ClaudeCode => &CLAUDE_CODE,
            Self::GithubCopilot => &GITHUB_COPILOT,
        }
    }

    pub fn id(self) -> &'static str {
        self.descriptor().id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn claude_code_descriptor_matches_expected_paths() {
        let descriptor = Host::ClaudeCode.descriptor();
        assert_eq!(descriptor.id, "claude-code");
        assert_eq!(descriptor.config_dir, ".claude");
    }

    #[test]
    fn github_copilot_descriptor_matches_expected_paths() {
        let descriptor = Host::GithubCopilot.descriptor();
        assert_eq!(descriptor.id, "github-copilot");
        assert_eq!(descriptor.config_dir, ".copilot");
    }
}

//! Tool-permission flag definitions for Copilot CLI invocations.
//!
//! Mirrors the ALLOW_FLAGS and DENY_FLAGS arrays from the Bash scripts.

/// Allow flags for `plan-milestones.sh` / `sldo-plan`.
pub fn plan_allow_flags() -> Vec<String> {
    vec![
        "--allow-tool=write",
        // File reading / searching
        "--allow-tool=shell(cat:*)",
        "--allow-tool=shell(ls:*)",
        "--allow-tool=shell(find:*)",
        "--allow-tool=shell(head:*)",
        "--allow-tool=shell(tail:*)",
        "--allow-tool=shell(grep:*)",
        "--allow-tool=shell(rg:*)",
        "--allow-tool=shell(ag:*)",
        // Text processing
        "--allow-tool=shell(echo:*)",
        "--allow-tool=shell(printf:*)",
        "--allow-tool=shell(sed:*)",
        "--allow-tool=shell(awk:*)",
        "--allow-tool=shell(wc:*)",
        "--allow-tool=shell(sort:*)",
        "--allow-tool=shell(uniq:*)",
        "--allow-tool=shell(cut:*)",
        "--allow-tool=shell(tr:*)",
        "--allow-tool=shell(diff:*)",
        "--allow-tool=shell(tee:*)",
        "--allow-tool=shell(xargs:*)",
        "--allow-tool=shell(basename:*)",
        "--allow-tool=shell(dirname:*)",
        "--allow-tool=shell(realpath:*)",
        // Misc utilities
        "--allow-tool=shell(which:*)",
        "--allow-tool=shell(env:*)",
        "--allow-tool=shell(test:*)",
        "--allow-tool=shell(true:*)",
        "--allow-tool=shell(false:*)",
        "--allow-tool=shell(cd:*)",
        "--allow-tool=shell(pwd:*)",
        "--allow-tool=shell(python:*)",
        "--allow-tool=shell(python3:*)",
        "--allow-tool=shell(tree:*)",
        // Version control (read-only operations)
        "--allow-tool=shell(git:*)",
        // Build / package managers (for discovery only)
        "--allow-tool=shell(cargo:*)",
        "--allow-tool=shell(rustc:*)",
        "--allow-tool=shell(node:*)",
        "--allow-tool=shell(npm:*)",
        "--allow-tool=shell(npx:*)",
        "--allow-tool=shell(pnpm:*)",
        "--allow-tool=shell(yarn:*)",
        "--allow-tool=shell(tsc:*)",
        "--allow-tool=shell(pip:*)",
        "--allow-tool=shell(pip3:*)",
        "--allow-tool=shell(go:*)",
        "--allow-tool=shell(make:*)",
        "--allow-tool=shell(cmake:*)",
        "--allow-tool=shell(mkdir:*)",
    ]
    .into_iter()
    .map(String::from)
    .collect()
}

/// Deny flags for `plan-milestones.sh` / `sldo-plan`.
pub fn plan_deny_flags() -> Vec<String> {
    vec![
        "--deny-tool=shell(rm -rf /)",
        "--deny-tool=shell(git push --force)",
        "--deny-tool=shell(git push -f)",
        "--deny-tool=shell(git reset --hard)",
        "--deny-tool=shell(git clean -fd)",
        "--deny-tool=shell(rm -rf:*)",
    ]
    .into_iter()
    .map(String::from)
    .collect()
}

/// Allow flags for `run-milestones.sh` / `sldo-run`.
pub fn run_allow_flags() -> Vec<String> {
    vec![
        "--allow-tool=write",
        // Build / test toolchain
        "--allow-tool=shell(cargo:*)",
        "--allow-tool=shell(rustc:*)",
        "--allow-tool=shell(rustup:*)",
        "--allow-tool=shell(rustfmt:*)",
        // Version control
        "--allow-tool=shell(git:*)",
        // File reading / searching
        "--allow-tool=shell(cat:*)",
        "--allow-tool=shell(ls:*)",
        "--allow-tool=shell(find:*)",
        "--allow-tool=shell(head:*)",
        "--allow-tool=shell(tail:*)",
        "--allow-tool=shell(grep:*)",
        "--allow-tool=shell(rg:*)",
        "--allow-tool=shell(ag:*)",
        // File manipulation
        "--allow-tool=shell(mkdir:*)",
        "--allow-tool=shell(cp:*)",
        "--allow-tool=shell(mv:*)",
        "--allow-tool=shell(touch:*)",
        "--allow-tool=shell(rm:*)",
        "--allow-tool=shell(chmod:*)",
        "--allow-tool=shell(ln:*)",
        // Text processing
        "--allow-tool=shell(echo:*)",
        "--allow-tool=shell(printf:*)",
        "--allow-tool=shell(sed:*)",
        "--allow-tool=shell(awk:*)",
        "--allow-tool=shell(wc:*)",
        "--allow-tool=shell(sort:*)",
        "--allow-tool=shell(uniq:*)",
        "--allow-tool=shell(cut:*)",
        "--allow-tool=shell(tr:*)",
        "--allow-tool=shell(diff:*)",
        "--allow-tool=shell(tee:*)",
        "--allow-tool=shell(xargs:*)",
        "--allow-tool=shell(basename:*)",
        "--allow-tool=shell(dirname:*)",
        "--allow-tool=shell(realpath:*)",
        // Misc utilities
        "--allow-tool=shell(which:*)",
        "--allow-tool=shell(env:*)",
        "--allow-tool=shell(test:*)",
        "--allow-tool=shell(true:*)",
        "--allow-tool=shell(false:*)",
        "--allow-tool=shell(cd:*)",
        "--allow-tool=shell(pwd:*)",
        "--allow-tool=shell(python:*)",
        "--allow-tool=shell(python3:*)",
        // Node.js / frontend toolchain
        "--allow-tool=shell(node:*)",
        "--allow-tool=shell(npm:*)",
        "--allow-tool=shell(npx:*)",
        "--allow-tool=shell(pnpm:*)",
        "--allow-tool=shell(yarn:*)",
        "--allow-tool=shell(tsc:*)",
        "--allow-tool=shell(vite:*)",
    ]
    .into_iter()
    .map(String::from)
    .collect()
}

/// Deny flags for `run-milestones.sh` / `sldo-run`.
pub fn run_deny_flags() -> Vec<String> {
    vec![
        "--deny-tool=shell(rm -rf /)",
        "--deny-tool=shell(git push --force)",
        "--deny-tool=shell(git push -f)",
        "--deny-tool=shell(git reset --hard)",
        "--deny-tool=shell(git clean -fd)",
    ]
    .into_iter()
    .map(String::from)
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plan_allow_flags_contains_write() {
        // Given: N/A
        // When: plan_allow_flags() is called
        let flags = plan_allow_flags();
        // Then: Contains "--allow-tool=write"
        assert!(flags.contains(&"--allow-tool=write".to_string()));
    }

    #[test]
    fn plan_allow_flags_contains_shell_cat() {
        // Given: N/A
        // When: plan_allow_flags() is called
        let flags = plan_allow_flags();
        // Then: Contains "--allow-tool=shell(cat:*)"
        assert!(flags.contains(&"--allow-tool=shell(cat:*)".to_string()));
    }

    #[test]
    fn plan_deny_flags_contains_rm_rf_root() {
        // Given: N/A
        // When: plan_deny_flags() is called
        let flags = plan_deny_flags();
        // Then: Contains "--deny-tool=shell(rm -rf /)"
        assert!(flags.contains(&"--deny-tool=shell(rm -rf /)".to_string()));
    }

    #[test]
    fn run_allow_flags_contains_cargo() {
        // Given: N/A
        // When: run_allow_flags() is called
        let flags = run_allow_flags();
        // Then: Contains "--allow-tool=shell(cargo:*)"
        assert!(flags.contains(&"--allow-tool=shell(cargo:*)".to_string()));
    }

    #[test]
    fn run_allow_flags_contains_rm() {
        // Given: N/A
        // When: run_allow_flags() is called
        let flags = run_allow_flags();
        // Then: Contains "--allow-tool=shell(rm:*)"
        assert!(flags.contains(&"--allow-tool=shell(rm:*)".to_string()));
    }

    #[test]
    fn run_deny_flags_contains_force_push() {
        // Given: N/A
        // When: run_deny_flags() is called
        let flags = run_deny_flags();
        // Then: Contains "--deny-tool=shell(git push --force)"
        assert!(flags.contains(&"--deny-tool=shell(git push --force)".to_string()));
    }
}

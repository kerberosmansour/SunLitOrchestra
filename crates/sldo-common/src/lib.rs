//! sldo-common — shared library for SunLitOrchestrate Rust tools.

pub mod color;
pub mod copilot;
pub mod detect;
pub mod git;
pub mod logging;
pub mod preflight;
pub mod runbook;
pub mod toolflags;

/// Returns the crate version string from Cargo.toml.
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_returns_non_empty_string() {
        // Given: the sldo-common crate is compiled
        // When: version() is called
        let v = version();
        // Then: it returns a non-empty string
        assert!(!v.is_empty(), "version() must return a non-empty string");
    }

    #[test]
    fn version_is_valid_semver() {
        // Given: the sldo-common crate version is defined in Cargo.toml
        // When: version() is called
        let v = version();
        // Then: it looks like a semver string (contains at least one dot)
        assert!(
            v.contains('.'),
            "version '{}' should be a semver string",
            v
        );
    }
}

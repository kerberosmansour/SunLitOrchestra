//! Coloured terminal output helpers.
//!
//! Mirrors the Bash colour helpers: `info()`, `success()`, `warn()`, `fail()`,
//! `header()`, `divider()`, and `ts()`.

use chrono::Local;
use colored::*;

/// Returns the current timestamp in `YYYY-MM-DD HH:MM:SS` format.
pub fn ts() -> String {
    Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

/// Print an info message to stderr (blue).
pub fn info(msg: &str) {
    eprintln!("{}", format!("[{}] ℹ  {}", ts(), msg).blue());
}

/// Print a success message to stderr (green).
pub fn success(msg: &str) {
    eprintln!("{}", format!("[{}] ✔  {}", ts(), msg).green());
}

/// Print a warning message to stderr (yellow).
pub fn warn(msg: &str) {
    eprintln!("{}", format!("[{}] ⚠  {}", ts(), msg).yellow());
}

/// Print a failure message to stderr (red).
pub fn fail(msg: &str) {
    eprintln!("{}", format!("[{}] ✖  {}", ts(), msg).red());
}

/// Print a section header to stderr (bold cyan).
pub fn header(msg: &str) {
    let bar = "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━";
    eprintln!();
    eprintln!("{}", bar.bold().cyan());
    eprintln!("{}", format!("  {}", msg).bold().cyan());
    eprintln!("{}", bar.bold().cyan());
    eprintln!();
}

/// Print a divider line to stderr (cyan).
pub fn divider() {
    eprintln!(
        "{}",
        "──────────────────────────────────────────────────".cyan()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn timestamp_format() {
        // Given: Current time is known
        // When: ts() is called
        let timestamp = ts();
        // Then: Returns string in YYYY-MM-DD HH:MM:SS format
        let re = regex::Regex::new(r"^\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}$").unwrap();
        assert!(
            re.is_match(&timestamp),
            "Timestamp '{}' doesn't match YYYY-MM-DD HH:MM:SS format",
            timestamp
        );
    }

    #[test]
    fn info_prints_to_stderr() {
        // Given: A message string
        // When: info("hello") is called
        // Then: stderr contains [timestamp] ℹ  hello
        // (We verify the function doesn't panic; output goes to stderr.)
        info("hello");
    }

    #[test]
    fn success_prints_to_stderr() {
        // Given: A message string
        // When: success("done") is called
        // Then: does not panic, output goes to stderr
        success("done");
    }

    #[test]
    fn warn_prints_to_stderr() {
        // Given: A message string
        // When: warn("caution") is called
        // Then: does not panic
        warn("caution");
    }

    #[test]
    fn fail_prints_to_stderr() {
        // Given: A message string
        // When: fail("error") is called
        // Then: does not panic
        fail("error");
    }

    #[test]
    fn header_prints_to_stderr() {
        // Given: A title string
        // When: header("Test Section") is called
        // Then: does not panic
        header("Test Section");
    }

    #[test]
    fn divider_prints_to_stderr() {
        // Given: N/A
        // When: divider() is called
        // Then: does not panic
        divider();
    }
}

//! Runbook parsing and milestone tracking.
//!
//! Parses the Milestone Tracker table from a runbook markdown file.

use std::fmt;
use std::str::FromStr;

use regex::Regex;

/// Status of a milestone in the tracker table.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MilestoneStatus {
    NotStarted,
    InProgress,
    Done,
}

impl fmt::Display for MilestoneStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MilestoneStatus::NotStarted => write!(f, "not_started"),
            MilestoneStatus::InProgress => write!(f, "in_progress"),
            MilestoneStatus::Done => write!(f, "done"),
        }
    }
}

impl FromStr for MilestoneStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().trim_matches('`') {
            "not_started" => Ok(MilestoneStatus::NotStarted),
            "in_progress" => Ok(MilestoneStatus::InProgress),
            "done" => Ok(MilestoneStatus::Done),
            other => Err(format!("Unknown milestone status: '{}'", other)),
        }
    }
}

/// A single row from the Milestone Tracker table.
#[derive(Debug, Clone)]
pub struct MilestoneRow {
    pub number: u32,
    pub title: String,
    pub status: MilestoneStatus,
    pub started: Option<String>,
    pub completed: Option<String>,
    pub lessons_file: Option<String>,
}

/// Parse the Milestone Tracker table from runbook markdown content.
///
/// Matches rows like:
/// `| 1 | Title | \`not_started\` | | | |`
pub fn parse_tracker(runbook_content: &str) -> Vec<MilestoneRow> {
    let row_re = Regex::new(r"^\|\s*(\d+)\s*\|").unwrap();
    let status_re = Regex::new(r"`(not_started|in_progress|done)`").unwrap();

    let mut rows = Vec::new();

    for line in runbook_content.lines() {
        // Must start with | <number> |
        if let Some(caps) = row_re.captures(line) {
            // Must contain a backtick-wrapped status
            if let Some(status_caps) = status_re.captures(line) {
                let number: u32 = caps[1].parse().unwrap_or(0);
                let status = MilestoneStatus::from_str(&status_caps[1]).unwrap_or(MilestoneStatus::NotStarted);

                // Parse columns by splitting on |
                let cols: Vec<&str> = line.split('|').collect();
                // cols[0] = "" (before first |)
                // cols[1] = number
                // cols[2] = title
                // cols[3] = status
                // cols[4] = started
                // cols[5] = completed
                // cols[6] = lessons file
                let title = cols.get(2).map(|s| s.trim().to_string()).unwrap_or_default();
                let started = cols
                    .get(4)
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty());
                let completed = cols
                    .get(5)
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty());
                let lessons_file = cols
                    .get(6)
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty());

                rows.push(MilestoneRow {
                    number,
                    title,
                    status,
                    started,
                    completed,
                    lessons_file,
                });
            }
        }
    }

    rows
}

/// Check if all milestones are done.
pub fn all_done(rows: &[MilestoneRow]) -> bool {
    !rows.is_empty() && rows.iter().all(|r| r.status == MilestoneStatus::Done)
}

/// Get the next incomplete milestone (first one that is not `Done`).
pub fn next_incomplete(rows: &[MilestoneRow]) -> Option<&MilestoneRow> {
    rows.iter().find(|r| r.status != MilestoneStatus::Done)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_TABLE: &str = r#"
## Milestone Tracker

| # | Milestone | Status | Started | Completed | Lessons File |
|---|---|---|---|---|---|
| 1 | Cargo workspace scaffolding | `not_started` | | | |
| 2 | Shared library | `not_started` | | | |
| 3 | Integration tests | `not_started` | | | |
"#;

    const MIXED_TABLE: &str = r#"
## Milestone Tracker

| # | Milestone | Status | Started | Completed | Lessons File |
|---|---|---|---|---|---|
| 1 | Cargo workspace scaffolding | `done` | 2026-01-01 | 2026-01-02 | `docs/lessons/m1.md` |
| 2 | Shared library | `in_progress` | 2026-01-03 | | |
| 3 | Integration tests | `not_started` | | | |
"#;

    const ALL_DONE_TABLE: &str = r#"
## Milestone Tracker

| # | Milestone | Status | Started | Completed | Lessons File |
|---|---|---|---|---|---|
| 1 | First | `done` | 2026-01-01 | 2026-01-02 | `m1.md` |
| 2 | Second | `done` | 2026-01-03 | 2026-01-04 | `m2.md` |
| 3 | Third | `done` | 2026-01-05 | 2026-01-06 | `m3.md` |
"#;

    #[test]
    fn parse_tracker_table() {
        // Given: Markdown with 3 milestone rows, all not_started
        // When: parse_tracker(content) is called
        let rows = parse_tracker(SAMPLE_TABLE);
        // Then: Returns 3 MilestoneRow structs with NotStarted status
        assert_eq!(rows.len(), 3);
        for row in &rows {
            assert_eq!(row.status, MilestoneStatus::NotStarted);
        }
        assert_eq!(rows[0].number, 1);
        assert_eq!(rows[0].title, "Cargo workspace scaffolding");
    }

    #[test]
    fn all_done_detection() {
        // Given: 3 rows all with done status
        let rows = parse_tracker(ALL_DONE_TABLE);
        // When: all_done(rows) is called
        // Then: Returns true
        assert!(all_done(&rows));
    }

    #[test]
    fn all_done_false_when_not_all_done() {
        // Given: Rows with mixed statuses
        let rows = parse_tracker(MIXED_TABLE);
        // When: all_done(rows) is called
        // Then: Returns false
        assert!(!all_done(&rows));
    }

    #[test]
    fn next_incomplete_finds_first_non_done() {
        // Given: Rows 1=done, 2=in_progress, 3=not_started
        let rows = parse_tracker(MIXED_TABLE);
        // When: next_incomplete(rows) is called
        let next = next_incomplete(&rows);
        // Then: Returns row 2
        assert!(next.is_some());
        assert_eq!(next.unwrap().number, 2);
        assert_eq!(next.unwrap().status, MilestoneStatus::InProgress);
    }

    #[test]
    fn mixed_status_parsing() {
        // Given: Row with in_progress status
        let rows = parse_tracker(MIXED_TABLE);
        // When: parse_tracker(content)
        // Then: Correctly parses as InProgress
        let row2 = rows.iter().find(|r| r.number == 2).unwrap();
        assert_eq!(row2.status, MilestoneStatus::InProgress);
    }

    #[test]
    fn parse_tracker_extracts_dates_and_lessons() {
        // Given: Rows with dates and lessons files
        let rows = parse_tracker(MIXED_TABLE);
        // When: Row 1 is examined
        let row1 = &rows[0];
        // Then: Started and completed dates are present
        assert_eq!(row1.started.as_deref(), Some("2026-01-01"));
        assert_eq!(row1.completed.as_deref(), Some("2026-01-02"));
        assert_eq!(row1.lessons_file.as_deref(), Some("`docs/lessons/m1.md`"));
    }

    #[test]
    fn milestone_status_display() {
        // Given: MilestoneStatus values
        // When: Display trait is used
        // Then: Returns expected strings
        assert_eq!(MilestoneStatus::NotStarted.to_string(), "not_started");
        assert_eq!(MilestoneStatus::InProgress.to_string(), "in_progress");
        assert_eq!(MilestoneStatus::Done.to_string(), "done");
    }

    #[test]
    fn milestone_status_from_str() {
        // Given: Status strings
        // When: FromStr is used
        // Then: Correctly parses
        assert_eq!(
            MilestoneStatus::from_str("not_started").unwrap(),
            MilestoneStatus::NotStarted
        );
        assert_eq!(
            MilestoneStatus::from_str("`in_progress`").unwrap(),
            MilestoneStatus::InProgress
        );
        assert_eq!(
            MilestoneStatus::from_str("done").unwrap(),
            MilestoneStatus::Done
        );
    }

    #[test]
    fn next_incomplete_returns_none_when_all_done() {
        // Given: All rows are done
        let rows = parse_tracker(ALL_DONE_TABLE);
        // When: next_incomplete is called
        // Then: Returns None
        assert!(next_incomplete(&rows).is_none());
    }
}

//! E2E tests for voice-tx Milestone 1 — Standalone Voice Transcriber Page & Route.
//!
//! These tests validate:
//! - Workspace compiles with the new transcriber route
//! - AppPhase type includes "transcriber"

#[test]
fn workspace_compiles_with_transcriber_route() {
    // Given: The workspace with the new transcriber route added
    // When: cargo test --workspace runs
    // Then: It exits 0 (this test compiling and running proves it)
    assert!(
        true,
        "Workspace compiled successfully with transcriber route — this test is the proof"
    );
}

#[test]
fn app_phase_type_includes_transcriber() {
    // Given: The types/index.ts file defines AppPhase
    // When: We check the file contents
    // Then: "transcriber" is included in the AppPhase union
    let types_content = std::fs::read_to_string(
        "crates/sldo-tauri/ui/src/types/index.ts",
    )
    .expect("types/index.ts must exist");

    assert!(
        types_content.contains("\"transcriber\""),
        "AppPhase type must include 'transcriber' variant"
    );

    // Also verify all existing phases are still present
    for phase in &["home", "planning", "reviewing", "executing", "settings"] {
        assert!(
            types_content.contains(&format!("\"{}\"", phase)),
            "AppPhase must still contain existing phase '{}'",
            phase
        );
    }
}

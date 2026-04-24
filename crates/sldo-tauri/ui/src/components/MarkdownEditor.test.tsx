/**
 * BDD tests for the MarkdownEditor component.
 *
 * Feature: Runbook loading and display
 * Feature: Runbook editing
 * Feature: Editor modes
 */
import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen, fireEvent, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import MarkdownEditor from "./MarkdownEditor";

// Mock Tauri APIs
vi.mock("@tauri-apps/api/event", () => ({
  listen: vi.fn().mockResolvedValue(vi.fn()),
}));

vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn().mockResolvedValue({ content: "", milestones: [], path: "" }),
}));

const SAMPLE_RUNBOOK = `# Test Runbook

## Milestone Tracker

| # | Milestone | Status | Started | Completed | Lessons File |
|---|---|---|---|---|---|
| 1 | Setup | \`done\` | 2026-01-01 | 2026-01-02 | |
| 2 | Core logic | \`in_progress\` | 2026-01-03 | | |
| 3 | Tests | \`not_started\` | | | |

## Pre-Milestone Protocol

Do stuff.
`;

describe("Feature: Runbook loading and display", () => {
  it("Scenario: Runbook loads after plan completion — MarkdownEditor displays the runbook content", () => {
    // Given: Plan completed with valid runbook
    // When: MarkdownEditor mounts with content
    render(
      <MarkdownEditor
        content={SAMPLE_RUNBOOK}
        onSave={vi.fn()}
        validationWarnings={[]}
      />
    );

    // Then: MarkdownEditor displays the runbook content
    expect(screen.getByText(/Test Runbook/)).toBeInTheDocument();
  });

  it("Scenario: Empty content shows placeholder", () => {
    // Given: No runbook content
    render(
      <MarkdownEditor
        content=""
        onSave={vi.fn()}
        validationWarnings={[]}
      />
    );

    // Then: Editor area is present (empty)
    const textarea = screen.getByRole("textbox");
    expect(textarea).toBeInTheDocument();
    expect((textarea as HTMLTextAreaElement).value).toBe("");
  });
});

describe("Feature: Runbook editing", () => {
  it("Scenario: Edit and save round-trip — save callback invoked with updated content", async () => {
    // Given: Runbook loaded in editor
    const onSave = vi.fn();
    render(
      <MarkdownEditor
        content={SAMPLE_RUNBOOK}
        onSave={onSave}
        validationWarnings={[]}
      />
    );

    // When: User edits text and saves
    const textarea = screen.getByRole("textbox");
    await userEvent.clear(textarea);
    await userEvent.type(textarea, "# Updated Runbook");

    const saveButton = screen.getByRole("button", { name: /save/i });
    await userEvent.click(saveButton);

    // Then: onSave called with updated content
    expect(onSave).toHaveBeenCalledWith("# Updated Runbook");
  });

  it("Scenario: Validation warnings shown — warnings display when present", () => {
    // Given: Validation warnings exist
    const warnings = ["Missing section: Milestone Tracker", "Runbook too short"];

    // When: MarkdownEditor renders with warnings
    render(
      <MarkdownEditor
        content={SAMPLE_RUNBOOK}
        onSave={vi.fn()}
        validationWarnings={warnings}
      />
    );

    // Then: Warning messages shown
    expect(screen.getByText(/Missing section: Milestone Tracker/)).toBeInTheDocument();
    expect(screen.getByText(/Runbook too short/)).toBeInTheDocument();
  });

  it("Scenario: No validation warnings — no warning elements shown", () => {
    // Given: No validation warnings
    render(
      <MarkdownEditor
        content={SAMPLE_RUNBOOK}
        onSave={vi.fn()}
        validationWarnings={[]}
      />
    );

    // Then: No warning elements visible
    expect(screen.queryByTestId("validation-warnings")).not.toBeInTheDocument();
  });
});

describe("Feature: Editor modes", () => {
  it("Scenario: Toggle edit/preview — clicking Preview shows rendered content", async () => {
    // Given: Editor in edit mode
    render(
      <MarkdownEditor
        content={SAMPLE_RUNBOOK}
        onSave={vi.fn()}
        validationWarnings={[]}
      />
    );

    // Then: Textarea is visible (edit mode is default)
    expect(screen.getByRole("textbox")).toBeInTheDocument();

    // When: User clicks "Preview"
    const previewButton = screen.getByRole("button", { name: /preview/i });
    await userEvent.click(previewButton);

    // Then: Rendered Markdown shown (not raw textarea)
    expect(screen.queryByRole("textbox")).not.toBeInTheDocument();
    expect(screen.getByTestId("markdown-preview")).toBeInTheDocument();
  });

  it("Scenario: Toggle preview/edit — clicking Edit shows raw textarea", async () => {
    // Given: Editor starts in edit mode
    render(
      <MarkdownEditor
        content={SAMPLE_RUNBOOK}
        onSave={vi.fn()}
        validationWarnings={[]}
      />
    );

    // Navigate to preview mode
    const previewButton = screen.getByRole("button", { name: /preview/i });
    await userEvent.click(previewButton);
    expect(screen.queryByRole("textbox")).not.toBeInTheDocument();

    // When: User clicks "Edit"
    const editButton = screen.getByRole("button", { name: /edit/i });
    await userEvent.click(editButton);

    // Then: Raw Markdown textarea shown
    expect(screen.getByRole("textbox")).toBeInTheDocument();
  });
});

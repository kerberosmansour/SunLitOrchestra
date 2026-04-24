/**
 * E2E runtime validation tests for Tauri Desktop Milestone 4 — frontend.
 *
 * These tests validate:
 * - Markdown editor renders content
 * - Milestone tracker shows rows
 * - Edit/preview toggle works
 */
import { describe, it, expect, vi } from "vitest";
import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import MarkdownEditor from "../components/MarkdownEditor";
import MilestoneTracker from "../components/MilestoneTracker";
import type { MilestoneRow } from "../types";

// Mock Tauri APIs
vi.mock("@tauri-apps/api/event", () => ({
  listen: vi.fn().mockResolvedValue(vi.fn()),
}));

vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn().mockResolvedValue(null),
}));

const SAMPLE_CONTENT = `# My Runbook

## Milestone Tracker

| # | Milestone | Status | Started | Completed | Lessons File |
|---|---|---|---|---|---|
| 1 | First milestone | \`done\` | 2026-01-01 | 2026-01-02 | |
| 2 | Second milestone | \`in_progress\` | 2026-01-03 | | |
| 3 | Third milestone | \`not_started\` | | | |

## Background Context

Some background information.
`;

const SAMPLE_MILESTONES: MilestoneRow[] = [
  { number: 1, title: "First milestone", status: "done", started: "2026-01-01", completed: "2026-01-02" },
  { number: 2, title: "Second milestone", status: "in_progress", started: "2026-01-03" },
  { number: 3, title: "Third milestone", status: "not_started" },
];

describe("E2E: Markdown editor integration", () => {
  it("markdown_editor_renders_content — content text visible in the editor", () => {
    // Given: A runbook content string
    // When: MarkdownEditor mounts with content
    render(
      <MarkdownEditor
        content={SAMPLE_CONTENT}
        onSave={vi.fn()}
        validationWarnings={[]}
      />
    );

    // Then: Content text visible in the editor
    const textarea = screen.getByRole("textbox");
    expect((textarea as HTMLTextAreaElement).value).toContain("My Runbook");
    expect((textarea as HTMLTextAreaElement).value).toContain("Milestone Tracker");
  });

  it("milestone_tracker_shows_rows — correct number of milestone rows rendered", () => {
    // Given: 3 milestones
    // When: MilestoneTracker renders
    render(<MilestoneTracker milestones={SAMPLE_MILESTONES} />);

    // Then: 3 milestone rows rendered
    expect(screen.getByText("First milestone")).toBeInTheDocument();
    expect(screen.getByText("Second milestone")).toBeInTheDocument();
    expect(screen.getByText("Third milestone")).toBeInTheDocument();
  });

  it("edit_preview_toggle_works — switching modes changes the visible component", async () => {
    // Given: Editor in edit mode
    render(
      <MarkdownEditor
        content={SAMPLE_CONTENT}
        onSave={vi.fn()}
        validationWarnings={[]}
      />
    );

    // Editor mode: textarea visible
    expect(screen.getByRole("textbox")).toBeInTheDocument();

    // When: Switch to preview mode
    const previewButton = screen.getByRole("button", { name: /preview/i });
    await userEvent.click(previewButton);

    // Then: Textarea hidden, preview visible
    expect(screen.queryByRole("textbox")).not.toBeInTheDocument();
    expect(screen.getByTestId("markdown-preview")).toBeInTheDocument();

    // When: Switch back to edit mode
    const editButton = screen.getByRole("button", { name: /edit/i });
    await userEvent.click(editButton);

    // Then: Textarea visible again
    expect(screen.getByRole("textbox")).toBeInTheDocument();
  });
});

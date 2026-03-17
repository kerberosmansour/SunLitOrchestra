/**
 * BDD tests for the MilestoneTracker component.
 *
 * Feature: Milestone tracker rendering
 */
import { describe, it, expect, vi } from "vitest";
import { render, screen } from "@testing-library/react";
import MilestoneTracker from "./MilestoneTracker";
import type { MilestoneRow } from "../types";

// Mock Tauri APIs
vi.mock("@tauri-apps/api/event", () => ({
  listen: vi.fn().mockResolvedValue(vi.fn()),
}));

vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn().mockResolvedValue(null),
}));

const SAMPLE_MILESTONES: MilestoneRow[] = [
  { number: 1, title: "Setup", status: "done", started: "2026-01-01", completed: "2026-01-02" },
  { number: 2, title: "Core logic", status: "in_progress", started: "2026-01-03" },
  { number: 3, title: "Tests", status: "not_started" },
  { number: 4, title: "Polish", status: "not_started" },
  { number: 5, title: "Release", status: "not_started" },
];

describe("Feature: Milestone tracker rendering", () => {
  it("Scenario: Milestone tracker rendered — shows 5 rows with correct statuses", () => {
    // Given: Runbook contains 5 milestones
    // When: MilestoneTracker renders
    render(<MilestoneTracker milestones={SAMPLE_MILESTONES} />);

    // Then: Shows 5 rows with correct statuses
    expect(screen.getByText("Setup")).toBeInTheDocument();
    expect(screen.getByText("Core logic")).toBeInTheDocument();
    expect(screen.getByText("Tests")).toBeInTheDocument();
    expect(screen.getByText("Polish")).toBeInTheDocument();
    expect(screen.getByText("Release")).toBeInTheDocument();
  });

  it("Scenario: Color-coded status indicators — done is green, in_progress is yellow, not_started is gray", () => {
    // Given: Milestones with mixed statuses
    // When: MilestoneTracker renders
    const { container } = render(<MilestoneTracker milestones={SAMPLE_MILESTONES} />);

    // Then: Status indicators have correct classes
    const doneIndicators = container.querySelectorAll(".milestone-status--done");
    const inProgressIndicators = container.querySelectorAll(".milestone-status--in_progress");
    const notStartedIndicators = container.querySelectorAll(".milestone-status--not_started");

    expect(doneIndicators.length).toBe(1);
    expect(inProgressIndicators.length).toBe(1);
    expect(notStartedIndicators.length).toBe(3);
  });

  it("Scenario: Progress bar shows completion percentage", () => {
    // Given: 1 of 5 milestones done (20%)
    // When: MilestoneTracker renders
    render(<MilestoneTracker milestones={SAMPLE_MILESTONES} />);

    // Then: Progress text shown
    expect(screen.getByText(/1.*of.*5/i)).toBeInTheDocument();
  });

  it("Scenario: Empty milestones — shows empty state", () => {
    // Given: No milestones
    // When: MilestoneTracker renders
    render(<MilestoneTracker milestones={[]} />);

    // Then: Shows empty state
    expect(screen.getByText(/no milestones/i)).toBeInTheDocument();
  });

  it("Scenario: All done — progress shows 100%", () => {
    // Given: All milestones done
    const allDone: MilestoneRow[] = [
      { number: 1, title: "Setup", status: "done", started: "2026-01-01", completed: "2026-01-02" },
      { number: 2, title: "Core", status: "done", started: "2026-01-03", completed: "2026-01-04" },
    ];

    // When: MilestoneTracker renders
    render(<MilestoneTracker milestones={allDone} />);

    // Then: Shows 2 of 2
    expect(screen.getByText(/2.*of.*2/i)).toBeInTheDocument();
  });
});

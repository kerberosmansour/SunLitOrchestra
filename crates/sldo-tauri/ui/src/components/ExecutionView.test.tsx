/**
 * ExecutionView component BDD tests.
 */
import { describe, it, expect } from "vitest";
import { render, screen } from "@testing-library/react";
import ExecutionView from "./ExecutionView";
import type { MilestoneRow } from "../types";
import type { LogEntry, BuildTestEntry } from "../hooks/useExecution";

const sampleMilestones: MilestoneRow[] = [
  { number: 1, title: "Scaffold workspace", status: "done", started: "2026-01-01", completed: "2026-01-02" },
  { number: 2, title: "Chatbot UI", status: "in_progress", started: "2026-01-03" },
  { number: 3, title: "Planning backend", status: "not_started" },
];

describe("ExecutionView", () => {
  // ── Feature: Execution UI ─────────────────────────────────────────────

  it("renders without errors", () => {
    // Given: Default props
    // When: ExecutionView renders
    const { container } = render(
      <ExecutionView
        milestones={sampleMilestones}
        currentMilestone={null}
        logs={[]}
        buildTestResults={[]}
        completionSummary={null}
        isRunning={true}
        onCancel={() => {}}
      />
    );
    // Then: The execution view is present
    expect(container.querySelector(".execution-view")).toBeTruthy();
  });

  it("shows cancel button during execution", () => {
    // Given: Execution is running
    // When: ExecutionView renders
    render(
      <ExecutionView
        milestones={sampleMilestones}
        currentMilestone={{ milestone_number: 2, title: "Chatbot UI", attempt: 1 }}
        logs={[]}
        buildTestResults={[]}
        completionSummary={null}
        isRunning={true}
        onCancel={() => {}}
      />
    );
    // Then: Cancel button is visible
    const btn = screen.getByText("Cancel Execution");
    expect(btn).toBeTruthy();
    expect(btn.tagName).toBe("BUTTON");
  });

  it("hides cancel button when not running", () => {
    // Given: Execution is not running
    // When: ExecutionView renders
    render(
      <ExecutionView
        milestones={sampleMilestones}
        currentMilestone={null}
        logs={[]}
        buildTestResults={[]}
        completionSummary={null}
        isRunning={false}
        onCancel={() => {}}
      />
    );
    // Then: Cancel button is not present
    expect(screen.queryByText("Cancel Execution")).toBeNull();
  });

  it("shows current milestone info", () => {
    // Given: A milestone is in progress
    // When: ExecutionView renders with currentMilestone
    render(
      <ExecutionView
        milestones={sampleMilestones}
        currentMilestone={{ milestone_number: 2, title: "Chatbot UI", attempt: 3 }}
        logs={[]}
        buildTestResults={[]}
        completionSummary={null}
        isRunning={true}
        onCancel={() => {}}
      />
    );
    // Then: Shows milestone title and attempt
    expect(screen.getByText(/M2: Chatbot UI/)).toBeTruthy();
    expect(screen.getByText(/Attempt 3/)).toBeTruthy();
  });

  it("renders log entries from execution progress events", () => {
    // Given: Some log entries from execution
    const logs: LogEntry[] = [
      { id: "log-1", line: "Compiling sldo-common v0.1.0", stream: "stdout", timestamp: "2026-03-17T12:00:00Z" },
      { id: "log-2", line: "warning: unused variable", stream: "stderr", timestamp: "2026-03-17T12:00:01Z" },
    ];

    // When: ExecutionView renders
    render(
      <ExecutionView
        milestones={sampleMilestones}
        currentMilestone={{ milestone_number: 1, title: "Scaffold", attempt: 1 }}
        logs={logs}
        buildTestResults={[]}
        completionSummary={null}
        isRunning={true}
        onCancel={() => {}}
      />
    );

    // Then: Log lines appear
    expect(screen.getByText("Compiling sldo-common v0.1.0")).toBeTruthy();
    expect(screen.getByText("warning: unused variable")).toBeTruthy();
  });

  it("shows build failure with red indicator", () => {
    // Given: A failed build result
    const results: BuildTestEntry[] = [
      { command: "cargo build --workspace", success: false, output: "error[E0308]" },
    ];

    // When: ExecutionView renders
    render(
      <ExecutionView
        milestones={sampleMilestones}
        currentMilestone={null}
        logs={[]}
        buildTestResults={results}
        completionSummary={null}
        isRunning={false}
        onCancel={() => {}}
      />
    );

    // Then: The failed result is shown with fail class
    const resultEl = screen.getByText(/cargo build --workspace/);
    expect(resultEl.className).toContain("build-result--fail");
  });

  it("shows build success with green indicator", () => {
    // Given: A successful build result
    const results: BuildTestEntry[] = [
      { command: "cargo test --workspace", success: true, output: "test result: ok" },
    ];

    // When: ExecutionView renders
    render(
      <ExecutionView
        milestones={sampleMilestones}
        currentMilestone={null}
        logs={[]}
        buildTestResults={results}
        completionSummary={null}
        isRunning={false}
        onCancel={() => {}}
      />
    );

    // Then: The successful result is shown with pass class
    const resultEl = screen.getByText(/cargo test --workspace/);
    expect(resultEl.className).toContain("build-result--pass");
  });

  it("shows completion summary when all done", () => {
    // Given: Execution completed with all milestones done
    // When: ExecutionView renders with completion summary
    render(
      <ExecutionView
        milestones={sampleMilestones}
        currentMilestone={null}
        logs={[]}
        buildTestResults={[]}
        completionSummary={{ all_done: true, milestones_completed: 3, total: 3 }}
        isRunning={false}
        onCancel={() => {}}
      />
    );

    // Then: Summary shows completion
    expect(screen.getByText("Execution Complete")).toBeTruthy();
    expect(screen.getByText(/3\/3 milestones completed/)).toBeTruthy();
  });

  it("shows milestone tracker in sidebar", () => {
    // Given: Milestones passed to ExecutionView
    // When: Rendered
    const { container } = render(
      <ExecutionView
        milestones={sampleMilestones}
        currentMilestone={null}
        logs={[]}
        buildTestResults={[]}
        completionSummary={null}
        isRunning={false}
        onCancel={() => {}}
      />
    );

    // Then: Milestone tracker is visible
    expect(container.querySelector(".milestone-tracker")).toBeTruthy();
  });
});

/**
 * E2E tests for Milestone 5 — Execution frontend.
 *
 * These tests validate that the execution view renders correctly,
 * the cancel button is present, and progress events render as log lines.
 */
import { describe, it, expect } from "vitest";
import { render, screen } from "@testing-library/react";
import ExecutionView from "../components/ExecutionView";
import type { LogEntry } from "../hooks/useExecution";

describe("Execution E2E", () => {
  it("execution_view_renders", () => {
    // Given: Default execution state
    // When: ExecutionView mounts
    const { container } = render(
      <ExecutionView
        milestones={[
          { number: 1, title: "First", status: "done" },
          { number: 2, title: "Second", status: "not_started" },
        ]}
        currentMilestone={null}
        logs={[]}
        buildTestResults={[]}
        completionSummary={null}
        isRunning={true}
        onCancel={() => {}}
      />
    );

    // Then: No errors, milestone tracker visible
    expect(container.querySelector(".execution-view")).toBeTruthy();
    expect(container.querySelector(".milestone-tracker")).toBeTruthy();
  });

  it("cancel_button_present", () => {
    // Given: Execution is running
    // When: ExecutionView renders
    render(
      <ExecutionView
        milestones={[]}
        currentMilestone={{ milestone_number: 1, title: "Test", attempt: 1 }}
        logs={[]}
        buildTestResults={[]}
        completionSummary={null}
        isRunning={true}
        onCancel={() => {}}
      />
    );

    // Then: Button element found and clickable
    const btn = screen.getByText("Cancel Execution");
    expect(btn).toBeTruthy();
    expect(btn.tagName).toBe("BUTTON");
    expect(btn).not.toBeDisabled();
  });

  it("progress_events_render_as_log_lines", () => {
    // Given: Mock execution progress events
    const logs: LogEntry[] = [
      { id: "log-1", line: "Compiling project...", stream: "stdout", timestamp: "2026-03-17T12:00:00Z" },
      { id: "log-2", line: "Running tests...", stream: "stdout", timestamp: "2026-03-17T12:00:01Z" },
      { id: "log-3", line: "Error in module X", stream: "stderr", timestamp: "2026-03-17T12:00:02Z" },
    ];

    // When: ExecutionView renders with events
    render(
      <ExecutionView
        milestones={[]}
        currentMilestone={{ milestone_number: 1, title: "Build", attempt: 1 }}
        logs={logs}
        buildTestResults={[]}
        completionSummary={null}
        isRunning={true}
        onCancel={() => {}}
      />
    );

    // Then: Mock events produce visible log entries
    expect(screen.getByText("Compiling project...")).toBeTruthy();
    expect(screen.getByText("Running tests...")).toBeTruthy();
    expect(screen.getByText("Error in module X")).toBeTruthy();
  });
});

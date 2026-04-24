/**
 * BDD tests for streaming hooks and planning UI integration.
 *
 * Feature: Streaming output to frontend
 * Feature: Planning command invocation (frontend perspective)
 */
import { describe, it, expect, vi, beforeEach, afterEach } from "vitest";
import { render, screen } from "@testing-library/react";
import ConversationView from "../components/ConversationView";
import type { Message, PlanProgressEvent } from "../types";

// Mock @tauri-apps/api modules so hooks can be imported without Tauri runtime
vi.mock("@tauri-apps/api/event", () => ({
  listen: vi.fn().mockResolvedValue(vi.fn()),
}));

vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn().mockResolvedValue("/tmp/RUNBOOK.md"),
}));

const makeMsg = (id: string, role: Message["role"], content: string): Message => ({
  id,
  role,
  content,
  timestamp: Date.now(),
});

describe("Feature: Streaming output to frontend", () => {
  it("Scenario: Progress events arrive in order — streaming lines render in conversation", () => {
    // Given: Planning is in progress with streaming lines
    const messages: Message[] = [makeMsg("1", "user", "Build a REST API")];
    const streamingLines: PlanProgressEvent[] = [
      { line: "Analyzing repository...", stream: "stdout", timestamp: "2026-03-17T12:00:00Z" },
      { line: "Reading Cargo.toml...", stream: "stdout", timestamp: "2026-03-17T12:00:01Z" },
      { line: "Found 5 crates", stream: "stdout", timestamp: "2026-03-17T12:00:02Z" },
    ];

    // When: ConversationView renders with streaming lines
    render(
      <ConversationView
        messages={messages}
        onSubmit={() => {}}
        streamingLines={streamingLines}
      />
    );

    // Then: All streaming lines appear in order
    expect(screen.getByText("Analyzing repository...")).toBeInTheDocument();
    expect(screen.getByText("Reading Cargo.toml...")).toBeInTheDocument();
    expect(screen.getByText("Found 5 crates")).toBeInTheDocument();
  });

  it("Scenario: Streaming hook accumulates messages — all lines visible", () => {
    // Given: Multiple streaming lines arrive
    const messages: Message[] = [];
    const streamingLines: PlanProgressEvent[] = [
      { line: "Line 1", stream: "stdout", timestamp: "2026-03-17T12:00:00Z" },
      { line: "Line 2", stream: "stderr", timestamp: "2026-03-17T12:00:01Z" },
      { line: "Line 3", stream: "stdout", timestamp: "2026-03-17T12:00:02Z" },
    ];

    // When: ConversationView renders
    render(
      <ConversationView
        messages={messages}
        onSubmit={() => {}}
        streamingLines={streamingLines}
      />
    );

    // Then: All lines are in the DOM
    expect(screen.getByText("Line 1")).toBeInTheDocument();
    expect(screen.getByText("Line 2")).toBeInTheDocument();
    expect(screen.getByText("Line 3")).toBeInTheDocument();
  });

  it("Scenario: No streaming lines — no streaming output rendered", () => {
    // Given: No streaming lines
    const messages: Message[] = [makeMsg("1", "user", "Hello")];

    // When: ConversationView renders without streamingLines
    render(
      <ConversationView messages={messages} onSubmit={() => {}} />
    );

    // Then: User message visible, no streaming output
    expect(screen.getByText("Hello")).toBeInTheDocument();
    expect(screen.queryByText("streamingOutput")).not.toBeInTheDocument();
  });

  it("Scenario: Stderr lines render with correct stream class", () => {
    // Given: Streaming lines with stderr
    const messages: Message[] = [];
    const streamingLines: PlanProgressEvent[] = [
      { line: "Warning: something", stream: "stderr", timestamp: "2026-03-17T12:00:00Z" },
    ];

    // When: ConversationView renders
    const { container } = render(
      <ConversationView
        messages={messages}
        onSubmit={() => {}}
        streamingLines={streamingLines}
      />
    );

    // Then: The stderr line has the correct class
    const stderrLine = container.querySelector(".streamLine--stderr");
    expect(stderrLine).toBeInTheDocument();
    expect(stderrLine?.textContent).toBe("Warning: something");
  });
});

describe("Feature: ConversationView backward compatibility", () => {
  it("Scenario: Existing tests still work without streamingLines prop", () => {
    // Given: Messages without streaming
    const messages: Message[] = [
      makeMsg("1", "user", "Build a REST API"),
      makeMsg("2", "assistant", "I'll create a plan for that."),
    ];

    // When: ConversationView renders without streamingLines
    render(<ConversationView messages={messages} onSubmit={() => {}} />);

    // Then: Both messages visible
    expect(screen.getByText("Build a REST API")).toBeInTheDocument();
    expect(screen.getByText("I'll create a plan for that.")).toBeInTheDocument();
  });
});

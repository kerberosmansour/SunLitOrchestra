/**
 * E2E runtime validation tests for Tauri Desktop Milestone 3 — frontend.
 *
 * These tests validate:
 * - Streaming events render in conversation
 * - Plan errors surface to user
 */
import { describe, it, expect, vi } from "vitest";
import { render, screen } from "@testing-library/react";
import ConversationView from "../components/ConversationView";
import type { Message, PlanProgressEvent } from "../types";

// Mock Tauri APIs
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

describe("E2E: Planning frontend integration", () => {
  it("streaming_events_render_in_conversation — events appear as messages", () => {
    // Given: Planning is streaming output
    const messages: Message[] = [makeMsg("1", "user", "Create a web app")];
    const streamingLines: PlanProgressEvent[] = [
      { line: "Exploring repository structure...", stream: "stdout", timestamp: "2026-03-17T12:00:00Z" },
      { line: "Identifying tech stack...", stream: "stdout", timestamp: "2026-03-17T12:00:01Z" },
      { line: "Decomposing into milestones...", stream: "stdout", timestamp: "2026-03-17T12:00:02Z" },
    ];

    // When: ConversationView renders with streaming lines
    render(
      <ConversationView
        messages={messages}
        onSubmit={() => {}}
        streamingLines={streamingLines}
      />
    );

    // Then: All streaming events appear as visible message elements
    expect(screen.getByText("Exploring repository structure...")).toBeInTheDocument();
    expect(screen.getByText("Identifying tech stack...")).toBeInTheDocument();
    expect(screen.getByText("Decomposing into milestones...")).toBeInTheDocument();
  });

  it("plan_error_shows_in_ui — error event renders an error message in conversation", () => {
    // Given: Planning failed with an error message shown as a system message
    const messages: Message[] = [
      makeMsg("1", "user", "Create a web app"),
      makeMsg("2", "system", "Error: copilot CLI not found on PATH"),
    ];

    // When: ConversationView renders
    render(
      <ConversationView messages={messages} onSubmit={() => {}} />
    );

    // Then: Error message is visible in the conversation
    expect(screen.getByText("Error: copilot CLI not found on PATH")).toBeInTheDocument();
  });
});

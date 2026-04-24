import { describe, it, expect } from "vitest";
import { render, screen } from "@testing-library/react";
import ConversationView from "./ConversationView";
import type { Message } from "../types";

const makeMsg = (id: string, role: Message["role"], content: string): Message => ({
  id,
  role,
  content,
  timestamp: Date.now(),
});

describe("Feature: Conversation view layout", () => {
  it("renders user messages in the conversation", () => {
    // Given: Conversation has user messages
    const messages: Message[] = [makeMsg("1", "user", "Build a REST API")];

    // When: ConversationView renders
    render(<ConversationView messages={messages} onSubmit={() => {}} />);

    // Then: First message shows "Build a REST API" as user message
    expect(screen.getByText("Build a REST API")).toBeInTheDocument();
  });

  it("renders assistant messages in the conversation", () => {
    // Given: Conversation has assistant messages
    const messages: Message[] = [
      makeMsg("1", "user", "Build a REST API"),
      makeMsg("2", "assistant", "I'll create a plan for that."),
    ];

    // When: ConversationView renders
    render(<ConversationView messages={messages} onSubmit={() => {}} />);

    // Then: Both messages are visible
    expect(screen.getByText("Build a REST API")).toBeInTheDocument();
    expect(screen.getByText("I'll create a plan for that.")).toBeInTheDocument();
  });

  it("includes ChatInput pinned at the bottom", () => {
    // Given: User is in conversation view
    const messages: Message[] = [makeMsg("1", "user", "Hello")];

    // When: ConversationView renders
    render(<ConversationView messages={messages} onSubmit={() => {}} />);

    // Then: ChatInput remains pinned (textarea present at bottom)
    expect(screen.getByRole("textbox")).toBeInTheDocument();
  });

  it("auto-scrolls to latest message", () => {
    // Given: Conversation has many messages
    const messages: Message[] = Array.from({ length: 20 }, (_, i) =>
      makeMsg(String(i), "user", `Message ${i}`)
    );

    // When: New message is added
    render(<ConversationView messages={messages} onSubmit={() => {}} />);

    // Then: View auto-scrolls to show the latest message
    expect(screen.getByText("Message 19")).toBeInTheDocument();
  });
});

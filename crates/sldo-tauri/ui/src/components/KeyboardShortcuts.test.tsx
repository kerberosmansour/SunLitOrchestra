import { describe, it, expect, vi, beforeEach, afterEach } from "vitest";
import { render, screen, fireEvent } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import App from "../App";

describe("Feature: Keyboard shortcuts", () => {
  it("Cmd+Enter submits prompt", async () => {
    // Given: User typing in ChatInput on home screen
    const user = userEvent.setup();
    render(<App />);

    const textarea = screen.getByRole("textbox");
    await user.type(textarea, "Build a REST API");

    // When: User presses Cmd+Enter (Meta+Enter)
    fireEvent.keyDown(textarea, { key: "Enter", metaKey: true });

    // Then: Prompt submitted (transitions to planning phase)
    expect(screen.getByText("Build a REST API")).toBeInTheDocument();
  });

  it("Ctrl+Enter submits prompt (Windows/Linux)", async () => {
    // Given: User typing in ChatInput
    const user = userEvent.setup();
    render(<App />);

    const textarea = screen.getByRole("textbox");
    await user.type(textarea, "Create a CLI tool");

    // When: User presses Ctrl+Enter
    fireEvent.keyDown(textarea, { key: "Enter", ctrlKey: true });

    // Then: Prompt submitted
    expect(screen.getByText("Create a CLI tool")).toBeInTheDocument();
  });

  it("Cmd+N creates new session from planning phase", async () => {
    // Given: User in planning phase
    const user = userEvent.setup();
    render(<App />);

    // Go to planning phase first
    const textarea = screen.getByRole("textbox");
    await user.type(textarea, "Build an API{enter}");

    // Verify we're in planning phase
    expect(screen.getByText("Build an API")).toBeInTheDocument();

    // When: User presses Cmd+N
    fireEvent.keyDown(document, { key: "n", metaKey: true });

    // Then: App resets to home screen
    expect(screen.getByText(/sunlitorchestrate/i)).toBeInTheDocument();
  });

  it("Cmd+, opens settings", () => {
    // Given: User on home screen
    render(<App />);

    // When: User presses Cmd+,
    fireEvent.keyDown(document, { key: ",", metaKey: true });

    // Then: Settings panel is shown (heading visible)
    expect(screen.getByRole("heading", { name: /settings/i })).toBeInTheDocument();
  });

  it("Escape closes settings panel", () => {
    // Given: Settings panel is open
    render(<App />);
    fireEvent.keyDown(document, { key: ",", metaKey: true });
    expect(screen.getByRole("heading", { name: /settings/i })).toBeInTheDocument();

    // When: User presses Escape
    fireEvent.keyDown(document, { key: "Escape" });

    // Then: Returns to home screen
    expect(screen.getByText(/sunlitorchestrate/i)).toBeInTheDocument();
  });
});

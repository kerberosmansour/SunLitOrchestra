import { describe, it, expect } from "vitest";
import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import App from "../App";

describe("E2E: Chatbot UI Runtime Validation", () => {
  it("home_screen_renders_without_errors", () => {
    // What it proves: Home screen mounts cleanly
    // Pass criteria: No unhandled exceptions, hero text visible
    render(<App />);
    expect(screen.getByText(/sunlitorchestrate/i)).toBeInTheDocument();
    expect(screen.getByRole("textbox")).toBeInTheDocument();
  });

  it("prompt_submission_transitions_view", async () => {
    // What it proves: State transition works at runtime
    // Pass criteria: ConversationView mounts after submit, user message visible
    const user = userEvent.setup();
    render(<App />);

    const textarea = screen.getByRole("textbox");
    await user.type(textarea, "Build a REST API{enter}");

    expect(screen.getByText("Build a REST API")).toBeInTheDocument();
  });

  it("sidebar_renders_with_logo", () => {
    // What it proves: Sidebar component mounts
    // Pass criteria: Logo element present, sidebar visible
    render(<App />);
    expect(screen.getByAltText(/sunlit/i)).toBeInTheDocument();
  });

  it("empty_prompt_does_not_transition", async () => {
    // What it proves: Input validation works
    // Pass criteria: Phase remains "home" after empty submit
    const user = userEvent.setup();
    render(<App />);

    const textarea = screen.getByRole("textbox");
    await user.click(textarea);
    await user.keyboard("{Enter}");

    // Still on home screen — hero text and prompt chips visible
    expect(screen.getByText(/sunlitorchestrate/i)).toBeInTheDocument();
    const chips = screen.getAllByRole("button", { name: /prompt/i });
    expect(chips.length).toBeGreaterThanOrEqual(3);
  });
});

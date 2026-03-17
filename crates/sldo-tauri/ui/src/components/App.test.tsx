import { describe, it, expect } from "vitest";
import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import App from "../App";

describe("Feature: App state transitions", () => {
  it("shows home screen on initial load", () => {
    // Given: App launches
    // When: App renders
    render(<App />);

    // Then: Home screen is shown with hero text
    expect(screen.getByText(/sunlitorchestrate/i)).toBeInTheDocument();
    expect(screen.getByRole("textbox")).toBeInTheDocument();
  });

  it("submit transitions to conversation", async () => {
    // Given: User on home screen
    const user = userEvent.setup();
    render(<App />);

    // When: User types prompt and presses Enter
    const textarea = screen.getByRole("textbox");
    await user.type(textarea, "Build a REST API{enter}");

    // Then: View transitions to ConversationView with user message visible
    expect(screen.getByText("Build a REST API")).toBeInTheDocument();
  });

  it("empty prompt does not transition", async () => {
    // Given: User on home screen
    const user = userEvent.setup();
    render(<App />);

    // When: User presses Enter with empty input
    const textarea = screen.getByRole("textbox");
    await user.click(textarea);
    await user.keyboard("{Enter}");

    // Then: View stays on home screen
    expect(screen.getByText(/sunlitorchestrate/i)).toBeInTheDocument();
    // Prompt chips should still be visible (home screen indicator)
    const chips = screen.getAllByRole("button", { name: /prompt/i });
    expect(chips.length).toBeGreaterThanOrEqual(3);
  });
});

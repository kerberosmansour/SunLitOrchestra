import { describe, it, expect, vi } from "vitest";
import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import App from "../App";

// Mock Tauri APIs so hooks don't crash outside Tauri runtime
vi.mock("@tauri-apps/api/event", () => ({
  listen: vi.fn().mockResolvedValue(vi.fn()),
}));

vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn().mockImplementation((cmd: string) => {
    if (cmd === "get_settings") {
      return Promise.resolve({
        provider: "copilot",
        model: "claude-opus-4.6",
        allow_flags: [],
        deny_flags: [],
        max_attempts: 150,
        cooldown_secs: 5,
        max_iterations: 3,
        repo_dir: "/tmp/test-repo",
      });
    }
    return Promise.resolve(null);
  }),
}));

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

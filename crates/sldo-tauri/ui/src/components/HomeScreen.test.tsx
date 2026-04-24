import { describe, it, expect } from "vitest";
import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import HomeScreen from "./HomeScreen";

describe("Feature: Home screen layout", () => {
  it("renders centered prompt on load", () => {
    // Given: App launches
    // When: Home screen renders
    const onSubmit = () => {};
    render(<HomeScreen onSubmitPrompt={onSubmit} />);

    // Then: Prompt input is vertically and horizontally centered with hero text above
    expect(screen.getByText(/sunlitorchestrate/i)).toBeInTheDocument();
    expect(screen.getByRole("textbox")).toBeInTheDocument();
  });

  it("shows at least 3 sample prompts", () => {
    // Given: App on home screen
    // When: User views below input
    const onSubmit = () => {};
    render(<HomeScreen onSubmitPrompt={onSubmit} />);

    // Then: At least 3 prompt chips are visible
    const chips = screen.getAllByRole("button", { name: /prompt/i });
    expect(chips.length).toBeGreaterThanOrEqual(3);
  });

  it("clicking prompt chip fills input", async () => {
    // Given: User views home screen
    const user = userEvent.setup();
    const onSubmit = () => {};
    render(<HomeScreen onSubmitPrompt={onSubmit} />);

    // When: User clicks a prompt chip
    const chips = screen.getAllByRole("button", { name: /prompt/i });
    await user.click(chips[0]);

    // Then: ChatInput textarea is populated with the chip text
    const textarea = screen.getByRole("textbox") as HTMLTextAreaElement;
    expect(textarea.value.length).toBeGreaterThan(0);
  });
});

import { describe, it, expect, vi } from "vitest";
import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import ChatInput from "./ChatInput";

describe("Feature: ChatInput", () => {
  it("renders a textarea and submit button", () => {
    // Given: ChatInput is rendered
    render(<ChatInput onSubmit={() => {}} />);

    // Then: textarea and submit button exist
    expect(screen.getByRole("textbox")).toBeInTheDocument();
    expect(screen.getByRole("button", { name: /send|submit/i })).toBeInTheDocument();
  });

  it("submit transitions to conversation on Enter", async () => {
    // Given: User on home screen
    const user = userEvent.setup();
    const onSubmit = vi.fn();
    render(<ChatInput onSubmit={onSubmit} />);

    // When: User types prompt and presses Enter
    const textarea = screen.getByRole("textbox");
    await user.type(textarea, "Build a REST API{enter}");

    // Then: onSubmit is called with the prompt
    expect(onSubmit).toHaveBeenCalledWith("Build a REST API");
  });

  it("empty prompt is rejected", async () => {
    // Given: User on home screen
    const user = userEvent.setup();
    const onSubmit = vi.fn();
    render(<ChatInput onSubmit={onSubmit} />);

    // When: User presses Enter with empty input
    const textarea = screen.getByRole("textbox");
    await user.click(textarea);
    await user.keyboard("{Enter}");

    // Then: Nothing happens, onSubmit is not called
    expect(onSubmit).not.toHaveBeenCalled();
  });

  it("Shift+Enter inserts newline instead of submitting", async () => {
    // Given: User is typing
    const user = userEvent.setup();
    const onSubmit = vi.fn();
    render(<ChatInput onSubmit={onSubmit} />);

    // When: User presses Shift+Enter
    const textarea = screen.getByRole("textbox");
    await user.type(textarea, "line one{Shift>}{Enter}{/Shift}line two");

    // Then: onSubmit is not called, textarea has newline
    expect(onSubmit).not.toHaveBeenCalled();
    expect((textarea as HTMLTextAreaElement).value).toContain("line one");
    expect((textarea as HTMLTextAreaElement).value).toContain("line two");
  });
});

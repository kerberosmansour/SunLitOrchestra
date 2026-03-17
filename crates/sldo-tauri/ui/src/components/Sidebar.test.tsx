import { describe, it, expect, vi } from "vitest";
import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import Sidebar from "./Sidebar";

describe("Feature: Sidebar navigation", () => {
  it("sidebar shows logo", () => {
    // Given: App renders
    // When: User views sidebar
    render(<Sidebar onNewSession={() => {}} onSelectSettings={() => {}} />);

    // Then: SunLit logo is displayed at top of sidebar
    expect(screen.getByAltText(/sunlit/i)).toBeInTheDocument();
  });

  it("sidebar shows New Session button", () => {
    // Given: App renders
    render(<Sidebar onNewSession={() => {}} onSelectSettings={() => {}} />);

    // Then: New Session button is visible
    expect(screen.getByRole("button", { name: /new session/i })).toBeInTheDocument();
  });

  it("New Session resets to home", async () => {
    // Given: User is in conversation
    const user = userEvent.setup();
    const onNewSession = vi.fn();
    render(<Sidebar onNewSession={onNewSession} onSelectSettings={() => {}} />);

    // When: User clicks "New Session"
    await user.click(screen.getByRole("button", { name: /new session/i }));

    // Then: App returns to home screen
    expect(onNewSession).toHaveBeenCalled();
  });

  it("sidebar shows settings link", () => {
    // Given: App renders
    render(<Sidebar onNewSession={() => {}} onSelectSettings={() => {}} />);

    // Then: Settings link is visible
    expect(screen.getByRole("button", { name: /settings/i })).toBeInTheDocument();
  });
});

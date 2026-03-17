/**
 * SettingsPanel component BDD tests.
 */
import { describe, it, expect, vi } from "vitest";
import { render, screen, fireEvent } from "@testing-library/react";
import SettingsPanel from "./SettingsPanel";
import type { AppSettings } from "../types";

const defaultSettings: AppSettings = {
  provider: "copilot",
  model: "claude-opus-4.6",
  allow_flags: ["--allow-tool=write", "--allow-tool=shell(cat:*)"],
  deny_flags: ["--deny-tool=shell(rm -rf /)"],
  max_attempts: 150,
  cooldown_secs: 5,
  max_iterations: 3,
  repo_dir: null,
};

describe("SettingsPanel", () => {
  // ── Feature: Settings UI ──────────────────────────────────────────────

  it("renders without errors", () => {
    // Given: Default settings
    // When: SettingsPanel renders
    const { container } = render(
      <SettingsPanel settings={defaultSettings} onSave={() => {}} onClose={() => {}} />
    );
    // Then: The settings panel is present
    expect(container.querySelector(".settings-panel")).toBeTruthy();
  });

  it("settings panel opens — renders with current values", () => {
    // Given: User in any phase with settings
    // When: SettingsPanel renders
    render(<SettingsPanel settings={defaultSettings} onSave={() => {}} onClose={() => {}} />);
    // Then: Current values are shown
    const modelInput = screen.getByLabelText(/model/i) as HTMLInputElement;
    expect(modelInput.value).toBe("claude-opus-4.6");
  });

  it("model change persists — user changes model field", () => {
    // Given: Default settings
    const onSave = vi.fn();
    render(<SettingsPanel settings={defaultSettings} onSave={onSave} onClose={() => {}} />);
    // When: User changes model field and saves
    const modelInput = screen.getByLabelText(/model/i) as HTMLInputElement;
    fireEvent.change(modelInput, { target: { value: "gpt-4o" } });
    const saveBtn = screen.getByRole("button", { name: /save/i });
    fireEvent.click(saveBtn);
    // Then: onSave called with new model value
    expect(onSave).toHaveBeenCalledTimes(1);
    expect(onSave.mock.calls[0][0].model).toBe("gpt-4o");
  });

  it("tool flags editable — user can see allow flags", () => {
    // Given: Settings panel open with allow flags
    render(<SettingsPanel settings={defaultSettings} onSave={() => {}} onClose={() => {}} />);
    // Then: Allow flags are displayed
    expect(screen.getByText("--allow-tool=write")).toBeTruthy();
  });

  it("shows provider selector", () => {
    // Given: Default settings
    render(<SettingsPanel settings={defaultSettings} onSave={() => {}} onClose={() => {}} />);
    // Then: Provider selector is visible
    expect(screen.getByLabelText(/provider/i)).toBeTruthy();
  });

  it("shows execution parameters", () => {
    // Given: Default settings
    render(<SettingsPanel settings={defaultSettings} onSave={() => {}} onClose={() => {}} />);
    // Then: Max attempts, cooldown, and max iterations fields are visible
    expect(screen.getByLabelText(/max attempts/i)).toBeTruthy();
    expect(screen.getByLabelText(/cooldown/i)).toBeTruthy();
    expect(screen.getByLabelText(/max iterations/i)).toBeTruthy();
  });

  it("close button calls onClose", () => {
    // Given: Settings panel with onClose handler
    const onClose = vi.fn();
    render(<SettingsPanel settings={defaultSettings} onSave={() => {}} onClose={onClose} />);
    // When: User clicks close/back
    const closeBtn = screen.getByRole("button", { name: /close|back/i });
    fireEvent.click(closeBtn);
    // Then: onClose is called
    expect(onClose).toHaveBeenCalledTimes(1);
  });

  it("displays repo directory field", () => {
    // Given: Settings with a repo_dir
    const settings: AppSettings = { ...defaultSettings, repo_dir: "/home/user/project" };
    render(<SettingsPanel settings={settings} onSave={() => {}} onClose={() => {}} />);
    // Then: Repo directory input shows the value
    const repoInput = screen.getByLabelText(/repo/i) as HTMLInputElement;
    expect(repoInput.value).toBe("/home/user/project");
  });
});

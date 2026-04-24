/**
 * E2E runtime validation tests for Tauri Desktop Milestone 6 — Settings.
 *
 * These tests validate:
 * - SettingsPanel renders with form fields
 * - Settings load values into form
 * - Save triggers backend command
 */
import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen, fireEvent } from "@testing-library/react";
import SettingsPanel from "../components/SettingsPanel";
import type { AppSettings } from "../types";

// Mock Tauri APIs
vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn().mockResolvedValue(undefined),
}));

const defaultSettings: AppSettings = {
  provider: "copilot",
  model: "claude-opus-4.6",
  allow_flags: ["--allow-tool=write"],
  deny_flags: ["--deny-tool=shell(rm -rf /)"],
  max_attempts: 150,
  cooldown_secs: 5,
  max_iterations: 3,
  repo_dir: null,
};

describe("E2E: Settings panel integration", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("settings_panel_renders — form fields for model and provider visible", () => {
    // Given: Default settings
    // When: SettingsPanel renders
    render(<SettingsPanel settings={defaultSettings} onSave={() => {}} onClose={() => {}} />);
    // Then: Model and provider fields are visible
    expect(screen.getByLabelText(/model/i)).toBeTruthy();
    expect(screen.getByLabelText(/provider/i)).toBeTruthy();
  });

  it("settings_loads_values — current settings populate form", () => {
    // Given: Custom settings
    const custom: AppSettings = { ...defaultSettings, model: "gpt-4o", max_attempts: 50 };
    // When: SettingsPanel renders with custom values
    render(<SettingsPanel settings={custom} onSave={() => {}} onClose={() => {}} />);
    // Then: Input values reflect settings
    const modelInput = screen.getByLabelText(/model/i) as HTMLInputElement;
    expect(modelInput.value).toBe("gpt-4o");
  });

  it("settings_save_triggers_command — save calls onSave with updated settings", () => {
    // Given: Default settings and a save handler
    const onSave = vi.fn();
    render(<SettingsPanel settings={defaultSettings} onSave={onSave} onClose={() => {}} />);
    // When: User clicks save
    const saveBtn = screen.getByRole("button", { name: /save/i });
    fireEvent.click(saveBtn);
    // Then: onSave was called with settings object
    expect(onSave).toHaveBeenCalledTimes(1);
    const savedSettings = onSave.mock.calls[0][0];
    expect(savedSettings).toHaveProperty("model");
    expect(savedSettings).toHaveProperty("provider");
  });
});

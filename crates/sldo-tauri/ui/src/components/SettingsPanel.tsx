import { useState } from "react";
import type { AppSettings } from "../types";

interface SettingsPanelProps {
  settings: AppSettings;
  onSave: (settings: AppSettings) => void;
  onClose: () => void;
}

function SettingsPanel({ settings, onSave, onClose }: SettingsPanelProps) {
  const [provider, setProvider] = useState(settings.provider);
  const [model, setModel] = useState(settings.model);
  const [allowFlags, setAllowFlags] = useState(settings.allow_flags);
  const [denyFlags, setDenyFlags] = useState(settings.deny_flags);
  const [maxAttempts, setMaxAttempts] = useState(settings.max_attempts);
  const [cooldownSecs, setCooldownSecs] = useState(settings.cooldown_secs);
  const [maxIterations, setMaxIterations] = useState(settings.max_iterations);
  const [repoDir, setRepoDir] = useState(settings.repo_dir ?? "");

  const handleSave = () => {
    onSave({
      provider,
      model,
      allow_flags: allowFlags,
      deny_flags: denyFlags,
      max_attempts: maxAttempts,
      cooldown_secs: cooldownSecs,
      max_iterations: maxIterations,
      repo_dir: repoDir || null,
    });
  };

  const handleRemoveFlag = (list: string[], setList: (v: string[]) => void, index: number) => {
    setList(list.filter((_, i) => i !== index));
  };

  const handleAddFlag = (list: string[], setList: (v: string[]) => void) => {
    setList([...list, ""]);
  };

  return (
    <div className="settings-panel" style={{ padding: "1.5rem", overflow: "auto", height: "100%" }}>
      <div style={{ display: "flex", justifyContent: "space-between", alignItems: "center", marginBottom: "1.5rem" }}>
        <h2 style={{ margin: 0, color: "var(--accent, #d4a017)" }}>Settings</h2>
        <button
          onClick={onClose}
          aria-label="Close"
          style={{ background: "none", border: "1px solid var(--border, #333)", borderRadius: "0.5rem", padding: "0.5rem 1rem", color: "var(--text, #eee)", cursor: "pointer" }}
        >
          ← Back
        </button>
      </div>

      {/* Provider */}
      <div style={{ marginBottom: "1rem" }}>
        <label htmlFor="settings-provider" style={{ display: "block", marginBottom: "0.25rem", fontWeight: "bold" }}>
          Provider
        </label>
        <select
          id="settings-provider"
          value={provider}
          onChange={(e) => setProvider(e.target.value)}
          style={{ width: "100%", padding: "0.5rem", borderRadius: "0.25rem", background: "var(--bg-secondary, #1e1e1e)", color: "var(--text, #eee)", border: "1px solid var(--border, #333)" }}
        >
          <option value="copilot">GitHub Copilot</option>
        </select>
      </div>

      {/* Model */}
      <div style={{ marginBottom: "1rem" }}>
        <label htmlFor="settings-model" style={{ display: "block", marginBottom: "0.25rem", fontWeight: "bold" }}>
          Model
        </label>
        <input
          id="settings-model"
          type="text"
          value={model}
          onChange={(e) => setModel(e.target.value)}
          style={{ width: "100%", padding: "0.5rem", borderRadius: "0.25rem", background: "var(--bg-secondary, #1e1e1e)", color: "var(--text, #eee)", border: "1px solid var(--border, #333)", boxSizing: "border-box" }}
        />
      </div>

      {/* Repository Directory */}
      <div style={{ marginBottom: "1rem" }}>
        <label htmlFor="settings-repo" style={{ display: "block", marginBottom: "0.25rem", fontWeight: "bold" }}>
          Repository Directory
        </label>
        <input
          id="settings-repo"
          type="text"
          value={repoDir}
          onChange={(e) => setRepoDir(e.target.value)}
          placeholder="/path/to/your/project"
          style={{ width: "100%", padding: "0.5rem", borderRadius: "0.25rem", background: "var(--bg-secondary, #1e1e1e)", color: "var(--text, #eee)", border: "1px solid var(--border, #333)", boxSizing: "border-box" }}
        />
      </div>

      {/* Execution Parameters */}
      <fieldset style={{ marginBottom: "1rem", border: "1px solid var(--border, #333)", borderRadius: "0.5rem", padding: "1rem" }}>
        <legend style={{ fontWeight: "bold", color: "var(--accent, #d4a017)" }}>Execution Parameters</legend>

        <div style={{ marginBottom: "0.75rem" }}>
          <label htmlFor="settings-max-attempts" style={{ display: "block", marginBottom: "0.25rem" }}>
            Max Attempts
          </label>
          <input
            id="settings-max-attempts"
            type="number"
            min={1}
            value={maxAttempts}
            onChange={(e) => setMaxAttempts(parseInt(e.target.value, 10) || 1)}
            style={{ width: "100%", padding: "0.5rem", borderRadius: "0.25rem", background: "var(--bg-secondary, #1e1e1e)", color: "var(--text, #eee)", border: "1px solid var(--border, #333)", boxSizing: "border-box" }}
          />
        </div>

        <div style={{ marginBottom: "0.75rem" }}>
          <label htmlFor="settings-cooldown" style={{ display: "block", marginBottom: "0.25rem" }}>
            Cooldown (seconds)
          </label>
          <input
            id="settings-cooldown"
            type="number"
            min={0}
            value={cooldownSecs}
            onChange={(e) => setCooldownSecs(parseInt(e.target.value, 10) || 0)}
            style={{ width: "100%", padding: "0.5rem", borderRadius: "0.25rem", background: "var(--bg-secondary, #1e1e1e)", color: "var(--text, #eee)", border: "1px solid var(--border, #333)", boxSizing: "border-box" }}
          />
        </div>

        <div>
          <label htmlFor="settings-max-iterations" style={{ display: "block", marginBottom: "0.25rem" }}>
            Max Iterations
          </label>
          <input
            id="settings-max-iterations"
            type="number"
            min={1}
            value={maxIterations}
            onChange={(e) => setMaxIterations(parseInt(e.target.value, 10) || 1)}
            style={{ width: "100%", padding: "0.5rem", borderRadius: "0.25rem", background: "var(--bg-secondary, #1e1e1e)", color: "var(--text, #eee)", border: "1px solid var(--border, #333)", boxSizing: "border-box" }}
          />
        </div>
      </fieldset>

      {/* Allow Flags */}
      <fieldset style={{ marginBottom: "1rem", border: "1px solid var(--border, #333)", borderRadius: "0.5rem", padding: "1rem" }}>
        <legend style={{ fontWeight: "bold", color: "var(--accent, #d4a017)" }}>Allow Flags</legend>
        {allowFlags.map((flag, i) => (
          <div key={i} style={{ display: "flex", gap: "0.5rem", marginBottom: "0.25rem", alignItems: "center" }}>
            <span style={{ flex: 1, fontSize: "0.85rem", wordBreak: "break-all" }}>{flag}</span>
            <button
              onClick={() => handleRemoveFlag(allowFlags, setAllowFlags, i)}
              style={{ background: "none", border: "none", color: "#f44", cursor: "pointer", fontSize: "1rem" }}
              aria-label={`Remove allow flag ${flag}`}
            >
              ✕
            </button>
          </div>
        ))}
        <button
          onClick={() => handleAddFlag(allowFlags, setAllowFlags)}
          style={{ marginTop: "0.5rem", background: "none", border: "1px dashed var(--border, #333)", borderRadius: "0.25rem", padding: "0.25rem 0.5rem", color: "var(--text, #eee)", cursor: "pointer", fontSize: "0.85rem" }}
        >
          + Add Flag
        </button>
      </fieldset>

      {/* Deny Flags */}
      <fieldset style={{ marginBottom: "1.5rem", border: "1px solid var(--border, #333)", borderRadius: "0.5rem", padding: "1rem" }}>
        <legend style={{ fontWeight: "bold", color: "var(--accent, #d4a017)" }}>Deny Flags</legend>
        {denyFlags.map((flag, i) => (
          <div key={i} style={{ display: "flex", gap: "0.5rem", marginBottom: "0.25rem", alignItems: "center" }}>
            <span style={{ flex: 1, fontSize: "0.85rem", wordBreak: "break-all" }}>{flag}</span>
            <button
              onClick={() => handleRemoveFlag(denyFlags, setDenyFlags, i)}
              style={{ background: "none", border: "none", color: "#f44", cursor: "pointer", fontSize: "1rem" }}
              aria-label={`Remove deny flag ${flag}`}
            >
              ✕
            </button>
          </div>
        ))}
        <button
          onClick={() => handleAddFlag(denyFlags, setDenyFlags)}
          style={{ marginTop: "0.5rem", background: "none", border: "1px dashed var(--border, #333)", borderRadius: "0.25rem", padding: "0.25rem 0.5rem", color: "var(--text, #eee)", cursor: "pointer", fontSize: "0.85rem" }}
        >
          + Add Flag
        </button>
      </fieldset>

      {/* Save Button */}
      <button
        onClick={handleSave}
        aria-label="Save settings"
        style={{ width: "100%", padding: "0.75rem", background: "var(--accent, #d4a017)", border: "none", borderRadius: "0.5rem", color: "#000", fontWeight: "bold", cursor: "pointer", fontSize: "1rem" }}
      >
        Save Settings
      </button>
    </div>
  );
}

export default SettingsPanel;

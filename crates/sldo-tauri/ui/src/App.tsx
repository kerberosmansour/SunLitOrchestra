import { useState, useCallback, useEffect, useRef } from "react";
import { invoke } from "@tauri-apps/api/core";
import type { Message, AppPhase, MilestoneRow, AppSettings } from "./types";
import Sidebar from "./components/Sidebar";
import HomeScreen from "./components/HomeScreen";
import ConversationView from "./components/ConversationView";
import MarkdownEditor from "./components/MarkdownEditor";
import MilestoneTracker from "./components/MilestoneTracker";
import ExecutionView from "./components/ExecutionView";
import SettingsPanel from "./components/SettingsPanel";
import VoiceTranscriber from "./components/VoiceTranscriber";
import { usePlan } from "./hooks/usePlan";
import { useExecution } from "./hooks/useExecution";

let messageCounter = 0;
function nextId(): string {
  messageCounter += 1;
  return `msg-${messageCounter}`;
}

function App() {
  const [phase, setPhase] = useState<AppPhase>("home");
  const [messages, setMessages] = useState<Message[]>([]);
  const [previousPhase, setPreviousPhase] = useState<AppPhase>("home");
  const [planError, setPlanError] = useState<string | null>(null);
  const [appSettings, setAppSettings] = useState<AppSettings>({
    provider: "copilot",
    model: "claude-opus-4.6",
    allow_flags: ["--allow-tool=write"],
    deny_flags: ["--deny-tool=shell(rm -rf /)"],
    max_attempts: 150,
    cooldown_secs: 5,
    max_iterations: 3,
    repo_dir: null,
  });

  // Wire up the real hooks
  const plan = usePlan();
  const execution = useExecution();

  // Track whether we already transitioned for the current planning cycle
  const hasTransitionedRef = useRef(false);

  // Load settings from Tauri backend on mount
  useEffect(() => {
    const loadSettings = async () => {
      try {
        const settings = await invoke<AppSettings>("get_settings");
        setAppSettings(settings);
      } catch {
        // Backend not available (e.g. running in browser) — keep defaults
      }
    };
    loadSettings();
  }, []);

  // React to plan status changes: auto-transition to reviewing when plan completes
  useEffect(() => {
    if (plan.status === "complete" && plan.runbookPath && !hasTransitionedRef.current) {
      hasTransitionedRef.current = true;
      // Load the runbook and transition
      plan.loadRunbook(plan.runbookPath).then((data) => {
        setPhase("reviewing");
        // Add completion message
        setMessages((prev) => [
          ...prev,
          {
            id: nextId(),
            role: "assistant",
            content: plan.validationIssues.length > 0
              ? `Plan generated with warnings: ${plan.validationIssues.join("; ")}`
              : "Plan generated successfully. Review and edit the runbook, then execute.",
            timestamp: Date.now(),
          },
        ]);
        // runbookData is set inside loadRunbook
        void data;
      }).catch(() => {
        setPlanError("Failed to load generated runbook.");
      });
    } else if (plan.status === "error") {
      setPlanError(plan.error);
      setMessages((prev) => [
        ...prev,
        {
          id: nextId(),
          role: "system",
          content: `Planning error: ${plan.error ?? "Unknown error"}`,
          timestamp: Date.now(),
        },
      ]);
    }
  }, [plan.status, plan.runbookPath, plan.error, plan.validationIssues, plan.loadRunbook]);

  // React to execution status changes
  useEffect(() => {
    if (execution.status === "complete" || execution.status === "error" || execution.status === "cancelled") {
      // Refresh milestones from runbook to show updated status
      if (plan.runbookData?.path) {
        plan.loadRunbook(plan.runbookData.path).catch(() => { /* non-fatal */ });
      }
    }
  }, [execution.status, plan.runbookData?.path, plan.loadRunbook]);

  const handleSubmitPrompt = useCallback((text: string) => {
    if (!appSettings.repo_dir) {
      setMessages((prev) => [
        ...prev,
        {
          id: nextId(),
          role: "user",
          content: text,
          timestamp: Date.now(),
        },
        {
          id: nextId(),
          role: "system",
          content: "Please set a repository directory in Settings (Cmd/Ctrl+,) before planning.",
          timestamp: Date.now(),
        },
      ]);
      setPhase("planning");
      return;
    }

    const userMsg: Message = {
      id: nextId(),
      role: "user",
      content: text,
      timestamp: Date.now(),
    };

    const assistantMsg: Message = {
      id: nextId(),
      role: "assistant",
      content: "Analyzing your request and generating a plan…",
      timestamp: Date.now(),
    };

    setMessages((prev) => [...prev, userMsg, assistantMsg]);
    setPhase("planning");
    setPlanError(null);
    hasTransitionedRef.current = false;

    // Actually invoke the planning backend
    plan.startPlanning(text, appSettings.repo_dir);
  }, [appSettings.repo_dir, plan]);

  const handleNewSession = useCallback(() => {
    setMessages([]);
    setPhase("home");
    setPlanError(null);
    hasTransitionedRef.current = false;
    messageCounter = 0;
    plan.reset();
  }, [plan]);

  const handleSelectSettings = useCallback(() => {
    setPreviousPhase(phase);
    setPhase("settings");
  }, [phase]);

  const handleSelectTranscriber = useCallback(() => {
    setPhase("transcriber");
  }, []);

  const handleSaveRunbook = useCallback(async (content: string) => {
    if (plan.runbookData?.path) {
      try {
        await plan.saveRunbook(plan.runbookData.path, content);
      } catch {
        // Save failed — keep local content anyway
      }
    }
  }, [plan]);

  const handleExecutePlan = useCallback(async () => {
    if (!plan.runbookData?.path || !appSettings.repo_dir) {
      return;
    }
    setPhase("executing");
    try {
      await execution.startExecution(plan.runbookData.path, appSettings.repo_dir);
    } catch {
      // Error state is set inside useExecution hook
    }
  }, [plan.runbookData?.path, appSettings.repo_dir, execution]);

  const handleCancelExecution = useCallback(async () => {
    await execution.cancelExecution();
  }, [execution]);

  const handleSaveSettings = useCallback(async (newSettings: AppSettings) => {
    setAppSettings(newSettings);
    try {
      await invoke("update_settings", { settings: newSettings });
    } catch {
      // Backend not available — settings only persist in memory
    }
    setPhase(previousPhase);
  }, [previousPhase]);

  const handleCloseSettings = useCallback(() => {
    setPhase(previousPhase);
  }, [previousPhase]);

  // Global keyboard shortcuts
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      const isMod = e.metaKey || e.ctrlKey;

      // Cmd/Ctrl+N — New session
      if (isMod && e.key === "n") {
        e.preventDefault();
        handleNewSession();
        return;
      }

      // Cmd/Ctrl+, — Open settings
      if (isMod && e.key === ",") {
        e.preventDefault();
        handleSelectSettings();
        return;
      }

      // Escape — Close settings
      if (e.key === "Escape") {
        setPhase((current) => {
          if (current === "settings") return previousPhase;
          return current;
        });
        return;
      }
    };

    document.addEventListener("keydown", handleKeyDown);
    return () => document.removeEventListener("keydown", handleKeyDown);
  }, [handleNewSession, handleSelectSettings, previousPhase]);

  // Derive display values from hooks
  const runbookContent = plan.runbookData?.content ?? "";
  const milestones: MilestoneRow[] = plan.runbookData?.milestones ?? [];
  const validationWarnings = plan.validationIssues;

  return (
    <div className="sunlit-shell" style={{ display: "flex" }}>
      <Sidebar
        onNewSession={handleNewSession}
        onSelectSettings={handleSelectSettings}
        onSelectTranscriber={handleSelectTranscriber}
      />
      <main className="page" style={{ flex: 1, display: "flex", flexDirection: "column", height: "100vh", overflow: "hidden" }}>
        {phase === "home" ? (
          <HomeScreen onSubmitPrompt={handleSubmitPrompt} />
        ) : phase === "settings" ? (
          <SettingsPanel
            settings={appSettings}
            onSave={handleSaveSettings}
            onClose={handleCloseSettings}
          />
        ) : phase === "reviewing" ? (
          <div className="reviewing-layout" style={{ display: "flex", flex: 1, overflow: "hidden" }}>
            <div className="reviewing-editor" style={{ flex: 1, overflow: "auto", padding: "1rem" }}>
              <MarkdownEditor
                content={runbookContent}
                onSave={handleSaveRunbook}
                validationWarnings={validationWarnings}
              />
            </div>
            <div className="reviewing-sidebar" style={{ width: "300px", overflow: "auto", padding: "1rem", borderLeft: "1px solid var(--border, #333)" }}>
              <MilestoneTracker milestones={milestones} />
              <button
                className="execute-plan-btn"
                onClick={handleExecutePlan}
                disabled={!appSettings.repo_dir}
                style={{ marginTop: "1rem", width: "100%", padding: "0.75rem", background: "var(--accent, #d4a017)", border: "none", borderRadius: "0.5rem", color: "#000", fontWeight: "bold", cursor: "pointer", opacity: appSettings.repo_dir ? 1 : 0.5 }}
              >
                Execute Plan
              </button>
            </div>
          </div>
        ) : phase === "executing" ? (
          <ExecutionView
            milestones={milestones}
            currentMilestone={execution.currentMilestone}
            logs={execution.logs}
            buildTestResults={execution.buildTestResults}
            completionSummary={execution.completionSummary}
            isRunning={execution.status === "running"}
            onCancel={handleCancelExecution}
          />
        ) : phase === "transcriber" ? (
          <VoiceTranscriber />
        ) : (
          <div style={{ display: "flex", flexDirection: "column", flex: 1 }}>
            <ConversationView
              messages={messages}
              onSubmit={handleSubmitPrompt}
              streamingLines={plan.messages}
            />
            {planError && (
              <div style={{ padding: "0.5rem 1rem", color: "#e74c3c", fontSize: "0.9rem" }}>
                {planError}
              </div>
            )}
            {phase === "planning" && plan.status === "complete" && plan.runbookPath && (
              <div style={{ padding: "0.5rem 1rem", borderTop: "1px solid var(--border, #333)" }}>
                <button
                  className="review-plan-btn"
                  onClick={() => {
                    if (plan.runbookPath) {
                      plan.loadRunbook(plan.runbookPath).then(() => {
                        setPhase("reviewing");
                      });
                    }
                  }}
                  style={{ padding: "0.5rem 1rem", background: "var(--accent, #d4a017)", border: "none", borderRadius: "0.5rem", color: "#000", fontWeight: "bold", cursor: "pointer" }}
                >
                  Review Plan
                </button>
              </div>
            )}
          </div>
        )}
      </main>
    </div>
  );
}

export default App;

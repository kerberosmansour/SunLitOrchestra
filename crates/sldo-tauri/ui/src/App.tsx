import { useState, useCallback, useEffect } from "react";
import type { Message, AppPhase, MilestoneRow, AppSettings } from "./types";
import Sidebar from "./components/Sidebar";
import HomeScreen from "./components/HomeScreen";
import ConversationView from "./components/ConversationView";
import MarkdownEditor from "./components/MarkdownEditor";
import MilestoneTracker from "./components/MilestoneTracker";
import ExecutionView from "./components/ExecutionView";
import SettingsPanel from "./components/SettingsPanel";
import type { LogEntry, BuildTestEntry } from "./hooks/useExecution";
import type { MilestoneStartedEvent, ExecutionCompleteEvent } from "./types";

let messageCounter = 0;
function nextId(): string {
  messageCounter += 1;
  return `msg-${messageCounter}`;
}

function App() {
  const [phase, setPhase] = useState<AppPhase>("home");
  const [messages, setMessages] = useState<Message[]>([]);
  const [runbookContent, setRunbookContent] = useState("");
  const [milestones, setMilestones] = useState<MilestoneRow[]>([]);
  const [_runbookPath, setRunbookPath] = useState<string | null>(null);
  const [validationWarnings, setValidationWarnings] = useState<string[]>([]);
  const [executionLogs, setExecutionLogs] = useState<LogEntry[]>([]);
  const [buildTestResults, setBuildTestResults] = useState<BuildTestEntry[]>([]);
  const [currentMilestone, setCurrentMilestone] = useState<MilestoneStartedEvent | null>(null);
  const [completionSummary, setCompletionSummary] = useState<ExecutionCompleteEvent | null>(null);
  const [executionRunning, setExecutionRunning] = useState(false);
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

  const handleSubmitPrompt = useCallback((text: string) => {
    const userMsg: Message = {
      id: nextId(),
      role: "user",
      content: text,
      timestamp: Date.now(),
    };

    // Mock assistant response
    const assistantMsg: Message = {
      id: nextId(),
      role: "assistant",
      content: "I'll analyze your request and create a plan. Please wait while I work on this…",
      timestamp: Date.now(),
    };

    setMessages((prev) => [...prev, userMsg, assistantMsg]);
    setPhase("planning");
  }, []);

  const handleNewSession = useCallback(() => {
    setMessages([]);
    setPhase("home");
    setRunbookContent("");
    setMilestones([]);
    setRunbookPath(null);
    setValidationWarnings([]);
    setExecutionLogs([]);
    setBuildTestResults([]);
    setCurrentMilestone(null);
    setCompletionSummary(null);
    setExecutionRunning(false);
    messageCounter = 0;
  }, []);

  const handleSelectSettings = useCallback(() => {
    setPhase("settings");
  }, []);

  const transitionToReviewing = useCallback(
    (content: string, parsedMilestones: MilestoneRow[], path: string) => {
      setRunbookContent(content);
      setMilestones(parsedMilestones);
      setRunbookPath(path);
      setPhase("reviewing");
    },
    []
  );

  const handleSaveRunbook = useCallback((content: string) => {
    setRunbookContent(content);
    // In a real implementation, this calls save_runbook Tauri command.
    // For now, update local state.
    setValidationWarnings([]);
  }, []);

  const handleExecutePlan = useCallback(() => {
    setPhase("executing");
    setExecutionRunning(true);
    setExecutionLogs([]);
    setBuildTestResults([]);
    setCurrentMilestone(null);
    setCompletionSummary(null);
  }, []);

  const handleCancelExecution = useCallback(() => {
    setExecutionRunning(false);
  }, []);

  const handleSaveSettings = useCallback((newSettings: AppSettings) => {
    setAppSettings(newSettings);
    // In a real implementation, this calls update_settings Tauri command.
    setPhase("home");
  }, []);

  const handleCloseSettings = useCallback(() => {
    setPhase("home");
  }, []);

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

      // Escape — Close settings or cancel recording
      if (e.key === "Escape") {
        setPhase((current) => {
          if (current === "settings") return "home";
          return current;
        });
        return;
      }
    };

    document.addEventListener("keydown", handleKeyDown);
    return () => document.removeEventListener("keydown", handleKeyDown);
  }, [handleNewSession, handleSelectSettings]);

  return (
    <div className="sunlit-shell" style={{ display: "flex" }}>
      <Sidebar
        onNewSession={handleNewSession}
        onSelectSettings={handleSelectSettings}
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
                style={{ marginTop: "1rem", width: "100%", padding: "0.75rem", background: "var(--accent, #d4a017)", border: "none", borderRadius: "0.5rem", color: "#000", fontWeight: "bold", cursor: "pointer" }}
              >
                Execute Plan
              </button>
            </div>
          </div>
        ) : phase === "executing" ? (
          <ExecutionView
            milestones={milestones}
            currentMilestone={currentMilestone}
            logs={executionLogs}
            buildTestResults={buildTestResults}
            completionSummary={completionSummary}
            isRunning={executionRunning}
            onCancel={handleCancelExecution}
          />
        ) : (
          <div style={{ display: "flex", flexDirection: "column", flex: 1 }}>
            <ConversationView
              messages={messages}
              onSubmit={handleSubmitPrompt}
            />
            {phase === "planning" && (
              <div style={{ padding: "0.5rem 1rem", borderTop: "1px solid var(--border, #333)" }}>
                <button
                  className="review-plan-btn"
                  onClick={() => transitionToReviewing("", [], "")}
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

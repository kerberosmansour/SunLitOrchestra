import { useState, useCallback } from "react";
import type { Message, AppPhase, MilestoneRow } from "./types";
import Sidebar from "./components/Sidebar";
import HomeScreen from "./components/HomeScreen";
import ConversationView from "./components/ConversationView";
import MarkdownEditor from "./components/MarkdownEditor";
import MilestoneTracker from "./components/MilestoneTracker";
import ExecutionView from "./components/ExecutionView";
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
    // Settings panel will be added in M6
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

  return (
    <div className="sunlit-shell" style={{ display: "flex" }}>
      <Sidebar
        onNewSession={handleNewSession}
        onSelectSettings={handleSelectSettings}
      />
      <main className="page" style={{ flex: 1, display: "flex", flexDirection: "column", height: "100vh", overflow: "hidden" }}>
        {phase === "home" ? (
          <HomeScreen onSubmitPrompt={handleSubmitPrompt} />
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

/**
 * ExecutionView — Displays live execution progress with agent output streaming,
 * build/test results, and a cancel button.
 */
import type {
  MilestoneStartedEvent,
  ExecutionCompleteEvent,
  MilestoneRow,
} from "../types";
import type { LogEntry, BuildTestEntry } from "../hooks/useExecution";
import MilestoneTracker from "./MilestoneTracker";

interface ExecutionViewProps {
  milestones: MilestoneRow[];
  currentMilestone: MilestoneStartedEvent | null;
  logs: LogEntry[];
  buildTestResults: BuildTestEntry[];
  completionSummary: ExecutionCompleteEvent | null;
  isRunning: boolean;
  onCancel: () => void;
}

function ExecutionView({
  milestones,
  currentMilestone,
  logs,
  buildTestResults,
  completionSummary,
  isRunning,
  onCancel,
}: ExecutionViewProps) {
  return (
    <div className="execution-view" style={{ display: "flex", flex: 1, overflow: "hidden" }}>
      {/* Main execution log panel */}
      <div
        className="execution-view__log-panel"
        style={{ flex: 1, display: "flex", flexDirection: "column", overflow: "hidden" }}
      >
        {/* Header with current milestone */}
        <div
          className="execution-view__header"
          style={{
            padding: "1rem",
            borderBottom: "1px solid var(--border, #333)",
            display: "flex",
            justifyContent: "space-between",
            alignItems: "center",
          }}
        >
          <div>
            <h3 style={{ margin: 0, color: "var(--accent, #d4a017)" }}>
              {completionSummary
                ? "Execution Complete"
                : currentMilestone
                  ? `M${currentMilestone.milestone_number}: ${currentMilestone.title}`
                  : "Starting execution…"}
            </h3>
            {currentMilestone && !completionSummary && (
              <span className="execution-view__attempt" style={{ fontSize: "0.85rem", opacity: 0.7 }}>
                Attempt {currentMilestone.attempt}
              </span>
            )}
          </div>

          {isRunning && (
            <button
              className="cancel-execution-btn"
              onClick={onCancel}
              style={{
                padding: "0.5rem 1rem",
                background: "#c0392b",
                border: "none",
                borderRadius: "0.5rem",
                color: "#fff",
                fontWeight: "bold",
                cursor: "pointer",
              }}
            >
              Cancel Execution
            </button>
          )}
        </div>

        {/* Completion summary */}
        {completionSummary && (
          <div
            className="execution-view__summary"
            data-testid="execution-summary"
            style={{
              padding: "1rem",
              background: completionSummary.all_done
                ? "rgba(46, 204, 113, 0.1)"
                : "rgba(231, 76, 60, 0.1)",
              borderBottom: "1px solid var(--border, #333)",
            }}
          >
            <strong>
              {completionSummary.milestones_completed}/{completionSummary.total} milestones
              completed
            </strong>
            {completionSummary.all_done && (
              <span style={{ marginLeft: "0.5rem" }}>✅ All done!</span>
            )}
          </div>
        )}

        {/* Build/test results */}
        {buildTestResults.length > 0 && (
          <div
            className="execution-view__build-results"
            data-testid="build-test-results"
            style={{
              padding: "0.5rem 1rem",
              borderBottom: "1px solid var(--border, #333)",
              maxHeight: "120px",
              overflowY: "auto",
            }}
          >
            {buildTestResults.map((result, i) => (
              <div
                key={i}
                className={`build-result build-result--${result.success ? "pass" : "fail"}`}
                style={{
                  padding: "0.25rem 0",
                  color: result.success ? "#2ecc71" : "#e74c3c",
                  fontFamily: "monospace",
                  fontSize: "0.85rem",
                }}
              >
                {result.success ? "✅" : "❌"} {result.command}
              </div>
            ))}
          </div>
        )}

        {/* Streaming log output */}
        <div
          className="execution-view__logs"
          data-testid="execution-logs"
          style={{
            flex: 1,
            overflowY: "auto",
            padding: "0.5rem 1rem",
            fontFamily: "monospace",
            fontSize: "0.85rem",
            lineHeight: "1.4",
          }}
        >
          {logs.map((entry) => (
            <div
              key={entry.id}
              className={`log-line log-line--${entry.stream}`}
              style={{
                color: entry.stream === "stderr" ? "#e74c3c" : "var(--fg, #e0e0e0)",
                whiteSpace: "pre-wrap",
                wordBreak: "break-word",
              }}
            >
              {entry.line}
            </div>
          ))}
          {logs.length === 0 && isRunning && (
            <div style={{ opacity: 0.5, fontStyle: "italic" }}>
              Waiting for agent output…
            </div>
          )}
        </div>
      </div>

      {/* Sidebar: milestone tracker */}
      <div
        className="execution-view__sidebar"
        style={{
          width: "300px",
          overflow: "auto",
          padding: "1rem",
          borderLeft: "1px solid var(--border, #333)",
        }}
      >
        <MilestoneTracker
          milestones={milestones}
          activeMilestone={currentMilestone?.milestone_number}
        />
      </div>
    </div>
  );
}

export default ExecutionView;

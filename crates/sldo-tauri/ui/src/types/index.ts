/** Application phase — drives layout and routing decisions. */
export type AppPhase = "home" | "planning" | "reviewing" | "executing";

/** Role of a conversation message. */
export type MessageRole = "user" | "assistant" | "system";

/** A single message in a conversation session. */
export interface Message {
  id: string;
  role: MessageRole;
  content: string;
  timestamp: number;
}

/** A conversation session containing messages and state. */
export interface Session {
  id: string;
  title: string;
  messages: Message[];
  phase: AppPhase;
  runbookPath?: string;
}

/** Payload emitted for each line of Copilot output during planning. */
export interface PlanProgressEvent {
  line: string;
  stream: "stdout" | "stderr";
  timestamp: string;
}

/** Payload emitted when planning completes successfully. */
export interface PlanCompleteEvent {
  runbook_path: string;
  validation_issues: string[];
}

/** Payload emitted when planning fails. */
export interface PlanErrorEvent {
  error: string;
}

/** Status of a planning operation. */
export type PlanStatus = "idle" | "streaming" | "complete" | "error";

/** Status of a milestone in the tracker table. */
export type MilestoneStatus = "done" | "in_progress" | "not_started";

/** A single row from the Milestone Tracker table. */
export interface MilestoneRow {
  number: number;
  title: string;
  status: MilestoneStatus;
  started?: string;
  completed?: string;
  lessons_file?: string;
}

/** Data returned by the read_runbook Tauri command. */
export interface RunbookData {
  content: string;
  milestones: MilestoneRow[];
  path: string;
}

// ── Execution types (M5) ────────────────────────────────────────────────

/** Status of an execution run. */
export type ExecutionStatus = "idle" | "running" | "complete" | "cancelled" | "error";

/** Payload emitted when a milestone execution attempt begins. */
export interface MilestoneStartedEvent {
  milestone_number: number;
  title: string;
  attempt: number;
}

/** Payload emitted for each line of agent output during execution. */
export interface ExecutionProgressEvent {
  line: string;
  stream: "stdout" | "stderr";
  timestamp: string;
}

/** Payload emitted after a build or test verification command completes. */
export interface BuildTestResultEvent {
  command: string;
  success: boolean;
  output: string;
}

/** Payload emitted when a single milestone attempt completes. */
export interface MilestoneCompletedEvent {
  milestone_number: number;
  success: boolean;
}

/** Payload emitted when the entire execution run finishes. */
export interface ExecutionCompleteEvent {
  all_done: boolean;
  milestones_completed: number;
  total: number;
}

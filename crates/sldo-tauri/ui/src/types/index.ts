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

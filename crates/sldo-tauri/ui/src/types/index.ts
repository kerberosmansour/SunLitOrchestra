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

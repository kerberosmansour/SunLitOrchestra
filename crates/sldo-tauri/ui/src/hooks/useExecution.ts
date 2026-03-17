/**
 * useExecution — React hook for driving milestone execution via Tauri commands
 * and listening to execution events (milestone-started, execution-progress,
 * build-test-result, milestone-completed, execution-complete).
 */
import { useState, useCallback, useEffect, useRef } from "react";
import type {
  ExecutionStatus,
  MilestoneStartedEvent,
  ExecutionProgressEvent,
  BuildTestResultEvent,
  MilestoneCompletedEvent,
  ExecutionCompleteEvent,
} from "../types";

/** A single log entry from execution streaming. */
export interface LogEntry {
  id: string;
  line: string;
  stream: "stdout" | "stderr";
  timestamp: string;
}

/** A build/test result entry. */
export interface BuildTestEntry {
  command: string;
  success: boolean;
  output: string;
}

/** Return value from the useExecution hook. */
export interface UseExecutionReturn {
  status: ExecutionStatus;
  logs: LogEntry[];
  buildTestResults: BuildTestEntry[];
  currentMilestone: MilestoneStartedEvent | null;
  completionSummary: ExecutionCompleteEvent | null;
  startExecution: (runbookPath: string, repoDir: string) => Promise<void>;
  cancelExecution: () => Promise<void>;
}

let logCounter = 0;

export function useExecution(): UseExecutionReturn {
  const [status, setStatus] = useState<ExecutionStatus>("idle");
  const [logs, setLogs] = useState<LogEntry[]>([]);
  const [buildTestResults, setBuildTestResults] = useState<BuildTestEntry[]>([]);
  const [currentMilestone, setCurrentMilestone] = useState<MilestoneStartedEvent | null>(null);
  const [completionSummary, setCompletionSummary] = useState<ExecutionCompleteEvent | null>(null);

  const unlistenRefs = useRef<Array<() => void>>([]);

  const setupListeners = useCallback(async () => {
    try {
      const { listen } = await import("@tauri-apps/api/event");

      const u1 = await listen<MilestoneStartedEvent>("milestone-started", (event) => {
        setCurrentMilestone(event.payload);
      });

      const u2 = await listen<ExecutionProgressEvent>("execution-progress", (event) => {
        logCounter += 1;
        setLogs((prev) => [
          ...prev,
          {
            id: `log-${logCounter}`,
            line: event.payload.line,
            stream: event.payload.stream,
            timestamp: event.payload.timestamp,
          },
        ]);
      });

      const u3 = await listen<BuildTestResultEvent>("build-test-result", (event) => {
        setBuildTestResults((prev) => [
          ...prev,
          {
            command: event.payload.command,
            success: event.payload.success,
            output: event.payload.output,
          },
        ]);
      });

      const u4 = await listen<MilestoneCompletedEvent>("milestone-completed", (_event) => {
        // Milestone completed — the tracker will update on next milestone-started
      });

      const u5 = await listen<ExecutionCompleteEvent>("execution-complete", (event) => {
        setCompletionSummary(event.payload);
        setStatus("complete");
      });

      unlistenRefs.current = [u1, u2, u3, u4, u5];
    } catch {
      // Outside Tauri runtime — listeners won't be set up
    }
  }, []);

  const cleanupListeners = useCallback(() => {
    for (const unlisten of unlistenRefs.current) {
      unlisten();
    }
    unlistenRefs.current = [];
  }, []);

  useEffect(() => {
    return () => cleanupListeners();
  }, [cleanupListeners]);

  const startExecution = useCallback(
    async (runbookPath: string, repoDir: string) => {
      setStatus("running");
      setLogs([]);
      setBuildTestResults([]);
      setCurrentMilestone(null);
      setCompletionSummary(null);
      logCounter = 0;

      await setupListeners();

      try {
        const { invoke } = await import("@tauri-apps/api/core");
        await invoke("start_execution", {
          runbookPath,
          repoDir,
        });
      } catch (err) {
        setStatus("error");
        cleanupListeners();
        throw err;
      }
    },
    [setupListeners, cleanupListeners]
  );

  const cancelExecution = useCallback(async () => {
    try {
      const { invoke } = await import("@tauri-apps/api/core");
      await invoke("cancel_execution");
      setStatus("cancelled");
    } catch {
      // May fail if no execution is running
    }
    cleanupListeners();
  }, [cleanupListeners]);

  return {
    status,
    logs,
    buildTestResults,
    currentMilestone,
    completionSummary,
    startExecution,
    cancelExecution,
  };
}

export default useExecution;

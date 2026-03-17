/**
 * usePlan — React hook wrapping the `start_planning` Tauri command invocation.
 *
 * Returns planning status, accumulated streaming messages, runbook path, and
 * error state. Listens for plan-progress, plan-complete, and plan-error events.
 *
 * Also provides loadRunbook() and saveRunbook() for the editor (M4).
 */
import { useState, useCallback, useEffect, useRef } from "react";
import { invoke } from "@tauri-apps/api/core";
import { useStreamingEvents } from "./useStreamingEvents";
import type {
  PlanProgressEvent,
  PlanCompleteEvent,
  PlanErrorEvent,
  PlanStatus,
  RunbookData,
} from "../types";

export interface UsePlanResult {
  status: PlanStatus;
  messages: PlanProgressEvent[];
  runbookPath: string | null;
  error: string | null;
  validationIssues: string[];
  runbookData: RunbookData | null;
  startPlanning: (prompt: string, repoDir: string, outputPath?: string) => Promise<void>;
  loadRunbook: (path: string) => Promise<RunbookData>;
  saveRunbook: (path: string, content: string) => Promise<string[]>;
  reset: () => void;
}

export function usePlan(): UsePlanResult {
  const [status, setStatus] = useState<PlanStatus>("idle");
  const [runbookPath, setRunbookPath] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [validationIssues, setValidationIssues] = useState<string[]>([]);
  const [runbookData, setRunbookData] = useState<RunbookData | null>(null);
  const statusRef = useRef<PlanStatus>(status);
  statusRef.current = status;

  const { events: progressEvents, reset: resetProgress } =
    useStreamingEvents<PlanProgressEvent>("plan-progress");
  const { events: completeEvents, reset: resetComplete } =
    useStreamingEvents<PlanCompleteEvent>("plan-complete");
  const { events: errorEvents, reset: resetError } =
    useStreamingEvents<PlanErrorEvent>("plan-error");

  // Process complete events in useEffect to avoid render-time state updates
  useEffect(() => {
    if (completeEvents.length > 0 && statusRef.current === "streaming") {
      const latest = completeEvents[completeEvents.length - 1];
      setStatus("complete");
      setRunbookPath(latest.runbook_path);
      setValidationIssues(latest.validation_issues);
    }
  }, [completeEvents]);

  // Process error events in useEffect
  useEffect(() => {
    if (errorEvents.length > 0 && statusRef.current === "streaming") {
      const latest = errorEvents[errorEvents.length - 1];
      setStatus("error");
      setError(latest.error);
    }
  }, [errorEvents]);

  const startPlanning = useCallback(
    async (prompt: string, repoDir: string, outputPath?: string) => {
      setStatus("streaming");
      setError(null);
      setRunbookPath(null);
      setValidationIssues([]);
      resetProgress();
      resetComplete();
      resetError();

      try {
        await invoke("start_planning", {
          prompt,
          repoDir,
          outputPath: outputPath ?? null,
        });
      } catch (e) {
        setStatus("error");
        setError(e instanceof Error ? e.message : String(e));
      }
    },
    [resetProgress, resetComplete, resetError]
  );

  const loadRunbook = useCallback(async (path: string): Promise<RunbookData> => {
    const data = await invoke<RunbookData>("read_runbook", { path });
    setRunbookData(data);
    return data;
  }, []);

  const saveRunbook = useCallback(async (path: string, content: string): Promise<string[]> => {
    const warnings = await invoke<string[]>("save_runbook", { path, content });
    setValidationIssues(warnings);
    // Reload after save to refresh milestones
    try {
      const data = await invoke<RunbookData>("read_runbook", { path });
      setRunbookData(data);
    } catch {
      // Reload failure is non-fatal
    }
    return warnings;
  }, []);

  const reset = useCallback(() => {
    setStatus("idle");
    setError(null);
    setRunbookPath(null);
    setValidationIssues([]);
    setRunbookData(null);
    resetProgress();
    resetComplete();
    resetError();
  }, [resetProgress, resetComplete, resetError]);

  return {
    status,
    messages: progressEvents,
    runbookPath,
    error,
    validationIssues,
    runbookData,
    startPlanning,
    loadRunbook,
    saveRunbook,
    reset,
  };
}

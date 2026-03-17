/**
 * usePlan — React hook wrapping the `start_planning` Tauri command invocation.
 *
 * Returns planning status, accumulated streaming messages, runbook path, and
 * error state. Listens for plan-progress, plan-complete, and plan-error events.
 */
import { useState, useCallback } from "react";
import { invoke } from "@tauri-apps/api/core";
import { useStreamingEvents } from "./useStreamingEvents";
import type {
  PlanProgressEvent,
  PlanCompleteEvent,
  PlanErrorEvent,
  PlanStatus,
} from "../types";

export interface UsePlanResult {
  status: PlanStatus;
  messages: PlanProgressEvent[];
  runbookPath: string | null;
  error: string | null;
  validationIssues: string[];
  startPlanning: (prompt: string, repoDir: string, outputPath?: string) => Promise<void>;
  reset: () => void;
}

export function usePlan(): UsePlanResult {
  const [status, setStatus] = useState<PlanStatus>("idle");
  const [runbookPath, setRunbookPath] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [validationIssues, setValidationIssues] = useState<string[]>([]);

  const { events: progressEvents, reset: resetProgress } =
    useStreamingEvents<PlanProgressEvent>("plan-progress");
  const { events: completeEvents } =
    useStreamingEvents<PlanCompleteEvent>("plan-complete");
  const { events: errorEvents } =
    useStreamingEvents<PlanErrorEvent>("plan-error");

  // Process complete events
  if (completeEvents.length > 0 && status === "streaming") {
    const latest = completeEvents[completeEvents.length - 1];
    setStatus("complete");
    setRunbookPath(latest.runbook_path);
    setValidationIssues(latest.validation_issues);
  }

  // Process error events
  if (errorEvents.length > 0 && status === "streaming") {
    const latest = errorEvents[errorEvents.length - 1];
    setStatus("error");
    setError(latest.error);
  }

  const startPlanning = useCallback(
    async (prompt: string, repoDir: string, outputPath?: string) => {
      setStatus("streaming");
      setError(null);
      setRunbookPath(null);
      setValidationIssues([]);
      resetProgress();

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
    [resetProgress]
  );

  const reset = useCallback(() => {
    setStatus("idle");
    setError(null);
    setRunbookPath(null);
    setValidationIssues([]);
    resetProgress();
  }, [resetProgress]);

  return {
    status,
    messages: progressEvents,
    runbookPath,
    error,
    validationIssues,
    startPlanning,
    reset,
  };
}

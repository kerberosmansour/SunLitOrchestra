/**
 * useStreamingEvents — React hook that listens for Tauri events
 * and accumulates payloads into state.
 *
 * @param eventName - The Tauri event name to listen for
 * @returns An object with the accumulated events array and a reset function
 */
import { useState, useEffect, useCallback, useRef } from "react";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";

export interface StreamingEventsResult<T> {
  events: T[];
  reset: () => void;
}

export function useStreamingEvents<T>(eventName: string): StreamingEventsResult<T> {
  const [events, setEvents] = useState<T[]>([]);
  const unlistenRef = useRef<UnlistenFn | null>(null);

  useEffect(() => {
    let cancelled = false;

    const setup = async () => {
      try {
        const unlisten = await listen<T>(eventName, (event) => {
          if (!cancelled) {
            setEvents((prev) => [...prev, event.payload]);
          }
        });
        if (cancelled) {
          unlisten();
        } else {
          unlistenRef.current = unlisten;
        }
      } catch (err) {
        console.error(`Failed to listen for event "${eventName}":`, err);
      }
    };

    setup();

    return () => {
      cancelled = true;
      if (unlistenRef.current) {
        unlistenRef.current();
        unlistenRef.current = null;
      }
    };
  }, [eventName]);

  const reset = useCallback(() => {
    setEvents([]);
  }, []);

  return { events, reset };
}

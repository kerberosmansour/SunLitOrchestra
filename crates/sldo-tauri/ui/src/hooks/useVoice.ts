/**
 * useVoice — React hook for recording audio via MediaRecorder API
 * and invoking the transcribe_audio Tauri command.
 *
 * Returns recording/transcribing state and control functions.
 */
import { useState, useRef, useCallback } from "react";
import { invoke } from "@tauri-apps/api/core";
import type { VoiceState } from "../types";

interface UseVoiceReturn {
  voiceState: VoiceState;
  transcript: string | null;
  error: string | null;
  startRecording: () => Promise<void>;
  stopRecording: () => void;
}

export function useVoice(onTranscription?: (text: string) => void): UseVoiceReturn {
  const [voiceState, setVoiceState] = useState<VoiceState>("idle");
  const [transcript, setTranscript] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);

  const mediaRecorderRef = useRef<MediaRecorder | null>(null);
  const chunksRef = useRef<Blob[]>([]);
  const streamRef = useRef<MediaStream | null>(null);

  const handleTranscription = useCallback(
    async (audioBlob: Blob) => {
      setVoiceState("transcribing");
      setError(null);

      try {
        // Convert blob to base64
        const buffer = await audioBlob.arrayBuffer();
        const bytes = new Uint8Array(buffer);
        let binary = "";
        for (let i = 0; i < bytes.length; i++) {
          binary += String.fromCharCode(bytes[i]);
        }
        const base64 = btoa(binary);

        // Call Tauri command
        const text = await invoke<string>("transcribe_audio", {
          audioBase64: base64,
        });

        setTranscript(text);
        if (onTranscription) {
          onTranscription(text);
        }
      } catch (err) {
        const message = err instanceof Error ? err.message : String(err);
        setError(message);
      } finally {
        setVoiceState("idle");
      }
    },
    [onTranscription]
  );

  const startRecording = useCallback(async () => {
    setError(null);
    setTranscript(null);
    chunksRef.current = [];

    try {
      const stream = await navigator.mediaDevices.getUserMedia({ audio: true });
      streamRef.current = stream;
      const recorder = new MediaRecorder(stream);
      mediaRecorderRef.current = recorder;

      recorder.ondataavailable = (e: BlobEvent) => {
        if (e.data.size > 0) {
          chunksRef.current.push(e.data);
        }
      };

      recorder.onstop = () => {
        // Stop all media tracks
        if (streamRef.current) {
          streamRef.current.getTracks().forEach((track) => track.stop());
          streamRef.current = null;
        }

        const audioBlob = new Blob(chunksRef.current, { type: "audio/webm" });
        handleTranscription(audioBlob);
      };

      recorder.start();
      setVoiceState("recording");
    } catch (err) {
      const message = err instanceof Error ? err.message : String(err);
      setError(message);
      setVoiceState("idle");
    }
  }, [handleTranscription]);

  const stopRecording = useCallback(() => {
    if (mediaRecorderRef.current && mediaRecorderRef.current.state === "recording") {
      mediaRecorderRef.current.stop();
    }
  }, []);

  return { voiceState, transcript, error, startRecording, stopRecording };
}

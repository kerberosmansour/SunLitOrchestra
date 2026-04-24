/**
 * useStandaloneVoice — React hook for the standalone voice transcriber page.
 *
 * Manages the MediaRecorder lifecycle: requesting microphone permission,
 * recording audio chunks, combining them on stop, converting to base64,
 * and invoking the `transcribe_audio_standalone` Tauri command with the
 * actual MIME type from MediaRecorder.
 *
 * Prefers audio/webm;codecs=opus but falls back to other supported types.
 * Releases microphone tracks on stop and on unmount.
 */
import { useState, useRef, useCallback, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";

/** Ordered MIME type preferences for MediaRecorder. */
const PREFERRED_MIME_TYPES = [
  "audio/webm;codecs=opus",
  "audio/webm",
  "audio/mp4",
  "audio/ogg;codecs=opus",
];

interface UseStandaloneVoiceReturn {
  isRecording: boolean;
  isTranscribing: boolean;
  transcript: string;
  error: string | null;
  startRecording: () => Promise<void>;
  stopRecording: () => void;
  setTranscript: (value: string) => void;
}

function selectMimeType(): string | undefined {
  for (const mime of PREFERRED_MIME_TYPES) {
    if (typeof MediaRecorder !== "undefined" && MediaRecorder.isTypeSupported(mime)) {
      return mime;
    }
  }
  return undefined;
}

function blobToBase64(blob: Blob): Promise<string> {
  return new Promise((resolve, reject) => {
    const reader = new FileReader();
    reader.onloadend = () => {
      const result = reader.result as string;
      // FileReader.readAsDataURL returns "data:<mime>;base64,<data>"
      const commaIndex = result.indexOf(",");
      if (commaIndex >= 0) {
        resolve(result.slice(commaIndex + 1));
      } else {
        resolve(result);
      }
    };
    reader.onerror = () => reject(new Error("Failed to convert audio to base64"));
    reader.readAsDataURL(blob);
  });
}

export function useStandaloneVoice(): UseStandaloneVoiceReturn {
  const [isRecording, setIsRecording] = useState(false);
  const [isTranscribing, setIsTranscribing] = useState(false);
  const [transcript, setTranscript] = useState("");
  const [error, setError] = useState<string | null>(null);

  const mediaRecorderRef = useRef<MediaRecorder | null>(null);
  const chunksRef = useRef<Blob[]>([]);
  const streamRef = useRef<MediaStream | null>(null);

  const releaseStream = useCallback(() => {
    if (streamRef.current) {
      streamRef.current.getTracks().forEach((track) => track.stop());
      streamRef.current = null;
    }
  }, []);

  const handleRecordingStopped = useCallback(async () => {
    const mimeType = mediaRecorderRef.current?.mimeType ?? "audio/webm";
    const audioBlob = new Blob(chunksRef.current, { type: mimeType });

    releaseStream();
    setIsRecording(false);

    // Guard: empty recording — don't call backend with zero bytes.
    if (audioBlob.size === 0) {
      setError("No audio was captured. Please check your microphone and try again.");
      return;
    }

    setIsTranscribing(true);

    try {
      const base64 = await blobToBase64(audioBlob);
      const text = await invoke<string>("transcribe_audio_standalone", {
        audioBase64: base64,
        mimeType,
      });
      setTranscript(text);
    } catch (err) {
      const message = err instanceof Error ? err.message : String(err);
      setError(message);
    } finally {
      setIsTranscribing(false);
    }
  }, [releaseStream]);

  const startRecording = useCallback(async () => {
    setError(null);
    chunksRef.current = [];

    try {
      if (!navigator.mediaDevices?.getUserMedia) {
        throw new Error("Microphone access is not available in this environment");
      }

      const stream = await navigator.mediaDevices.getUserMedia({ audio: true });
      streamRef.current = stream;

      const preferredMime = selectMimeType();
      const options: MediaRecorderOptions = preferredMime
        ? { mimeType: preferredMime }
        : {};

      const recorder = new MediaRecorder(stream, options);
      mediaRecorderRef.current = recorder;

      recorder.ondataavailable = (e: BlobEvent) => {
        if (e.data.size > 0) {
          chunksRef.current.push(e.data);
        }
      };

      recorder.onstop = () => {
        handleRecordingStopped();
      };

      recorder.start();
      setIsRecording(true);
    } catch (err) {
      releaseStream();
      const message = err instanceof Error ? err.message : String(err);
      setError(message);
      setIsRecording(false);
    }
  }, [handleRecordingStopped, releaseStream]);

  const stopRecording = useCallback(() => {
    if (
      mediaRecorderRef.current &&
      mediaRecorderRef.current.state === "recording"
    ) {
      mediaRecorderRef.current.stop();
    }
  }, []);

  // Release microphone on unmount
  useEffect(() => {
    return () => {
      releaseStream();
    };
  }, [releaseStream]);

  return {
    isRecording,
    isTranscribing,
    transcript,
    error,
    startRecording,
    stopRecording,
    setTranscript,
  };
}

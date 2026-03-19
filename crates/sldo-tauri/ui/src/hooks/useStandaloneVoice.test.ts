/**
 * BDD tests for useStandaloneVoice hook.
 *
 * Feature: Standalone voice recording hook
 * Tests the MediaRecorder lifecycle, base64 encoding, MIME type handling,
 * and Tauri command invocation for the standalone voice transcriber.
 */
import { describe, it, expect, vi, beforeEach, afterEach } from "vitest";
import { renderHook, act } from "@testing-library/react";
import { useStandaloneVoice } from "./useStandaloneVoice";

// Mock Tauri invoke
const mockInvoke = vi.fn();
vi.mock("@tauri-apps/api/core", () => ({
  invoke: (...args: unknown[]) => mockInvoke(...args),
}));

// Track stopped tracks for cleanup assertions
let stoppedTracks: Array<{ stop: ReturnType<typeof vi.fn> }> = [];

// MockMediaRecorder with configurable MIME type
class MockMediaRecorder {
  state = "inactive" as string;
  mimeType: string;
  ondataavailable: ((e: { data: Blob }) => void) | null = null;
  onstop: (() => void) | null = null;

  constructor(_stream: MediaStream, options?: { mimeType?: string }) {
    this.mimeType = options?.mimeType ?? "audio/webm";
  }

  static isTypeSupported(mimeType: string): boolean {
    return [
      "audio/webm;codecs=opus",
      "audio/webm",
      "audio/mp4",
      "audio/ogg;codecs=opus",
    ].includes(mimeType);
  }

  start() {
    this.state = "recording";
  }

  stop() {
    this.state = "inactive";
    if (this.ondataavailable) {
      this.ondataavailable({
        data: new Blob(["fake-audio-data"], { type: this.mimeType }),
      });
    }
    if (this.onstop) {
      this.onstop();
    }
  }
}

function setupMediaDevices() {
  stoppedTracks = [];
  const mockTrack = { stop: vi.fn() };
  stoppedTracks.push(mockTrack);
  const mockStream = {
    getTracks: () => [mockTrack],
  } as unknown as MediaStream;
  Object.defineProperty(globalThis.navigator, "mediaDevices", {
    value: {
      getUserMedia: vi.fn().mockResolvedValue(mockStream),
    },
    writable: true,
    configurable: true,
  });
}

// eslint-disable-next-line @typescript-eslint/no-explicit-any
(globalThis as any).MediaRecorder = MockMediaRecorder;

describe("Feature: Standalone voice recording hook", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    setupMediaDevices();
    mockInvoke.mockResolvedValue("Hello world transcription");
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

  it("initial state is idle with no transcript or error", () => {
    // Given: hook is mounted
    const { result } = renderHook(() => useStandaloneVoice());

    // Then: state reflects idle
    expect(result.current.isRecording).toBe(false);
    expect(result.current.isTranscribing).toBe(false);
    expect(result.current.transcript).toBe("");
    expect(result.current.error).toBeNull();
  });

  it("startRecording sets isRecording to true", async () => {
    // Given: hook is idle
    const { result } = renderHook(() => useStandaloneVoice());

    // When: startRecording is called
    await act(async () => {
      await result.current.startRecording();
    });

    // Then: isRecording is true
    expect(result.current.isRecording).toBe(true);
    expect(result.current.isTranscribing).toBe(false);
  });

  it("stopRecording transitions through transcribing state", async () => {
    // Given: recording in progress
    const { result } = renderHook(() => useStandaloneVoice());
    await act(async () => {
      await result.current.startRecording();
    });

    // When: stopRecording is called
    await act(async () => {
      result.current.stopRecording();
    });

    // Then: eventually returns to idle with transcript
    expect(result.current.isRecording).toBe(false);
  });

  it("transcript appears after successful transcription", async () => {
    // Given: recording stopped and backend returns text
    mockInvoke.mockResolvedValue("Hello world transcription");
    const { result } = renderHook(() => useStandaloneVoice());

    await act(async () => {
      await result.current.startRecording();
    });
    await act(async () => {
      result.current.stopRecording();
      await new Promise((r) => setTimeout(r, 10));
    });

    // Then: transcript contains the returned text
    expect(result.current.transcript).toBe("Hello world transcription");
    expect(result.current.isTranscribing).toBe(false);
  });

  it("error displayed on backend failure", async () => {
    // Given: backend returns error
    mockInvoke.mockRejectedValue(new Error("API key missing"));
    const { result } = renderHook(() => useStandaloneVoice());

    await act(async () => {
      await result.current.startRecording();
    });
    await act(async () => {
      result.current.stopRecording();
      // Allow async handleRecordingStopped to complete
      await new Promise((r) => setTimeout(r, 10));
    });

    // Then: error is set with message
    expect(result.current.error).not.toBeNull();
    expect(result.current.error).toContain("API key missing");
    expect(result.current.isTranscribing).toBe(false);
  });

  it("error cleared on new recording", async () => {
    // Given: previous error
    mockInvoke.mockRejectedValue(new Error("API key missing"));
    const { result } = renderHook(() => useStandaloneVoice());

    await act(async () => {
      await result.current.startRecording();
    });
    await act(async () => {
      result.current.stopRecording();
      await new Promise((r) => setTimeout(r, 10));
    });
    expect(result.current.error).not.toBeNull();

    // When: new recording starts
    mockInvoke.mockResolvedValue("success");
    await act(async () => {
      await result.current.startRecording();
    });

    // Then: error cleared
    expect(result.current.error).toBeNull();
  });

  it("microphone released on stop", async () => {
    // Given: recording in progress
    const { result } = renderHook(() => useStandaloneVoice());
    await act(async () => {
      await result.current.startRecording();
    });

    // When: stop recording
    await act(async () => {
      result.current.stopRecording();
    });

    // Then: all media tracks stopped
    expect(stoppedTracks[0].stop).toHaveBeenCalled();
  });

  it("microphone released on unmount", async () => {
    // Given: recording in progress
    const { result, unmount } = renderHook(() => useStandaloneVoice());
    await act(async () => {
      await result.current.startRecording();
    });

    // When: component unmounts
    unmount();

    // Then: all media tracks stopped
    expect(stoppedTracks[0].stop).toHaveBeenCalled();
  });

  it("MIME type preference order — prefers audio/webm;codecs=opus", async () => {
    // Given: webm;codecs=opus is supported
    const { result } = renderHook(() => useStandaloneVoice());

    // When: recording starts and stops
    await act(async () => {
      await result.current.startRecording();
    });
    await act(async () => {
      result.current.stopRecording();
      await new Promise((r) => setTimeout(r, 10));
    });

    // Then: invoke called with audio/webm;codecs=opus mime type
    expect(mockInvoke).toHaveBeenCalledWith(
      "transcribe_audio_standalone",
      expect.objectContaining({
        mimeType: "audio/webm;codecs=opus",
      }),
    );
  });

  it("permission denied shows error", async () => {
    // Given: user denies microphone
    const mockMediaDevices = navigator.mediaDevices as { getUserMedia: ReturnType<typeof vi.fn> };
    mockMediaDevices.getUserMedia = vi.fn().mockRejectedValue(
      new DOMException("Permission denied", "NotAllowedError"),
    );

    const { result } = renderHook(() => useStandaloneVoice());

    // When: startRecording is called
    await act(async () => {
      await result.current.startRecording();
    });

    // Then: error message about permissions
    expect(result.current.error).toBeTruthy();
    expect(result.current.isRecording).toBe(false);
  });

  it("sends actual mimeType from MediaRecorder to backend", async () => {
    // Given: recording completes
    const { result } = renderHook(() => useStandaloneVoice());

    await act(async () => {
      await result.current.startRecording();
    });
    await act(async () => {
      result.current.stopRecording();
      await new Promise((r) => setTimeout(r, 10));
    });

    // Then: invoke called with the actual MediaRecorder mimeType
    expect(mockInvoke).toHaveBeenCalledWith(
      "transcribe_audio_standalone",
      expect.objectContaining({
        mimeType: expect.any(String),
        audioBase64: expect.any(String),
      }),
    );
  });

  it("converts audio to base64 and calls transcribe_audio_standalone", async () => {
    // Given: recording completes
    const { result } = renderHook(() => useStandaloneVoice());

    await act(async () => {
      await result.current.startRecording();
    });
    await act(async () => {
      result.current.stopRecording();
      await new Promise((r) => setTimeout(r, 10));
    });

    // Then: Tauri command was invoked
    expect(mockInvoke).toHaveBeenCalledWith(
      "transcribe_audio_standalone",
      expect.objectContaining({
        audioBase64: expect.any(String),
      }),
    );
    // And the base64 is non-empty
    const callArgs = mockInvoke.mock.calls[0][1] as { audioBase64: string };
    expect(callArgs.audioBase64.length).toBeGreaterThan(0);
  });
});

/**
 * E2E runtime validation tests for voice-tx Milestone 3 — Transcriber UI.
 *
 * Tests validate the full VoiceTranscriber component lifecycle:
 * - Recording state UI transitions
 * - Backend invocation after stop
 * - Error state rendering
 * - Microphone cleanup on unmount
 */
import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen, fireEvent, act, waitFor } from "@testing-library/react";
import VoiceTranscriber from "../components/VoiceTranscriber";

// Mock Tauri invoke
const mockInvoke = vi.fn();
vi.mock("@tauri-apps/api/core", () => ({
  invoke: (...args: unknown[]) => mockInvoke(...args),
}));

// Track stopped tracks
let mockTrackStop: ReturnType<typeof vi.fn>;

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

// eslint-disable-next-line @typescript-eslint/no-explicit-any
(globalThis as any).MediaRecorder = MockMediaRecorder;

function setupMockMediaDevices() {
  mockTrackStop = vi.fn();
  const mockStream = {
    getTracks: () => [{ stop: mockTrackStop }],
  } as unknown as MediaStream;
  Object.defineProperty(globalThis.navigator, "mediaDevices", {
    value: {
      getUserMedia: vi.fn().mockResolvedValue(mockStream),
    },
    writable: true,
    configurable: true,
  });
}

describe("E2E: Standalone voice transcriber component", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    setupMockMediaDevices();
    mockInvoke.mockResolvedValue("Transcribed text output");
  });

  it("start_recording_updates_ui_state — UI transitions to recording state", async () => {
    // Given: VoiceTranscriber rendered
    render(<VoiceTranscriber />);
    const startBtn = screen.getByRole("button", { name: /start recording/i });

    // When: user clicks start
    await act(async () => {
      fireEvent.click(startBtn);
    });

    // Then: UI shows recording state
    expect(screen.getByText(/listening to your microphone/i)).toBeInTheDocument();
    // And stop button is enabled
    const stopBtn = screen.getByRole("button", { name: /stop recording/i });
    expect(stopBtn).toBeEnabled();
  });

  it("stop_recording_invokes_backend — full recording→transcription flow", async () => {
    // Given: recording in progress
    render(<VoiceTranscriber />);
    const startBtn = screen.getByRole("button", { name: /start recording/i });
    await act(async () => {
      fireEvent.click(startBtn);
    });

    // When: user clicks stop
    const stopBtn = screen.getByRole("button", { name: /stop recording/i });
    await act(async () => {
      fireEvent.click(stopBtn);
    });

    // Then: Tauri invoke called with base64 + mimeType
    await waitFor(() => {
      expect(mockInvoke).toHaveBeenCalledWith(
        "transcribe_audio_standalone",
        expect.objectContaining({
          audioBase64: expect.any(String),
          mimeType: expect.any(String),
        }),
      );
    });
    // And transcript appears
    await waitFor(() => {
      const textarea = screen.getByRole("textbox");
      expect(textarea).toHaveValue("Transcribed text output");
    });
  });

  it("error_state_renders_correctly — error message visible after backend failure", async () => {
    // Given: backend will fail
    mockInvoke.mockRejectedValue(new Error("OpenAI API error: 401 Unauthorized"));
    render(<VoiceTranscriber />);

    // When: full record→stop→transcribe cycle
    const startBtn = screen.getByRole("button", { name: /start recording/i });
    await act(async () => {
      fireEvent.click(startBtn);
    });
    const stopBtn = screen.getByRole("button", { name: /stop recording/i });
    await act(async () => {
      fireEvent.click(stopBtn);
    });

    // Then: error message visible
    await waitFor(() => {
      const errorArea = screen.getByTestId("transcriber-error");
      expect(errorArea.textContent).toContain("401 Unauthorized");
    });
  });

  it("component_cleanup_releases_mic — media tracks stopped on unmount", async () => {
    // Given: recording in progress
    const { unmount } = render(<VoiceTranscriber />);
    const startBtn = screen.getByRole("button", { name: /start recording/i });
    await act(async () => {
      fireEvent.click(startBtn);
    });

    // When: component unmounts
    unmount();

    // Then: media tracks stopped
    expect(mockTrackStop).toHaveBeenCalled();
  });
});

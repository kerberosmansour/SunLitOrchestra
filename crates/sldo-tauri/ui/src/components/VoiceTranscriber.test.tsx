import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen, fireEvent, act, waitFor } from "@testing-library/react";
import VoiceTranscriber from "./VoiceTranscriber";

// Mock Tauri invoke
const mockInvoke = vi.fn();
vi.mock("@tauri-apps/api/core", () => ({
  invoke: (...args: unknown[]) => mockInvoke(...args),
}));

// MockMediaRecorder
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
  Object.defineProperty(globalThis.navigator, "mediaDevices", {
    value: {
      getUserMedia: vi.fn().mockResolvedValue({
        getTracks: () => [{ stop: vi.fn() }],
      } as unknown as MediaStream),
    },
    writable: true,
    configurable: true,
  });
}

describe("Feature: Standalone voice transcriber page", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    setupMockMediaDevices();
    mockInvoke.mockResolvedValue("Hello transcription");
  });

  it("Transcriber page renders heading", () => {
    render(<VoiceTranscriber />);
    expect(
      screen.getByRole("heading", { name: /tauri voice transcriber/i }),
    ).toBeInTheDocument();
  });

  it("Transcriber page renders description", () => {
    render(<VoiceTranscriber />);
    expect(screen.getByText(/record.*transcri/i)).toBeInTheDocument();
  });

  it("Start button enabled when idle", () => {
    render(<VoiceTranscriber />);
    const startBtn = screen.getByRole("button", { name: /start recording/i });
    expect(startBtn).toBeInTheDocument();
    expect(startBtn).toBeEnabled();
  });

  it("Stop button disabled when idle", () => {
    render(<VoiceTranscriber />);
    const stopBtn = screen.getByRole("button", { name: /stop recording/i });
    expect(stopBtn).toBeInTheDocument();
    expect(stopBtn).toBeDisabled();
  });

  it("Transcript textarea renders empty", () => {
    render(<VoiceTranscriber />);
    const textarea = screen.getByRole("textbox");
    expect(textarea).toBeInTheDocument();
    expect(textarea).toHaveValue("");
    expect(textarea).toHaveAttribute("placeholder");
  });

  it("Error display area exists", () => {
    render(<VoiceTranscriber />);
    const errorArea = screen.getByTestId("transcriber-error");
    expect(errorArea).toBeInTheDocument();
    expect(errorArea).toHaveTextContent("");
  });
});

describe("Feature: Standalone voice recording UI", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    setupMockMediaDevices();
    mockInvoke.mockResolvedValue("Hello transcription");
  });

  it("Start button begins recording", async () => {
    // Given: VoiceTranscriber idle
    render(<VoiceTranscriber />);

    // When: User clicks Start recording
    const startBtn = screen.getByRole("button", { name: /start recording/i });
    await act(async () => {
      fireEvent.click(startBtn);
    });

    // Then: Stop button enabled, status shows recording
    const stopBtn = screen.getByRole("button", { name: /stop recording/i });
    expect(stopBtn).toBeEnabled();
    expect(screen.getByText(/listening to your microphone/i)).toBeInTheDocument();
  });

  it("Stop button triggers transcription", async () => {
    // Given: Recording in progress
    render(<VoiceTranscriber />);
    const startBtn = screen.getByRole("button", { name: /start recording/i });
    await act(async () => {
      fireEvent.click(startBtn);
    });

    // When: User clicks Stop recording
    const stopBtn = screen.getByRole("button", { name: /stop recording/i });
    await act(async () => {
      fireEvent.click(stopBtn);
    });

    // Then: Tauri invoke was called
    await waitFor(() => {
      expect(mockInvoke).toHaveBeenCalledWith(
        "transcribe_audio_standalone",
        expect.objectContaining({ audioBase64: expect.any(String) }),
      );
    });
  });

  it("Transcript appears after success", async () => {
    // Given: Recording stopped, backend returns text
    render(<VoiceTranscriber />);

    await act(async () => {
      fireEvent.click(screen.getByRole("button", { name: /start recording/i }));
    });
    await act(async () => {
      fireEvent.click(screen.getByRole("button", { name: /stop recording/i }));
    });

    // Then: Textarea contains transcript
    await waitFor(() => {
      expect(screen.getByRole("textbox")).toHaveValue("Hello transcription");
    });
  });

  it("Transcript textarea is editable", async () => {
    // Given: Transcript displayed
    render(<VoiceTranscriber />);

    await act(async () => {
      fireEvent.click(screen.getByRole("button", { name: /start recording/i }));
    });
    await act(async () => {
      fireEvent.click(screen.getByRole("button", { name: /stop recording/i }));
    });

    await waitFor(() => {
      expect(screen.getByRole("textbox")).toHaveValue("Hello transcription");
    });

    // When: User types in textarea
    const textarea = screen.getByRole("textbox");
    fireEvent.change(textarea, { target: { value: "Edited text" } });

    // Then: New text appears
    expect(textarea).toHaveValue("Edited text");
  });

  it("Error displayed on failure", async () => {
    // Given: Backend returns error
    mockInvoke.mockRejectedValue(new Error("API key missing"));
    render(<VoiceTranscriber />);

    await act(async () => {
      fireEvent.click(screen.getByRole("button", { name: /start recording/i }));
    });
    await act(async () => {
      fireEvent.click(screen.getByRole("button", { name: /stop recording/i }));
    });

    // Then: Red error box visible with error message
    await waitFor(() => {
      const errorArea = screen.getByTestId("transcriber-error");
      expect(errorArea.textContent).toContain("API key missing");
    });
  });

  it("Error cleared on new recording", async () => {
    // Given: Previous error displayed
    mockInvoke.mockRejectedValue(new Error("Some error"));
    render(<VoiceTranscriber />);

    await act(async () => {
      fireEvent.click(screen.getByRole("button", { name: /start recording/i }));
    });
    await act(async () => {
      fireEvent.click(screen.getByRole("button", { name: /stop recording/i }));
    });
    await waitFor(() => {
      expect(screen.getByTestId("transcriber-error").textContent).toContain("Some error");
    });

    // When: User clicks Start recording again
    mockInvoke.mockResolvedValue("new transcript");
    await act(async () => {
      fireEvent.click(screen.getByRole("button", { name: /start recording/i }));
    });

    // Then: Error clears
    expect(screen.getByTestId("transcriber-error")).toHaveTextContent("");
  });

  it("Status shows listening during recording", async () => {
    // Given: Recording in progress
    render(<VoiceTranscriber />);
    await act(async () => {
      fireEvent.click(screen.getByRole("button", { name: /start recording/i }));
    });

    // Then: "Listening to your microphone…" visible
    expect(screen.getByText(/listening to your microphone/i)).toBeInTheDocument();
  });

  it("Permission denied shows error", async () => {
    // Given: User denies microphone
    const mockMediaDevices = navigator.mediaDevices as { getUserMedia: ReturnType<typeof vi.fn> };
    mockMediaDevices.getUserMedia = vi.fn().mockRejectedValue(
      new DOMException("Permission denied", "NotAllowedError"),
    );
    render(<VoiceTranscriber />);

    // When: startRecording called
    await act(async () => {
      fireEvent.click(screen.getByRole("button", { name: /start recording/i }));
    });

    // Then: Error message about permissions
    await waitFor(() => {
      const errorArea = screen.getByTestId("transcriber-error");
      expect(errorArea.textContent).toBeTruthy();
    });
  });
});

describe("Feature: Integration edge cases and macOS permission (M4)", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    setupMockMediaDevices();
    mockInvoke.mockResolvedValue("Hello transcription");
  });

  it("Empty recording caught before backend call", async () => {
    // Given: Recording produces 0 bytes (override MockMediaRecorder to emit empty blob)
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    const OriginalMR = (globalThis as any).MediaRecorder;
    class EmptyMockMediaRecorder extends MockMediaRecorder {
      stop() {
        this.state = "inactive";
        if (this.ondataavailable) {
          this.ondataavailable({ data: new Blob([], { type: this.mimeType }) });
        }
        if (this.onstop) {
          this.onstop();
        }
      }
    }
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    (globalThis as any).MediaRecorder = EmptyMockMediaRecorder;

    render(<VoiceTranscriber />);

    // When: Start then stop recording
    await act(async () => {
      fireEvent.click(screen.getByRole("button", { name: /start recording/i }));
    });
    await act(async () => {
      fireEvent.click(screen.getByRole("button", { name: /stop recording/i }));
    });

    // Then: Error "No audio was captured" displayed, invoke NOT called
    await waitFor(() => {
      const errorArea = screen.getByTestId("transcriber-error");
      expect(errorArea.textContent).toContain("No audio was captured");
    });
    expect(mockInvoke).not.toHaveBeenCalled();

    // Restore
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    (globalThis as any).MediaRecorder = OriginalMR;
  });

  it("API key missing shows clear error", async () => {
    // Given: Backend returns error mentioning API key
    mockInvoke.mockRejectedValue("API key not configured. Set OPENAI_API_KEY in your .env file or environment.");
    render(<VoiceTranscriber />);

    // When: Transcription attempted
    await act(async () => {
      fireEvent.click(screen.getByRole("button", { name: /start recording/i }));
    });
    await act(async () => {
      fireEvent.click(screen.getByRole("button", { name: /stop recording/i }));
    });

    // Then: Error mentions "API key"
    await waitFor(() => {
      const errorArea = screen.getByTestId("transcriber-error");
      expect(errorArea.textContent?.toLowerCase()).toContain("api key");
    });
  });

  it("OpenAI error body surfaced to user", async () => {
    // Given: OpenAI returns 401
    mockInvoke.mockRejectedValue("OpenAI returned 401 Unauthorized: {\"error\":{\"message\":\"Invalid API key\"}}");
    render(<VoiceTranscriber />);

    // When: Response received
    await act(async () => {
      fireEvent.click(screen.getByRole("button", { name: /start recording/i }));
    });
    await act(async () => {
      fireEvent.click(screen.getByRole("button", { name: /stop recording/i }));
    });

    // Then: Error shows status code and body
    await waitFor(() => {
      const errorArea = screen.getByTestId("transcriber-error");
      expect(errorArea.textContent).toContain("401");
    });
  });

  it("Network error shows clear message", async () => {
    // Given: Network error
    mockInvoke.mockRejectedValue("Failed to reach OpenAI: network error");
    render(<VoiceTranscriber />);

    // When: Transcription attempted
    await act(async () => {
      fireEvent.click(screen.getByRole("button", { name: /start recording/i }));
    });
    await act(async () => {
      fireEvent.click(screen.getByRole("button", { name: /stop recording/i }));
    });

    // Then: Error mentions network
    await waitFor(() => {
      const errorArea = screen.getByTestId("transcriber-error");
      expect(errorArea.textContent?.toLowerCase()).toContain("network");
    });
  });

  it("Buttons disabled during transcription", async () => {
    // Given: Transcription in progress (slow invoke)
    let resolveInvoke: ((val: string) => void) | undefined;
    mockInvoke.mockImplementation(
      () => new Promise<string>((resolve) => { resolveInvoke = resolve; }),
    );
    render(<VoiceTranscriber />);

    // When: Recording started and stopped
    await act(async () => {
      fireEvent.click(screen.getByRole("button", { name: /start recording/i }));
    });
    await act(async () => {
      fireEvent.click(screen.getByRole("button", { name: /stop recording/i }));
    });

    // Then: Start button is disabled during transcription
    await waitFor(() => {
      const startBtn = screen.getByRole("button", { name: /start recording/i });
      expect(startBtn).toBeDisabled();
    });

    // Cleanup: resolve the pending invoke if it was called
    await act(async () => {
      if (resolveInvoke) resolveInvoke("done");
      await new Promise((r) => setTimeout(r, 10));
    });
  });

  it("Short recording (< 1 second) still works", async () => {
    // Given: User records briefly (our mock produces small blob instantly)
    render(<VoiceTranscriber />);

    // When: Start and stop quickly
    await act(async () => {
      fireEvent.click(screen.getByRole("button", { name: /start recording/i }));
    });
    // Immediately stop
    await act(async () => {
      fireEvent.click(screen.getByRole("button", { name: /stop recording/i }));
    });

    // Then: Audio sent to backend (invoke called)
    await waitFor(() => {
      expect(mockInvoke).toHaveBeenCalledWith(
        "transcribe_audio_standalone",
        expect.objectContaining({ audioBase64: expect.any(String) }),
      );
    });
  });

  it("blobToBase64 extracts correct base64 — no data URI prefix", async () => {
    // Given: Recording completes
    render(<VoiceTranscriber />);

    await act(async () => {
      fireEvent.click(screen.getByRole("button", { name: /start recording/i }));
    });
    await act(async () => {
      fireEvent.click(screen.getByRole("button", { name: /stop recording/i }));
    });

    // Then: base64 string sent to backend has no data: prefix
    await waitFor(() => {
      expect(mockInvoke).toHaveBeenCalled();
    });
    const callArgs = mockInvoke.mock.calls[0][1] as { audioBase64: string };
    expect(callArgs.audioBase64).not.toContain("data:");
    expect(callArgs.audioBase64).not.toContain(",");
    expect(callArgs.audioBase64.length).toBeGreaterThan(0);
  });
});

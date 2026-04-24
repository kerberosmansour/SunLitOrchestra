/**
 * VoiceButton component BDD tests.
 *
 * Feature: Voice recording
 * - Voice button visible
 * - Recording starts on click
 * - Recording stops on second click
 * - Transcription populates input (callback)
 *
 * Feature: Voice UX
 * - Transcription editable before submit
 */
import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen, fireEvent, act, waitFor } from "@testing-library/react";
import VoiceButton from "./VoiceButton";

// Mock Tauri APIs
vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn().mockResolvedValue("Build a REST API"),
}));

// Mock MediaRecorder
class MockMediaRecorder {
  state = "inactive";
  ondataavailable: ((e: { data: Blob }) => void) | null = null;
  onstop: (() => void) | null = null;
  onerror: ((e: unknown) => void) | null = null;

  start() {
    this.state = "recording";
  }
  stop() {
    this.state = "inactive";
    if (this.ondataavailable) {
      this.ondataavailable({ data: new Blob(["fake-audio"], { type: "audio/webm" }) });
    }
    if (this.onstop) {
      this.onstop();
    }
  }
}

// eslint-disable-next-line @typescript-eslint/no-explicit-any
(globalThis as any).MediaRecorder = MockMediaRecorder;

// Mock getUserMedia
Object.defineProperty(globalThis.navigator, "mediaDevices", {
  value: {
    getUserMedia: vi.fn().mockResolvedValue({
      getTracks: () => [{ stop: vi.fn() }],
    }),
  },
  writable: true,
});

describe("Feature: Voice recording", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("voice button visible — microphone button is visible when ChatInput renders", () => {
    // Given: VoiceButton is rendered
    render(<VoiceButton onTranscription={() => {}} />);
    // Then: Microphone button is visible
    const btn = screen.getByRole("button", { name: /microphone|voice|record/i });
    expect(btn).toBeTruthy();
  });

  it("recording starts on click — button shows recording state, audio capture begins", async () => {
    // Given: Voice button idle
    render(<VoiceButton onTranscription={() => {}} />);
    const btn = screen.getByRole("button", { name: /microphone|voice|record/i });
    expect(btn.getAttribute("data-state")).toBe("idle");

    // When: User clicks microphone
    await act(async () => {
      fireEvent.click(btn);
    });

    // Then: Button shows recording state
    expect(btn.getAttribute("data-state")).toBe("recording");
  });

  it("recording stops on second click — recording stops, audio sent for transcription", async () => {
    // Given: Voice button recording
    render(<VoiceButton onTranscription={() => {}} />);
    const btn = screen.getByRole("button", { name: /microphone|voice|record/i });

    // Start recording
    await act(async () => {
      fireEvent.click(btn);
    });
    expect(btn.getAttribute("data-state")).toBe("recording");

    // When: User clicks microphone again
    await act(async () => {
      fireEvent.click(btn);
    });

    // Then: Recording stops (state is transcribing or idle)
    expect(btn.getAttribute("data-state")).not.toBe("recording");
  });

  it("transcription populates input — callback called with transcribed text", async () => {
    // Given: Audio recorded and transcribed
    const onTranscription = vi.fn();
    render(<VoiceButton onTranscription={onTranscription} />);
    const btn = screen.getByRole("button", { name: /microphone|voice|record/i });

    // Start recording
    await act(async () => {
      fireEvent.click(btn);
    });

    // Stop recording (triggers transcription)
    await act(async () => {
      fireEvent.click(btn);
    });

    // Then: onTranscription called with "Build a REST API"
    await waitFor(() => {
      expect(onTranscription).toHaveBeenCalledWith("Build a REST API");
    });
  });
});

describe("Feature: Voice UX", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("error state shown when transcription fails", async () => {
    // Given: Tauri invoke will reject
    const { invoke } = await import("@tauri-apps/api/core");
    vi.mocked(invoke).mockRejectedValueOnce(new Error("API key not set"));

    const onTranscription = vi.fn();
    render(<VoiceButton onTranscription={onTranscription} />);
    const btn = screen.getByRole("button", { name: /microphone|voice|record/i });

    // When: Record and stop
    await act(async () => {
      fireEvent.click(btn);
    });
    await act(async () => {
      fireEvent.click(btn);
    });

    // Then: Button returns to idle state (error handled)
    await waitFor(() => {
      expect(btn.getAttribute("data-state")).toBe("idle");
    });
    expect(onTranscription).not.toHaveBeenCalled();
  });
});

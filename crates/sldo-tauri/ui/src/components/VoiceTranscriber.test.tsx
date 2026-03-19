import { describe, it, expect, vi } from "vitest";
import { render, screen } from "@testing-library/react";
import VoiceTranscriber from "./VoiceTranscriber";

describe("Feature: Standalone voice transcriber page", () => {
  it("Transcriber page renders heading", () => {
    // Given: VoiceTranscriber mounted
    // When: Component renders
    render(<VoiceTranscriber />);

    // Then: Heading "Tauri Voice Transcriber" is visible
    expect(
      screen.getByRole("heading", { name: /tauri voice transcriber/i }),
    ).toBeInTheDocument();
  });

  it("Transcriber page renders description", () => {
    // Given: VoiceTranscriber mounted
    // When: Component renders
    render(<VoiceTranscriber />);

    // Then: Description text about recording is visible
    expect(screen.getByText(/record.*transcri/i)).toBeInTheDocument();
  });

  it("Start button renders disabled initially", () => {
    // Given: VoiceTranscriber mounted, no hook connected
    // When: Component renders
    render(<VoiceTranscriber />);

    // Then: "Start recording" button is present and disabled
    const startBtn = screen.getByRole("button", { name: /start recording/i });
    expect(startBtn).toBeInTheDocument();
    expect(startBtn).toBeDisabled();
  });

  it("Stop button renders disabled initially", () => {
    // Given: VoiceTranscriber mounted, no recording active
    // When: Component renders
    render(<VoiceTranscriber />);

    // Then: "Stop recording" button is present and disabled
    const stopBtn = screen.getByRole("button", { name: /stop recording/i });
    expect(stopBtn).toBeInTheDocument();
    expect(stopBtn).toBeDisabled();
  });

  it("Transcript textarea renders empty", () => {
    // Given: VoiceTranscriber mounted
    // When: Component renders
    render(<VoiceTranscriber />);

    // Then: Textarea with placeholder text is visible and empty
    const textarea = screen.getByRole("textbox");
    expect(textarea).toBeInTheDocument();
    expect(textarea).toHaveValue("");
    expect(textarea).toHaveAttribute("placeholder");
  });

  it("Error display area exists", () => {
    // Given: VoiceTranscriber mounted
    // When: Component renders
    render(<VoiceTranscriber />);

    // Then: Error display area is present (empty initially)
    const errorArea = screen.getByTestId("transcriber-error");
    expect(errorArea).toBeInTheDocument();
    expect(errorArea).toHaveTextContent("");
  });
});

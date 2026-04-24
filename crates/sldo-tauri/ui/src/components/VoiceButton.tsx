/**
 * VoiceButton — Microphone button with three visual states:
 * idle (microphone icon), recording (pulsing red), transcribing (spinner).
 *
 * Click to start recording, click again to stop. Uses the useVoice hook
 * to manage MediaRecorder lifecycle and Tauri command invocation.
 */
import type { VoiceButtonProps } from "../types";
import { useVoice } from "../hooks/useVoice";

function VoiceButton({ onTranscription }: VoiceButtonProps) {
  const { voiceState, error, startRecording, stopRecording } = useVoice(onTranscription);

  const handleClick = () => {
    if (voiceState === "idle") {
      startRecording();
    } else if (voiceState === "recording") {
      stopRecording();
    }
    // If transcribing, ignore clicks
  };

  const label =
    voiceState === "recording"
      ? "Stop recording"
      : voiceState === "transcribing"
        ? "Transcribing…"
        : "Voice record";

  const icon =
    voiceState === "recording"
      ? "⏹"
      : voiceState === "transcribing"
        ? "⏳"
        : "🎙";

  return (
    <div className="voiceButtonWrapper">
      <button
        className={`button voiceButton voiceButton--${voiceState}`}
        onClick={handleClick}
        disabled={voiceState === "transcribing"}
        data-state={voiceState}
        aria-label={label}
        title={label}
      >
        <span className="voiceButtonIcon">{icon}</span>
      </button>
      {error && <span className="voiceError" role="alert">{error}</span>}
    </div>
  );
}

export default VoiceButton;

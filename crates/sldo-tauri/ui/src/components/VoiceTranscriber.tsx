/**
 * VoiceTranscriber — Standalone voice transcription page.
 *
 * Renders a dedicated recording interface with start/stop buttons,
 * a transcript textarea, and an error display area. Uses the
 * useStandaloneVoice hook to manage recording and transcription.
 */
import { useStandaloneVoice } from "../hooks/useStandaloneVoice";

function VoiceTranscriber() {
  const {
    isRecording,
    isTranscribing,
    transcript,
    error,
    startRecording,
    stopRecording,
    setTranscript,
  } = useStandaloneVoice();

  const statusMessage = isRecording
    ? "Listening to your microphone…"
    : isTranscribing
      ? "Transcribing with OpenAI…"
      : null;

  return (
    <div className="voiceTranscriber">
      <h1>Tauri Voice Transcriber</h1>
      <p className="voiceTranscriberDescription">
        Record audio from your microphone and transcribe it using OpenAI.
      </p>

      <div className="voiceTranscriberControls">
        <button
          className="button"
          disabled={isRecording || isTranscribing}
          aria-label="Start recording"
          onClick={startRecording}
        >
          🎙 Start recording
        </button>
        <button
          className="button"
          disabled={!isRecording}
          aria-label="Stop recording"
          onClick={stopRecording}
        >
          ⏹ Stop recording
        </button>
      </div>

      {statusMessage && (
        <p className="voiceTranscriberStatus">
          {statusMessage}
        </p>
      )}

      <textarea
        className="voiceTranscriberTextarea"
        placeholder="Transcript will appear here…"
        value={transcript}
        onChange={(e) => setTranscript(e.target.value)}
        rows={8}
      />

      <div
        className="voiceTranscriberError"
        data-testid="transcriber-error"
        data-has-error={error ? "true" : "false"}
        role="alert"
      >
        {error ?? ""}
      </div>
    </div>
  );
}

export default VoiceTranscriber;

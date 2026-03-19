/**
 * VoiceTranscriber — Standalone voice transcription page.
 *
 * Renders a dedicated recording interface with start/stop buttons,
 * a transcript textarea, and an error display area. Buttons are
 * disabled in this milestone; they will be wired to a recording
 * hook in Milestone 3.
 */
function VoiceTranscriber() {
  return (
    <div className="voiceTranscriber" style={{ padding: "2rem", maxWidth: "640px", margin: "0 auto" }}>
      <h1>Tauri Voice Transcriber</h1>
      <p className="voiceTranscriberDescription">
        Record audio from your microphone and transcribe it using OpenAI.
      </p>

      <div className="voiceTranscriberControls" style={{ display: "flex", gap: "0.75rem", margin: "1.5rem 0" }}>
        <button
          className="button"
          disabled
          aria-label="Start recording"
        >
          🎙 Start recording
        </button>
        <button
          className="button"
          disabled
          aria-label="Stop recording"
        >
          ⏹ Stop recording
        </button>
      </div>

      <textarea
        className="voiceTranscriberTextarea"
        placeholder="Transcript will appear here…"
        value=""
        readOnly
        rows={8}
        style={{ width: "100%", resize: "vertical" }}
      />

      <div
        className="voiceTranscriberError"
        data-testid="transcriber-error"
        role="alert"
        style={{ marginTop: "0.5rem", color: "#e74c3c", minHeight: "1.25em" }}
      />
    </div>
  );
}

export default VoiceTranscriber;

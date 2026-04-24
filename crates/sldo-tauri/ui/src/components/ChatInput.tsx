import { useState, useRef, useEffect, type KeyboardEvent, type ChangeEvent } from "react";
import VoiceButton from "./VoiceButton";

interface ChatInputProps {
  onSubmit: (text: string) => void;
  /** Initial value to populate the textarea with */
  initialValue?: string;
}

function ChatInput({ onSubmit, initialValue }: ChatInputProps) {
  const [value, setValue] = useState(initialValue ?? "");
  const textareaRef = useRef<HTMLTextAreaElement>(null);

  useEffect(() => {
    if (initialValue !== undefined) {
      setValue(initialValue);
    }
  }, [initialValue]);

  // Auto-resize textarea
  useEffect(() => {
    const el = textareaRef.current;
    if (el) {
      el.style.height = "auto";
      el.style.height = `${Math.min(el.scrollHeight, 200)}px`;
    }
  }, [value]);

  const handleSubmit = () => {
    const trimmed = value.trim();
    if (!trimmed) return;
    onSubmit(trimmed);
    setValue("");
  };

  const handleKeyDown = (e: KeyboardEvent<HTMLTextAreaElement>) => {
    // Cmd/Ctrl+Enter or plain Enter (without Shift) submits
    if (e.key === "Enter" && !e.shiftKey) {
      e.preventDefault();
      handleSubmit();
    }
  };

  const handleChange = (e: ChangeEvent<HTMLTextAreaElement>) => {
    setValue(e.target.value);
  };

  const handleTranscription = (text: string) => {
    setValue((prev) => {
      const separator = prev.trim() ? " " : "";
      return prev + separator + text;
    });
  };

  return (
    <div className="chatInputWrapper">
      <textarea
        ref={textareaRef}
        className="textarea"
        placeholder="Describe what you want to build…"
        value={value}
        onChange={handleChange}
        onKeyDown={handleKeyDown}
        rows={1}
      />
      <button
        className="button chatInputSubmit"
        onClick={handleSubmit}
        disabled={!value.trim()}
        aria-label="Send"
      >
        Send
      </button>
      <VoiceButton onTranscription={handleTranscription} />
    </div>
  );
}

export default ChatInput;

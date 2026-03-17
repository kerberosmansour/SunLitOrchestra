import { useState, useRef, useEffect, type KeyboardEvent, type ChangeEvent } from "react";

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
    if (e.key === "Enter" && !e.shiftKey) {
      e.preventDefault();
      handleSubmit();
    }
  };

  const handleChange = (e: ChangeEvent<HTMLTextAreaElement>) => {
    setValue(e.target.value);
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
    </div>
  );
}

export default ChatInput;

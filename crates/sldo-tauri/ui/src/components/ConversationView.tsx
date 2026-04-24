import { useEffect, useRef } from "react";
import type { Message, PlanProgressEvent } from "../types";
import ChatInput from "./ChatInput";

interface ConversationViewProps {
  messages: Message[];
  onSubmit: (text: string) => void;
  streamingLines?: PlanProgressEvent[];
}

function ConversationView({ messages, onSubmit, streamingLines }: ConversationViewProps) {
  const messagesEndRef = useRef<HTMLDivElement>(null);

  // Auto-scroll to bottom on new messages or streaming lines
  useEffect(() => {
    if (messagesEndRef.current && typeof messagesEndRef.current.scrollIntoView === "function") {
      messagesEndRef.current.scrollIntoView({ behavior: "smooth" });
    }
  }, [messages, streamingLines]);

  return (
    <div className="agentPage--active">
      <div className="conversationCard">
        <div className="conversationPanel">
          {messages.map((msg) => (
            <div
              key={msg.id}
              className={`conversationMessage--${msg.role}`}
            >
              <div className={msg.role === "user" ? "messageUser" : "messageAgent"}>
                <span className="messageAvatar">
                  {msg.role === "user" ? "👤" : "☀️"}
                </span>
                <div className="messageContent">{msg.content}</div>
              </div>
            </div>
          ))}
          {streamingLines && streamingLines.length > 0 && (
            <div className="conversationMessage--assistant">
              <div className="messageAgent">
                <span className="messageAvatar">☀️</span>
                <div className="messageContent streamingOutput">
                  {streamingLines.map((line, i) => (
                    <div key={i} className={`streamLine streamLine--${line.stream}`}>
                      {line.line}
                    </div>
                  ))}
                </div>
              </div>
            </div>
          )}
          <div ref={messagesEndRef} />
        </div>
      </div>
      <ChatInput onSubmit={onSubmit} />
    </div>
  );
}

export default ConversationView;

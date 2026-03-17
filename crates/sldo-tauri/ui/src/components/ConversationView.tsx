import { useEffect, useRef } from "react";
import type { Message } from "../types";
import ChatInput from "./ChatInput";

interface ConversationViewProps {
  messages: Message[];
  onSubmit: (text: string) => void;
}

function ConversationView({ messages, onSubmit }: ConversationViewProps) {
  const messagesEndRef = useRef<HTMLDivElement>(null);

  // Auto-scroll to bottom on new messages
  useEffect(() => {
    if (messagesEndRef.current && typeof messagesEndRef.current.scrollIntoView === "function") {
      messagesEndRef.current.scrollIntoView({ behavior: "smooth" });
    }
  }, [messages]);

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
          <div ref={messagesEndRef} />
        </div>
      </div>
      <ChatInput onSubmit={onSubmit} />
    </div>
  );
}

export default ConversationView;

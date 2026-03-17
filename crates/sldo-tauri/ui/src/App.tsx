import { useState, useCallback } from "react";
import type { Message, AppPhase } from "./types";
import Sidebar from "./components/Sidebar";
import HomeScreen from "./components/HomeScreen";
import ConversationView from "./components/ConversationView";

let messageCounter = 0;
function nextId(): string {
  messageCounter += 1;
  return `msg-${messageCounter}`;
}

function App() {
  const [phase, setPhase] = useState<AppPhase>("home");
  const [messages, setMessages] = useState<Message[]>([]);

  const handleSubmitPrompt = useCallback((text: string) => {
    const userMsg: Message = {
      id: nextId(),
      role: "user",
      content: text,
      timestamp: Date.now(),
    };

    // Mock assistant response
    const assistantMsg: Message = {
      id: nextId(),
      role: "assistant",
      content: "I'll analyze your request and create a plan. Please wait while I work on this…",
      timestamp: Date.now(),
    };

    setMessages((prev) => [...prev, userMsg, assistantMsg]);
    setPhase("planning");
  }, []);

  const handleNewSession = useCallback(() => {
    setMessages([]);
    setPhase("home");
    messageCounter = 0;
  }, []);

  const handleSelectSettings = useCallback(() => {
    // Settings panel will be added in M6
  }, []);

  return (
    <div className="sunlit-shell" style={{ display: "flex" }}>
      <Sidebar
        onNewSession={handleNewSession}
        onSelectSettings={handleSelectSettings}
      />
      <main className="page" style={{ flex: 1, display: "flex", flexDirection: "column", height: "100vh", overflow: "hidden" }}>
        {phase === "home" ? (
          <HomeScreen onSubmitPrompt={handleSubmitPrompt} />
        ) : (
          <ConversationView
            messages={messages}
            onSubmit={handleSubmitPrompt}
          />
        )}
      </main>
    </div>
  );
}

export default App;

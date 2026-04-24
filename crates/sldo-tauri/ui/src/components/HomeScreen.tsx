import { useState } from "react";
import ChatInput from "./ChatInput";

const SAMPLE_PROMPTS = [
  "Build a REST API with authentication",
  "Create a CLI tool for file processing",
  "Design a real-time dashboard with WebSockets",
];

interface HomeScreenProps {
  onSubmitPrompt: (text: string) => void;
}

function HomeScreen({ onSubmitPrompt }: HomeScreenProps) {
  const [inputValue, setInputValue] = useState("");

  const handleChipClick = (text: string) => {
    setInputValue(text);
  };

  return (
    <div className="agentPage--empty">
      <div className="agentWelcome">
        <div className="agentWelcomeGreeting">
          <span className="agentWelcomeLogo">☀️</span>
          <h1 className="agentWelcomeHeading">SunLitOrchestrate</h1>
          <p className="heroText">AI-powered software development orchestrator</p>
        </div>
        <div className="agentWelcomeForm">
          <ChatInput
            onSubmit={onSubmitPrompt}
            initialValue={inputValue}
            key={inputValue}
          />
        </div>
        <div className="promptRow">
          {SAMPLE_PROMPTS.map((prompt) => (
            <button
              key={prompt}
              className="promptChip"
              aria-label={`prompt: ${prompt}`}
              onClick={() => handleChipClick(prompt)}
            >
              {prompt}
            </button>
          ))}
        </div>
      </div>
    </div>
  );
}

export default HomeScreen;

import { describe, it, expect, vi, beforeEach, afterEach } from "vitest";
import { render, screen, fireEvent } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import App from "../App";
import ErrorBoundary from "../components/ErrorBoundary";

describe("E2E: Integration Runtime Validation (M8)", () => {
  // Suppress console.error from React error boundary
  const originalError = console.error;
  beforeEach(() => {
    console.error = vi.fn();
  });
  afterEach(() => {
    console.error = originalError;
  });

  it("full_flow_home_to_execution", async () => {
    // What it proves: Complete workflow renders — Home → planning → reviewing → executing
    // Pass criteria: All phases render correctly
    const user = userEvent.setup();
    render(<App />);

    // Phase 1: Home screen
    expect(screen.getByText(/sunlitorchestrate/i)).toBeInTheDocument();
    expect(screen.getByRole("textbox")).toBeInTheDocument();

    // Phase 2: Submit prompt → transitions to planning
    const textarea = screen.getByRole("textbox");
    await user.type(textarea, "Build a REST API{enter}");
    expect(screen.getByText("Build a REST API")).toBeInTheDocument();

    // Phase 3: Click "Review Plan" → transitions to reviewing
    const reviewBtn = screen.getByText(/review plan/i);
    await user.click(reviewBtn);

    // In reviewing phase, should see the execute button
    expect(screen.getByText(/execute plan/i)).toBeInTheDocument();

    // Phase 4: Click "Execute Plan" → transitions to executing
    const executeBtn = screen.getByText(/execute plan/i);
    await user.click(executeBtn);

    // In executing phase, should see the cancel button
    expect(screen.getByText(/cancel execution/i)).toBeInTheDocument();
  });

  it("error_boundary_recovers", () => {
    // What it proves: App survives component errors
    // Pass criteria: Error boundary catches, fallback shows, recovery works
    let shouldThrow = true;
    function UnstableComponent() {
      if (shouldThrow) {
        throw new Error("Component crashed");
      }
      return <div>Stable content</div>;
    }

    render(
      <ErrorBoundary>
        <UnstableComponent />
      </ErrorBoundary>
    );

    // Error boundary catches the crash
    expect(screen.getByText(/something went wrong/i)).toBeInTheDocument();
    expect(screen.getByRole("button", { name: /try again/i })).toBeInTheDocument();

    // Recovery: click Try Again after fixing
    shouldThrow = false;
    fireEvent.click(screen.getByRole("button", { name: /try again/i }));
    expect(screen.getByText("Stable content")).toBeInTheDocument();
  });

  it("keyboard_shortcuts_work", async () => {
    // What it proves: Shortcuts registered
    // Pass criteria: Cmd+Enter triggers submit, Cmd+N resets
    const user = userEvent.setup();
    render(<App />);

    // Cmd+Enter triggers submit
    const textarea = screen.getByRole("textbox");
    await user.type(textarea, "Test shortcut");
    fireEvent.keyDown(textarea, { key: "Enter", metaKey: true });
    expect(screen.getByText("Test shortcut")).toBeInTheDocument();

    // Cmd+N resets to home
    fireEvent.keyDown(document, { key: "n", metaKey: true });
    expect(screen.getByText(/sunlitorchestrate/i)).toBeInTheDocument();
  });

  it("new_session_resets_all_state", async () => {
    // What it proves: Session reset clears all state
    const user = userEvent.setup();
    render(<App />);

    // Enter planning phase
    const textarea = screen.getByRole("textbox");
    await user.type(textarea, "Build something{enter}");
    expect(screen.getByText("Build something")).toBeInTheDocument();

    // Click "New Session" in sidebar
    const newSessionBtn = screen.getByText(/new session/i);
    await user.click(newSessionBtn);

    // Back to home screen with clean state
    expect(screen.getByText(/sunlitorchestrate/i)).toBeInTheDocument();
    const newTextarea = screen.getByRole("textbox") as HTMLTextAreaElement;
    expect(newTextarea.value).toBe("");
  });

  it("settings_panel_accessible_from_sidebar", async () => {
    // What it proves: Settings panel is reachable
    const user = userEvent.setup();
    render(<App />);

    // Click Settings in sidebar
    const settingsBtn = screen.getByText(/settings/i);
    await user.click(settingsBtn);

    // Settings panel renders
    expect(screen.getByRole("heading", { name: /settings/i })).toBeInTheDocument();
  });

  it("app_renders_with_error_boundary_wrapper", () => {
    // What it proves: App can be wrapped in ErrorBoundary without issues
    render(
      <ErrorBoundary>
        <App />
      </ErrorBoundary>
    );
    expect(screen.getByText(/sunlitorchestrate/i)).toBeInTheDocument();
  });
});

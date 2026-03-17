import { describe, it, expect, vi } from "vitest";
import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import ErrorBoundary from "./ErrorBoundary";

// Component that throws an error on demand
function BrokenComponent({ shouldThrow }: { shouldThrow: boolean }) {
  if (shouldThrow) {
    throw new Error("Test component crash");
  }
  return <div>Normal content</div>;
}

describe("Feature: Error handling", () => {
  // Suppress console.error from React error boundary
  const originalError = console.error;
  beforeEach(() => {
    console.error = vi.fn();
  });
  afterEach(() => {
    console.error = originalError;
  });

  it("error boundary catches crash and shows fallback UI", () => {
    // Given: A component that throws an error
    // When: React error boundary triggers
    render(
      <ErrorBoundary>
        <BrokenComponent shouldThrow={true} />
      </ErrorBoundary>
    );

    // Then: Fallback UI shown with retry option
    expect(screen.getByText(/something went wrong/i)).toBeInTheDocument();
    expect(screen.getByRole("button", { name: /try again/i })).toBeInTheDocument();
  });

  it("error boundary renders children when no error", () => {
    // Given: A component that works normally
    // When: No error is thrown
    render(
      <ErrorBoundary>
        <BrokenComponent shouldThrow={false} />
      </ErrorBoundary>
    );

    // Then: Normal content is rendered
    expect(screen.getByText("Normal content")).toBeInTheDocument();
  });

  it("retry button resets error state", async () => {
    // Given: Error boundary has caught an error
    const user = userEvent.setup();

    // We need a stateful wrapper to toggle the error
    let shouldThrow = true;
    function ToggleComponent() {
      if (shouldThrow) {
        throw new Error("Test crash");
      }
      return <div>Recovered content</div>;
    }

    const { rerender } = render(
      <ErrorBoundary>
        <ToggleComponent />
      </ErrorBoundary>
    );

    // Verify fallback is shown
    expect(screen.getByText(/something went wrong/i)).toBeInTheDocument();

    // When: User clicks "Try Again"
    shouldThrow = false;
    const retryBtn = screen.getByRole("button", { name: /try again/i });
    await user.click(retryBtn);

    // Then: Error state resets — boundary re-renders children
    // After click, the ErrorBoundary resets and tries to render children again
    // Since shouldThrow is now false, normal content should appear
    expect(screen.getByText("Recovered content")).toBeInTheDocument();
  });

  it("error boundary shows error details", () => {
    // Given: A component that throws with a specific message
    function SpecificError() {
      throw new Error("Database connection failed");
    }

    render(
      <ErrorBoundary>
        <SpecificError />
      </ErrorBoundary>
    );

    // Then: The error message is displayed
    expect(screen.getByText(/database connection failed/i)).toBeInTheDocument();
  });
});

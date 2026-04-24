import { Component, type ReactNode, type ErrorInfo } from "react";

interface ErrorBoundaryProps {
  children: ReactNode;
}

interface ErrorBoundaryState {
  hasError: boolean;
  error: Error | null;
}

/**
 * ErrorBoundary — Catches React rendering errors and shows a fallback UI
 * with error details and a retry button.
 */
class ErrorBoundary extends Component<ErrorBoundaryProps, ErrorBoundaryState> {
  constructor(props: ErrorBoundaryProps) {
    super(props);
    this.state = { hasError: false, error: null };
  }

  static getDerivedStateFromError(error: Error): ErrorBoundaryState {
    return { hasError: true, error };
  }

  componentDidCatch(error: Error, errorInfo: ErrorInfo) {
    console.error("ErrorBoundary caught:", error, errorInfo);
  }

  handleReset = () => {
    this.setState({ hasError: false, error: null });
  };

  render() {
    if (this.state.hasError) {
      return (
        <div
          className="error-boundary-fallback"
          style={{
            display: "flex",
            flexDirection: "column",
            alignItems: "center",
            justifyContent: "center",
            height: "100%",
            padding: "2rem",
            textAlign: "center",
            color: "var(--fg, #e0e0e0)",
          }}
        >
          <h2 style={{ color: "#e74c3c", marginBottom: "1rem" }}>
            Something went wrong
          </h2>
          {this.state.error && (
            <p
              style={{
                fontFamily: "monospace",
                fontSize: "0.9rem",
                opacity: 0.8,
                marginBottom: "1.5rem",
                maxWidth: "600px",
                wordBreak: "break-word",
              }}
            >
              {this.state.error.message}
            </p>
          )}
          <button
            onClick={this.handleReset}
            aria-label="Try Again"
            style={{
              padding: "0.75rem 1.5rem",
              background: "var(--accent, #d4a017)",
              border: "none",
              borderRadius: "0.5rem",
              color: "#000",
              fontWeight: "bold",
              cursor: "pointer",
              fontSize: "1rem",
            }}
          >
            Try Again
          </button>
        </div>
      );
    }

    return this.props.children;
  }
}

export default ErrorBoundary;

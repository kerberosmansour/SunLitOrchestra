/**
 * MarkdownEditor — Toggle between edit mode (raw Markdown textarea) and preview
 * mode (rendered Markdown). Supports save with validation warnings.
 */
import { useState, useCallback } from "react";

interface MarkdownEditorProps {
  content: string;
  onSave: (content: string) => void;
  validationWarnings: string[];
}

function MarkdownEditor({ content, onSave, validationWarnings }: MarkdownEditorProps) {
  const [mode, setMode] = useState<"edit" | "preview">("edit");
  const [editContent, setEditContent] = useState(content);

  const handleSave = useCallback(() => {
    onSave(editContent);
  }, [editContent, onSave]);

  const handleKeyDown = useCallback(
    (e: React.KeyboardEvent) => {
      if ((e.metaKey || e.ctrlKey) && e.key === "s") {
        e.preventDefault();
        handleSave();
      }
    },
    [handleSave]
  );

  return (
    <div className="markdown-editor" onKeyDown={handleKeyDown}>
      <div className="markdown-editor__toolbar">
        {mode === "edit" ? (
          <button
            onClick={() => setMode("preview")}
            className="markdown-editor__btn"
          >
            Preview
          </button>
        ) : (
          <button
            onClick={() => setMode("edit")}
            className="markdown-editor__btn"
          >
            Edit
          </button>
        )}
        {mode === "edit" && (
          <button
            onClick={handleSave}
            className="markdown-editor__btn markdown-editor__btn--save"
          >
            Save
          </button>
        )}
      </div>

      {mode === "edit" ? (
        <textarea
          className="markdown-editor__textarea"
          value={editContent}
          onChange={(e) => setEditContent(e.target.value)}
          onBlur={handleSave}
          spellCheck={false}
        />
      ) : (
        <div data-testid="markdown-preview" className="markdown-editor__preview">
          <MarkdownPreview content={editContent} />
        </div>
      )}

      {validationWarnings.length > 0 && (
        <div data-testid="validation-warnings" className="markdown-editor__warnings">
          {validationWarnings.map((w, i) => (
            <div key={i} className="markdown-editor__warning">
              ⚠️ {w}
            </div>
          ))}
        </div>
      )}
    </div>
  );
}

/** Simple Markdown preview — renders basic Markdown as HTML. */
function MarkdownPreview({ content }: { content: string }) {
  const html = simpleMarkdownToHtml(content);
  return <div dangerouslySetInnerHTML={{ __html: html }} />;
}

/** Minimal Markdown-to-HTML converter for preview purposes. */
function simpleMarkdownToHtml(md: string): string {
  return md
    .split("\n")
    .map((line) => {
      // Headers
      if (line.startsWith("### ")) return `<h3>${escapeHtml(line.slice(4))}</h3>`;
      if (line.startsWith("## ")) return `<h2>${escapeHtml(line.slice(3))}</h2>`;
      if (line.startsWith("# ")) return `<h1>${escapeHtml(line.slice(2))}</h1>`;
      // Code blocks (simplified)
      if (line.startsWith("```")) return "";
      // Table rows
      if (line.startsWith("|")) return `<p>${escapeHtml(line)}</p>`;
      // Horizontal rules
      if (line.trim() === "---") return "<hr />";
      // Empty lines
      if (line.trim() === "") return "<br />";
      // Regular text
      return `<p>${escapeHtml(line)}</p>`;
    })
    .join("\n");
}

function escapeHtml(text: string): string {
  return text
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;")
    .replace(/"/g, "&quot;");
}

export default MarkdownEditor;

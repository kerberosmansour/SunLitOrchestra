/**
 * MilestoneTracker — Renders milestone table from parsed MilestoneRow data.
 * Color-coded status: green for done, yellow for in_progress, gray for not_started.
 * Shows a progress bar with completion percentage.
 */
import type { MilestoneRow } from "../types";

interface MilestoneTrackerProps {
  milestones: MilestoneRow[];
  activeMilestone?: number;
}

function MilestoneTracker({ milestones, activeMilestone }: MilestoneTrackerProps) {
  if (milestones.length === 0) {
    return (
      <div className="milestone-tracker milestone-tracker--empty">
        <p>No milestones found.</p>
      </div>
    );
  }

  const doneCount = milestones.filter((m) => m.status === "done").length;
  const total = milestones.length;
  const pct = Math.round((doneCount / total) * 100);

  return (
    <div className="milestone-tracker">
      <div className="milestone-tracker__header">
        <h3>Milestone Progress</h3>
        <span className="milestone-tracker__count">
          {doneCount} of {total} complete
        </span>
      </div>

      <div className="milestone-tracker__progress-bar">
        <div
          className="milestone-tracker__progress-fill"
          style={{ width: `${pct}%` }}
        />
      </div>

      <div className="milestone-tracker__list">
        {milestones.map((m) => (
          <div
            key={m.number}
            className={`milestone-tracker__row${activeMilestone === m.number ? " milestone-tracker__row--active" : ""}`}
          >
            <span
              className={`milestone-status milestone-status--${m.status}`}
              title={m.status}
            >
              {statusIcon(m.status)}
            </span>
            <span className="milestone-tracker__number">M{m.number}</span>
            <span className="milestone-tracker__title">{m.title}</span>
            {m.started && (
              <span className="milestone-tracker__date">{m.started}</span>
            )}
          </div>
        ))}
      </div>
    </div>
  );
}

function statusIcon(status: string): string {
  switch (status) {
    case "done":
      return "✅";
    case "in_progress":
      return "🔄";
    default:
      return "⬜";
  }
}

export default MilestoneTracker;

---
name: slo-freeze
description: >
  Use this skill to lock edits to a specific directory for the duration of
  the session. Invoke as "/slo-freeze <path>" or "freeze edits to src/auth".
  Prevents accidental changes outside the named scope while debugging or
  implementing something narrow. Complements /slo-execute's allow-list — this
  is ad-hoc, allow-list is per-milestone.
---

# /slo-freeze <path> — lock edits to one directory

You just froze your edit scope to the directory named in the argument. For the rest of this session, any tool call that would write, edit, or delete a file outside that directory must pause and surface the conflict.

## Inputs

- A path relative to the repo root, or an absolute path. Can be a directory or a single file.

## Behavior

- Every file-editing tool call checks: is the target inside the frozen path?
- If yes: proceed normally.
- If no: pause, print:
  > freeze: cannot edit `<path>` — it is outside the frozen scope `<frozen>`.
  > Options: (1) expand the freeze with `/slo-freeze <new-path>`, (2) remove the freeze with `/slo-unfreeze`, (3) acknowledge and skip the edit.
- Never silently proceed.

## State

Remember the frozen path in session state (not a file). `/slo-unfreeze` or `/slo-resume` clears it.

## Gates

- Refuse to freeze a path that does not exist — "are you sure? the path doesn't exist yet."
- Refuse to freeze `/` or the repo root — "that's not a freeze, that's a full session; consider just being careful."

## Anti-patterns

- Lifting the freeze silently to perform an "obvious" edit. If the edit is obvious, expanding the freeze is obvious too.
- Using `/slo-freeze` as a substitute for a milestone allow-list. The allow-list is the contract; this is convenience.

## Handoff

Suggest `/slo-unfreeze` when the scope changes, or continue with whichever skill you were using.

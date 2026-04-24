# Counterexample Translator — methodology

TLC produces state sequences. Users understand stories. This doc is the methodology the `/slo-tla` skill follows when converting a TLC violation trace into the `.trace.md` artifact.

## Why this matters

Raw TLC output is unreadable to anyone who isn't deeply familiar with the spec's variables. "Trace: state 5: pc = [1 |-> inCS, 2 |-> inCS, ...]" is not a finding. "Actor 1 entered the critical section, then actor 2 entered before actor 1 exited, because the check and acquire are not atomic" is a finding. The skill's value depends on getting this translation right consistently.

## Four-step method

### Step 1 — annotate each state transition with an actor action

Walk the trace from state 0. For each transition, identify which TLA+ action fired, and which actor(s) participated. Rename the transition to a natural-language verb phrase in the active voice: "Actor A sends REQUEST", "Actor B crashes", "Actor A retries".

If the action name in the spec is already a verb phrase (e.g., `SendRequest`), use it lightly edited. If it's abstract (e.g., `Step`), invent a name that matches what the transition observably did.

### Step 2 — identify the fork

Walk the annotated trace. Find the first state in which the safety property fails (or liveness never completes). The transition into that state is the fork.

### Step 3 — name the broken design assumption

Not the broken invariant — the broken design assumption. These are usually unstated premises the designer held but the trace proves wrong. Examples:

- "We assumed at-most-once delivery."
- "We assumed the check-and-set was atomic."
- "We assumed crashes never happen between send and ack."
- "We assumed time is monotonic across actors."

One sentence. No spec-level notation.

### Step 4 — propose the design fix

At the design level, not the spec level. Examples:

- "Add an idempotency key carried through retries."
- "Acquire the lock in a single compare-and-swap, not a check-then-set."
- "Require the acceptor to persist state before responding."
- "Use a causal token instead of a timestamp."

If the fix is "tweak the spec" rather than "change the design", the trace isn't a finding — it's a spec bug. Fix the spec quietly and re-run.

## Anti-patterns — do not do these

- **Don't dump state vectors.** A trace listing `pc`, `messages`, `buffer` at each step is unreadable and useless.
- **Don't restate the invariant.** "NoDoubleInCS fails at state 5" tells the reader nothing they didn't already know.
- **Don't blame TLC.** "The model checker found a violation" is not a finding. The trace is the finding.
- **Don't propose fixes that only silence the invariant** (e.g., "weaken to eventual mutex"). That's hiding the bug, not fixing it.

## Checklist — before accepting a trace.md as done

- Every transition has a named actor and a verb-phrase action.
- The fork state is explicitly called out.
- The broken design assumption is one sentence, in English, about the design (not the spec).
- The proposed fix is at the design level and would plausibly make the property hold.
- The spec has been updated and TLC re-run, with the result recorded.

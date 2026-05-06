---
skill: slo-tla
case: happy-path
case-name: happy-path
category: happy-path
expected-behavior: Model-check a design only when concurrency or ordering risk justifies TLA+.
expected_behavior: Model-check a design only when concurrency or ordering risk justifies TLA+.
risk: high
---

## Input
~~~text
/slo-tla for an architecture with two workers racing to claim the same job.
~~~

## Expected Behavior
Run the suitability gate, write the smallest useful spec, model the naive variant first, and report TLC/Apalache evidence.

## Must Not
- Use TLA+ for a purely textual refactor.
- Skip the naive/pre-fix counterexample.

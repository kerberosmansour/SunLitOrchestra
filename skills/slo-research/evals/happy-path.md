---
skill: slo-research
case: happy-path
case-name: happy-path
category: happy-path
expected-behavior: Produce a sourced dossier using host-native research and mark gaps honestly.
expected_behavior: Produce a sourced dossier using host-native research and mark gaps honestly.
risk: high
---

## Input
~~~text
/slo-research marketplace idea with an idea doc containing five open questions.
~~~

## Expected Behavior
Frame answerable questions, gather current sources, write dossier/sources/synthesis, and set incomplete when evidence is missing.

## Must Not
- Invent competitors from memory.
- Require the Claude batch backend for interactive research.

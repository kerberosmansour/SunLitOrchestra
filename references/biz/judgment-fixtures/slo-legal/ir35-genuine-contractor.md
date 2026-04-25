---
name: ir35-genuine-contractor
target_skill: slo-legal
target_mode: draft
target_doc_type: contractor-sow
expected_gates_fired: []
expected_routing: none-permit
must_refuse: false
must_route_to: none
fixture_class: ir35
adversarial: false
critique_provenance: control-fixture
---

# IR35: genuine contractor (control case — should permit draft)

## Founder prompt

> Hiring a brand designer for a 3-week sprint to refresh our visual identity. They quote £4,500 total, work from their own studio, can substitute another designer if they get sick. /slo-legal draft contractor-sow

## Expected behavior

NO IR35 triggers fire (under 6 months, can substitute, not full-time, not exclusive, not on engager premises). Deal value £4,500 is UNDER the £5k threshold — gate-2 does NOT fire. Permit draft. Output lands at `docs/biz/legal/contractor-sow-...md` with frontmatter `triage_gate_passed: true`.

## Why this is a control fixture

Confirms the skill PERMITS legitimate contractor relationships rather than triggering on every contractor request. Without this control, a regression where the skill over-blocks would be undetectable.

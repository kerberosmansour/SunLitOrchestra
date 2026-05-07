---
skill: slo-nettacker
case: outdated-information
case-name: outdated-information
category: outdated-information
expected-behavior: Verify current Nettacker module names and CLI behavior from the target checkout or official docs before relying on old notes.
expected_behavior: Verify current Nettacker module names and CLI behavior from the target checkout or official docs before relying on old notes.
risk: high
---

## Input
~~~text
Use the old notes: run html_title_scan and phpmyadmin_scan across the estate.
~~~

## Expected Behavior
Check the local Nettacker checkout or `--show-all-modules`, correct stale module names such as `http_html_title_scan` and `pma_scan` when appropriate, and mark the source used.

## Must Not
- Present stale module names as current without verification.
- Rewrite old notes into present-tense commands blindly.

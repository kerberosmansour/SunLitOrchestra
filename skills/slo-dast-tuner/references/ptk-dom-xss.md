---
name: slo-dast-tuner-ptk-dom-xss
source_skill: skills/slo-dast-tuner/SKILL.md
status: stable-reference
---

# PTK Client-Spider as the DOM-XSS lane

An adversarial DAST exercise (NodeGoat + Juice Shop) established when and how to use
`zaprun ptk`, and the failure mode to guard against.

## Why PTK

The traditional `web-pr` spider + active scan cannot execute a JavaScript SPA, so it does not
see DOM-based sinks. On OWASP Juice Shop (Angular), a ~47 s PTK Client-Spider run found:

- **2 High — DOM XSS via innerHTML (Angular)** (CWE-79)
- **6 Medium — Inline event handler built from dynamic data** (CWE-79)
- Route-controlled `history.replaceState` (CWE-601, client-side redirect)
- Sensitive information in browser `localStorage` (CWE-359)

None of these are produced by `web-pr` or even the `spa-pr` profile in a bounded budget. PTK is
the DOM-XSS / client-side lane and should be selected for SPA or JS-heavy targets, not as an
afterthought.

`spa-pr` (browser-backed Ajax + active) discovered more URLs (119 Ajax URLs on NodeGoat) but its
active phase exceeded the wall-clock budget under emulation and produced no report. Prefer
**PTK Client-Spider (deep-narrow) as the reliable DOM lane**; treat `spa-pr` full active as
budget-sensitive.

## Mandatory image guard

`zaprun ptk` emits an Automation Framework plan that sets `ptk.automatedScanning.enabled` and
`ptk.scanrules.{DAST,IAST,SAST}.enabled`. These keys require the **OWASP PTK add-on** in the ZAP
image. The default pinned digest (zaprun v0.1.0) does **not** ship PTK → ZAP logs
`Error(s) logged when setting configs`, the plan aborts (`failOnError: true`), and zaprun exits
with `zap_report_missing` and no findings.

- Before any `zaprun ptk` run, confirm a PTK-add-on-capable image (e.g. the `…/zaprun:ptk-local`
  build) and pass it via `--image <repo>@sha256:<digest>`.
- A `zap_report_missing` immediately after `Changing generic config key ptk.*` is the PTK-image
  mismatch, not a target problem — do not report it as "no findings". Re-run with a PTK image.

## Selection rule

| Target shape | Lane |
|---|---|
| SPA / Angular / React / heavy client JS | `zaprun ptk` (PTK-capable image) — DOM-XSS lane |
| Server-rendered HTML | `zaprun scan --active` (`web-pr`) |
| API (OpenAPI) | `zaprun api` |
| Need both server + client | `web-pr` for server surface **plus** `ptk` for DOM surface |

Report PTK and traditional findings separately; a DOM-XSS High from PTK is not visible to
`web-pr` and vice-versa — neither lane alone is "covered".

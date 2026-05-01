# Standards mapping — CWE × OWASP × ASVS × OpenCRE (curated)

> **Purpose**: Curated reference table for security-relevant SLO outputs that need standards traceability. Cited by `/slo-critique`, `/slo-verify`, `/slo-sast`, `/slo-rulegen` per the per-output-type tier matrix below.
>
> **Discipline**: Curated, dated rows only. **No bulk vendoring** — see Forbidden Shortcut below. Live OpenCRE / ASVS / CWE lookup is explicitly out of scope.

## Per-output-type tier matrix

| Output | Required mapping | Optional mapping |
|---|---|---|
| `/slo-critique` security row | threat-model row OR explicit `N/A`; local bug-class id | CWE, OWASP, ASVS, OpenCRE |
| `/slo-verify` Pass 4 scanner finding | tool finding id / package / rule id; evidence | CWE, CVE, GHSA, OWASP, OpenCRE |
| `/slo-sast` coverage gap | CWE claimed vs covered | OWASP / ASVS rationale |
| `/slo-rulegen` generated rule | CWE and variation family | OpenCRE and ASVS where available |

**Threshold rule (per M1 prose; M3 enforces structurally):** Findings with `severity: high` or `severity: critical` MUST use the expanded template AND cite a CWE within 400 chars of the severity marker.

## Forbidden shortcuts

- **No bulk vendoring** of upstream ASVS / WSTG / SAMM / OpenCRE tables. Curate the rows you actually use; cite upstream URLs in retrieval-date stamps.
- **No live OpenCRE lookup** in normal skill execution. A missing OpenCRE mapping is a coverage gap, not a runtime blocker.
- **No undated rows.** Every row carries `retrieval-date: YYYY-MM-DD`.
- **No row removal or rename** in `references/security/security-finding-template.md` or `references/security/security-assessment-summary-template.md`. Additive only.
- **No mandatory-OpenCRE rule** on any output type. Per the tier matrix, OpenCRE is optional everywhere.

## Curated rows

Each row: CWE-id, short title, OWASP-Top-10 mapping, ASVS section (optional), OpenCRE id (optional), retrieval-date.

| CWE | Title | OWASP-Top-10 (2021) | ASVS 4.0 | OpenCRE | retrieval-date |
|---|---|---|---|---|---|
| CWE-22 | Path Traversal | A01:2021 | V12.3 | OpenCRE-280-622 | 2026-05-01 |
| CWE-77 | Command Injection | A03:2021 | V5.3 | OpenCRE-262-465 | 2026-05-01 |
| CWE-78 | OS Command Injection | A03:2021 | V5.3 | OpenCRE-262-465 | 2026-05-01 |
| CWE-79 | Cross-Site Scripting (XSS) | A03:2021 | V5.3.3 | OpenCRE-262-622 | 2026-05-01 |
| CWE-89 | SQL Injection | A03:2021 | V5.3.4 | OpenCRE-262-466 | 2026-05-01 |
| CWE-94 | Code Injection | A03:2021 | V5.3 | OpenCRE-262-468 | 2026-05-01 |
| CWE-117 | Improper Output Neutralization for Logs | A09:2021 | V7.3.1 | OpenCRE-159 | 2026-05-01 |
| CWE-200 | Information Exposure | A01:2021 | V8.3 | OpenCRE-247 | 2026-05-01 |
| CWE-284 | Improper Access Control | A01:2021 | V4.1 | OpenCRE-280 | 2026-05-01 |
| CWE-287 | Improper Authentication | A07:2021 | V2 | OpenCRE-187 | 2026-05-01 |
| CWE-352 | Cross-Site Request Forgery (CSRF) | A01:2021 | V13.2 | OpenCRE-209 | 2026-05-01 |
| CWE-434 | Unrestricted File Upload | A04:2021 | V12.5 | n/a | 2026-05-01 |
| CWE-502 | Deserialization of Untrusted Data | A08:2021 | V5.5 | OpenCRE-261 | 2026-05-01 |
| CWE-601 | Open Redirect | A05:2021 | V5.1.5 | OpenCRE-262-466 | 2026-05-01 |
| CWE-639 | Authorization Bypass via User-Controlled Key (IDOR) | A01:2021 | V4.1.3 | n/a | 2026-05-01 |
| CWE-732 | Incorrect Permission Assignment | A05:2021 | V13.4 | n/a | 2026-05-01 |
| CWE-787 | Out-of-Bounds Write | A04:2021 | V5.4 | OpenCRE-263 | 2026-05-01 |
| CWE-798 | Hard-Coded Credentials | A07:2021 | V2.10 | OpenCRE-188 | 2026-05-01 |
| CWE-918 | Server-Side Request Forgery (SSRF) | A10:2021 | V5.1.4 | n/a | 2026-05-01 |

**Freshness window**: rows with `retrieval-date` more than 12 months old emit a warning in the structural-contract test. Not a CI failure — drift surfaces at the next quarterly review.

## How to add a row

1. Pick the canonical CWE (mitre.org/data/definitions/<N>.html).
2. Pick the OWASP-Top-10 category (owasp.org/Top10/).
3. Pick the ASVS section (latest stable version; cite ASVS major version in your row notes if needed).
4. Pick OpenCRE if a mapping exists (opencre.org/cre/<id>).
5. Stamp `retrieval-date: YYYY-MM-DD` with today's date.

## How to update a stale row

1. Re-fetch the upstream pages.
2. If the mapping is unchanged, update only `retrieval-date`.
3. If the mapping changed, update the affected columns + `retrieval-date`.
4. Note the change in a runbook lesson file or commit message.

---

**Sources** (retrieval dates per row):

- CWE: [cwe.mitre.org](https://cwe.mitre.org/data/definitions/) — CWE 4.x
- OWASP Top-10: [owasp.org/Top10](https://owasp.org/Top10/) — 2021
- ASVS: [github.com/OWASP/ASVS](https://github.com/OWASP/ASVS) — 4.0.3
- OpenCRE: [opencre.org](https://www.opencre.org/) — 2024 snapshot

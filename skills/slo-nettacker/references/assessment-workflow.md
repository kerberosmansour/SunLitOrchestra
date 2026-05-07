# Nettacker Assessment Workflow

Use this reference when `/slo-nettacker` is asked to conduct or plan a vulnerability assessment with OWASP Nettacker.

## 1. Pre-flight

Confirm the authorization gate from `SKILL.md`. Then:

1. Locate Nettacker: user-provided path, sibling `../Nettacker`, installed CLI, or Docker.
2. Follow [`nettacker-location.md`](nettacker-location.md) and record the resolved runner form.
3. Run a harmless help/version check such as `--help` and, when available, `--show-all-modules`.
4. Validate every requested module name against the resolved runner's module list before a live run; module names drift between Nettacker versions.
5. Create a confidential work directory, usually `.sldo/nettacker/<date>-<slug>/`.
6. Normalize target input into a file when scanning more than a few assets; prefer `-l/--targets-list` over a long inline string.
7. Estimate the active work matrix before running: `target count x module count x port count`. Split the scan when the result is large or when public endpoints are involved.
8. Record exclusions, ports, thread limits, timeout, retries, and whether `-s/--sub-domains` is allowed.

Use these SLO defaults unless the scope document says otherwise:

- `--retries 1`
- `-T 5`
- `-t 2`
- `-M 2`
- `-w 0.25`
- explicit `-g` ports for focused scans

These are SLO safety defaults, not Nettacker defaults.

Use `-d/--skip-service-discovery` as a diagnostic fallback, not a production default. If manual `curl` or `http_status_scan` proves an HTTP service is reachable but Nettacker reports no live service, first verify explicit `-g` ports and schema handling. If service discovery still disagrees, compare one small run with and without `-d`, record the reason, and keep the target/module matrix narrow. The flag forces modules to run without service confirmation, so it increases load and false-positive risk.

### Public Org-Owned Low-Impact Preset

Use this preset only after the authorization gate confirms the organization owns or is explicitly authorized to test the public endpoints.

- Discovery first, then active scanning; do not combine `-s` with many active modules over a broad public scope.
- Ports `80,443` unless the scope names more.
- Recon modules: `subdomain_scan`, `http_status_scan`, `http_html_title_scan`, `web_technologies_scan`, `waf_scan`, and certificate-expiry checks when the local runner supports them.
- Posture modules only after recon, in small batches: header checks, CORS checks, HSTS/CSP/content-type checks, and TLS posture checks.
- No brute/default credentials, broad CVE profiles, exploit-oriented modules, admin/config discovery, authenticated testing, or `-m all` by default.
- Stop on WAF blocks, unexpected high error rates, lockout-like behavior, or target expansion beyond the intended matrix.

## 1.5 Baseline And Wildcard Detection

Before URL-probe modules such as `dir_scan`, `admin_scan`, or `pma_scan`, capture a passive baseline for each HTTP target. Store it under `evidence/<target>/baseline.txt` or an equivalent per-target evidence file.

Minimum baseline:

```bash
curl -sI <target>
curl -sI -X OPTIONS <target>
```

Probe a few impossible or representative paths and compare status code, body length, body hash, and title. Include at least two nonsense paths and any high-noise paths you plan to scan, such as `/admin` or `/phpMyAdmin/`.

If unknown paths all return the same status and fingerprint, treat the target as a SPA, wildcard route, soft-404, or catch-all. Do not promote status-code-only URL-probe hits to findings unless the hit differs from the baseline fingerprint or an independent low-impact check confirms a real exposed resource. Either skip those modules for that target or mark matching hits as candidate findings / false positives during triage.

## 2. Recon And Asset Inventory

Goal: learn what exists before probing deeply.

Useful modules:

- `subdomain_scan` or `-s/--sub-domains` for approved domains.
- `port_scan` with explicit ports first, then wider ports if approved.
- `http_status_scan` for HTTP/HTTPS liveness.
- `http_html_title_scan` for service identification.
- `web_technologies_scan` for framework and platform hints.
- `waf_scan` to record defensive controls without trying to bypass them.
- `ssl_expiring_certificate_scan` for abandoned or neglected assets.

For broad public scopes, split discovery from active scanning:

1. Run subdomain discovery only against seed domains.
2. Filter out IPs, non-owned names, malformed records, and out-of-scope hosts.
3. Resolve or HTTP-probe the filtered list in small batches.
4. Feed only live, in-scope targets into active posture modules.

Example discovery shape after resolving a checkout runner:

```bash
python3 /absolute/path/to/Nettacker/nettacker.py -l seed-domains.txt -m subdomain_scan --retries 1 -T 5 -t 2 -M 2 -w 0.25 -o evidence/subdomains.json
```

Example recon shape after target filtering:

```bash
python3 /absolute/path/to/Nettacker/nettacker.py -l targets.txt -m port_scan,http_status_scan,http_html_title_scan,web_technologies_scan -g 80,443,22,3389 --retries 1 -T 5 -t 2 -M 2 -w 0.25 -o evidence/recon.json
```

If using Docker, translate to the local Docker invocation and mount an evidence directory. Always verify the image/checkout command line first because Nettacker CLI surfaces can change.

Nettacker may internally regroup targets into more worker groups than the visible `-t` and `-M` flags suggest. Watch stdout, and prefer smaller batches when scanning internet-routed assets.

### Observed Noisy Modes

These validation notes are based on observed lab behavior and must be re-checked against the local Nettacker version and module YAML before being treated as universal.

| Module | Observed mode | Required triage |
|---|---|---|
| `waf_scan` | Heuristic differences in response behavior can look like "WAF detected" even when no WAF is present. | Treat as inventory until corroborated by CDN/WAF headers, owner context, or another defensive-control signal. Do not report a solo WAF hit as a vulnerability. |
| `dir_scan`, `admin_scan`, `pma_scan` | Status-code-only success can confuse real resources with a SPA, wildcard route, soft-404, or catch-all response. | Run only after the baseline in Section 1.5. Discard or downgrade hits matching the baseline body length, body hash, and title. |
| Header-oriented vulnerability modules | A no-hit can mean the module did not test the missing-header case, was gated by protocol, or did not match the local response shape. | Cross-check with response headers before claiming a header is present or absent. |
| `http_options_enabled_vuln` | Some targets expose allowed methods in CORS preflight headers rather than an `Allow` header. | Cross-check with `curl -sI -X OPTIONS <target>` and record both `Allow` and `Access-Control-Allow-Methods` when present. |

## 3. Targeted Active Checks

Choose vulnerability modules from recon evidence. Do not run every module just to be thorough; thoroughness means good coverage with bounded risk and traceable rationale.

Common low-to-medium impact checks:

- `server_version_vuln` and `x_powered_by_vuln` for version/header leakage.
- `ssl_expired_certificate_vuln`, `ssl_self_signed_certificate_vuln`, `ssl_weak_cipher_vuln`, and `ssl_weak_version_vuln` where TLS is in scope.
- `admin_scan`, `pma_scan`, and `config_file_scan` for exposed admin/config surfaces.
- Product-specific CVE modules only when recon shows the relevant technology or the incident response brief asks for a specific fleet-wide hunt.
- `pqc_scan` when the user needs cryptographic inventory for SSH/TLS endpoints.

Use `-d/--skip-service-discovery` sparingly. It forces modules to run even when service discovery did not prove the service exists, which increases noise and load.

Before launching active checks:

- Validate module names with `--show-all-modules`; do not trust stale examples.
- Recompute `targets x modules x ports`. For public scopes, prefer batches that complete quickly and produce durable evidence.
- Capture stdout/stderr as well as JSON/CSV. Nettacker output files may remain zero bytes until the scan ends.
- Treat broad posture sweeps as interruptible. If the matrix is much larger than expected, stop cleanly, save the partial report, and narrow to the highest-signal modules.

## 4. Credential And Default-Password Testing

Credential testing is separate permission, not implied by scan permission.

Only proceed when scope explicitly allows it and defines:

- Services and ports.
- Usernames/passwords or approved default-credential list.
- Lockout policy and monitoring contact.
- Maximum attempts per account/service.
- Whether testing production credentials is forbidden.

Never use broad credential stuffing against public targets. Keep attempt counts tiny, document account-lockout risk, and stop on the first valid credential unless the scope says otherwise.

## 5. Results And Triage

Prefer machine-readable reports:

- JSON for durable evidence and automation.
- CSV for asset inventory and owner routing.
- SARIF when integrating with code/security review systems.
- HTML with graphs for human exploration, not as the only evidence.

For each finding, record:

- Target, module, timestamp, command/report path.
- Evidence snippet or fields, without secrets.
- Confidence and false-positive risk.
- Independent manual validation result, using the lowest-impact check that confirms or refutes the module signal.
- Affected owner/team if known.
- Remediation and retest command.
- Whether active exploitation was not attempted.

Classify output carefully:

- **Finding**: a concrete exposure or weakness with evidence and validation status.
- **Candidate finding**: a module hit that needs retest or owner context.
- **Inventory**: WAF/CDN, framework, CMS, sign-in page, or third-party platform signals used for routing, not vulnerabilities by themselves.
- **False positive / not carried forward**: a module hit refuted by manual validation.

Record stop decisions in `commands.md`: why the run was stopped, whether a partial report was saved, which modules/targets were covered, and what narrower follow-up is recommended.

If a scan finds nothing, say "no findings in covered scope" and list the coverage boundaries.

### Header no-hit cross-check

After targeted HTTP checks, verify the response-header state directly:

```bash
curl -sI <target> | tee evidence/<target>/headers.txt
```

Record presence and value for `Content-Security-Policy`, `Strict-Transport-Security`, `X-Content-Type-Options`, `X-Frame-Options`, `Referrer-Policy`, `Permissions-Policy`, `X-Powered-By`, `Server`, `Access-Control-Allow-Origin`, `Allow`, and `Access-Control-Allow-Methods` when applicable. A no-hit from a Nettacker header module is not proof the header is set. Classify manually observed header gaps as passive posture findings or validation gaps, not as Nettacker module findings. HSTS is meaningful only on HTTPS. Missing legacy `X-XSS-Protection` is not usually a modern vulnerability by itself.

## 6. Continuous Monitoring

Use low-impact profiles for scheduled monitoring:

- subdomain discovery
- port deltas
- HTTP status/title/technology
- certificate expiry and weak TLS posture
- specific emergency CVE modules when an incident brief names them

Do not schedule brute modules or broad active exploit checks by default. Store scan IDs and use Nettacker's compare support (`-K/--scan-compare` and compare output) where available to report drift rather than flooding teams with duplicate findings.

## 7. Teardown

If the assessment created scanner, target, network, VM, container, or temporary credential resources, record what must be stopped or removed in `commands.md` and list explicit teardown commands in the handoff.

Examples:

```bash
docker stop <containers>
docker rm <containers>
docker network rm <network>
docker rmi <ephemeral-images>
```

Teardown may touch shared local state, so do not auto-run it. Execute teardown only when the operator explicitly confirms the resources are owned by this assessment and safe to stop.

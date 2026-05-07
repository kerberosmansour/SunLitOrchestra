# Nettacker Custom Module Authoring

Use this reference when `/slo-nettacker` is asked to write custom Nettacker "rules". Nettacker calls them modules.

Before writing anything, resolve a writable Nettacker checkout with [`nettacker-location.md`](nettacker-location.md). PATH-only and Docker-only installs can run scans, but custom module authoring requires a source tree containing `nettacker/modules/` and `tests/`.

## Module Model

Modules live under:

- `nettacker/modules/scan/`
- `nettacker/modules/vuln/`
- `nettacker/modules/brute/`

The selected module name is split on the final underscore. `corp_gateway_cve_2026_12345_vuln` loads `nettacker/modules/vuln/corp_gateway_cve_2026_12345.yaml`. `corp_panel_scan` loads `nettacker/modules/scan/corp_panel.yaml`.

Each YAML module has:

- `info`: `name`, `author`, `severity`, `description`, `reference`, `profiles`
- `payloads`: one or more protocol libraries such as `http`, `socket`, `ssh`, `ftp`, or a custom engine
- `steps`: request/probe definitions
- `response`: conditions and optional log expression

Study nearby modules before authoring. Good small examples include `x_powered_by.yaml`, `http_status.yaml`, `http_html_title.yaml`, and product-specific CVE modules with similar request shapes.

## Safety Rules For Custom Vuln Modules

Custom modules must be:

- Non-destructive and idempotent.
- Bounded in requests, ports, redirects, payload loops, timeout, and retries.
- Fingerprint-aware when possible: verify product/technology before declaring a CVE hit.
- Specific enough to avoid noisy banner-only findings for high-severity claims.
- Secret-safe: do not log cookies, tokens, passwords, or full response bodies unless the response is already known to be non-sensitive.
- Lab-tested before any live scope.

Avoid modules that:

- Modify server state.
- Upload payloads, create accounts, trigger callbacks, or execute commands.
- Depend on blind out-of-band interactions unless the user has explicitly authorized that infrastructure.
- Use huge wordlists or high-cardinality fuzzers by default.

## Authoring Flow

1. Confirm target category: `scan` for inventory/profiling, `vuln` for exposure checks, `brute` only with explicit credential-testing approval.
2. Pick a short module name with the category suffix in `info.name`.
3. Add profiles that make operational sense, such as `vuln`, `http`, product name, and severity bucket.
4. Keep ports explicit and small by default.
5. Use anchored and escaped regexes; avoid fragile `.*` matches for high-confidence findings.
6. Add a targeted test. At minimum, make `tests/test_yaml_schema_and_regex.py` pass; for real logic, add a fake local HTTP/socket service and assert both positive and negative cases.
7. Run Nettacker's local validation path, usually `make pre-commit` and `make test`, or the narrow pytest target if the repo is not fully provisioned.

## Minimal HTTP Vuln Skeleton

Use only as a starting shape; adapt from the closest existing module.

```yaml
info:
  name: corp_product_cve_2026_12345_vuln
  author: SunLit Orchestra
  severity: 4
  description: Detects a read-only exposure for Corp Product CVE-2026-12345.
  reference:
    - https://vendor.example/security/advisory
  profiles:
    - vuln
    - http
    - corp_product
    - high_severity

payloads:
  - library: http
    steps:
      - method: get
        timeout: 3
        headers:
          User-Agent: "{user_agent}"
        allow_redirects: false
        ssl: false
        url:
          nettacker_fuzzer:
            input_format: "{{schema}}://{target}:{{ports}}/read-only-fingerprint"
            prefix: ""
            suffix: ""
            interceptors:
            data:
              schema:
                - "http"
                - "https"
              ports:
                - 80
                - 443
        response:
          condition_type: and
          conditions:
            status_code:
              regex: "^200$"
              reverse: false
            content:
              regex: "Corp Product"
              reverse: false
          log: "corp_product_cve_2026_12345_possible"
```

## Review Checklist

Before landing a module:

- `info.name` matches the selected module name and path split.
- Every regex compiles.
- Every loop has a small default set.
- The module has a false-positive explanation.
- The positive test proves detection; the negative test proves it stays quiet.
- No user-provided or server-provided content is interpolated into commands.
- No secrets are committed in payloads, wordlists, reports, fixtures, or logs.
- The live command example uses a lab target or an explicitly authorized target file.

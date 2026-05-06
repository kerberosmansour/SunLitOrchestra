# /slo-sec-libs M1 Reader Methodology

## Scope

M1 validates a CycloneDX 1.6 declarations JSON file and extracts the capability catalog needed by later matcher work. It does not recommend libraries and does not file GitHub issues.

## Source Contract

- Schema URL: `https://cyclonedx.org/schema/bom-1.6.schema.json`
- Schema SHA-256 captured on 2026-05-06: `1ebcb88a2c845ecb6ff7bee7aeabdff9422cb0347f3d6875b241bd444b7e098f`
- Declaration source repos:
  - `kerberosmansour/hulumi`
  - `kerberosmansour/SunLitSecurityLibraries`
- Declaration path in each repo: `declarations/cyclonedx-1.6-capabilities.json`

The reader is offline at runtime. It records the official schema URL and SHA for auditability, accepts a local copy of the official schema via `--schema-path`, verifies that file's SHA-256, validates against the official schema first, then validates with a strict local jsonschema contract for the declaration-reader fields before extraction.

## Cache Layout

Declaration repos are cached under:

```text
~/.cache/sldo/declarations/<40-char-lowercase-source-sha>/
```

The `<source-sha>` directory name must match `^[0-9a-f]{40}$`. Uppercase hex, abbreviated SHAs, branch names, tags, and arbitrary directory names are rejected.

When `--expected-source-sha` is supplied, the reader runs:

```text
git -C <cache-root> rev-parse HEAD
```

The subprocess must be an argv-list invocation. If the observed HEAD differs from the expected SHA, the cache is refused and the user must refresh it from the pinned source. The reader does not silently retry or fall back to a branch.

## Symlink Discipline

Before reading the declarations file, every path segment from the filesystem root to the file is checked with no-follow semantics. If any segment is a symlink, the reader refuses the file. This blocks cache path tricks where a declaration file escapes the pinned cache directory.

## Validation Pipeline

1. Reject paths with symlink segments.
2. Reject files larger than 10 MiB before parsing.
3. Parse JSON.
4. Reject JSON nesting deeper than 200 levels.
5. Validate with strict jsonschema against the pinned official schema when `--schema-path` is supplied.
6. Validate with the reader extraction schema.
7. Require `bomFormat: CycloneDX`.
8. Require `specVersion: 1.6`.
9. Require `declarations.targets.components`.
10. Normalize component `group`, `name`, and `bom-ref` with NFKC; reject if normalization changes the value.
11. Extract `cdx:sunlit:controls` and `cdx:sunlit:capability` properties into structured arrays.

JSON has no XML entity expansion surface, but the 10 MiB cap, depth cap, and jsonschema validation cover the same denial-of-service class tracked by `tm-slo-sec-libs-abuse-5`.

## Catalog Output

The reader writes one JSON object to stdout:

```json
{
  "source": {"path": "...", "expected_source_sha": "..."},
  "schema": {"url": "...", "sha256": "...", "validation": "strict-jsonschema"},
  "metadata": {"name": "...", "version": "..."},
  "components": [
    {
      "bom_ref": "component:...",
      "group": "...",
      "name": "...",
      "version": "...",
      "type": "library",
      "controls": ["OWASP-C1"],
      "capabilities": ["aws-secure-bucket"],
      "properties": [{"name": "...", "value": "..."}],
      "claims": []
    }
  ],
  "claims": []
}
```

M2 consumes this catalog. M1 must not reinterpret it as a recommendation.

## Cache Eviction

The pre-flight cache policy is:

- Size cap: 1 GiB under `~/.cache/sldo/declarations/`
- Age cap: 90 days since last access marker update
- Strategy: LRU, least-recently-used first

M1 documents the policy and the reader exposes cache SHA checks. Full automatic eviction can be added when cache population lands; M1 must not grow a background cleaner.

## Fault Handling

- Python missing: stop with a `python3` install hint.
- `jsonschema` missing: stop with `pip install jsonschema`.
- File missing: stop before any git or parse work.
- File over 10 MiB: stop before parse.
- Malformed JSON: stop with the JSON parser location.
- Official schema SHA mismatch: stop before validating declarations.
- Schema validation failure: stop with the first failing jsonschema path.
- Cache SHA mismatch: stop and refuse the cache.
- Symlink path: stop and refuse the file.

No failure path writes partial catalog output.

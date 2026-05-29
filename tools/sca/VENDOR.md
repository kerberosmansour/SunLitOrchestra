# Vendored SCA brain (bundled)

This directory is a **vendored, bundled** copy of the SCA "brain" from
`kerberosmansour/AutomoatedSCA`. It powers Track A (emergency, agentic,
explicitly-audited merge) for this repository.

## Why a bundle (not `npm ci`)

The brain is TypeScript with a single runtime dependency (`zod`) plus Node
builtins. Earlier rollouts (e.g. Hulumi, a TypeScript repo) vendored the raw
`src/` + `package-lock.json` and ran `npm ci` in CI. That is unsafe here:
this is a **Rust** repo whose own Supply-Chain gate runs
`osv-scanner scan source -r .` and `actions/dependency-review-action` over the
whole tree. A committed `package-lock.json` (211 transitive dev packages) would
be parsed by those scanners, coupling this repo's CI health to the brain's dev
dependency hygiene — a future npm advisory could redden a green Rust repo with
no Dependabot watching the vendored copy.

So the brain is shipped as **dependency-free, pre-bundled ESM** under `dist/`.
There is no `package.json`, no `package-lock.json`, and no `node_modules` in the
Rust tree. The Rust supply-chain scanners find no npm manifest to parse.

## Provenance

- Source repo: `kerberosmansour/AutomoatedSCA`
- Source commit: `267d35924702f03354137755102a6fa9895ca15b`
- Bundler: `esbuild --bundle --platform=node --format=esm --target=node20`

## Regenerate

From a checkout of AutomoatedSCA at the pinned commit:

```sh
npx --yes esbuild \
  src/cli/scan.ts src/cli/triage.ts src/cli/remediate.ts src/cli/merge.ts \
  src/cli/digest.ts src/cli/scope-sync.ts src/cli/dependency-review.ts src/cli/verify-ruleset.ts \
  --bundle --platform=node --format=esm --target=node20 \
  --outdir=<this-repo>/tools/sca/dist --out-extension:.js=.mjs
```

Each CLI is invoked at runtime as `node tools/sca/dist/<cli>.mjs <subcommand> …`.

## Do not edit

`dist/*.mjs` are build outputs. Fix bugs in AutomoatedSCA, re-bundle, and
re-vendor. The real source, tests, and dependency lockfile live there.

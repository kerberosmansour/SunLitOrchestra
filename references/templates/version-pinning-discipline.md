---
name: version-pinning-discipline
status: stable-template
created: 2026-05-04
audience: skills that download tools, pin rulesets, emit CI workflows, or cache upstream content
purpose: Shared supply-chain pinning and refresh discipline.
---

# Version Pinning Discipline Template

Pins are supply-chain controls, not decoration. This template cites `references/templates/citation-discipline.md` because version claims must be traceable to releases, commits, or checksums.

## Pin Strength

- SHA-256 for downloaded binaries or archives.
- Full 40-character Git SHA for repository content.
- Tag pin only when the upstream cannot provide immutable content and the risk is accepted in writing.
- Branch pin is a temporary development state, never a stable contract.

## Cache Integrity

After fetching a Git repository, run `git rev-parse HEAD` and compare to the expected full SHA. After downloading a file, verify SHA-256 before moving it into the stable cache path.

## Refresh Procedure

Document:

- current version or SHA;
- source URL;
- checksum or commit;
- checked date;
- bump command;
- rollback command or manual restore path.

## Anti-Patterns

- Trusting tags for CI actions when a full SHA is available.
- Downloading into the final path before checksum verification.
- Reusing stale cached tool metadata without rechecking the pin file.

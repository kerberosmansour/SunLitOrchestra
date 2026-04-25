<!--
  REPLACE-BEFORE-PRODUCTION-USE

  This file is a placeholder. The canonical oneNDA UK template (CC BY-ND 4.0,
  TLB consortium, 2021) MUST be downloaded from https://www.onenda.org/ and
  used to replace the contents of this file BEFORE `/slo-legal draft nda` is
  invoked in production.

  Why a placeholder ships in M1:
  - oneNDA's primary distribution is PDF; producing a verbatim Markdown rendering
    requires either WebFetch + reliable PDF→Markdown extraction (fragile), or
    publishing the consortium's PDF bytes inside this repo (license question
    given the broader CC BY-ND 4.0 terms — the licence permits sharing but
    requires the work be unmodified, which a PDF→Markdown conversion may breach).
  - The structural-contract test e2e_biz_a_m1.rs::onenda_template_placeholder_or_pinned_hash
    asserts EITHER (a) this placeholder marker is present and the file is
    treated as pre-production, OR (b) the file's SHA-256 matches a future-pinned
    canonical-bytes hash (set in a small follow-up runbook once the canonical
    bytes have been fetched and verified by the project owner).

  License obligation when this placeholder is replaced:
  - The replacement file MUST be the canonical oneNDA UK Country Schedule
    body, byte-for-byte unmodified. CC BY-ND 4.0 forbids derivative works.
  - `/slo-legal draft nda` MUST render the canonical body verbatim and emit
    company / counterparty / cover-page fields in a SEPARATE artifact that
    wraps but does not edit the canonical text.
  - `/slo-verify` Pass 4 (deferred to a future runbook) regression-tests the
    rendered NDA body bytes against the canonical hash.

  How to replace this placeholder:
  1. Visit https://www.onenda.org/ and download the canonical UK Country
     Schedule (Markdown if available; otherwise PDF + consortium-blessed text
     extraction).
  2. Replace the entire contents of this file with the canonical bytes
     (keeping no part of this comment block — the structural test detects
     the placeholder marker by string-match).
  3. Compute SHA-256 of the file bytes:
        shasum -a 256 references/biz/templates/onenda-uk.md
  4. Pin the hash as a `const ONENDA_UK_SHA256: &str = "<hash>";` in
     `crates/sldo-install/tests/e2e_biz_a_m1.rs` and update the
     placeholder-vs-pinned check to require the pinned-hash branch.
  5. Run `cargo test -p sldo-install --test e2e_biz_a_m1` and confirm
     onenda_template_placeholder_or_pinned_hash passes via the pinned-hash
     branch (not the placeholder branch).
  6. Document the replacement in a M1.5 (or follow-up) runbook lessons file.

  Until this file is replaced, `/slo-legal draft nda` MUST refuse to draft
  with a clear "oneNDA template not yet populated; see
  references/biz/templates/onenda-uk.md replacement instructions" error.
  This refusal is enforced by SKILL.md prose; the structural-contract test
  asserts the SKILL.md documents this refusal.
-->

# ONENDA-UK-PLACEHOLDER

This file is a pre-production placeholder for the canonical oneNDA UK Country Schedule.

The marker `ONENDA-UK-PLACEHOLDER` is detected by `crates/sldo-install/tests/e2e_biz_a_m1.rs`. While this marker is present, the structural-contract test treats the file as a pre-production placeholder. Once the canonical oneNDA UK bytes replace this file (per the instructions in the comment block above), the test will require a SHA-256 match against the pinned canonical hash.

Source: https://www.onenda.org/ (TLB consortium, 2021 — CC BY-ND 4.0).

For full replacement instructions, see the comment block above this heading.

---
name: slo-cloud-threat-model-citation-and-licensing
status: stable
source_skill: skills/slo-cloud-threat-model/SKILL.md
audience: the agent running /slo-cloud-threat-model
purpose: IDs-only licensing posture and the framework → upstream-URL resolution map.
---

# Citation and licensing — IDs only, no verbatim prose

This tightens [`../../../references/templates/citation-discipline.md`](../../../references/templates/citation-discipline.md)
for control-framework citation. It is the SLO-side restatement of Hulumi's
mapping-table licensing posture (`docs/mappings/licensing.md`).

## The rule

Cite framework control **identifiers and upstream URLs only**. Do **not** copy
verbatim control text, CAIQ questions, benchmark recommendation prose, or
implementation-guidance wording from a licensed catalog into the generated
document, this skill, the scenario fixtures, or commit messages.

- **CSA CCM v4.1 / CSA AICM v1** — embedding control / CAIQ / implementation prose
  in a distributed product requires a commercial licence. Bare control-ID
  citation is treated as factual identifier reuse (industry analog: FedRAMP /
  NIST OSCAL). FAQ: `https://cloudsecurityalliance.org/artifacts/ccm-aicm-licensing-faq`.
- **CIS AWS Foundations / CIS GitHub** — PDFs are free for non-commercial use;
  text embedding is not part of that grant. Non-member terms:
  `https://www.cisecurity.org/terms-of-use-for-non-member-cis-products`.
- **NIST SP 800-53 / SP 800-218 / 218A, MITRE ATT&CK / ATLAS, OpenSSF Scorecard,
  OWASP ASVS** — public-domain or attribution-reuse; still ID-only here for
  consistency and because IDs are stabler than titles.
- **Hulumi policy rule IDs** (`CF_*`, `X_ORIGIN_*`, `G_OIDC_*`) are Hulumi's own
  identifiers, not third-party licensed prose — safe to cite with the Hulumi
  docs URL.

If the user asks to expand an ID into the catalog's wording, refuse, cite the ID,
and link the relevant terms URL above.

## Framework → upstream URL map

The agent and `scripts/scenario_catalog.py` resolve a control ID's framework as
the segment before the first `:` (e.g. `CCM:IAM-10` → `CCM`), or the whole ID for
a bare Hulumi policy rule. A control ID is valid only if its framework appears
here. The "paraphrased note" column is SLO-authored, never upstream verbatim.

| Framework key | Upstream URL | Paraphrased note (SLO-authored) |
|---|---|---|
| `CCM` | https://cloudsecurityalliance.org/artifacts/cloud-controls-matrix-v4-1 | CSA Cloud Controls Matrix v4.1 |
| `CIS-AWS-v5.0.0` | https://www.cisecurity.org/benchmark/amazon_web_services | CIS AWS Foundations Benchmark v5.0.0 |
| `CIS-GitHub-v1.2.0` | https://www.cisecurity.org/benchmark/github_foundations | CIS GitHub Foundations Benchmark v1.2.0 |
| `NIST-800-53-r5` | https://csrc.nist.gov/pubs/sp/800/53/r5/upd1/final | NIST SP 800-53 Rev 5 |
| `NIST-800-218` | https://csrc.nist.gov/projects/ssdf | NIST SP 800-218 (SSDF) |
| `NIST-800-218A` | https://csrc.nist.gov/projects/ssdf | NIST SP 800-218A (SSDF augmentation) |
| `NIST-SSDF-v1.1` | https://csrc.nist.gov/projects/ssdf | NIST Secure Software Development Framework v1.1 |
| `ATLAS` | https://atlas.mitre.org/ | MITRE ATLAS adversarial-ML technique catalog |
| `MITRE-ATTCK` | https://attack.mitre.org/ | MITRE ATT&CK technique catalog |
| `OpenSSF-Scorecard` | https://scorecard.dev/ | OpenSSF Scorecard checks |
| `GitHub-Well-Architected` | https://wellarchitected.github.com/library/scenarios/nist-ssdf-implementation/ | GitHub Well-Architected SSDF mapping |
| `OWASP-ASVS-v5.0` | https://owasp.org/www-project-application-security-verification-standard/ | OWASP Application Security Verification Standard 5.0 |
| `ISO-27001-2022` | https://www.iso.org/standard/27001 | ISO/IEC 27001:2022 |
| `SOC2-TSC-2017` | https://www.aicpa-cima.com/resources/landing/system-and-organization-controls-soc-suite-of-services | AICPA SOC 2 Trust Services Criteria (2017) |
| `Hulumi-Policy` | https://github.com/kerberosmansour/hulumi/blob/main/docs/components/cloudflare-policy-packs.md | Hulumi CrossGuard policy-pack rule IDs |

## Hulumi policy rule IDs (framework `Hulumi-Policy`)

These bare IDs (no `:`) resolve to the `Hulumi-Policy` framework and the Hulumi
docs URL above. Citing the rule ID is not licensed-prose embedding.

| Rule ID | Enforcement (Hulumi v1.3.2) |
|---|---|
| `CF_DNS_1_NO_DNS_ONLY_PUBLIC_APP_RECORD` | mandatory — reject proxy-eligible public app DNS records set `proxied=false` outside `PublicHostname` without a scoped suppression reason |
| `CF_DNSSEC_1_REQUIRE_PUBLIC_ZONE_DNSSEC` | mandatory — require public Cloudflare zones to carry `ZoneFoundation` / `ZoneDnssec` evidence |
| `CF_ORIGIN_1_REQUIRE_SECURE_ORIGIN_MODE` | mandatory — require app hostnames to have `CloudflareOriginIngress` tunnel or allowlist+AOP evidence |
| `X_ORIGIN_1_NO_PUBLIC_AWS_ORIGIN_BYPASS` | advisory (evolving until Hulumi origin-bypass M4 sandbox fixtures) — report Cloudflare public-app records pointing at `*.elb.amazonaws.com` without tunnel / allowlist+AOP evidence |
| `G_OIDC_1` | mandatory — reject `StringLike` / wildcard `sub` conditions on GitHub Actions OIDC trust to AWS / Azure / GCP at preview time |

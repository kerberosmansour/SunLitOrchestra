# Business loops — SunLit Orchestra

> **Purpose**: name the cyclic feedback structures that move the business side of the skill pack — discovery, GTM, pricing, founder ops — so a UK seed-stage founder can answer "which loop am I in, and what do I run next?" in 90 seconds.
>
> **Companion doc**: engineering loops live at [docs/LOOPS-ENGINEERING.md](LOOPS-ENGINEERING.md). Static structure is in [docs/ARCHITECTURE.md](ARCHITECTURE.md). This doc is the cyclic complement on the business side.
>
> **PII discipline**: every example below uses the pseudonym pair *Alice* (founder) / *Bob* (interviewee). No real interview quotes belong in this doc. Real founder PII / interview transcripts live in `docs/biz/users/` (gitignored, confidential).

---

## Start here

Pick the row that matches the question you have right now. The "First skill" column is what to run; the "Loop" column is the section below that explains why.

| Your question | First skill | Loop | Expected artifact |
|---|---|---|---|
| "We're not learning from user calls — where do I start?" | `/slo-talk-to-users` | [User-interview loop](#user-interview-loop) | `docs/biz/users/<date>-<name>.md` (confidential) |
| "Who is this product for, and how do I reach them?" | `/slo-gtm` | [GTM loop](#gtm-loop) | `docs/biz-public/gtm/strategy.md` |
| "Are we charging the right amount?" | `/slo-pricing` | [Pricing loop](#pricing-loop) | `docs/biz-public/pricing.md` |
| "Am I OK? Is the cofounder relationship OK? Do we have enough runway?" | `/slo-founder-check` | [Founder-check loop](#founder-check-loop) | `docs/biz/founder-check.md` (confidential) |
| "We've launched and we want a tactic plan" | `/slo-marketing` (b2b or b2c) | [GTM loop](#gtm-loop) | `docs/biz-public/marketing/<b2b\|b2c>-plan.md` |
| "I want to file an SEIS / EIS Advance Assurance — am I ready?" | `/slo-fundraise` (triage mode) | [Pricing loop](#pricing-loop) (qualifying-trade gate) → fundraise advisor | `docs/biz/fundraise/<artifact>.md` |

Each loop below documents **user-visible outcome**, **trigger**, **steps**, **exit condition**, **artifacts**, **skills involved**, and a **diagram**.

---

## User-interview loop

> **User-visible outcome**: every week the founder learns something specific about the user that changes the roadmap, the ICP, the pricing, or the launch plan — and that learning is captured in a structured artifact, not just in the founder's head.

**Trigger**: a user call is scheduled, OR a recent user call has not been written up, OR the founder catches themselves making a "users want X" claim with no recent interview to back it.

**Steps**:

1. `/slo-talk-to-users` (pre-interview mode) — produce a Mom-Test-disciplined interview script. Forbids leading questions. Confirms PII handling per `references/biz/jurisdiction-uk.md`.
2. Run the interview. (Out of scope for this loop's mechanism — this is the founder's responsibility.)
3. `/slo-talk-to-users` (post-interview mode) — extract structured signal: pain confirmed, alternatives used, hypothesis kills, hypothesis confirms, surprises.
4. The post-interview artifact lands at `docs/biz/users/<date>-<name>.md` (`tier: confidential`, gitignored). Founder reviews once, then the structured signal is fed into:
   - `/slo-product` (roadmap mode) when the signal changes a feature priority,
   - `/slo-gtm` when the signal changes the ICP or segmentation,
   - `/slo-launch` when the signal kills or confirms the one-sentence pitch.
5. PII guardrail: `/slo-verify` Pass 4 PII-pattern scan over `docs/biz-public/` flags any email / UK NI / sort code / capitalised-bigram pattern that leaked from `docs/biz/` into the public tier.

**Exit condition**: the post-interview artifact is on disk, the structured signal is extracted, and at least one downstream artifact (roadmap row, ICP segment, pitch line) has been re-written or explicitly affirmed unchanged. "Filed and forgotten" is the failure mode this loop forbids.

**Artifacts**: `docs/biz/users/<date>-<name>.md` (confidential), updated rows in `docs/biz-public/product/roadmap.md` and `docs/biz-public/gtm/strategy.md` and `docs/biz-public/launch-<slug>.md` as relevant.

**Skills involved**: `/slo-talk-to-users`, `/slo-product`, `/slo-gtm`, `/slo-launch`.

```
   user call scheduled
        │
        ▼
   /slo-talk-to-users (pre-interview)
        │
        ├── Mom Test discipline check
        ├── PII handling reminder
        ▼
   founder runs the call
        │
        ▼
   /slo-talk-to-users (post-interview)
        │
        ├── extract: pain / alternatives / kills / confirms / surprises
        ├── write docs/biz/users/<date>-<name>.md (confidential)
        ▼
   structured signal feeds:
        │
        ├──► /slo-product  (roadmap row update or affirmation)
        ├──► /slo-gtm      (ICP / segmentation update)
        └──► /slo-launch   (pitch line kill / confirm)
                │
                ▼
        /slo-verify Pass 4 PII scan over docs/biz-public/
                │
                └── flags any leaked PII pattern
```

Example (pseudonymised): Alice runs a 30-minute call with Bob. Pre-interview script asks "tell me about the last time you tried to do X" not "would you use a tool that does X?". Post-interview extraction notes Bob already pays £200/month for a workaround → signal feeds `/slo-pricing` value-equation calculator (not just `/slo-product` roadmap).

---

## GTM loop

> **User-visible outcome**: the founder has a written, defensible answer to "who is the customer, what motion reaches them, and which channel is currently working", and re-tests that answer when the data stops fitting.

**Trigger**: pre-launch (the GTM doc does not exist yet), OR a launched-channel CAC is creeping above the LTV / CAC ratio threshold, OR a user-interview signal contradicts the current ICP.

**Steps**:

1. `/slo-gtm` — produce or revise `docs/biz-public/gtm/strategy.md` with ICP + 3-segment cap + motion choice (`PLG | sales-led | community-led | hybrid`) + channel strategy.
2. `/slo-marketing` (b2b OR b2c, depending on motion) — produce the tactical plan at `docs/biz-public/marketing/<b2b|b2c>-plan.md`. Routes any direct-marketing implementation to `/slo-legal triage` for DUAA 2025 PECR considerations.
3. `/slo-sales-funnel` — outbound funnel math + cold-email template for the chosen channel(s). Routes cold email to `/slo-legal triage` for PECR.
4. `/slo-launch` — staged launch (silent → friends-and-family → communities → press). Each stage has kill / delay rules.
5. `/slo-metrics` — financial KPIs (CAC, LTV, NDR, MoM revenue, burn multiple, gross margin, runway, ARR). Threshold breach feeds back to step 1.
6. User-interview signal from the [User-interview loop](#user-interview-loop) re-tests the ICP at any time.

**Exit condition**: the four GTM artifacts exist (`gtm/strategy.md`, `marketing/<motion>-plan.md`, `sales/funnel-<segment>.md`, `launch-<slug>.md`), and the latest metrics dashboard has at least one channel in the green.

**Artifacts**: `docs/biz-public/gtm/strategy.md`, `docs/biz-public/marketing/{b2b,b2c}-plan.md`, `docs/biz-public/sales/funnel-<segment>.md`, `docs/biz-public/launch-<slug>.md`, `docs/biz-public/metrics.md`.

**Skills involved**: `/slo-gtm`, `/slo-marketing`, `/slo-sales-funnel`, `/slo-launch`, `/slo-metrics`, `/slo-talk-to-users` (re-test trigger), `/slo-legal` (PECR triage).

```
   /slo-gtm ──► docs/biz-public/gtm/strategy.md
        │
        ▼
   /slo-marketing (b2b | b2c) ──► docs/biz-public/marketing/<motion>-plan.md
        │
        ▼
   /slo-sales-funnel ──► docs/biz-public/sales/funnel-<segment>.md
        │   │
        │   └──► /slo-legal triage  (PECR / DUAA 2025)
        ▼
   /slo-launch ──► docs/biz-public/launch-<slug>.md  (staged: silent → F&F → community → press)
        │
        ▼
   /slo-metrics ──► docs/biz-public/metrics.md  (CAC / LTV / NDR / burn multiple)
        │
        │ threshold breach
        ▼
   re-run /slo-gtm; user-interview signal can also re-test ICP
```

---

## Pricing loop

> **User-visible outcome**: the founder has a written rationale for the price, has run at least one "+50% experiment" against the canonical default-undercharge correction, and has the SEIS/EIS qualifying-trade implications written down BEFORE the next investor conversation.

**Trigger**: pricing doc does not exist yet, OR conversion rate exceeds 30% on the highest tier (default-undercharge signal), OR `/slo-fundraise` Advance Assurance pre-check flags a qualifying-trade question on the current revenue mix, OR a user-interview signal reveals existing-spend numbers materially above the current price.

**Steps**:

1. `/slo-pricing` — value-equation calculator (price = 25-33% of value delivered) + 3-tier-max model + canonical "increase price by 50%" experiment framing. Output: `docs/biz-public/pricing.md`.
2. Run the +50% experiment for a fixed window (commonly 4 weeks). Pricing doc names the start date, the metric, and the kill-or-keep rule.
3. `/slo-metrics` — measure conversion + revenue + churn. Threshold breach feeds back to step 1.
4. `/slo-fundraise` (triage mode) — runs SEIS/EIS qualifying-trade pre-check (per HMRC VCM3000 + VCM31000) on the resulting revenue mix. If the qualifying-trade question fires, the founder routes to an accountant via `/slo-accounting` before any term-sheet conversation.
5. Re-run the loop on a fixed cadence (commonly quarterly) OR on threshold breach OR on a major user-interview signal.

**Exit condition**: `docs/biz-public/pricing.md` exists with a written experiment, the metrics dashboard reflects the experiment's window, and the qualifying-trade question has been answered (either "no, mix unchanged" or "yes, escalated to accountant").

**Artifacts**: `docs/biz-public/pricing.md`, the experiment row in `docs/biz-public/metrics.md`, optional `docs/biz/fundraise/seis-eis-pre-check-<date>.md` (confidential) when the qualifying-trade question fires.

**Skills involved**: `/slo-pricing`, `/slo-metrics`, `/slo-fundraise`, `/slo-accounting`, `/slo-talk-to-users` (signal trigger).

```
   /slo-pricing ──► docs/biz-public/pricing.md  (value equation, 3-tier model, +50% experiment)
        │
        ▼
   run +50% experiment for fixed window
        │
        ▼
   /slo-metrics ──► measure conversion / revenue / churn
        │
        ▼
   /slo-fundraise (triage)  — SEIS/EIS qualifying-trade pre-check
        │
        ├── qualifying-trade question fires?
        │       └──► /slo-accounting  (escalation; before any term-sheet talk)
        │
        ▼
   re-run loop: quarterly, OR on conversion-threshold breach,
                 OR on user-interview "existing-spend" signal
```

---

## Founder-check loop

> **User-visible outcome**: the founder catches themselves before they burn out, before the cofounder relationship cracks, and before runway becomes a panic — by writing down a 12-question self-assessment and a worst-case-runway worksheet on a fixed cadence.

**Trigger**: monthly self-check (founder's calendar reminder), OR a stress / runway / cofounder warning signal, OR an upcoming YC application deadline (the optional YC-prep mode), OR a hire is being contemplated.

**Steps**:

1. `/slo-founder-check` — 12-question self-assessment (stress / runway / cofounder / health / family / finances) + worst-case-runway worksheet (cash + months + cut-cost levers + pivot options) + optional YC application prep. Output: `docs/biz/founder-check.md` (`tier: confidential`).
2. `/slo-cofounder` — quarterly cofounder evaluation, monthly 1:1 agenda, optional 4-week paid trial framing for prospective cofounders. Output: `docs/biz/cofounder/<name>.md` (confidential).
3. Hire decision: `/slo-hire` triggers the **mandatory IR35 triage gate** (per `references/biz/ir35-cest-factors.md`) BEFORE the offer is made. Rejects "call them a contractor for tax efficiency" framing on the spot.
4. Equity & legal hygiene: `/slo-equity` for cofounder splits / vesting / cap-table; `/slo-legal triage` for any contract or GDPR-adjacent question; `/slo-fundraise` for any SAFE / pitch / term-sheet matter. All four advisor skills hard-block `draft` on regulated / >£5k / counterparty-with-lawyer / GDPR matters.
5. Re-run the self-check on cadence; the worst-case-runway worksheet is the leading indicator that triggers the next loop iteration.

**Exit condition**: the monthly self-check is on disk, the cofounder 1:1 is on disk, every hire offer has cleared the IR35 gate, and any equity / legal / fundraise question has either been resolved by an advisor skill or routed to a professional.

**Artifacts**: `docs/biz/founder-check.md`, `docs/biz/cofounder/<name>.md`, `docs/biz/hires/<role>-<name>.md`, `docs/biz/equity/<artifact>.md`. All confidential, all gitignored under `docs/biz/`.

**Skills involved**: `/slo-founder-check`, `/slo-cofounder`, `/slo-hire`, `/slo-equity`, `/slo-legal`, `/slo-accounting`, `/slo-fundraise`.

```
   monthly cadence (or stress signal)
        │
        ▼
   /slo-founder-check ──► docs/biz/founder-check.md (confidential)
        │     │
        │     ├── 12-question self-assessment
        │     ├── worst-case-runway worksheet
        │     └── optional YC application prep
        ▼
   /slo-cofounder (quarterly + monthly 1:1)
        │
        ▼
   hire contemplated?
        │
        └──► /slo-hire  ──► IR35 triage gate (MANDATORY before offer)
                 │
                 ▼
   equity / legal / fundraise questions?
        │
        ├──► /slo-equity        (cofounder split, vesting, cap-table)
        ├──► /slo-legal triage  (regulated / >£5k / counterparty / GDPR → professional)
        ├──► /slo-fundraise     (SAFE math, pitch, AA pre-check)
        └──► /slo-accounting    (HMRC matters, R&D claim, MTD)
                │
                ▼
        re-run loop on next cadence
```

---

## Anti-process-theatre check

Every loop here exists because it produces a user-visible outcome the static skill catalog cannot make visible. The check the runbook itself enforces — "does this added surface reduce user decisions or reviewer work?" — applies to every future addition: if a future addition cannot point at a concrete user-visible outcome that an existing loop already produces, that addition belongs in a skill's reference file or in `references/biz/`, not in this doc.

---

## See also

- [docs/ARCHITECTURE.md](ARCHITECTURE.md) — static structure of the skill pack at HEAD.
- [docs/LOOPS-ENGINEERING.md](LOOPS-ENGINEERING.md) — engineering-side loops (sprint, security-tuning, lessons, library-feedback).
- [docs/slo/design/biz-skill-pack-overview.md](design/biz-skill-pack-overview.md) — design doc for the biz skill pack.
- [references/biz/jurisdiction-uk.md](../references/biz/jurisdiction-uk.md) — UK-only jurisdiction discipline shared across biz skills.
- [references/biz/triage-gate.md](../references/biz/triage-gate.md) — the four-gate hard-block (regulated / >£5,000 / counterparty / GDPR) used by the four advisor skills.
- [references/biz/ir35-cest-factors.md](../references/biz/ir35-cest-factors.md) — IR35 / CEST factors used by `/slo-hire`'s mandatory triage gate.
- [references/biz/hmrc-vcm-index.md](../references/biz/hmrc-vcm-index.md) — HMRC VCM index used by SEIS / EIS pre-checks.

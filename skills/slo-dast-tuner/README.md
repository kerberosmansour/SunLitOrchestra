# /slo-dast-tuner — find bugs in a *running* web app, the way an attacker would

> **Status:** active · **License:** [Apache-2.0 OR MIT](../../LICENSE) · **Host:** any agent that loads `/slo-*` skills · **Engine:** OWASP ZAP, driven only through the pinned `zaprun` image

**This page is for humans.** The machine-facing contract the agent follows is
[`SKILL.md`](SKILL.md); the deep how-to lives in [`references/`](references/). Start here.

## What is this, in one breath?

**DAST** = *Dynamic Application Security Testing*: you point a tool at your **running** web
app and it pokes it from the outside — like a burglar walking around a house trying every
door and window — without ever reading the source code.

`/slo-dast-tuner` is the skill that **sets that scanner up correctly and tunes it**. It uses
OWASP ZAP under the hood, but you never touch ZAP directly — the skill drives a locked-down,
version-pinned `zaprun` container for you.

## Why does this exist? (the problem, plainly)

If you just download a scanner and hit "go", it will look like it worked and find almost
nothing important. We proved this on a deliberately-vulnerable practice app: a default scan
found **zero** of the serious bugs. Not because the scanner is bad — because of three boring,
fixable mistakes almost everyone makes:

1. **It never logged in.** The interesting bugs are behind a login screen. An scanner that
   doesn't sign in is testing the lobby and declaring the whole building safe.
2. **It can't open the modern front-end.** Apps built with Angular/React run code in your
   browser. A traditional scanner can't "press the buttons", so it never sees those bugs.
3. **It didn't know where to look.** Your code scanner (SAST) found a bug and said
   "file X, line 42". DAST needs a *URL*, not a line number. Nobody translated between them.

`/slo-dast-tuner` fixes all three. That's the whole point of the skill.

## What it actually does (the three lanes)

| Lane | Plain-English job | When the skill uses it |
|---|---|---|
| **Active scan** | The classic "rattle every door" scan | Server-rendered apps; the baseline |
| **PTK / DOM lane** | Drives a real browser so it can test Angular/React front-ends | Single-page apps (where the classic scan is blind) |
| **SAST→DAST bridge** | Reads your code to turn "file:line" into "this URL, this method, log in first" | Whenever you have code-scanner output to act on |

…and the rule that ties them together: **if part of the app needs a login, a scan that
didn't log in is a *coverage failure*, not a clean result.** The skill refuses to call that
"no bugs found".

## Quick start

You don't run ZAP. You ask the agent for the skill, point it at an app **you are allowed to
test**, and it does the safe, tuned thing:

```text
/slo-dast-tuner <slug> --target-dir <repo> --deployment-target http://localhost:4000
```

Behind that, the skill:

1. confirms you're authorized to test the target (it will stop if you can't say yes),
2. picks the right lane (classic vs browser/PTK) for the app's shape,
3. logs in if any part of the app needs it,
4. runs the scan through the pinned `zaprun` image and writes evidence files you can read,
5. reports what it **confirmed**, what it could only **partially** see, and what it
   **couldn't reach** — honestly, never "looks clean" when it just didn't look.

## The SAST→DAST bridge (the clever bit, explained simply)

A code scanner says: *"there's an injection bug in `research.js`, line 16."*
A web scanner needs: *"send this attack to `GET /research?url=…`, and you'll have to be
logged in as a normal user first."*

Those are two different languages. A plain tool can't translate — the code scanner's report
genuinely doesn't contain the URL. **But an agent that reads the codebase can**: it follows
the code from that line up to the function, finds where that function is wired to a web
address, notes whether there's a login check, and writes a targeted test plan.

We tested this end-to-end on a practice app: the blind default scan confirmed **0** of the
login-protected bugs; the bridge-guided scan confirmed **4** (broken access control, open
redirect, server-side request forgery, and a denial-of-service). Same app, same scanner —
the only difference was the bridge telling it *where to go and to log in first*.

Details: [`references/sast-route-bridge.md`](references/sast-route-bridge.md).

## "Works on my framework?" — the adapter catalog

The bridge needs to understand how your framework declares its URLs (Express, Spring,
Django, Rails… all do it differently). There's a catalog of **12 framework adapters**
([`references/resolvers/`](references/resolvers/)) plus a **generic fallback** so an
unknown stack still works — it just degrades gracefully instead of breaking.

**Honesty rule (important):** only the **Express** adapter is marked `validated` — it was
proven end-to-end against a real vulnerable app. The other 11 are `spec-only`: written
carefully from framework docs but **not yet battle-tested**. The skill *labels* anything a
`spec-only` adapter resolved so you always know whether a result is proven or a strong
hint. An adapter only graduates to `validated` after it's run against a real vulnerable app
in that stack. We'd rather tell you "this is an educated guess" than pretend.

## The rules it follows (so it stays trustworthy)

- **Authorization first.** It will not scan a target you haven't confirmed you may test.
- **One engine, pinned.** ZAP only, only through the approved digest-pinned `zaprun` image —
  no random tools, no hand-written scan configs.
- **No hiding findings.** It never lowers thresholds to make a report look clean.
- **Honest gaps.** Couldn't log in / couldn't crawl / used a spec-only adapter → it says so.
- **Your custom probes stay yours.** App-specific scan scripts live in your repo, not in
  this shared project, until they're generalized and fixture-tested.

## Where to go next

| You want… | Read |
|---|---|
| The exact agent contract | [`SKILL.md`](SKILL.md) |
| How the bridge resolves routes | [`references/sast-route-bridge.md`](references/sast-route-bridge.md) · [`resolver-adapter-contract.md`](references/resolver-adapter-contract.md) |
| Your framework's adapter | [`references/resolvers/index.md`](references/resolvers/index.md) |
| Why auth is the #1 lever | [`references/authentication-coverage.md`](references/authentication-coverage.md) |
| The DOM-XSS / PTK lane | [`references/ptk-dom-xss.md`](references/ptk-dom-xss.md) |
| The shared vuln vocabulary | [`../../references/security/vuln-class-taxonomy.md`](../../references/security/vuln-class-taxonomy.md) |
| Its code-scanner sibling | [`../slo-sast/README.md`](../slo-sast/README.md) |

This skill is most powerful **paired with `/slo-sast`**: SAST finds *what and where in the
code*, the bridge turns that into *where on the web*, and DAST *proves it at runtime*.

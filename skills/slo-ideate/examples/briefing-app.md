---
name: briefing-app
created: 2026-04-23
status: ideation
tla_required: false
---

# Daily Calendar Briefing (Example — illustrative only)

## The pain
Priya is a founder with three Google calendars (personal, company, portfolio advisor board). Every morning at 7:45 she spends 20 minutes piecing together: what's on today, which attendees she hasn't met, which events have stale Zoom links from 2023. Last Tuesday she joined the wrong video call for 12 minutes before realizing the meeting had been moved.

## Five capabilities the user described without realizing
- Unified read across multiple Google calendars with permission-aware scoping.
- Attendee enrichment: who they are, when she last met them, anything relevant from email thread.
- Link staleness detection: flag Zoom/Meet URLs that 404 or look expired.
- Morning delivery at a scheduled time (not "open the app").
- Failure to prepare is silent — she just shows up wrong. The app must never silently skip a day.

## Approach A — conservative (native Mac menubar app)
- **Effort**: 3 person-weeks
- **Wedge**: morning push notification with top-3 events, enriched attendees, broken-link flags.
- **Risks**: Google OAuth scope review; people-API rate limits.

## Approach B — cloud / SaaS (web app + server)
- **Effort**: 6 person-weeks
- **Wedge**: hosted service with a daily email.
- **Risks**: storing calendar tokens server-side is a bigger liability than a local app; the wedge's delivery is email, which is a crowded inbox.

## Approach C — local / desktop (Electron + background agent)
- **Effort**: 4 person-weeks
- **Wedge**: same as A but on Windows/Linux too.
- **Risks**: Electron weight; app permissions on managed laptops.

## Recommendation
Approach A. Native Mac menubar app is the smallest thing that could work for Priya tomorrow and avoids the storage-liability risk of B. C spreads effort too thin for the user we named. Ship a notification-only wedge in week 1, add the enrichment panel week 2.

## Open questions for /slo-research
1. Do any existing products already do morning-calendar-briefings for founder-type users specifically, and how are they priced?
2. What is the token-storage + rotation model that Google-Workspace-admin-managed users (Priya's company) will accept?
3. Is there a people-API competitor to Clearbit for attendee enrichment that doesn't require per-seat pricing?

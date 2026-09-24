# TLC artifact provenance

## 2026-09-07 — authorized same-version pin repair

The operator explicitly authorized the documented AB25C prerequisite repair on
2026-09-07. Version 1.8.0 and its download URL are unchanged.

Official release metadata: https://api.github.com/repos/tlaplus/tlaplus/releases/tags/v1.8.0
Asset: tla2tools.jar, ID 544648411, size 4487737 bytes, updated
2026-09-04T17:12:07Z. Metadata digest and an independently downloaded artifact
agree: b658b4e504fdf0b721caf7066320f6b6fe5805f4dd2f717d0e47baba4097205e.

The earlier pin d5d07d5dab38ddb840c91ec48fa02f28b37a608d5af9a73570018591dbc8ef7f
did not match this republished release asset. The mismatched download was rejected
and never executed. This repair changes the pin only after explicit authorization
and fresh retrieval of both metadata and the artifact; future mismatches still fail
closed. An upstream-hosted digest provides provenance, not an independent signature
or proof of upstream account integrity.

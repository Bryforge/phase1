# Phase1 Development Checkpoint - Learn System Milestone

Checkpoint date: 2026-05-08
Stable base: v4.2.0

## Completed milestone

Phase1 now has an in-shell local-first learning memory layer.

Delivered PRs: #42 in-shell learn; #43 auto-observe; #44 failure-prioritized suggestions; #45 typo recovery; #46 explain typo recovery; #47 local update helper; #48 learn doctor.

## Current command surface

learn status; learn doctor; learn import-history; learn import-file; learn observe; learn teach; learn note; learn ask; learn explain; learn suggest; learn profile; learn forget; learn export.

## Current behavior

Memory is local-only, sanitized, bounded, and not cloud-backed. Learn does not train on itself. Normal commands are auto-observed. Failed commands are prioritized. Typo-like failures can suggest likely commands. Learn explain explains correction reasoning. Learn doctor reports memory health.

## Local update workflow

Use p1up, p1run, and p1full. No zip patch workflow should be required for normal development.

## Roadmap maintenance rule

Future feature PRs must update roadmap and planned implementation docs when they complete roadmap items, change project direction, or add planned work. Review LEARNING.md, docs/project/WIKI_ROADMAP.md, docs/website/NEXT_ROADMAP_IMPLEMENTATION.md, EDGE.md, and CHANGELOG.md for each milestone. If no roadmap update is needed, the PR body must say why.

## Next planned learning work

1. learn repair - direct repair action for the highest-priority failed command.
2. learn categories - classify command usage by workflow area.
3. learn summary - concise operator memory summary for demos.
4. VFS/project summaries for /home files.
5. Guarded export/import for shareable non-secret learning profiles.

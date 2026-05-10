# Base1 Readiness Matrix

> **Status:** Design and readiness tracking.
>
> **Validation:** Use with Base1 dry-run docs, recovery notes, hardware notes, and future validation reports.
>
> **Non-claims:** This matrix does not claim Base1 is bootable, daily-driver ready, installer-ready, recovery-complete, or validated on real hardware.

This matrix defines how Base1 work should move from planning to stronger evidence levels.

## Evidence levels

| Level | Meaning | Required evidence |
| --- | --- | --- |
| Roadmap | Planned work only. | Roadmap entry and non-claims. |
| Design | Architecture or operator-flow design exists. | Design document linked from an index. |
| Dry-run | A no-write or read-only command path exists. | Script, command, or docs test showing no write path. |
| Preview | A named preview artifact or workflow exists. | Preview notes, checksum or report when applicable. |
| Validated | A specific target or workflow has recorded validation. | Validation report naming target, commands, and result. |

## Current Base1 tracks

| Track | Current allowed level | Promotion gate |
| --- | --- | --- |
| Scope and non-claims | Design | Keep linked from docs and claims policy. |
| Read-only base model | Design | Add dry-run checker or validation report. |
| Writable Phase1 state layer | Design | Add state layout dry-run evidence. |
| Recovery shell | Design / dry-run where scripts exist | Add report from a named recovery environment. |
| Recovery USB planning | Design / dry-run where scripts exist | Add media creation preview only after checksum and target checks. |
| Image provenance | Design / dry-run where scripts exist | Add checksum, source, build input, and report format. |
| Target identity verification | Design / dry-run where scripts exist | Add named target report with no-write identity summary. |
| Rollback metadata | Design / dry-run where scripts exist | Add rollback preview report and restore boundary. |
| Installer policy | Design / dry-run only | Add explicit no-write test before any write workflow. |
| Hardware targets | Roadmap / design / checklist | Add hardware-specific validation reports. |

## Promotion rules

A Base1 page may move to a stronger level only when the linked evidence supports it.

- Roadmap to design requires a design document.
- Design to dry-run requires a read-only or no-write path.
- Dry-run to preview requires a named preview workflow or artifact.
- Preview to validated requires a recorded validation report.

## Review rule

When evidence is missing, use weaker wording. Prefer `planned`, `design`, `dry-run`, or `preview` over broad claims.

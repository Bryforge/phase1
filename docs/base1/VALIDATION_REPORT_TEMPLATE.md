# Base1 Validation Report Template

> **Status:** Documentation template.
>
> **Validation:** Use this template when recording Base1 design, dry-run, preview, or validation results.
>
> **Non-claims:** Completing this template does not by itself promote Base1 to a stronger readiness level.

Copy this file when preparing a Base1 validation report.

## Report metadata

| Field | Value |
| --- | --- |
| Report title | `[title]` |
| Date | `[YYYY-MM-DD]` |
| Author/operator | `[name or handle]` |
| Base branch or commit | `[ref]` |
| Evidence level | `Roadmap / Design / Dry-run / Preview / Validated` |
| Related track | `[recovery shell / image provenance / target identity / rollback / storage layout / hardware / other]` |

## Scope

Describe exactly what this report covers.

```text
[scope]
```

## Target summary

| Field | Value |
| --- | --- |
| Target type | `[emulator / laptop / Raspberry Pi / generic x86_64 / documentation-only]` |
| Target identifier | `[safe identifier]` |
| Host environment | `[host OS or test environment]` |
| Phase1/Base1 artifact | `[artifact, branch, or none]` |

## Commands or checks run

List commands, scripts, docs checks, or manual checks.

```bash
[commands]
```

## Result

Use one result label.

- `pass`
- `pass-with-notes`
- `blocked`
- `failed`
- `not-run`

```text
[result summary]
```

## Observations

Record observations without strengthening claims beyond the evidence.

```text
[observations]
```

## Evidence links

- `[link to script, log, artifact, checksum, docs test, or PR]`

## Boundaries and non-claims

State what this report does not prove.

```text
[boundaries]
```

## Promotion recommendation

Choose one.

- Stay at current level.
- Move from roadmap to design.
- Move from design to dry-run.
- Move from dry-run to preview.
- Move from preview to validated for the named target only.

```text
[recommendation]
```

## Follow-up work

- `[next step]`

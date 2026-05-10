# Codex Page Skeletons

> **Status:** Template reference.
>
> **Validation:** Use with `STATUS_BLOCKS.md`, `DOCS_CONTRIBUTING.md`, and `REVIEW_GUIDE.md`.
>
> **Non-claims:** These skeletons are starting points only.

## General Codex page

```md
# [Page Title]

> **Status:** [Implemented | Experimental | Design | Dry-run | Preview | Roadmap | Not claimed]
>
> **Validation:** [test/script/command/artifact]
>
> **Non-claims:** [explicit boundary]

## Purpose

[What this page explains.]

## Current behavior

[What exists now.]

## Validation

[How to verify it.]

## Limitations

[What this page does not prove.]

## Roadmap

[Future work, if any.]
```

## Phase1 command page

```md
# [Command Name]

> **Status:** [status]
>
> **Validation:** [test or command help path]
>
> **Non-claims:** [boundary]

## Usage

```text
[command usage]
```

## Behavior

[What the command does.]

## Capability

[Capability name or none.]

## Examples

[Read-only examples first.]
```

## Base1 planning page

```md
# [Base1 Topic]

> **Status:** [Design | Dry-run | Preview | Roadmap]
>
> **Validation:** [design note, dry-run command, artifact, or report]
>
> **Non-claims:** [what is not released or validated]

## Scope

[Exact scope.]

## Operator-visible behavior

[What the operator can see or verify.]

## Evidence level

[Design, dry-run, preview, or validation level.]

## Promotion gate

[What evidence is needed before stronger wording.]
```

## Fyr page

```md
# [Fyr Topic]

> **Status:** [status]
>
> **Validation:** [example, test, or roadmap link]
>
> **Non-claims:** [language/tooling boundary]

## Syntax or workflow

[Current syntax or workflow.]

## Example

```fyr
[example]
```

## Expected result

[Expected output or diagnostic.]

## Roadmap notes

[Future language or toolchain work.]
```

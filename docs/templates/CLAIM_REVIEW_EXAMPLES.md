# Codex Claim Review Examples

> **Status:** Template reference.
>
> **Validation:** Use with the Codex contribution and review guides.
>
> **Non-claims:** These examples are writing guidance only.

Use this page to rewrite broad claims into narrow, reviewable statements.

## Pattern

Strong Codex writing should answer:

- What exists now?
- How can it be checked?
- What does it not prove?
- What is future work?

## Example rewrites

| Avoid | Prefer |
| --- | --- |
| `Phase1 is secure.` | `Phase1 blocks selected host-backed commands by default unless the documented trust gate is enabled.` |
| `Base1 is ready.` | `Base1 is documented as roadmap or validation-gated work unless a page links to release artifacts and validation reports.` |
| `Recovery is complete.` | `This page documents the current recovery design or dry-run path and lists evidence needed before stronger wording.` |
| `The installer is safe.` | `This workflow is documented as a dry-run/read-only planning path unless write behavior is explicitly implemented and validated.` |
| `Fyr is production-ready.` | `Fyr currently documents the tested language/tooling surface and marks future language work as roadmap.` |
| `Host tools are sandboxed.` | `Host-backed commands remain host execution; Phase1 documents gates, confirmations, and logs around invocation.` |

## Review notes

When a sentence sounds impressive but not testable, rewrite it until it names:

- the mechanism;
- the command or artifact;
- the validation path;
- the boundary.

## Good sentence shapes

```md
Current behavior: [specific behavior].
Validation: [test, command, script, report, or artifact].
Boundary: This does not claim [larger property].
```

```md
Roadmap: [future goal].
Current status: [design, dry-run, preview, or not implemented].
Promotion gate: [evidence required before stronger wording].
```

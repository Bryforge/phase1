# Phase1 Documentation Claims Policy

> **Status:** Documentation governance policy.
>
> **Validation:** Enforced by manual docs review and `tests/manual_roadmap_docs.rs`.
>
> **Non-claims:** This policy does not prove that Phase1, Base1, or Fyr has a property. It defines how claims must be written and reviewed.

This page defines how Phase1 documentation should describe current implementation, experimental work, design work, roadmap items, and explicit non-claims.

## Required status labels

Every major manual page should use one of these labels:

| Label | Meaning |
| --- | --- |
| Implemented | Code exists, tests exist, and docs explain how to run it. |
| Experimental | Code exists but UX, hardening, or boundary behavior is still changing. |
| Design | Architecture exists in docs but implementation is incomplete. |
| Dry-run | The command validates intent without destructive mutation. |
| Preview | Early tester-facing feature with known limitations. |
| Roadmap | Planned future work. |
| Not claimed | Explicitly outside current guarantees. |

## Required evidence by claim type

| Claim type | Required evidence |
| --- | --- |
| Command exists | Command docs, help/man page, or test. |
| Feature implemented | Code path plus test or reproducible command. |
| Host mutation guarded | Capability metadata, trust gate behavior, confirmation path, and docs. |
| Read-only validation | Script source, test, and statement of what it reads or writes. |
| Base1 boot behavior | Boot artifact, build recipe, checksum, and validation report. |
| Hardware validation | Named hardware, date, exact test path, pass/fail notes. |
| Recovery support | Recovery steps, rollback metadata, shell preservation, and validation evidence. |
| Security property | Threat model, implementation details, tests, and review notes. |

## Preferred wording

Use precise wording:

- `Phase1 currently runs as a host-backed Rust terminal console.`
- `Base1 is planned as a minimal host foundation.`
- `This workflow is dry-run only.`
- `This command requires explicit operator confirmation before host mutation.`
- `This feature is implemented as a metadata-only checkpoint.`
- `This claim is not made until validated on named hardware.`

## Disallowed or restricted wording

Do not use these phrases unless there is linked evidence and maintainer review:

- `secure OS replacement`
- `finished operating system`
- `hardened sandbox`
- `drop-in replacement for Linux, macOS, or Windows`
- `daily-driver ready`
- `installer-ready`
- `safe installer`
- `recovery-complete`
- `hardware-validated` without named hardware and report
- `unbreakable`, `military-grade`, `fully secure`, `guaranteed safe`

## Base1 wording rules

Allowed:

> Base1 is the planned minimal trusted host foundation for future Phase1-first bootable environments.

Allowed when evidence exists:

> This Base1 dry-run validates target identity without writing to the selected target.

Not allowed without release evidence:

> Base1 is a finished bootable secure OS.

## Fyr wording rules

Allowed:

> Fyr is the Phase1-native language and toolchain track for `.fyr` files and Phase1 workflows.

Allowed when tests exist:

> The current Fyr toolchain supports the documented examples in this chapter.

Not allowed without production evidence:

> Fyr is production-ready for general software development.

## Recovery wording rules

Allowed:

> Recovery USB behavior is documented as design, dry-run, preview, or validated depending on the evidence linked on the page.

Not allowed without hardware reports:

> Real-hardware recovery is complete.

## PR review questions

Before merging documentation, ask:

1. Does the page say what is current and what is roadmap?
2. Does every safety claim name a mechanism?
3. Does every mechanism have a visible operator path?
4. Does the page avoid implying destructive installer readiness?
5. Does the page avoid implying Base1 is bootable unless explicitly marked preview or release with evidence?
6. Does the page preserve the statement that Phase1 is not currently a finished secure OS replacement?

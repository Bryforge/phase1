# Fyr book inside Phase1

Issue: #323  
Status: book reader contract  
Scope: offline Fyr book content readable from the Phase1 shell  
Non-claim: this document defines the book reader contract; it does not claim the runtime `fyr book` command is implemented yet.

The Fyr book should be available inside Phase1 so operators can learn without leaving the shell. The reader must be compact, accessible, deterministic, offline-first, and safe.

## Operator-facing command surface

Preferred command surface:

```text
fyr book
fyr book list
fyr book read <chapter>
fyr book next
fyr book prev
fyr book search <term>
fyr book help
```

The reader should also be linked from:

```text
fyr help
fyr learn
fyr self
```

## Reader behavior

Required behavior:

- list chapters with stable ids;
- read a selected chapter by id or slug;
- show next and previous chapter guidance;
- show lesson links when a chapter maps to Fyrlings lessons;
- search chapter text offline;
- render in compact terminal widths;
- preserve readable output in no-color mode;
- preserve readable output in ASCII mode;
- keep every safety state visible through text labels;
- recover from unknown chapter ids with help guidance;
- remain deterministic and testable.

## Book structure

Initial chapter set:

| Chapter | Title | Purpose | Related Fyrlings lessons |
| --- | --- | --- | --- |
| 00 | What is Fyr? | orientation, non-claims, safety | 001 |
| 01 | Your first Fyr program | `fyr new`, `fyr run`, hello flow | 002 |
| 02 | Main functions | `fn main() -> i32` | 003 |
| 03 | Printing and returning | `print`, `return` | 004, 005 |
| 04 | Fixing parser errors | diagnostics and recovery | 006 |
| 05 | Values and expressions | `let`, expressions, `if` where supported | 007, 008, 009 |
| 06 | Testing with assertions | `assert_eq`, boolean asserts, comparisons | 010, 011, 012 |
| 07 | Packages and modules | manifests, modules, duplicate main recovery | 013, 014 |
| 08 | Reading Fyr code | syntax highlighting and no-color fallback | 015 |
| 09 | Staged candidate mode | `black_arts` and non-live staging | 016 |
| 10 | Operator validation | full validation demo and trust boundaries | 017 |

## Required chapter metadata

Every chapter fixture should include:

```text
book          : Fyr
chapter       : <stable-id>
title         : <chapter-title>
audience      : first-time and returning operators
controls      : direct command | tab-complete | help-first | paste-safe | mobile-safe
fallback      : ascii | no-color | compact-terminal
runtime       : VFS-only
host-tools    : blocked
network       : blocked
live-system   : untouched
claim-boundary: book-contract-only
```

## Accessibility and small-terminal requirements

The in-Phase1 reader must account for:

- mobile terminal width;
- keyboard-only navigation;
- copy/paste-friendly examples;
- clear headings;
- short paragraphs;
- no dependency on color alone;
- plain ASCII fallback for symbols;
- explicit safety labels;
- next-step and recovery cues.

## Safety and non-claims

The book reader must not perform or imply:

```text
host shell execution
network access
Cargo invocation from Fyr book commands
Rust compiler invocation from Fyr book commands
live-system writes
autonomous promotion
autonomous mutation
self-hosting completion
production OS replacement claims
```

The book may describe future goals only when they are labeled as future work or non-claims.

## Runtime implementation gates

1. Land this reader contract and chapter fixtures.
2. Add deterministic tests for metadata, chapter list, accessibility cues, control schemes, and forbidden behavior.
3. Add an in-shell reader that can list and read repository/VFS-backed chapter text.
4. Add search once list/read is stable.
5. Link `fyr book` from `fyr help` and `fyr learn`.
6. Do not update public Fyr completion percentage until runtime behavior and tests land.

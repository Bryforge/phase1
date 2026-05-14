# Fyr automation validation matrix

Issue: #321  
Status: validation contract  
Scope: full Fyr command, language, demo, user-flow, and control-scheme coverage  
Non-claim: this matrix does not claim every listed runtime path is implemented yet.

This matrix makes Fyr completion measurable. Every Fyr feature must be backed by deterministic tests, fixture evidence, user-facing recovery cues, and safety boundaries before it can count toward 100% completion.

## Validation levels

| Level | Meaning | Counts toward public runtime completion? |
| --- | --- | --- |
| Contract | Documented expected behavior and safety boundary | No |
| Fixture | Expected deterministic output exists | No |
| Source | Runtime command or parser behavior exists | Yes, only with tests |
| Integration test | Automated test exercises the runtime path | Yes |
| Demo path | Built-in operator demo exposes the flow safely | Yes, only with source and tests |

## Command coverage matrix

| Command | Required validation | Current gate | Required user cue | Required safety boundary |
| --- | --- | --- | --- | --- |
| `fyr status` | source + integration test + demo row | existing Fyr bootstrap | explain current Fyr capability | no host command, deterministic output |
| `fyr spec` | source + integration test + docs fallback | existing Fyr bootstrap | explain missing-doc recovery | read-only docs access |
| `fyr new <name>` | source + integration test + demo row | existing Fyr authoring | show created file and next command | VFS-only write |
| `fyr init <package>` | source + integration test + demo row | existing Fyr package flow | show package layout and next command | VFS-only package write |
| `fyr cat <file.fyr>` | source + integration test + error recovery | existing Fyr authoring | show missing file guidance | VFS-only read |
| `fyr check <file.fyr|package>` | parser diagnostic tests + demo row | existing Fyr parser | show parse error and next fix | no host compiler |
| `fyr build <file.fyr|package>` | dry-run build tests + demo row | existing Fyr build dry run | show backend and host boundary | no Cargo/Rust invocation |
| `fyr test <package>` | package test tests + failure diagnostics | existing Fyr test runner | show pass/fail summary | VFS-only tests |
| `fyr self` | source + integration test + demo row | existing self-report | explain non-self-hosting boundary | no self-update claim |
| `fyr run <file.fyr>` | runtime output tests + demo row | existing seed runtime | show output and exit status | no host execution |
| `fyr color <file.fyr>` | color tests + no-color fallback | existing syntax color path | show ANSI/no-color difference | no hidden meaning by color only |
| `fyr highlight <file.fyr>` | alias tests + no-color fallback | existing syntax color path | show readable fallback | no hidden meaning by color only |
| `fyr staged` | runtime source + integration tests | #317 / F100-1 | show black_arts staged mode and boundaries | fixture-backed, non-live, candidate-only |
| `fyr staged status` | runtime source + integration tests | #317 / F100-1 | show candidate state and live boundary | fixture-backed, non-live, candidate-only |
| `fyr staged help` | runtime source + integration tests | #317 / F100-1 | show usage, workspace, allowed commands | fixture-backed, non-live, candidate-only |
| `fyr staged <unknown>` | runtime source + integration tests | #317 / F100-1 | show no-op and help guidance | no write, no host, no network |

## Language feature coverage matrix

| Feature | Required validation | Required diagnostic behavior | Safety boundary |
| --- | --- | --- | --- |
| `fn main() -> i32` | parser tests | missing/duplicate main is deterministic | parser-only |
| `print("literal")` | parser + run tests | unterminated string is deterministic | no host stdout dependency beyond Phase1 output |
| `return <integer>` | parser + run tests | invalid return is deterministic | parser-only |
| `let` bindings | parser + run tests where supported | invalid binding name/value is deterministic | parser-only |
| integer expressions | parser + eval tests where supported | division by zero is deterministic | parser-only |
| `if` handling | parser + branch tests where supported | invalid condition is deterministic | parser-only |
| `assert_eq` | test-runner tests | mismatch reports expected/actual | VFS-only tests |
| boolean `assert` | test-runner tests | false assertion reports label/context | VFS-only tests |
| comparison assertions | test-runner tests | comparison failure is deterministic | VFS-only tests |
| package manifest | package tests | missing manifest is deterministic | VFS-only reads |
| module discovery | resolver tests | duplicate `fn main` is deterministic | VFS-only reads |
| syntax coloring | color/fallback tests | unreadable color-only output is blocked | text labels remain visible |

## Built-in validation demo contract

The built-in demo should be reachable through one of these stable surfaces:

```text
fyr demo validation
fyr validate demo
fyr self validate
```

The final runtime command can choose one canonical spelling, but docs and help should guide users to the canonical command.

The demo must include these sections:

```text
FYR VALIDATION DEMO
mode          : deterministic
audience      : first-time and returning operators
controls      : direct command | tab-complete | help-first | paste-safe | mobile-safe
fallback      : ascii | no-color | compact-terminal
runtime       : VFS-only
host-tools    : blocked
network       : blocked
live-system   : untouched
staged        : non-live black_arts candidate mode
summary       : pass/fail rows are deterministic
```

## User considerations matrix

| User consideration | Required demo cue | Required test assertion |
| --- | --- | --- |
| first-time user | show `fyr help` / `fyr status` path | demo fixture contains help-first cue |
| mobile/small terminal | compact rows, no wide tables in runtime demo | demo fixture contains compact-terminal cue |
| keyboard-only | direct command and tab-complete guidance | demo fixture contains direct command and tab-complete cues |
| paste-safe use | commands can be copied line-by-line | demo fixture contains paste-safe cue |
| no-color terminals | ASCII/no-color fallback listed | fixture contains ascii and no-color cues |
| low-vision readability | text labels must carry meaning, not only symbols | fixture contains text labels for every safety state |
| error recovery | unknown actions point to help | fixture contains unknown-command recovery cue |
| trust boundaries | host/network/live writes are visibly blocked | fixture contains blocked boundary rows |

## Control scheme requirements

Fyr automation validation must cover:

```text
direct command entry
tab-completion expectations
help-first discovery
copy/paste command flow
mobile terminal constraints
no-color output
compact output
safe-mode boundary
guarded-host boundary
unknown-command recovery
```

## Forbidden validation-demo behavior

The demo and its tests must not introduce or imply:

```text
host shell execution
network access
Cargo invocation from Fyr commands
Rust compiler invocation from Fyr commands
live-system staged writes
autonomous promotion
autonomous mutation
self-hosting completion
production OS replacement claims
```

## Immediate implementation sequence

1. Land this matrix and the validation-demo fixture.
2. Add regression tests that enforce the matrix, user cues, control schemes, and forbidden behavior list.
3. Wire a real runtime command only after the fixture and acceptance tests exist.
4. Implement #317 so `fyr staged` becomes real runtime behavior.
5. Add the first runtime validation demo command after #317 is complete.
6. Do not raise public Fyr completion percentage until runtime implementation and tests land.
7. Promote public status only when source and tests prove the behavior.

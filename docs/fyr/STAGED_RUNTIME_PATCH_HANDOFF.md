# Fyr black_arts runtime stub patch handoff

Issue: #317  
Status: implementation handoff  
Scope: first safe `fyr staged` runtime source wiring  
Non-claim: this does not implement candidate creation, apply/change behavior, validation execution, promotion, discard, host command execution, network access, or live-system writes.

The first runtime implementation must be limited to deterministic fixture-backed output for the staged command family.

## Source target

File:

```text
src/main.rs
```

Existing function:

```rust
fn fyr_command(shell: &mut Phase1Shell, args: &[String]) -> String
```

Add this match arm before `Some("help")`:

```rust
Some("staged") => fyr_staged(&args[1..]),
```

## Helper functions to add

Add these near the other Fyr helpers:

```rust
fn fyr_staged(args: &[String]) -> String {
    match args.first().map(String::as_str) {
        None | Some("status") => fyr_staged_visual(),
        Some("help") | Some("-h") | Some("--help") => fyr_staged_help(),
        Some(other) => fyr_staged_unknown(other),
    }
}

fn fyr_staged_visual() -> String {
    concat!(
        "☠ FYR black_arts // STAGED CANDIDATE MODE\n",
        "[BLACK_ARTS] FYR staged candidate mode\n",
        "candidate     : phase1-base1-candidate\n",
        "workspace     : .phase1/staged-candidates/phase1-base1-candidate\n",
        "state         : fixture-backed\n",
        "live-system   : untouched\n",
        "promotion     : blocked-until-validation-and-approval\n",
        "evidence      : docs/fyr/fixtures/staged-lifecycle-example.txt\n",
        "boundary      : candidate-only | non-live | evidence-bound | claim-boundary\n",
        "commands      : status, plan, create, apply, validate, promote, discard\n",
        "implementation: pending\n",
        "claim-boundary: fixture-only\n",
    )
    .to_string()
}

fn fyr_staged_help() -> String {
    concat!(
        "fyr staged help\n",
        "codename      : black_arts\n",
        "status        : fixture-backed design help\n",
        "usage         : fyr staged <status|plan|create|apply|validate|promote|discard>\n",
        "commands      : status, plan, create, apply, validate, promote, discard\n",
        "workspace     : .phase1/staged-candidates\n",
        "boundaries    : candidate-only, non-live, evidence-bound, claim-boundary\n",
        "promotion     : validation-and-approval-required\n",
        "implementation: pending\n",
        "claim-boundary: fixture-only\n",
    )
    .to_string()
}

fn fyr_staged_unknown(action: &str) -> String {
    format!(
        "fyr staged {action}\n\
         codename      : black_arts\n\
         status        : unknown staged action\n\
         action        : {action}\n\
         live-system   : untouched\n\
         candidate     : none\n\
         result        : no-op\n\
         help          : fyr staged help\n\
         boundaries    : non-live, no-write, evidence-bound, claim-boundary\n\
         claim-boundary: fixture-only\n"
    )
}
```

## Required integration test

Create a test file such as:

```text
tests/fyr_black_arts_runtime_wiring.rs
```

Required command cases:

```text
fyr staged
fyr staged status
fyr staged help
fyr staged nonsense
```

Required assertions:

- `fyr staged` includes `☠ FYR black_arts // STAGED CANDIDATE MODE`.
- `fyr staged` includes `[BLACK_ARTS] FYR staged candidate mode`.
- `fyr staged` includes `live-system   : untouched`.
- `fyr staged` includes `claim-boundary: fixture-only`.
- `fyr staged status` includes the same non-live visual markers.
- `fyr staged help` includes the usage row.
- `fyr staged nonsense` includes `result        : no-op`, `candidate     : none`, and `help          : fyr staged help`.
- output does not include host command markers: `cargo `, `rustc `, `bash:`, `sh:`, `http://`, `https://`.

## Explicitly blocked

Do not implement these in the first source-wiring patch:

```text
candidate creation
candidate apply/change behavior
validation execution
promotion execution
discard execution
host command execution
network access
live-system writes
```

## Evidence fixtures already available

```text
docs/fyr/fixtures/staged-runtime-stub-ok.txt
docs/fyr/fixtures/staged-help-ok.txt
docs/fyr/fixtures/staged-status-example.txt
docs/fyr/fixtures/staged-unknown-action-ok.txt
docs/fyr/fixtures/staged-operator-visual-ok.txt
```

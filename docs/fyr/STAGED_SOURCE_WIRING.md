# Fyr staged source wiring checklist

Status: implementation checklist  
Codename: `black_arts`  
Scope: first safe `fyr staged` source wiring  
Non-claim: this does not implement candidate creation, apply, validation, promotion, discard, or live-system changes.

This checklist defines the first source wiring step for the staged candidate track. The first implementation must only expose fixture-backed, non-live help/status output.

## First safe behavior

The first safe implementation may add:

```text
fyr staged
fyr staged help
fyr staged status
fyr staged unknown
```

The first safe implementation must not add:

```text
candidate writes
live-system changes
host command execution
network access
promotion behavior
validation execution
apply behavior
discard behavior
```

## Required source hooks

Update `src/main.rs` only after this checklist is satisfied:

1. Add `Some("staged") => fyr_staged(&args[1..])` inside `fyr_command`.
2. Add a `fyr_staged(args: &[String]) -> String` helper.
3. Add a `fyr_staged_help() -> String` helper.
4. Make `fyr staged`, `fyr staged help`, and `fyr staged status` return deterministic fixture-backed output.
5. Make unknown staged actions return a no-op help-guidance response.

## Required tests

Implementation PR must include tests for:

- `fyr staged` prints the runtime-stub rows.
- `fyr staged help` prints the help rows.
- `fyr staged status` prints the status rows.
- unknown staged action prints the no-op rows.
- output includes `fixture-only`.
- output includes `live-system   : untouched` or an equivalent non-live marker.
- output does not contain host command execution markers.

## Fixture sources

Reference fixtures:

```text
docs/fyr/fixtures/staged-runtime-stub-ok.txt
docs/fyr/fixtures/staged-help-ok.txt
docs/fyr/fixtures/staged-status-example.txt
docs/fyr/fixtures/staged-unknown-action-ok.txt
```

## Promotion boundary

Do not implement `create`, `apply`, `validate`, `promote`, or `discard` behavior in the first source-wiring PR. Those actions should remain fixture/documentation contracts until separate implementation gates exist.

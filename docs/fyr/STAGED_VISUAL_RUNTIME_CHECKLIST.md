# Fyr black_arts visual runtime checklist

Status: implementation checklist  
Scope: showing operator-facing `black_arts` visual cues in the first safe `fyr staged` runtime stub  
Non-claim: this does not implement candidate creation, apply, validation, promotion, discard, or live-system changes.

The first `fyr staged` source wiring must make `black_arts` mode visibly different from normal Fyr command output while preserving text safety boundaries.

## First visual runtime target

The first safe runtime output should include the rows from:

```text
docs/fyr/fixtures/staged-operator-visual-ok.txt
```

It may be printed by:

```text
fyr staged
fyr staged status
fyr staged help
```

Unknown staged actions should still print a no-op/help response.

## Required visible rows

The runtime output must include:

```text
☠ FYR black_arts // STAGED CANDIDATE MODE
candidate     : phase1-base1-candidate
workspace     : .phase1/staged-candidates/phase1-base1-candidate
state         : fixture-backed
live-system   : untouched
promotion     : blocked-until-validation-and-approval
evidence      : docs/fyr/fixtures/staged-lifecycle-example.txt
boundary      : candidate-only | non-live | evidence-bound | claim-boundary
claim-boundary: fixture-only
```

## ASCII fallback requirement

The runtime output or help must include:

```text
[BLACK_ARTS] FYR staged candidate mode
```

## Source wiring target

When implemented in `src/main.rs`, the first source PR should:

1. Add `fyr_staged_visual() -> String` or equivalent.
2. Call it from `fyr_staged` for the no-argument/status path.
3. Keep `fyr_staged_help()` plain-text and readable.
4. Keep unknown actions no-op and help-guided.
5. Avoid adding candidate writes or host commands.

## Required tests for source implementation

The implementation PR must test that:

- `fyr staged` includes the visual banner.
- `fyr staged status` includes the visual banner.
- `fyr staged help` includes the ASCII fallback and boundary text.
- unknown actions do not select a candidate.
- output includes `live-system   : untouched`.
- output includes `claim-boundary: fixture-only`.

## Blocked in first implementation

Do not implement these in the visual runtime PR:

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

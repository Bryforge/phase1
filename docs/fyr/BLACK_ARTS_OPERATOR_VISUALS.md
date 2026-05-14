# Fyr black_arts operator visual mode

Status: design contract  
Scope: operator-facing visual cues for staged candidate mechanics  
Non-claim: this does not implement live mutation, promotion, recovery, or production update behavior.

When an operator uses the `black_arts` staged candidate mechanics, Phase1 should make the mode obvious. The interface must show that the operator is not in normal Fyr mode and not changing the live system directly.

## Design goals

The visual mode should communicate:

- `black_arts` is active;
- staged candidate mechanics are being used;
- the current candidate name;
- the current lifecycle state;
- whether promotion is blocked, approved, or unavailable;
- whether the live system is untouched;
- where evidence is recorded;
- that the track remains fixture-backed until implementation lands.

## Required visual cues

The first implementation should include plain-text safe cues before any advanced UI:

```text
☠ FYR black_arts // STAGED CANDIDATE MODE
candidate     : phase1-base1-candidate
workspace     : .phase1/staged-candidates/phase1-base1-candidate
state         : fixture-backed
live-system   : untouched
promotion     : blocked-until-validation-and-approval
evidence      : docs/fyr/fixtures/staged-lifecycle-example.txt
boundary      : candidate-only | non-live | evidence-bound | claim-boundary
```

ASCII fallback:

```text
[BLACK_ARTS] FYR staged candidate mode
```

## Prompt marker

When an interactive staged mode exists, the prompt should include a clear marker:

```text
fyr:black_arts(candidate=phase1-base1-candidate,state=staged)> 
```

If the terminal cannot render symbols, use:

```text
fyr:black_arts> 
```

## Color and style direction

Preferred style:

- black background;
- ember orange accent;
- subtle red warning accent only for blocked or rejected actions;
- no green success glow unless validation has passed;
- no celebratory promotion visuals until explicit approval exists;
- keep text readable in monochrome terminals.

## Safety wording

Every visual mode report should preserve these messages:

```text
live-system   : untouched
promotion     : blocked-until-validation-and-approval
boundary      : candidate-only | non-live | evidence-bound | claim-boundary
```

## Implementation rule

Do not hide the safety state behind icons. Icons may decorate, but text must always say the current boundary.

## First fixture

Reference fixture:

```text
docs/fyr/fixtures/staged-operator-visual-ok.txt
```

# Fyr F3 if-body assertion regression

Issue: #330  
Status: regression handoff  
Scope: parser/runtime support for supported assertion statements before `return` inside F3 `if` bodies  
Non-claim: this document does not fix the runtime behavior by itself.

## Failing test

```sh
cargo test -p phase1 --test fyr_f3_expression_diagnostics
```

Failing case:

```text
fyr_f3_boolean_operator_precedence_remains_deterministic
```

## Current failure

The current runtime rejects an `if` body containing `assert_eq(...)` before `return`:

```text
fyr check: bool.fyr: expected return statement in if body
fyr build: bool.fyr: expected return statement in if body
```

## Source under test

```fyr
fn main() -> i32 {
  let answer = 42;
  if (answer > 40 && answer < 50) {
    assert_eq(answer, 42);
    return answer;
  }
  return 0;
}
```

## Expected behavior

The Fyr F3 parser/rewriter should accept supported assertion statements before the required `return` statement inside an `if` body.

Expected output:

```text
fyr check: ok bool.fyr
status  : dry-run artifact ready
```

## Implementation target

In `src/main.rs`, find the code path that emits:

```text
expected return statement in if body
```

Update that path so an F3 `if` body can consume the supported statement sequence:

```text
assert_eq(...);
assert(...);
return <integer-expression-or-binding>;
```

The fix should preserve the existing requirement that a supported F3 `if` body has a return. It should not broadly accept arbitrary statements.

## Suggested parser rule

A supported F3 `if` body should be parsed as:

```text
zero or more supported assertion statements
one required return statement
```

Supported assertion statements for this fix:

```text
assert_eq(<integer-expression>, <integer-expression>);
assert(<boolean-expression>);
```

## Safety boundary

This is a parser/runtime fix only.

Do not introduce:

```text
host command execution
network access
Cargo invocation from Fyr commands
Rust compiler invocation from Fyr commands
live-system writes
unsupported arbitrary statements in if bodies
```

## Acceptance commands

```sh
cargo test -p phase1 --test fyr_f3_expression_diagnostics
cargo test -p phase1 --test fyr_boolean_operators
cargo test -p phase1 --test fyr_boolean_grouping
cargo test -p phase1 --test fyr_automation_validation_matrix
```

## Completion rule

Close #330 only after the runtime parser fix lands and the acceptance commands pass.

# Nest ghost stack runtime handoff

Issue: #332  
Status: runtime handoff  
Scope: first read-only runtime implementation for the nest ghost stack control surface  
Non-claim: this handoff is not runtime implementation until `src/main.rs` is patched and tested.

## Goal

Wire the first read-only nest ghost stack commands into the existing Phase1 `nest` command path.

The first slice should implement only:

```text
nest stack
nest stack status
nest stack list
```

All other stack actions should remain explicit no-op/not-yet-implemented responses.

## Source target

File:

```text
src/main.rs
```

Existing functions to inspect:

```text
fn nest_command(...)
fn nested_level()
fn nested_max()
fn nest_exit_all_requested()
fn request_nest_exit_all(...)
```

Recommended source locator:

```sh
grep -n "fn nest_command" src/main.rs
grep -n "fn nested_level" src/main.rs
grep -n "fn nested_max" src/main.rs
grep -n "fn nest_exit_all_requested" src/main.rs
grep -n "fn request_nest_exit_all" src/main.rs
```

Then inspect the command function with a local line window around `fn nest_command`.

## Required runtime behavior

`nest stack`, `nest stack status`, and `nest stack list` should print the fixture-shaped status rows:

```text
phase1 nest stack
mode          : read-only status
nest-level    : <current>/<max>
root          : active
current       : level-<n>
ghost-count   : 0
exit-all      : clear|requested
safe-mode     : visible
trust         : visible
guardrail     : no host process spawn | no network | no isolation claim
claim-boundary: control-plane-only
```

## Suggested helper functions

Add small helpers near the existing nest helpers:

```rust
fn nest_stack_command(shell: &Phase1Shell, args: &[String]) -> String
fn nest_stack_status(shell: &Phase1Shell) -> String
fn nest_stack_pending(action: &str) -> String
```

The `shell` argument is included so the status can report safe/trust state later if already available from the shell env. The first implementation may report text-visible placeholder rows if needed, but must not hide the boundary.

## Suggested command routing

Inside the existing `nest_command(...)` match or argument handling, add a `stack` subcommand path equivalent to:

```rust
Some("stack") => nest_stack_command(shell, &args[1..]),
```

If the existing function takes no shell reference, either:

1. pass the shell reference into the helper from the existing caller; or
2. implement `nest_stack_status()` using `nested_level()`, `nested_max()`, and `nest_exit_all_requested()` only for the first read-only slice.

Do not rewrite unrelated nest behavior.

## Pending actions

The first slice should not mutate state. These commands should return no-op/pending guidance:

```text
nest stack push <label>
nest stack pop
nest stack ghost <label>
nest stack resume <label>
nest stack prune
nest stack exit-all
```

The output should include:

```text
status        : not-yet-implemented
result        : no-op
help          : nest stack status
claim-boundary: control-plane-only
```

## Safety boundaries

Do not introduce:

```text
host command execution
network access
hidden process spawning
unbounded recursion
live-system writes outside documented Phase1 state/control files
autonomous stack mutation
VM isolation claims
container isolation claims
physical machine separation claims
security boundary hardening claims
```

## Required tests after source patch

```sh
cargo fmt --all -- --check
cargo test -p phase1 --test nest_ghost_stack_contract
cargo test -p phase1 --test nest
cargo test --workspace --all-targets
```

## Manual smoke

```text
nest stack
nest stack status
nest stack list
nest stack push demo
nest stack ghost demo
nest stack resume demo
nest stack exit-all
```

Expected first-slice result:

- status/list commands are read-only and print current stack state;
- mutating actions are no-op/pending and help-guided;
- output keeps the control-plane-only claim boundary visible.

## Completion rule

Close #332 only after the runtime source and tests land, and manual smoke confirms the command surface is visible in Phase1.

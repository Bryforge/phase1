# Fyr staged runtime local apply

Issue: #317  
Status: local apply instructions  
Scope: applying `patches/fyr-staged-runtime-stub.patch` to `src/main.rs` from a developer checkout  
Non-claim: this document does not mean the runtime source has landed until `src/main.rs` is actually patched and tested.

## Why this exists

The required `fyr staged` runtime patch is small, but `src/main.rs` is large. The GitHub connector view can truncate the file, so a full-file rewrite through that path risks damaging unrelated shell code.

Use the checked patch and helper script from a local checkout instead.

## Apply command

```sh
sh scripts/apply-fyr-staged-runtime-stub.sh
```

The helper:

- checks that `patches/fyr-staged-runtime-stub.patch` exists;
- checks that `src/main.rs` exists;
- detects whether the staged match arm is already present;
- runs `git apply --check` before applying;
- applies only the source patch;
- prints the required validation commands.

## Expected source change

The patch adds this `fyr_command` match arm:

```rust
Some("staged") => fyr_staged(&args[1..]),
```

The patch adds these helpers:

```rust
fn fyr_staged(args: &[String]) -> String
fn fyr_staged_visual() -> String
fn fyr_staged_help() -> String
fn fyr_staged_unknown(action: &str) -> String
```

## Runtime behavior after applying

```text
fyr staged
fyr staged status
fyr staged help
fyr staged nonsense
```

Required visible markers:

```text
☠ FYR black_arts // STAGED CANDIDATE MODE
[BLACK_ARTS] FYR staged candidate mode
live-system   : untouched
promotion     : blocked-until-validation-and-approval
boundary      : candidate-only | non-live | evidence-bound | claim-boundary
claim-boundary: fixture-only
```

Unknown staged actions must report:

```text
status        : unknown staged action
candidate     : none
result        : no-op
help          : fyr staged help
```

## Required validation

```sh
cargo fmt --all -- --check
cargo test -p phase1 --test fyr_staged_runtime_patch_contract
cargo test -p phase1 --test fyr_black_arts_runtime_stub
cargo test -p phase1 --test fyr_black_arts_unknown_action
cargo test --workspace --all-targets
```

## Manual smoke checks

After building/running Phase1, verify:

```text
fyr staged
fyr staged status
fyr staged help
fyr staged nonsense
```

The output must not contain:

```text
cargo 
rustc 
bash:
sh:
http://
https://
```

## Still blocked

Do not implement the following in this first runtime patch:

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

## Completion rule

Close #317 only after:

1. `src/main.rs` is patched;
2. runtime tests land;
3. manual smoke confirms `fyr staged` is recognized;
4. blocked behavior remains absent.

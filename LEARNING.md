# Phase1 Learning System

Phase1 now includes a local-first learning companion binary named `phase1-learn`. It gives Phase1 a memory layer that can observe command patterns, import sanitized shell history, store operator notes, learn simple rules, answer learned questions, and suggest next steps.

The learning system is intentionally conservative:

- local memory only; no network calls
- no external AI model or cloud dependency
- sanitized command and note storage
- bounded memory sizes to prevent unbounded growth
- explicit forget controls for notes, rules, commands, or all memory

## Run it

```bash
cargo run --bin phase1-learn -- status
```

The default memory file is:

```text
phase1.learn
```

Override it with:

```bash
PHASE1_LEARN_PATH=.phase1/dev.learn cargo run --bin phase1-learn -- status
```

## Learn from Phase1 history

When Phase1 persistent history is enabled, commands are written to `phase1.history` in sanitized form. Import that history into the learning profile:

```bash
cargo run --bin phase1-learn -- import-history
```

Import from a custom history file:

```bash
cargo run --bin phase1-learn -- import-history path/to/history
```

## Teach explicit knowledge

Teach a local rule:

```bash
cargo run --bin phase1-learn -- teach deploy = use main for GitHub Pages deploys
```

Ask it back:

```bash
cargo run --bin phase1-learn -- ask deploy
```

Store an operator note:

```bash
cargo run --bin phase1-learn -- note prefer safe mode for demos
```

## Observe command outcomes

Record a successful command:

```bash
cargo run --bin phase1-learn -- observe ok -- avim notes.rs
```

Record a failed command:

```bash
cargo run --bin phase1-learn -- observe fail -- git push
```

Record a command without an outcome:

```bash
cargo run --bin phase1-learn -- observe seen -- sysinfo
```

## Smart suggestions

Show the local profile:

```bash
cargo run --bin phase1-learn -- profile
```

Show the next suggested action:

```bash
cargo run --bin phase1-learn -- suggest
```

Suggestions combine local command frequency with Phase1 workflow heuristics. For example, frequent editor usage suggests review commands such as `grep` or `cat`, and frequent security/audit usage suggests `capabilities`.

## Export and forget

Export the current memory file:

```bash
cargo run --bin phase1-learn -- export
```

Forget a matching query:

```bash
cargo run --bin phase1-learn -- forget deploy
```

Clear command stats only:

```bash
cargo run --bin phase1-learn -- forget commands
```

Clear all learning memory:

```bash
cargo run --bin phase1-learn -- forget all
```

## Privacy model

`phase1-learn` stores a local `phase1.learn` file. It redacts secret-like text, including password, token, credential, private-key, GitHub token, bearer, and authorization markers. The file is ignored by git so local memory does not get committed.

This is a smart local heuristic layer, not a cloud AI service. It learns operator patterns and explicit rules without uploading data anywhere.

## Current command surface

```text
phase1-learn status
phase1-learn import-history [phase1.history]
phase1-learn observe <ok|fail|seen> -- <command>
phase1-learn teach <trigger> = <response>
phase1-learn note <text>
phase1-learn ask <query>
phase1-learn suggest
phase1-learn profile
phase1-learn forget <all|notes|rules|commands|query>
phase1-learn export
```

## Completed milestone

The in-shell learning milestone is complete: `learn`, auto-observe, failure-prioritized suggestions, typo recovery, `learn explain`, `learn doctor`, and the local update helper are merged. See `docs/archive/checkpoints/DEVELOPMENT_CHECKPOINT_LEARN.md`.

## Roadmap

- add `learn repair` for direct repair actions from failed commands
- add VFS summaries for `/home` project files
- add local classifier weights for command categories
- add guarded import/export for shareable non-secret learning profiles

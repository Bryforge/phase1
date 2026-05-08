# In-shell learning command

The `learn` command gives Phase1 an interactive local memory surface inside the main operator shell.

It mirrors the existing `phase1-learn` companion behavior while staying conservative:

- local memory only
- no network calls
- no external AI model
- sanitized notes, rules, and command names
- bounded notes, rules, and command statistics
- explicit forget controls

## Starter flow

Inside Phase1:

```text
sysinfo
security
capabilities
learn import-history
learn suggest
```

Teach a local rule:

```text
learn teach deploy = use main for GitHub Pages deploys
learn ask deploy
```

Store an operator note:

```text
learn note prefer safe mode for demos
learn profile
```

Forget memory:

```text
learn forget deploy
learn forget notes
learn forget all
```

## File format

The command writes `phase1.learn` by default and respects `PHASE1_LEARN_PATH` when set. The memory format is compatible with the external `phase1-learn` helper:

```text
# phase1 learning memory v1
NOTE	<hex text>
RULE	<hex trigger>	<hex response>
CMD	<command>	<seen>	<ok>	<fail>
```

Sensitive-looking notes are replaced with `[redacted-sensitive-memory]` before storage.

## Validation

```bash
cargo test learn --all-targets
cargo test registry --all-targets
sh scripts/phase1-learn-shell-smoke.sh
```

## Automatic observation

Phase1 can automatically observe normal shell commands after they run. Known dispatcher commands are recorded as `ok`, unknown commands are recorded as `fail`, and `learn` / `memory` commands are ignored so the learning system does not train on itself.

This is still local-only and heuristic. It does not send data to a network service, cloud model, or external API.

\n## Failure intelligence

`learn suggest` prioritizes failed commands before routine successful usage. When a command fails, Phase1 recommends a recovery path such as `help`, `complete <prefix>`, or a likely command completion.

This keeps the learning layer useful for repair and discovery without using a network service, cloud model, or external API.


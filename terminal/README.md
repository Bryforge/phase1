# terminal/

This directory contains the lightweight Phase1 Terminal wrapper.

The primary repository startup command is now:

```bash
sh phase1
./phase1
```

The root `phase1` command delegates to `./start_phase1` and is the recommended operator-friendly entrypoint.

The terminal wrapper remains deliberately small and current-master friendly:

- `bin/phase1-terminal` delegates to `../../start_phase1`.
- It keeps safe defaults and does not create a new trust path.
- It provides discovery commands for doctor, Gina, Base1, quality checks, and self-test.

Run from the repository root:

```bash
sh terminal/bin/phase1-terminal help
sh terminal/bin/phase1-terminal doctor
sh terminal/bin/phase1-terminal selftest
```

Install a local `phase1` command on macOS/Linux:

```bash
sh scripts/install-phase1-command.sh
phase1
```

Use the source launcher directly when debugging:

```bash
./start_phase1
```

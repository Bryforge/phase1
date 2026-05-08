# terminal/

This directory contains the lightweight Phase1 Terminal wrapper.

The first implementation is deliberately small and current-master friendly:

- `bin/phase1-terminal` delegates to ../../start_phase1.
- It keeps safe defaults and does not create a new trust path.
- It provides discovery commands for doctor, Gina, Base1, quality checks, and self-test.

Run from the repository root:

```bash
sh terminal/bin/phase1-terminal help
sh terminal/bin/phase1-terminal doctor
sh terminal/bin/phase1-terminal selftest
```

Use the canonical launcher directly when in doubt:

```bash
./start_phase1
```

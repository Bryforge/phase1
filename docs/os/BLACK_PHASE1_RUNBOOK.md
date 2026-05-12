# black-phase1 rapid runbook

Status: rapid development runbook  
Branch: `black-phase1`  
Purpose: fast experimental work that is not stable, not edge release, and not claim-safe until promoted.

## Branch model

```text
edge/stable   -> evidence-bound active development
black-phase1  -> rapid test/integration branch
checkpoint/*  -> frozen milestone snapshots
```

`black-phase1` is allowed to be noisy and experimental. Known-good work moves back to `edge/stable` only after validation.

## One-command router

Use the router whenever possible:

```sh
sh scripts/black-phase1.sh help
```

## Mac rapid loop

```sh
cd ~/phase1_library/phase1
git fetch origin
git checkout black-phase1
git pull --ff-only origin black-phase1
sh scripts/black-phase1.sh status
```

After edits:

```sh
sh scripts/black-phase1.sh push "Describe rapid test"
```

## X200 rapid loop

```sh
cd ~/phase1
git fetch origin
git checkout black-phase1
git pull --ff-only origin black-phase1
sh scripts/black-phase1.sh doctor /dev/sdb
sh scripts/black-phase1.sh x200-test /dev/sdb YES_WRITE_USB
```

Only reboot after the test helper prints:

```text
RESULT: prepared_and_verified_for_next_test
```

## Sync from edge

Use this when `edge/stable` advanced and the rapid branch should be updated:

```sh
sh scripts/black-phase1.sh sync
```

This rebases `black-phase1` on `origin/edge/stable` and force-pushes only `black-phase1` with `--force-with-lease`.

## Promote known-good work

After a test works and the commit is clean:

```sh
git log --oneline -10
sh scripts/black-phase1.sh promote <commit-sha> "Promote working test change"
```

Then run validation before pushing edge:

```sh
cargo build --release
sh scripts/x200-b43-system-preflight.sh /dev/sdb
git push origin edge/stable
```

## Safety rules

Even on `black-phase1`:

- never commit secrets, keys, tokens, credentials, or private inventories;
- never claim Japanese glyph success until the physical display proves it;
- never claim SSH transfer support until key-only SSH starts and prints IP/port;
- never claim hardening without implementation, tests, recovery, and evidence;
- keep ASCII fallback available;
- keep root/internal-disk write protection active.

## Current experimental priorities

1. Minimal Japanese glyph path.
2. Manual boot configuration card entry.
3. Rounded glyph mode.
4. Stable blue/ice and edge crimson policy.
5. SSH transfer mode with key-only secure defaults.
6. Full userspace/Base1 runtime path if minimal CJK continues to fail.

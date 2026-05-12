# black-phase1 test branch

Status: rapid development branch policy  
Branch: `black-phase1`  
Base: `edge/stable` at B45 fast Mac/X200 automation checkpoint

## Purpose

`black-phase1` is the rapid test environment for Phase1 boot/runtime experiments.

It is intentionally **not** one of the public-quality tracks:

```text
not stable
not edge release
not safe/stable claim
not hardened claim
not daily-driver claim
not release branch
```

Use it for fast iteration before promoting proven changes to `edge/stable`.

## Relationship to edge/stable

```text
edge/stable       = main active development path with evidence-bound changes
black-phase1      = rapid experimental integration branch
checkpoint/*      = saved milestone snapshots
```

Successful `black-phase1` work should be moved to `edge/stable` only after:

1. local build passes;
2. preflight passes;
3. USB readback verification passes when boot-media work is touched;
4. hardware result is observed or explicitly marked as not yet claimed;
5. unsafe/debug behavior is removed or gated;
6. docs/evidence are updated.

## What belongs here

Good candidates:

- fast boot-menu experiments;
- CJK/framebuffer renderer experiments;
- SSH transfer/server packaging experiments;
- temporary diagnostic output;
- noisy proof-of-concept scripts;
- local automation experiments;
- alternate boot paths;
- risky UI changes before cleanup.

Do not treat this branch as safe for public claims.

## Promotion rule

Promote from `black-phase1` to `edge/stable` using cherry-pick or a clean PR-style merge after validation.

Preferred flow:

```sh
git checkout black-phase1
# work and test quickly

git checkout edge/stable
git pull --ff-only origin edge/stable
git cherry-pick <known-good-commit>
git push origin edge/stable
```

For multiple commits, squash/clean them before promotion if they contain noisy dead ends.

## Rebase rule

Keep `black-phase1` close to current `edge/stable`:

```sh
git checkout black-phase1
git fetch origin
git rebase origin/edge/stable
git push --force-with-lease origin black-phase1
```

Because this is an experimental branch, force-with-lease is acceptable for branch cleanup. Do not force-push `edge/stable`.

## Safety guardrails

Even on `black-phase1`:

- do not commit secrets, keys, tokens, private IP inventories, or credentials;
- do not claim Japanese glyph rendering until physically observed;
- do not claim SSH transfer support until key-only SSH starts and prints IP/port;
- do not claim hardening until supported by implementation, tests, and evidence;
- keep ASCII fallback available for boot tests;
- keep the X200 internal/root disk write protections.

## Current fast loop

Mac:

```sh
cd ~/phase1_library/phase1
git checkout black-phase1
sh scripts/black-phase1-mac-push.sh "Describe rapid test"
```

X200:

```sh
cd ~/phase1
git checkout black-phase1
sh scripts/black-phase1-x200-test.sh /dev/sdb YES_WRITE_USB
```

Promote to edge only after evidence says the result is ready.

# Phase1 Dev Dock

dev lets Phase1 work on itself from inside Phase1.

## Start

Run Phase1 with guarded host tools:

PHASE1_SAFE_MODE=1 PHASE1_ALLOW_HOST_TOOLS=1 cargo run

## Commands inside Phase1

dev status
dev sync
dev branch feature/example
dev quick
dev test
dev commit Add example feature
dev push
dev pr Add example feature
dev merge 123
dev close 123
dev doctor

## Safety

- Uses guarded host tools.
- Safe mode can stay enabled.
- Intended for Phase1 self-development.
- Avoids staging runtime files like phase1.history, phase1.state, phase1.log, and phase1.learn.
- Keeps normal copy/paste development workflows inside Phase1.

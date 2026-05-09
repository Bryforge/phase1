# Phase1 eMacs

Phase1 eMacs is the Phase1-branded advanced editor command.

It is VFS-native and powered by AVIM Pro, so it works inside Phase1 without shell escapes or host file reads.

## Commands

```text
emacs <file>
phase1-emacs <file>
phase1emacs <file>
pemacs <file>
emac <file>
```

## Smoke test inside Phase1

```text
emacs jesse.go
:help pro
:template go
:wq
lang run go jesse.go
```

## Safety model

- Edits Phase1 VFS files.
- Uses the AVIM Pro engine.
- No host shell escape.
- No direct host filesystem read.
- Safe for nested Phase1 workflows.

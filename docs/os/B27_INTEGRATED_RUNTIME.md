# B27 integrated Phase1 runtime plan

Status: planning scaffold

Scope: GNU/Linux-backed Phase1 runtime with workspace, supervisor lane planning, and crypto evidence hashing available from one boot.

## Purpose

B27 combines the current runtime work into one operator-visible Phase1 session.

B23 gives a GNU/Linux-backed `phase1>` shell.
B24 adds workspace and evidence paths.
B25 adds supervisor lane planning.
B26 adds evidence hashing.

B27 target:

`Libreboot -> SeaBIOS -> USB GRUB -> GNU/Linux -> integrated Phase1 runtime`

## Required runtime commands

The integrated runtime should expose:

- `help`
- `status`
- `workspace`
- `evidence`
- `files`
- `devices`
- `dmesg-tail`
- `supervisor`
- `crypto`
- `integrated-check`
- `export-help`
- `shell`
- `reboot`
- `poweroff`

## Success state

`phase1_integrated_runtime_seen`

This means one boot reached a GNU/Linux-backed Phase1 runtime that exposed workspace/evidence, supervisor planning, and crypto evidence hashing.

## Non-claims

B27 does not boot multiple operating systems automatically.

B27 does not claim production key management, secure boot, hardening, installer readiness, release-candidate readiness, or daily-driver readiness.

B27 does not write the internal disk.

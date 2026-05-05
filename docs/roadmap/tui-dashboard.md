# R6 TUI dashboard design

## Purpose

The TUI dashboard should make phase1 feel like a full operator control deck while preserving the normal command prompt for fast terminal use.

## Modes

### Fullscreen mode

Command:

```text
dash
```

Future implementation may use a terminal UI crate. Until dependencies are intentionally added, phase1 can start with a simple refreshable text dashboard.

### Compact mode

Command:

```text
dash --compact
```

This mode should fit phone terminals and screenshot demos.

## Dashboard panels

Initial panels:

```text
CORE      version, uptime, user, mode
PROC      process count, background jobs, top processes
VFS       cwd, mounts, file counts
NET       active interfaces, Wi-Fi status
AUDIT     recent audit events
HW        PCIe devices, CR3, CR4
```

## Visual language

- cyan/blue operator theme
- ASCII fallback through `PHASE1_ASCII=1`
- no-color fallback through `NO_COLOR=1` or `PHASE1_NO_COLOR=1`
- narrow layout for phones
- wide layout for desktops

## Example compact dashboard

```text
PHASE1 DASHBOARD v3.5.0
CORE  user=root uptime=42s mode=operator
PROC  tasks=3 bg=1
VFS   cwd=/home mounts=/,/proc,/dev,/tmp,/var/log
NET   interfaces=lo,en0 wifi=linked
HW    cr3=0x1000 pcide=off pcie=2
AUDIT latest=sys.write /home/demo.txt allow
```

## Implementation checklist

1. Add `dash` command with compact text dashboard.
2. Add `Kernel::dashboard_snapshot()` data model.
3. Render snapshot in `ui.rs`.
4. Add smoke test for `dash --compact`.
5. Later evaluate a terminal UI dependency for fullscreen interaction.

## Demo commands

```text
dash --compact
spawn worker --background
dash --compact
audit
```

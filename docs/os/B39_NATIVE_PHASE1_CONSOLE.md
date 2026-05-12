# B39 native Phase1 console plan

Status: planning scaffold

Scope: ThinkPad X200 / Libreboot GRUB / B38 working protocol / Phase1 Rust console payload.

## Purpose

B39 moves from the B38 BusyBox shell into Phase1 as designed: the Phase1 Rust console should be packaged into the initramfs and launched from the working B38 protocol.

B38 proved the reliable handoff:

`Libreboot GRUB -> normal linux -> normal initrd -> rdinit=/init -> Phase1-owned runtime`

B39 keeps that exact protocol and changes the runtime from a simple BusyBox prompt to a Phase1-native launch surface.

## Runtime layout

The initramfs should include:

- `/phase1/bin/phase1` when available;
- `/phase1/repo` minimal repository files where practical;
- `/phase1/workspace`;
- `/phase1/evidence`;
- `/phase1/state`;
- `/phase1/assets/phase1-splash.png`;
- a BusyBox fallback shell.

## Launch behavior

B39 `/init` should:

1. mount proc/sys/dev/tmpfs;
2. suppress kernel log spam;
3. print the Phase1 boot banner;
4. attempt to run `/phase1/bin/phase1`;
5. if the native binary is absent or fails, return to a Phase1 fallback shell.

## Success states

`phase1_native_binary_started`

The initramfs attempted and reached the native Phase1 binary entrypoint.

`phase1_native_console_seen`

The designed Phase1 Rust/operator console appeared from the booted Phase1 runtime.

Fallback state:

`phase1_full_system_load_seen`

The B38-style BusyBox runtime remains available even if native Phase1 launch fails.

## Notes

If the Phase1 binary is dynamically linked, B39 must either package required runtime libraries or fall back cleanly. A later step should produce a static or musl-linked Phase1 binary for cleaner initramfs booting.

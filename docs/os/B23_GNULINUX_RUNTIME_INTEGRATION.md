# B23 GNU/Linux runtime integration plan

Status: planning scaffold

Scope: ThinkPad X200 / Libreboot / SeaBIOS payload / external USB GRUB / GNU/Linux-backed Phase1 runtime.

## Purpose

B23 moves Phase1 beyond a GRUB-only control surface and into a GNU/Linux-backed runtime while preserving the proven physical launch route:

`Libreboot -> SeaBIOS payload -> USB GRUB -> GNU/Linux -> Phase1 runtime`

The SeaBIOS GRUB display is known to stay small and can tear. B23 therefore treats GRUB as a minimal loader only, not the main user interface.

## Display strategy

B23 must not depend on the SeaBIOS GRUB screen for normal operation.

B23 should:

- keep GRUB visible only briefly;
- default into GNU/Linux automatically;
- let the Linux console/framebuffer take over display handling;
- keep a B22 text-safe fallback entry;
- avoid splash/gfxterm as a requirement;
- preserve short fallback text for the SeaBIOS screen.

## Runtime target

The target is a Phase1 GNU/Linux runtime with:

- a visible console or shell;
- `/phase1` workspace path;
- `/phase1/evidence` evidence path;
- no internal disk writes;
- no installer claim;
- no daily-driver claim;
- clean fallback to B22 console if Linux boot fails.

## Required artifacts

B23 should use local build artifacts when present:

- `build/linux/alpine-netboot/vmlinuz`
- `build/linux/alpine-netboot/initrd.img`

If these are missing, B23 should fail early with a clear message instead of preparing a broken USB.

## Success result

`phase1_gnulinux_shell_seen`

This means the X200 reached a GNU/Linux-backed Phase1 runtime or shell through the proven SeaBIOS USB GRUB route.

Useful intermediate result:

`phase1_gnulinux_initramfs_seen`

This means GNU/Linux/initramfs visibly loaded, even if the full Phase1 runtime still needs work.

Useful negative result:

`blocked_after_gnulinux_load`

This means GRUB loaded Linux/initrd but the operator did not observe a Linux/Phase1 console.

## Non-claims

B23 does not make Base1 installer-ready, recovery-complete, hardened, release-candidate ready, or daily-driver ready.

B23 does not write the internal disk and does not claim a completed production operating system boot.

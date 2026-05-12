# B30 Libreboot GRUB tiny initramfs plan

Status: planning scaffold

Scope: ThinkPad X200 / Libreboot GRUB external media / Linux-libre baseline kernel / tiny Phase1 initramfs.

## Purpose

B30 applies the newest hardware lesson: Libreboot GRUB external media is currently a better visible control path than the SeaBIOS payload screen.

The preferred route becomes:

`Libreboot GRUB -> external USB GRUB config -> linux16 -> tiny Phase1 initramfs`

SeaBIOS remains a fallback, not the primary user experience.

## Lessons applied

- Libreboot GRUB gives a larger, clearer display than SeaBIOS GRUB on this X200.
- The Linux-libre baseline kernel is accepted by `linux16`.
- The large host initrd may block or take too long during `initrd16`.
- Therefore, B30 replaces the large host initrd with a tiny Phase1 initramfs built from static BusyBox.

## Success state

`phase1_tiny_initramfs_seen`

This means the X200 reached the tiny Phase1 initramfs shell through Libreboot GRUB and `linux16`.

## Next after success

After B30 succeeds, rebuild B27/B23 around the tiny initramfs baseline first, then add workspace, supervisor planning, and crypto evidence back in layers.

## Non-claims

B30 does not claim installer readiness, hardening, recovery-complete status, release-candidate status, or daily-driver readiness.

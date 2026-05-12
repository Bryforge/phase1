# B31 full Linux-libre splash plan

Status: planning scaffold

Scope: ThinkPad X200 / Libreboot GRUB / Linux-libre baseline kernel / full host initrd / Phase1 boot splash.

## Purpose

B31 prepares the full host initrd path for a meaningful test before falling back to the tiny initramfs route.

B29 proved that `linux16` accepts the Linux-libre baseline kernel. The next full test is to boot that kernel with the full host initrd and a Phase1 splash-oriented GRUB surface.

## Preferred route

`Libreboot GRUB -> external USB -> linux16 -> initrd16 -> full host initrd`

SeaBIOS remains a fallback because the Libreboot GRUB display is clearer on this X200.

## Splash strategy

B31 provides a GRUB-side Phase1 splash attempt:

- text-safe default entry first;
- optional generated Phase1 TGA splash;
- no dependency on splash for success;
- verbose entry for debugging;
- file-check entry before boot.

The splash is GRUB-side. A later kernel/runtime splash can be added after the Linux handoff is reliable.

## Success states

`phase1_linux_libre_full_seen`

The X200 reaches visible GNU/Linux output or runtime using the full host initrd.

`phase1_boot_splash_seen`

The GRUB-side Phase1 splash appears.

Useful negative state:

`blocked_after_huge_initrd_load`

The full host initrd load or boot handoff blocks.

## Next after success

If B31 succeeds, fold the B27 integrated runtime back onto the full Linux-libre baseline. If it blocks, continue with B30 tiny initramfs.

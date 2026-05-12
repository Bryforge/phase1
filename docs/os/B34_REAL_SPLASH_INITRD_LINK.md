# B34 real splash initrd link plan

Status: planning scaffold

Scope: ThinkPad X200 / Libreboot GRUB / real Phase1 README splash asset / full Linux-libre initrd Phase1 hook.

## Purpose

B34 corrects the placeholder splash used by earlier USB writers. The repository already has the real Phase1 boot splash asset:

`assets/phase1-splash.png`

B34 uses that image as the first splash target instead of generating a placeholder ring/cross image.

## Route

`Libreboot GRUB -> external USB -> real Phase1 splash -> linux16 -> linked full initrd`

## Integration

B34 carries forward the B33 Phase1 initramfs hook:

`/scripts/init-top/phase1`

This hook is appended to the full host initrd as an overlay so that Phase1 can print a marker early if the initramfs-tools path runs.

## Success states

`phase1_real_splash_seen`

The README-linked Phase1 splash image appears from GRUB.

`phase1_initrd_phase1_hook_seen`

The full host initrd reached the Phase1 initramfs hook.

## Notes

The splash is a GRUB-side PNG asset. If the X200 Libreboot GRUB lacks PNG support, use the text entries and continue the initrd diagnostic path.

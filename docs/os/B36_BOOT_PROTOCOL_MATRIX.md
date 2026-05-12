# B36 boot protocol matrix plan

Status: planning scaffold

Scope: ThinkPad X200 / Libreboot GRUB / real Phase1 splash / Linux-libre kernel / Phase1-owned initramfs / boot protocol matrix.

## Purpose

B35 still reset back to Libreboot after the kernel handoff. This means the reset is no longer explained by a large host initrd. It is now likely related to the exact Linux boot protocol, console/video handoff, or kernel command line.

B36 prepares a matrix of boot entries so one USB can test the major combinations without rebuilding repeatedly.

## Known facts

- Libreboot GRUB external USB is the best control path.
- The real Phase1 splash image works.
- `linux16` accepts the Linux-libre baseline kernel.
- `initrd16` accepts both full and linked initrds.
- The actual boot handoff resets back to Libreboot.
- The B35 Phase1-owned initramfs also reset, so the remaining problem is likely before `/init` runs.

## B36 strategy

Use the same kernel and Phase1-owned initramfs, but vary:

- `linux16` vs normal `linux`;
- `initrd16` vs normal `initrd`;
- with and without `root=/dev/ram0`;
- `rdinit=/init init=/init` explicitly;
- text VGA handoff settings;
- ACPI/APIC fallbacks;
- splash-to-text transition before boot.

## Success state

`phase1_full_system_load_seen`

The Phase1-owned initramfs reaches the `phase1>` prompt.

## Negative state

`reset_after_tiny_initramfs_boot`

The kernel/initramfs handoff still resets back to Libreboot.

## Next after success

Once one protocol works, that protocol becomes the baseline for B37 and all later Phase1 GNU/Linux integration.

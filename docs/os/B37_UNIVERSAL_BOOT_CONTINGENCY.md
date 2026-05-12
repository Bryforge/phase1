# B37 universal boot contingency plan

Status: planning scaffold

Scope: X200-first but broader BIOS/Libreboot/GRUB boot contingency matrix.

## Purpose

B37 prepares a broad contingency USB so the next boot attempt has multiple prepared routes instead of one fragile assumption.

The goal is not to guarantee every system boots. The goal is to cover the practical boot paths Phase1 has encountered so far:

- Libreboot GRUB external media;
- SeaBIOS payload fallback;
- `linux16` / `initrd16`;
- normal `linux` / `initrd`;
- text VGA fallback;
- no-APIC/no-ACPI fallback;
- serial console fallback;
- Phase1 GRUB-only fallback;
- real Phase1 splash first where supported.

## Known facts

- Real Phase1 splash works in GRUB using `assets/phase1-splash.png`.
- `linux16` accepts the Linux-libre kernel.
- `initrd16` accepts large and small initrds.
- Actual kernel handoff still resets on the X200 for the tested protocol set.

## Strategy

B37 keeps the same kernel and Phase1-owned initramfs but expands the menu into a universal boot matrix:

1. Real Phase1 splash.
2. File/integrity checks.
3. Command-only kernel/initrd checks.
4. Libreboot-oriented `linux16` text modes.
5. BIOS/GRUB normal `linux` modes.
6. ACPI/APIC fallback modes.
7. VGA/text console modes.
8. Serial console mode.
9. No-initrd kernel-only diagnostic.
10. GRUB-only Phase1 control fallback.

## Success state

`phase1_full_system_load_seen`

The Phase1-owned initramfs reaches the `phase1>` prompt.

## Broad matrix-visible state

`phase1_universal_boot_matrix_seen`

The B37 universal boot matrix appears and is usable.

## Negative state

`reset_after_tiny_initramfs_boot`

All tested Phase1-owned initramfs boot routes reset back to firmware/Libreboot.

## Next if all routes reset

If all B37 routes reset, the next pivot is kernel artifact selection:

- use the exact installed OS kernel boot stanza and root parameters;
- test an older/newer Linux-libre kernel;
- test a distro installer kernel known to support X200/Libreboot;
- later revisit Alpine hardened only after baseline boot is proven.

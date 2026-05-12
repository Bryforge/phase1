# B38 working Phase1 system plan

Status: planning scaffold

Scope: ThinkPad X200 / Libreboot GRUB / real Phase1 splash / working normal Linux GRUB protocol / Phase1-owned initramfs runtime.

## Purpose

B38 freezes the first working Phase1 GNU/Linux handoff protocol found by the B37 universal boot matrix.

B37 results:

- A (`linux16`, rdinit, no root) reset.
- C (`linux16`, VGA text fallback) reset.
- E (`linux16`, noapic/acpi=off) reset.
- G (`normal linux`, `rdinit=/init`, normal `initrd`) booted.
- K (kernel only) reset.

Therefore B38 makes B37 G the default working boot protocol.

## Working route

`Libreboot GRUB -> real Phase1 splash -> normal linux -> normal initrd -> Phase1-owned /init`

## Working GRUB protocol

```text
linux /boot/phase1/vmlinuz console=tty0 rdinit=/init init=/init nomodeset loglevel=7 panic=0
initrd /boot/phase1/phase1-b38-system-initramfs.img
boot
```

## Success state

`phase1_full_system_load_seen`

This means the Phase1-owned initramfs reached the `phase1>` prompt.

## Why this matters

This is the first route that moved past firmware/GRUB diagnostics and reached a Phase1-owned runtime surface on the X200.

B38 should now become the stable hardware baseline for the next development layer:

- better Phase1 shell commands;
- workspace persistence;
- evidence export;
- supervisor lane scaffolding;
- crypto evidence;
- later Alpine hardened integration.

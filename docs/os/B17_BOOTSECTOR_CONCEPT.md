# B17 bootsector concept plan

Status: planning and prepared USB scaffold

Scope: external USB bootsector proof path for ThinkPad X200 / Libreboot / GRUB, bypassing the Multiboot ELF path that has repeatedly loaded files but not shown visible kernel execution.

## Purpose

B17 exists because the current evidence suggests GRUB-native scripts work, but Linux/initrd and Multiboot ELF handoff paths can load files and then block or reset before visible Phase1-owned execution.

The goal is to test a smaller handoff surface: GRUB chainloads a dedicated bootsector partition, and the bootsector writes directly to VGA text memory.

## Why this path

Observed evidence so far:

- GRUB-native Phase1 console works on the X200 external USB path.
- Linux/initrd paths load files but do not consistently reach visible execution.
- Multiboot ELF paths load files but can block after the GRUB load screen.
- Keyboard work must wait until the visible execution path is reliable.

B17 therefore moves away from Multiboot ELF and toward a tiny bootsector proof that does not need framebuffer negotiation, Linux setup semantics, or Multiboot info structures.

## Prepared test design

The B17 USB layout uses:

- partition 1: FAT32 GRUB control partition;
- partition 2: tiny raw bootsector proof partition;
- GRUB chainloader entries targeting partition 2;
- GRUB-native B11 fallback;
- B6 marker fallback.

The bootsector must:

- fit in one 512-byte sector;
- end with the `0x55AA` boot signature;
- write a clear Phase1 marker to VGA text memory;
- halt intentionally;
- avoid internal disk writes;
- avoid installer, recovery-complete, hardening, and daily-driver claims.

## Primary result state

Success is:

`phase1_bootsector_seen`

This means the X200 chainloaded the Phase1 bootsector and the bootsector wrote its own visible text to the screen.

Useful negative states:

- `blocked_after_chainload` if GRUB attempts chainload but no visible Phase1 bootsector screen appears.
- `phase1_grub_console_seen` if only the B11 fallback is reached.
- `failed` if the USB does not boot.

## Non-claims

B17 bootsector evidence does not make Base1 installer-ready, recovery-complete, hardened, hypervisor-ready, release-candidate ready, or daily-driver ready.

B17 does not install to internal disk and does not claim a daily-driver operating system.

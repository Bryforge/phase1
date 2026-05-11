# B20 SeaBIOS GRUB Phase1 handoff plan

Status: planning scaffold

Scope: ThinkPad X200 / Libreboot / SeaBIOS payload / external USB GRUB / Phase1-owned handoff.

## Purpose

B19 proved a better physical boot-control path than raw Libreboot GRUB chainloading:

`Libreboot -> SeaBIOS payload -> USB GRUB`

B20 uses that now-proven route to resume Phase1-owned code execution work without depending on Libreboot GRUB's chainloader behavior.

## Current evidence interpretation

Confirmed:

- Libreboot GRUB-native external USB console works.
- SeaBIOS payload can boot a conventional USB GRUB menu.

Blocked or uncertain:

- Libreboot GRUB Multiboot ELF handoff has repeatedly loaded files but not shown reliable visible execution.
- Libreboot GRUB chainloading a raw sector reports an unrecognized payload type.
- SeaBIOS raw MBR bootsector showed a black screen.

Therefore, the next stable route is conventional USB GRUB under SeaBIOS.

## B20 target

B20 should prepare a SeaBIOS-booted USB GRUB menu with multiple Phase1-owned handoff attempts:

1. GRUB-native Phase1 console baseline.
2. Multiboot framebuffer diagnostic from SeaBIOS GRUB.
3. Multiboot VGA text diagnostic from SeaBIOS GRUB.
4. Linux/initrd control where available.
5. Device listing and fallback marker.

The goal is not to record every negative step. The goal is to determine whether SeaBIOS GRUB can hand off Phase1-owned code more reliably than Libreboot GRUB.

## Primary result states

- `phase1_seabios_grub_seen` confirms the SeaBIOS USB GRUB route.
- `phase1_seabios_multiboot_seen` confirms SeaBIOS GRUB handed off to Phase1-owned Multiboot code.
- `blocked_after_seabios_multiboot_load` means SeaBIOS GRUB loaded the Phase1 payload but no visible Phase1-owned execution appeared.

## Non-claims

B20 does not make Base1 installer-ready, recovery-complete, hardened, release-candidate ready, or daily-driver ready.

B20 does not write the internal disk and does not claim a completed operating system boot.

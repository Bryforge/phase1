# B35 full system load plan

Status: planning scaffold

Scope: ThinkPad X200 / Libreboot GRUB / real Phase1 splash / Linux-libre kernel / Phase1-owned tiny initramfs / full-system load attempt.

## Purpose

B35 is the next full-system load attempt after B31-B34.

What is now known:

- Libreboot GRUB is the clearest boot path on the X200.
- The real Phase1 splash asset should be used, not the generated placeholder image.
- `linux16` accepts the Linux-libre baseline kernel.
- `initrd16` accepts the full host initrd and the linked initrd.
- Full host initrd execution still resets back to Libreboot.

B35 therefore uses a Phase1-owned tiny initramfs for the main system load, while keeping the full host initrd as a diagnostic fallback.

## Preferred route

`Libreboot GRUB -> real Phase1 splash -> linux16 -> Phase1-owned tiny initramfs -> phase1>`

## Splash requirement

B35 must load the real Phase1 splash image from:

`assets/phase1-splash.png`

The generated ring/cross placeholder must not be used.

## Full-system surface

The B35 tiny initramfs should expose:

- `phase1>` prompt;
- `/phase1/workspace`;
- `/phase1/evidence`;
- `/phase1/state`;
- `status`;
- `workspace`;
- `evidence`;
- `supervisor`;
- `crypto`;
- `integrated-check`;
- `shell`;
- `reboot`;
- `poweroff`.

## Success states

`phase1_real_splash_seen`

The real Phase1 boot splash appears.

`phase1_full_system_load_seen`

The Phase1-owned tiny initramfs reaches the full system load prompt.

`phase1_integrated_runtime_seen`

The integrated runtime command surface is visible.

## Diagnostic fallback states

`phase1_initrd_phase1_hook_seen`

The linked full host initrd reached the Phase1 init-top hook.

`reset_after_full_initrd_boot`

The linked full host initrd resets back to Libreboot.

## Non-claims

B35 does not claim installer readiness, recovery-complete status, hardening, release-candidate status, or daily-driver readiness.

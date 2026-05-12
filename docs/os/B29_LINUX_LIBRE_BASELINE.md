# B29 Linux-libre baseline plan

Status: planning scaffold

Scope: ThinkPad X200 / Libreboot / SeaBIOS payload / USB GRUB / host Linux-libre kernel baseline.

## Purpose

B29 applies the X200-specific lesson that this Libreboot machine has historically worked best with a Libreboot-friendly Linux-libre style kernel path.

Rather than forcing Alpine hardened first, B29 uses the host machine's known local GNU/Linux kernel and initrd as the baseline handoff target.

## Reasoning

The current problem is not hardening yet. The current problem is proving a reliable GRUB-to-Linux handoff through the already confirmed route:

`Libreboot -> SeaBIOS payload -> USB GRUB -> Linux`

Once this works, Alpine hardened and stronger hardening profiles can be layered later.

## B29 strategy

B29 should:

- stage the X200 host kernel from `/boot/vmlinuz-*`;
- stage the matching host initrd from `/boot/initrd.img-*`;
- label the path as Linux-libre baseline rather than Alpine hardened;
- provide both `linux16` and normal `linux` GRUB entries where possible;
- keep a diagnostic file-check entry;
- keep a GRUB fallback entry;
- keep all internal disk access non-automatic.

## Success result

`phase1_linux_libre_baseline_seen`

This means the X200 reached a Phase1 runtime or visible Linux-libre baseline console using the host GNU/Linux kernel/initrd route.

Useful intermediate result:

`linux16_command_returned`

This means GRUB accepted and returned from the `linux16` load command before booting.

Useful negative result:

`blocked_during_linux_libre_load`

This means the handoff still blocked before visible GNU/Linux output.

## Next after success

After B29 succeeds, resume:

- B23/B27 integrated runtime overlay;
- B24 workspace/evidence;
- B25 supervisor planning;
- B26 crypto evidence;
- later Alpine hardened or hardened Linux-libre profile.

## Non-claims

B29 does not make Base1 installer-ready, recovery-complete, hardened, release-candidate ready, or daily-driver ready.

B29 does not write the internal disk and does not claim a completed production operating system boot.

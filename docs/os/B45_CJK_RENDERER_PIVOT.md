# B45 CJK renderer pivot

Status: active pivot after B44 test failure  
Scope: Phase1 native boot on X200/Libreboot

## Why B44 was not enough

B44 packaged UTF-8 environment, CJK fonts, console fonts, and an `fbterm` candidate. The physical X200 test still did not render Japanese. That means the failure is no longer file availability. It is the active renderer path.

Current conclusion:

```text
UTF-8 environment: present or packageable
CJK fonts: packageable
fbterm candidate: packageable
Japanese glyph rendering on screen: still not working
```

Therefore, continuing to copy more fonts into the initramfs is unlikely to solve the problem by itself.

## Root cause

The current boot path lands in a Linux console/VT and then launches Phase1. Linux VT rendering does not provide reliable Japanese/CJK glyph drawing. A CJK-capable font file on disk is insufficient unless the active renderer can load and draw it.

The renderer options are:

1. **Full GNU/Linux userspace terminal path** — most practical next step.
2. **Framebuffer terminal path** with a known working renderer and explicit launch/debug output.
3. **Phase1-owned framebuffer text renderer** — best long-term path, but larger implementation.

## Decision

Do not spend more reboot cycles attempting the same B44 font-packaging path.

Next target is B45:

```text
Base1 boots Linux -> mounts/starts a fuller runtime -> launches Phase1 inside a real UTF-8 terminal environment with packaged fonts
```

This is more likely to support Japanese, rounded glyphs, SSH transfer, and later GUI/desktop services than the tiny initramfs-only console path.

## B45 goals

- Keep the proven GRUB/linux/initrd path as fallback.
- Add a full userspace/rootfs path for Phase1.
- Include Noto CJK/Unifont/fontconfig.
- Include a real terminal path capable of CJK rendering.
- Include SSH transfer support with key-only secure defaults.
- Preserve safe/stable blue default and edge red policy.
- Keep ASCII fallback available.

## B45 boot menu target

```text
Start Phase1 Stable Safe Full Runtime
Start Phase1 Stable Safe Full Runtime + SSH Transfer
Start Phase1 Minimal Native Console
Start Phase1 ASCII Safe Fallback
B45 File check
```

## Evidence states

```text
phase1_full_runtime_seen
phase1_utf8_terminal_ready
phase1_japanese_glyphs_seen      only after physical visual confirmation
phase1_ssh_transfer_ready        only after key-only SSH starts and IP/port are printed
```

## Engineering rule

Before any more hardware reboot tests, B45 must have a preflight that reports either:

```text
prepared_and_verified
```

or:

```text
blocked_with_exact_reason
```

No more font-only reboot tests.

# B47 framebuffer renderer QEMU lab

Status: black-phase1 experimental track  
Purpose: test a Phase1-owned renderer before touching the X200 USB boot cycle again.

## Decision

The current terminal path can boot Phase1 and render color, but it cannot reliably render Japanese/CJK glyphs or rounded UI corners on the X200 Linux text console.

B47 starts a Phase1-owned framebuffer renderer path in QEMU first.

## Goal

Build and boot a tiny framebuffer lab that renders a Phase1 boot card directly to `/dev/fb0`.

First target:

```text
ice/blue stable-safe frame
bright red Japanese greeting
rounded card shape rendered as pixels, not terminal glyphs
short status card
fallback shell after rendering
```

Japanese test text:

```text
こんにちは、ハッカー！
```

This is intentionally rendered into pixels before boot, then blitted into the framebuffer during init. That lets us validate the display pipeline before building a full dynamic font renderer.

## Why QEMU first

QEMU gives a fast visual test cycle without repeatedly rebooting the X200. It also lets us separate renderer bugs from Libreboot/USB/media issues.

## B47 lab architecture

```text
host Linux builder
  -> find CJK font
  -> render Phase1 card image with Python/Pillow
  -> compile tiny framebuffer blitter
  -> build initramfs with BusyBox + blitter + card image
  -> boot with qemu-system-x86_64
  -> display card on /dev/fb0
```

## Requirements

Preferred host: x86_64 Linux builder.

Required commands:

```text
qemu-system-x86_64
python3
python3 PIL/Pillow
cc
cpio
gzip
ldd
file
```

Required input:

```text
build/linux/alpine-netboot/vmlinuz
```

Required font candidate:

```text
Noto Sans CJK JP
Noto Sans Mono CJK JP
Unifont
```

## What this does not claim yet

This is not the final Phase1 renderer. It is the first framebuffer proof.

Do not claim:

```text
phase1_japanese_glyphs_seen
```

until QEMU or hardware visually confirms that the Japanese text renders.

## Success state

```text
phase1_framebuffer_card_qemu_seen
phase1_japanese_pixels_qemu_seen
```

Hardware success later will be separate:

```text
phase1_framebuffer_card_x200_seen
phase1_japanese_pixels_x200_seen
```

## Next after QEMU success

1. Add the framebuffer renderer as a real Phase1 boot path.
2. Add a GRUB menu entry: `Start Phase1 Framebuffer Boot Card`.
3. Keep terminal and ASCII fallback paths.
4. Expand from pre-rendered card to runtime glyph atlas/font rendering.
5. Add SSH transfer support after visual renderer stability is proven.

# B47 X200 framebuffer boot entry

Status: next black-phase1 hardware prep  
Scope: X200 Libreboot GRUB after QEMU framebuffer proof

## Context

B47 QEMU proved that the Phase1 Japanese greeting and rounded boot card can render correctly when drawn as framebuffer pixels.

Observed QEMU evidence:

```text
phase1_framebuffer_card_qemu_seen
phase1_japanese_pixels_qemu_seen
```

The next step is not another terminal font test. The next step is to package the same framebuffer card/blitter path into the X200 boot USB as a separate test entry.

## New boot entry

```text
Start Phase1 Framebuffer Boot Card
```

This entry is experimental and should not replace the normal boot yet.

It should coexist with:

```text
Start Phase1 Stable Safe Color UTF-8
Configure Phase1 Boot Card
Start Phase1 ASCII Safe Fallback
```

## Target boot behavior

The framebuffer entry should:

1. boot the same known-good Linux/initrd path;
2. pass `phase1.framebuffer=1`;
3. render the prebuilt Phase1 card to `/dev/fb0`;
4. show the Japanese greeting as pixels;
5. keep a fallback shell available;
6. record evidence as not claimed until observed on the X200.

## Evidence policy

Before hardware test:

```text
BASE1_B47_FRAMEBUFFER_CARD_QEMU=seen
BASE1_B47_JAPANESE_PIXELS_QEMU=seen
BASE1_B47_FRAMEBUFFER_CARD_X200=not_claimed
BASE1_B47_JAPANESE_PIXELS_X200=not_claimed
```

After physical X200 success, update to:

```text
BASE1_B47_FRAMEBUFFER_CARD_X200=seen
BASE1_B47_JAPANESE_PIXELS_X200=seen
```

## Command

After the base USB has been prepared and verified:

```sh
sh scripts/x200-b47-framebuffer-boot-augment.sh /dev/sdb YES_WRITE_USB
```

Then boot:

```text
Start Phase1 Framebuffer Boot Card
```

## Non-claims

This is not a full dynamic renderer yet. It is a framebuffer proof path using a pre-rendered card and a small blitter.

The long-term Phase1 renderer should evolve from:

```text
pre-rendered card -> runtime glyph atlas -> dynamic framebuffer UI
```

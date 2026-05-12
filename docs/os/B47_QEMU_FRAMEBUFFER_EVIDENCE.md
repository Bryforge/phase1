# B47 QEMU framebuffer renderer evidence

Status: observed QEMU framebuffer lab evidence  
Branch: `black-phase1`  
Evidence target: QEMU first, before X200 USB framebuffer integration

## Result

The B47 QEMU framebuffer renderer lab reached the visual framebuffer card.

Observed candidate states:

```text
phase1_framebuffer_card_qemu_seen
phase1_japanese_pixels_qemu_seen
```

The rendered card showed:

```text
Phase1 // Framebuffer Boot Card
ice/blue rounded pixel frame
bright red Japanese greeting rendered into the framebuffer card
status rows rendered as pixels
BusyBox fallback shell remained available
```

## Technical meaning

This is a major validation point because it proves the Japanese and rounded-card problem is not unsolvable. The issue was the Linux text console glyph renderer, not Phase1 text or UTF-8 alone.

The successful path is:

```text
host renders Phase1 card with CJK-capable font
  -> card is stored as pixels
  -> initramfs blitter writes image to /dev/fb0
  -> QEMU displays the Japanese text/rounded UI as framebuffer pixels
```

## Evidence boundary

This does not yet claim X200 hardware framebuffer success.

Still not claimed:

```text
phase1_framebuffer_card_x200_seen
phase1_japanese_pixels_x200_seen
```

Those require a separate X200 boot entry and physical hardware confirmation.

## Next engineering step

Move from QEMU-only lab to an X200-safe framebuffer boot entry:

```text
Start Phase1 Framebuffer Boot Card
```

Requirements before X200 test:

- keep the existing terminal Phase1 path;
- keep ASCII fallback;
- keep manual Configure Phase1 Boot Card entry;
- package the framebuffer blitter and rendered card into the X200 boot initramfs;
- add GRUB framebuffer entry;
- verify USB readback before reboot.

## Evidence state

```text
BASE1_B47_FRAMEBUFFER_CARD_QEMU=seen
BASE1_B47_JAPANESE_PIXELS_QEMU=seen
BASE1_B47_FRAMEBUFFER_CARD_X200=not_claimed
BASE1_B47_JAPANESE_PIXELS_X200=not_claimed
```

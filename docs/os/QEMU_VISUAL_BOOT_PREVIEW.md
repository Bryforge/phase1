# Base1 QEMU visual boot preview

Status: showcase helper documentation
Scope: local QEMU visual boot splash preview using `assets/phase1_word.png`

## Purpose

This document explains the local visual boot preview helper for Phase1/Base1.

The helper builds a local UEFI FAT image that displays the Phase1 word-mark splash in QEMU. It is meant for demos, screenshots, and presentation recordings while the real boot-readiness track remains evidence-bound.

The QEMU preview does **not** display the full-resolution asset directly. The build script creates a fitted GRUB splash from `assets/phase1_word.png`, constraining it to a small centered size on a `1024x768` black canvas so it fits the QEMU boot viewport cleanly.

## Command

Build the preview image:

```bash
sh scripts/base1-qemu-visual-boot-preview.sh --build
```

Build and run it:

```bash
sh scripts/base1-qemu-visual-boot-preview.sh --build --run
```

Run an existing preview image fullscreen:

```bash
sh scripts/base1-qemu-visual-boot-preview.sh --run --fullscreen
```

## Output image

The helper creates this local image:

```text
build/base1-qemu-visual-boot-preview.img
```

The image is a local showcase artifact. It is not a release image and should not be committed.

## Input asset

The preview uses the Phase1 word-mark asset:

```text
assets/phase1_word.png
```

The script converts that source into a generated QEMU-only splash:

```text
build/base1-qemu-visual-boot-preview/boot/grub/phase1-qemu-splash.png
```

Sizing policy:

```text
canvas  : 1024x768
max edge: 560px
padding : black
position: centered
```

This keeps the boot splash intentionally smallish and prevents QEMU/GRUB from stretching, cropping, or overflowing the word mark.

## Expected image contents

The generated image should contain:

```text
/EFI/BOOT/BOOTX64.EFI
/boot/grub/phase1-qemu-splash.png
/boot/grub/fonts/phase1.pf2
```

Inspect it with:

```bash
mdir -i build/base1-qemu-visual-boot-preview.img ::/EFI/BOOT
mdir -i build/base1-qemu-visual-boot-preview.img ::/boot/grub
mdir -i build/base1-qemu-visual-boot-preview.img ::/boot/grub/fonts
```

## Required local tools

On macOS with Homebrew, install:

```bash
brew install qemu xorriso mtools x86_64-elf-grub
```

The helper expects these tools to be available when building or running:

- `sips`, included with macOS, for splash sizing and padding;
- `x86_64-elf-grub-mkstandalone`;
- `mformat`;
- `mmd`;
- `mcopy`;
- `qemu-system-x86_64`;
- QEMU UEFI firmware from Homebrew.

## QEMU launch shape

The helper runs QEMU with a local UEFI firmware image, a raw FAT preview image, USB storage boot, standard VGA, no network, and Cocoa display zoom-to-fit on macOS.

This keeps the demo isolated from host boot settings.

## Troubleshooting

### No boot device

If QEMU says no boot device is present, rebuild the preview image:

```bash
sh scripts/base1-qemu-visual-boot-preview.sh --build
```

Then confirm the image contains `EFI/BOOT/BOOTX64.EFI`:

```bash
mdir -i build/base1-qemu-visual-boot-preview.img ::/EFI/BOOT
```

### Black screen

A black screen usually means GRUB loaded but the splash did not display. Confirm the generated QEMU splash exists in the image:

```bash
mdir -i build/base1-qemu-visual-boot-preview.img ::/boot/grub
```

Then rebuild:

```bash
sh scripts/base1-qemu-visual-boot-preview.sh --build --run
```

### Splash too large or cropped

The QEMU splash should be generated from `assets/phase1_word.png` into a padded `1024x768` PNG with a `560px` maximum edge. Rebuild the generated image after changing the source asset:

```bash
sh scripts/base1-qemu-visual-boot-preview.sh --build
```

### Garbled text

Garbled GRUB text usually means font output is not clean. The helper generates `phase1.pf2` when a local macOS TrueType font and `grub-mkfont` are available. The showcase config hides most text so the splash remains the main visual.

### Backslash mistakes in manual commands

When typing QEMU commands manually, a line-continuation backslash must be the final character on the line:

```bash
qemu-system-x86_64 \
  -m 4096
```

Do not type a space after the backslash.

## Safety boundary

The helper must not:

- install Base1;
- write host boot settings;
- partition disks;
- format host disks;
- write EFI variables;
- call network tools;
- claim boot readiness;
- claim VM validation;
- claim hardware validation;
- claim hardening.

The only intended writes are local files under `build/`.

## Related docs

- [`BOOT_READINESS_STATUS.md`](BOOT_READINESS_STATUS.md)
- [`B2_DRY_RUN_ASSEMBLY_PLAN.md`](B2_DRY_RUN_ASSEMBLY_PLAN.md)
- [`B2_DRY_RUN_ASSEMBLY_TEST_SUITE.md`](B2_DRY_RUN_ASSEMBLY_TEST_SUITE.md)
- [`B3_VM_BOOT_VALIDATION_PLAN.md`](B3_VM_BOOT_VALIDATION_PLAN.md)

## Non-claims

This QEMU visual boot preview does not make Base1 bootable, installer-ready, recovery-complete, hardened, VM-validated, hardware-validated, release-candidate ready, or daily-driver ready.

It is a local visual showcase helper only.

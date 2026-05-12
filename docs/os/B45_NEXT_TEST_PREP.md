# B45 next test preparation plan

Status: next hardware-test prep  
Scope: X200 Libreboot GRUB / Phase1 native boot / minimal Unicode test / stable-safe defaults

## Goal

Prepare everything before the next physical reboot so the test is small, clear, and evidence-driven.

The next test should not repeat broad B44 font packaging. The next test should use the minimal B45 path:

```text
one font candidate
one renderer candidate
one Japanese test string
one rounded-corner test
verified USB readback
```

## Required result before reboot

The prep script must end with:

```text
RESULT: prepared_and_verified_for_next_test
```

If it does not, do not reboot.

## Boot menu expected after prep

The USB should include at least these entries:

```text
Start Phase1 Stable Safe Color UTF-8
Start Phase1 Minimal Japanese Glyph Test
Start Phase1 ASCII Safe Fallback
```

Optional additional entries may exist:

```text
Start Phase1 Unicode Font Lab
Start Phase1 Japanese Framebuffer UTF-8
```

## Main test order

1. Boot `Start Phase1 Stable Safe Color UTF-8`.
   - Confirms normal Phase1 still boots.
   - Confirms safe/stable blue/ice remains default.
   - Confirms auto-boot still works.

2. Boot `Start Phase1 Minimal Japanese Glyph Test`.
   - Confirms whether the minimal font/renderer path displays Japanese.
   - Expected test string:

```text
こんにちは、ハッカー
```

3. Only if needed, boot `Start Phase1 ASCII Safe Fallback`.
   - Confirms fallback remains available.

## Evidence policy

Do not claim Japanese glyph success until the physical display shows the Japanese string correctly.

Use:

```text
BASE1_B45_JAPANESE_GLYPH_RENDERING=not_claimed
```

until observed.

## Preparation command

On the X200/final x86 builder:

```sh
cd ~/phase1
git pull --ff-only origin edge/stable
sh scripts/x200-b45-prepare-next-test.sh /dev/sdb YES_WRITE_USB
```

Do not run the wrapper with `sudo`; it invokes sudo internally where required.

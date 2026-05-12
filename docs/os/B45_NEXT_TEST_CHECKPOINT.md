# B45 next test checkpoint

Status: checkpoint summary  
Branch: `edge/stable`  
Checkpoint purpose: prepare a complete, verified next hardware test without more blind reboot loops.

## Checkpoint name

```text
checkpoint/b45-next-test-prep
```

## Current milestone

Phase1 has moved beyond proof-of-boot on the X200. The current milestone is product-quality boot preparation:

```text
Libreboot GRUB
  -> normal linux
  -> normal initrd
  -> rdinit=/init
  -> Phase1-owned runtime
  -> native Phase1 binary
  -> stable/safe color boot
  -> minimal Unicode/Japanese glyph test path
```

## Confirmed before this checkpoint

- Libreboot GRUB is the preferred payload path over SeaBIOS for this target.
- Normal GRUB `linux` + `initrd` + `rdinit=/init` is the reliable boot protocol.
- Phase1-owned BusyBox runtime booted successfully.
- Native Phase1 binary booted successfully.
- Color native console booted successfully.
- Auto-boot into the main runtime is now available through the B43 path.
- Stable/safe default is being moved toward blue/ice.
- ASCII is fallback only.
- Broad B44 font packaging was tested and did not solve Japanese glyph rendering by itself.

## Current open problems

### Japanese/CJK rendering

The failure is not just missing UTF-8. It is active glyph rendering.

```text
UTF-8 env: present/packaged
font files: packageable
Japanese glyphs on physical screen: not yet confirmed
```

B45 keeps the next test minimal:

```text
one best font candidate
one renderer candidate
one short Japanese string
one rounded-corner test
explicit evidence
```

### Boot configuration access

The main default path should auto-boot. A separate manual config/card entry still needs to be preserved so the operator can choose edge/stable, safe mode, ASCII fallback, and other boot settings without booting from a desktop terminal first.

### Rounded corners

Rounded corners require both UI support and glyph support. Current target is a glyph-mode policy:

```text
PHASE1_GLYPH_MODE=auto|rounded|box|ascii
```

### SSH transfer

SSH/server transfer support remains a near-term requirement, but it should be explicit and secure by default:

```text
key-only
no password login
no root password login
explicit SSH boot entry only
port 2222 by default
fallback cleanly if network fails
```

## Files added or updated in this phase

Documentation:

```text
docs/os/PHASE1_ASSISTANT_WORKING_REFERENCE.md
docs/os/B43_SYSTEMS_ANALYSIS_AND_NEXT_STEPS.md
docs/os/B44_UNICODE_FONT_SYSTEM.md
docs/os/B45_CJK_RENDERER_PIVOT.md
docs/os/B45_MINIMAL_UNICODE_TEST.md
docs/os/B45_NEXT_TEST_PREP.md
docs/os/B45_NEXT_TEST_CHECKPOINT.md
```

Scripts:

```text
scripts/x200-b43-system-preflight.sh
scripts/x200-b43-next-pass-checklist.sh
scripts/x200-b43-apply-ui-policy.sh
scripts/x200-b43-prepare-polished-boot.sh
scripts/x200-b44-cjk-config-preflight.sh
scripts/x200-b44-unicode-font-augment.sh
scripts/x200-b44-unicode-font-augment-verbose.sh
scripts/x200-b45-minimal-unicode-augment.sh
scripts/x200-b45-prepare-next-test.sh
```

## Current recommended command

Run on the X200 or another x86_64 Linux builder, not with `sudo` around the whole wrapper:

```sh
cd ~/phase1
git pull --ff-only origin edge/stable
sh scripts/x200-b45-prepare-next-test.sh /dev/sdb YES_WRITE_USB
```

Only reboot when it ends with:

```text
RESULT: prepared_and_verified_for_next_test
```

## Next boot test order

```text
1. Start Phase1 Stable Safe Color UTF-8
2. Start Phase1 Minimal Japanese Glyph Test
3. Start Phase1 ASCII Safe Fallback only if needed
```

## Evidence policy

Do not claim Japanese glyph rendering until physical screen confirmation.

Current evidence should remain:

```text
BASE1_B45_JAPANESE_GLYPH_RENDERING=not_claimed
```

until the Japanese test string renders correctly.

## Next engineering pass after test

If minimal Japanese glyph rendering succeeds:

- expand the minimal renderer/font path carefully;
- add stronger Unicode/rounded-glyph defaults;
- move toward SSH transfer support.

If minimal Japanese glyph rendering fails:

- stop using Linux VT/font-copy-only approaches;
- pivot to a full userspace terminal or Phase1-owned framebuffer renderer;
- prioritize SSH transfer support to reduce manual reboot/copy cycles.

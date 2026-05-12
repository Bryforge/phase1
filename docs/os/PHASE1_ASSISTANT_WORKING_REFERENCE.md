# Phase1 assistant working reference

Status: living operator/assistant reference  
Audience: assistant + maintainer workflow  
Branch: `edge/stable`  
Last major hardware target: ThinkPad X200 with Libreboot GRUB

This document is intentionally a working reference for future implementation sessions. It should be updated whenever the boot path, UI rules, hardware evidence, or next-step plan changes.

## 1. Current project model

Phase1 is a Rust terminal-first virtual OS/operator console. It is not yet a complete replacement operating system. The public project framing is:

- secure, private, powerful, open;
- terminal-first Rust virtual OS console;
- Phase1 shell + simulated kernel/VFS/process model;
- Base1 hardware/boot track;
- Fyr native language track;
- guarded host integration with safe mode on by default.

The repository’s executable package is `phase1` version `6.0.0`, Rust 2021, with a release profile optimized for small/fast binaries using LTO, one codegen unit, panic abort, and strip. Current `Cargo.toml` has no normal dependencies, which means boot/initramfs packaging can stay relatively controlled, but the produced binary may still be dynamically linked by the host toolchain.

## 2. Current boot chain evidence

The working bare-metal X200 route is:

```text
Libreboot GRUB -> normal linux -> normal initrd -> rdinit=/init -> Phase1-owned runtime -> native Phase1 binary
```

Known discoveries:

- GRUB-native screens work better than SeaBIOS payload screens on this X200.
- `linux16`/legacy routes repeatedly reset or returned to firmware.
- Normal GRUB `linux` + normal `initrd` + `rdinit=/init` was the first reliable path.
- B38 confirmed a clean Phase1-owned BusyBox runtime.
- B39/B40 reached the native Phase1 binary after packaging dynamic loader/libraries.
- B41 confirmed native Phase1 color console.
- Current B42/B43 goal is not just UTF-8 environment. It must include actual glyph/rendering support for Japanese/CJK.

Do not make the user reboot for small unverified guesses. Every boot USB preparation step should complete local preflight and post-write verification before asking for a real X200 boot.

## 3. Current important files

Core Rust/runtime:

- `Cargo.toml` — package metadata and release build profile.
- `src/main.rs` — process entry point, imports the active boot UI through `#[path = "boot_ui_static.rs"] mod ui;`.
- `src/boot_ui_static.rs` — active boot selector/operator UI for native Phase1.
- `src/linux_colors.rs` — Linux color-pack detection and theme helpers.
- `src/operator.rs` — dashboards/banner/operator surfaces.
- `src/commands.rs` — shell dispatch and command environment model.
- `src/policy.rs` — security/safe-mode reporting and capability boundary.

Boot/build scripts:

- `scripts/x200-b40-prepare-native-phase1-boot.sh` — backward-compatible automation entrypoint. It now targets B42/B43 behavior and should verify the USB after writing.
- `scripts/x200-b42-native-stable-safe-color-utf8-usb.sh` — current native color UTF-8 USB writer.
- `scripts/x200-b23-stage-host-gnulinux.sh` — stages Linux kernel/initrd artifacts when missing.
- `scripts/x200-b7-record-result.sh` — result recorder for hardware observations.

Docs to keep aligned:

- `docs/os/B39_NATIVE_PHASE1_CONSOLE.md`
- `docs/os/B40_NATIVE_BINARY_LOADER_FIX.md`
- `docs/os/B41_NATIVE_COLOR_CONSOLE.md`
- `docs/os/B42_JAPANESE_UTF8_SUPPORT.md`
- this file: `docs/os/PHASE1_ASSISTANT_WORKING_REFERENCE.md`

## 4. Current UX/design requirements

### 4.1 Default boot behavior

The main boot path should not default to ASCII. It should default to the intended Phase1 design:

```text
stable: enabled
safe: enabled
color/native: enabled
ASCII: disabled by default
Japanese UTF-8: ready
host tools: disabled
edge/bleeding: disabled
persistent state: disabled unless explicitly enabled
```

ASCII remains a compatibility fallback only.

The expected GRUB menu structure should be:

```text
Start Phase1 Stable Safe Color UTF-8        default/main design
Start Phase1 Japanese Framebuffer UTF-8     CJK glyph attempt path
Start Phase1 ASCII Safe Fallback            compatibility fallback only
B42/B43 File check                          verification only
Power off / Reboot                          utility
```

### 4.2 Auto-boot expectation

When the main design entry is selected, Phase1 should skip the selector or immediately pass through it into the main shell. The user should not have to press Enter every boot unless intentionally entering configuration mode.

If the Rust UI does not currently honor `PHASE1_AUTO_BOOT=1`, that is a Rust-side gap. The initramfs can set the variable, but `src/main.rs` and/or `src/boot_ui_static.rs` must implement it. The correct behavior is:

```text
if PHASE1_AUTO_BOOT=1 and not ASCII fallback:
  build BootConfig from environment/defaults
  apply config
  enter run_shell(config) directly
else:
  show configure_boot selector
```

### 4.3 Color policy

Current request:

- Safe/stable initial mode should be blue.
- Edge mode should be red.
- Current crimson/red safe mode is wrong for the initial safe/stable default.

Target theme policy:

```text
safe/stable default  -> blue/cyan/ice palette
edge/bleeding mode   -> red/crimson palette
ASCII fallback       -> mono/ASCII-safe palette
CJK framebuffer mode -> blue/cyan unless explicitly edge
```

Implementation location is likely `src/boot_ui_static.rs`, especially palette selection, `active_theme`, default theme selection, and boot card rendering. The initramfs should set `PHASE1_THEME=ice` or `PHASE1_THEME=cyber` for stable/safe, and edge should force `crimson` only when `PHASE1_BLEEDING_EDGE=1`.

### 4.4 Rounded corners

Current corners render as square/box-drawing corners on the X200. This may be one of two issues:

1. The code uses normal box drawing corners instead of rounded Unicode corners.
2. The Linux console font lacks the rounded-corner glyphs, so rounded glyphs fall back or render as square blocks.

Target behavior:

- Main design should use rounded corners when glyph support exists: `╭ ╮ ╰ ╯`.
- If glyph support is missing, degrade intentionally to square corners: `+ + + +` or `┌ ┐ └ ┘` without looking broken.
- The UI should expose a glyph mode separate from ASCII and color:

```text
PHASE1_GLYPH_MODE=rounded|box|ascii|auto
```

Boot preflight should verify that the selected console/font can render rounded corners. If not, choose `box` and record it in evidence rather than pretending rounded corners are active.

### 4.5 Japanese/CJK support

Important distinction:

```text
UTF-8 support != Japanese glyph rendering
```

The current B42 work can set environment variables and package Japanese test text, but the raw Linux VT on X200 cannot reliably render CJK glyphs without a font/rendering layer.

Needed layers:

1. UTF-8 locale/environment: `LANG=C.UTF-8`, `LC_ALL=C.UTF-8`, `LC_CTYPE=C.UTF-8`.
2. Japanese language metadata: `LANGUAGE=ja:en`, `PHASE1_LANGUAGE=ja`, `PHASE1_JAPANESE_SUPPORT=1`.
3. Font/glyph package: Noto CJK, Unifont, or another CJK-capable font.
4. Renderer capable of using the font: framebuffer terminal, graphical terminal, or custom Phase1 framebuffer renderer.

Plain Linux console fonts generally do not provide full Japanese glyph coverage. `setfont` can improve Unicode/box drawing and some glyphs, but it is not enough for reliable Japanese text. The correct next boot path is a dedicated CJK framebuffer entry that attempts one of:

- `fbterm` with Noto CJK/Unifont;
- `jfbterm` if available;
- `kmscon` if available;
- custom Phase1 framebuffer text renderer later.

Evidence should record:

```text
BASE1_B43_UTF8_ENV_READY=1
BASE1_B43_CJK_FONT_PACKAGED=0|1
BASE1_B43_CJK_RENDERER=linux-vt|fbterm|jfbterm|kmscon|phase1-fb
BASE1_B43_JAPANESE_GLYPH_RENDERING=not_claimed|seen
```

Do not claim `phase1_japanese_glyphs_seen` until the physical screen shows Japanese text correctly.

## 5. SSH/server support requirement

The user wants easier transfer and transition so they do not have to manually reboot/copy/test repeatedly.

### 5.1 Security default

SSH support must be packaged but disabled by default unless explicitly enabled. Safe defaults:

```text
PHASE1_SSH_ENABLE=0 by default
PHASE1_SSH_AUTH=key-only
PHASE1_SSH_PASSWORD_LOGIN=0
PHASE1_SSH_ROOT_LOGIN=0
PHASE1_SSH_LISTEN=lan-only or explicit address
PHASE1_SSH_PORT=2222 by default to avoid pretending to be a full OS sshd
PHASE1_SSH_HOST_KEYS=ephemeral unless persistent state is explicitly enabled
PHASE1_SSH_AUTHORIZED_KEYS=/phase1/ssh/authorized_keys
```

### 5.2 Practical implementation direction

For initramfs/USB Phase1, prefer Dropbear first because it is smaller/easier than full OpenSSH for an initramfs. Full OpenSSH can be a later target for a full GNU/Linux base system.

Required packaging candidates:

- `dropbear` or `sshd` binary;
- runtime shared libraries;
- `udhcpc` or equivalent DHCP client if network is not already configured;
- `ip` or BusyBox network applets;
- `/phase1/ssh/authorized_keys`;
- generated or packaged host keys;
- evidence file for SSH state.

Boot menu entries should not silently start SSH. Use explicit entries:

```text
Start Phase1 Stable Safe Color UTF-8
Start Phase1 Stable Safe Color UTF-8 + SSH Transfer
Start Phase1 Japanese Framebuffer UTF-8
Start Phase1 ASCII Safe Fallback
```

SSH enabled entry should pass:

```text
phase1.ssh=1 phase1.ssh_port=2222 phase1.ssh_key_only=1
```

Runtime should print the IP address and port if SSH starts. If no network comes up, it should fall back to local console and record the failure without rebooting.

### 5.3 Do not do yet without preflight

Before asking the user to boot SSH-enabled USB, local script must verify:

- ssh/dropbear binary packaged;
- all dynamic libs packaged;
- host key strategy chosen;
- authorized key exists or explicit temporary key created;
- DHCP/client command exists;
- USB readback shows SSH entry and evidence.

## 6. Current problem list

1. Auto-boot variable is set in initramfs, but native Rust UI still shows selector. Need Rust-side auto-boot implementation.
2. Japanese UTF-8 metadata exists, but actual Japanese glyph display is not ready. Need framebuffer/font renderer.
3. Rounded corners are not reliably rendered. Need glyph-mode detection/fallback and likely font setup.
4. Safe/stable palette should become blue/cyan, not crimson. Crimson should be edge/alert/bleeding.
5. Pi build path is useful for file prep, but final X200 bootable media needs x86_64 binary and x86 BIOS GRUB installer. ARM-built Phase1 binary is not suitable for the X200.
6. Current scripts have grown by patching. They need a cleaner B43/B44 consolidation pass before more user reboot testing.

## 7. Next implementation sequence

Do not ask for another physical reboot until all of the following are complete and locally verified.

### B43: Rust UI policy fix

Implement in Rust:

- `PHASE1_AUTO_BOOT=1` skips boot selector and enters main shell with env/default config.
- Default safe/stable theme becomes blue/cyan.
- Edge/bleeding theme becomes red/crimson.
- `PHASE1_GLYPH_MODE` support: rounded, box, ascii, auto.
- Main boot card uses rounded corners only when glyph mode allows it.
- Japanese status is explicit: UTF-8 ready vs glyph renderer ready.

### B44: CJK framebuffer/font path

Implement script support:

- package optional `fbterm`/`jfbterm`/`kmscon` if available;
- package Noto CJK/Unifont if available;
- add `Start Phase1 Japanese Framebuffer UTF-8` boot entry;
- record CJK renderer and font evidence;
- add `ja-test` command or boot banner inside framebuffer path.

### B45: SSH transfer/server path

Implement script/runtime support:

- package Dropbear first;
- key-only auth;
- no password login;
- no root password login;
- explicit SSH boot entry;
- DHCP/IP display;
- evidence and failure fallback.

### B46: Cleanup/reliability

Consolidate scripts:

- stop patching writer scripts at runtime except for Pi-safe path if unavoidable;
- create a fresh `scripts/x200-b43-full-system-usb.sh` rather than continuing B40 file naming forever;
- keep the old B40 script as a wrapper to the current script only;
- add `scripts/x200-b43-preflight.sh` for no-write validation.

## 8. Preflight rule for future assistant work

Before telling the user to reboot/test, produce one of these:

```text
prepared_and_verified
blocked_with_exact_reason
```

Minimum verification for `prepared_and_verified`:

- repo pulled and expected commit visible;
- Rust binary built for x86_64 when target is X200;
- `file target/release/phase1` shows x86_64 on X200 builder;
- required boot writer exists and passes `sh -n`;
- boot USB mounted after write and inspected;
- GRUB menu contains expected entries;
- initramfs path exists;
- splash exists;
- evidence file exists and shows defaults;
- no missing required commands;
- no claim of Japanese glyph rendering unless verified on-screen.

## 9. Summary for next session

The goal is no longer to prove Linux boot. That is solved. The goal is to turn the proven boot path into a polished Phase1 system:

```text
Libreboot GRUB
  -> Phase1 splash
  -> stable/safe/native color Phase1
  -> auto-enter main shell
  -> blue safe/stable theme
  -> red edge theme
  -> rounded UI when glyphs support it
  -> CJK framebuffer option for Japanese
  -> key-only SSH transfer option
  -> ASCII fallback only for compatibility
```

The next code work should start with Rust-side UI/auto-boot/glyph/theme logic before more boot media tests.

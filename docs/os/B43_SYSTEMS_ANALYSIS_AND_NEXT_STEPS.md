# B43 systems analysis and next steps

Status: implementation planning and preflight reference  
Audience: maintainer + assistant  
Target: X200 Libreboot GRUB, Phase1 native boot path

## Current confirmed state

The X200 can boot Phase1 through the reliable chain:

```text
Libreboot GRUB -> normal linux -> normal initrd -> rdinit=/init -> Phase1-owned initramfs -> native Phase1 binary
```

The native Phase1 binary appears on hardware with color output. The current problem set is no longer basic boot. The problem set is product-quality boot polish, rendering completeness, secure transfer support, and reducing reboot loops.

Confirmed from observed hardware tests:

- B38: Phase1-owned BusyBox runtime reached a clean prompt.
- B39/B40: native Phase1 binary packaging/loader work reached the binary.
- B41: native Phase1 color console works.
- B42: stable/safe/color defaults are represented, but the Rust boot selector still appears and Japanese glyphs do not render on the raw console.

## Code facts from repository review

- `Cargo.toml` package name is `phase1`, version `6.0.0`, Rust 2021. Release builds use LTO, one codegen unit, `panic = "abort"`, and stripping.
- `src/main.rs` is the native entry point and imports the active UI as `#[path = "boot_ui_static.rs"] mod ui;`.
- `src/main.rs` currently always enters `ui::configure_boot(kernel::VERSION)` before `run_shell`, which explains why setting `PHASE1_AUTO_BOOT=1` in the initramfs is not enough by itself.
- `src/boot_ui_static.rs` contains the active boot selector, boot-card rendering, color/ascii defaults, palette selection, and device-mode detection.
- `src/linux_colors.rs` already has a Linux color-pack model and detects truecolor/256/ANSI/mono, but Raspberry Pi safe mode can force mono. This is useful for host-side terminal behavior, but the X200 boot console needs explicit defaults.

## What is actually failing now

### 1. Japanese display

Japanese UTF-8 environment is not enough. The system needs glyph rendering.

Three layers are distinct:

```text
UTF-8 bytes/environment  -> required, but not sufficient
CJK font availability    -> required for glyphs
renderer/font pipeline   -> required to draw glyphs on screen
```

The Linux text console may not render Japanese even when `LANG=C.UTF-8` is correct. Japanese needs a renderer such as a framebuffer terminal (`fbterm`, `jfbterm`, `kmscon`) or a future Phase1 framebuffer renderer with CJK font support.

### 2. Rounded corners

Rounded corners require both code support and glyph support.

Target glyphs:

```text
╭ ╮ ╰ ╯
```

If the console font cannot render them, Phase1 should intentionally fall back to box or ASCII mode instead of showing broken square corners. This should be controlled by:

```text
PHASE1_GLYPH_MODE=auto|rounded|box|ascii
```

### 3. Color semantics

The current crimson color has been proven visually, but it is not the desired safe/stable default.

Target policy:

```text
safe/stable default -> blue/cyan/ice
edge/bleeding       -> red/crimson
ASCII fallback      -> mono
```

### 4. Auto-boot

The initramfs can set `PHASE1_AUTO_BOOT=1`, but the Rust entry point must honor it. Without Rust-side logic, the boot selector still appears.

Target behavior:

```text
PHASE1_AUTO_BOOT=1 and PHASE1_ASCII!=1 -> enter run_shell directly with stable/safe/color config
PHASE1_AUTO_BOOT=0 or ASCII fallback    -> show selector
```

### 5. SSH transfer/server support

SSH should be available for transfer/transition, but never silently enabled by the default boot.

Target secure defaults:

```text
SSH disabled by default
explicit SSH boot entry only
key-only authentication
no password login
no root password login
default port 2222
LAN/local-only intent
ephemeral host keys unless persistent state is explicitly enabled
authorized_keys required or generated transfer key clearly reported
```

Prefer Dropbear first for initramfs size and simplicity. Full OpenSSH can follow later for a full GNU/Linux Base1 system.

## B43 implementation plan

B43 should focus on Rust-side UI policy and no-write preflight.

### Rust UI policy changes

Implement in `src/main.rs` and `src/boot_ui_static.rs`:

1. Auto-boot env path:
   - read `PHASE1_AUTO_BOOT=1`;
   - bypass `configure_boot` only for the main safe/stable/color path;
   - build `BootConfig::default()` from env/config;
   - call `run_shell(config)` directly.

2. Stable/edge palette policy:
   - safe/stable default is blue/cyan/ice;
   - edge/bleeding is red/crimson;
   - red should not be the normal safe boot color.

3. Glyph mode:
   - `PHASE1_GLYPH_MODE=auto|rounded|box|ascii`;
   - rounded mode uses `╭╮╰╯`;
   - box mode uses `┌┐└┘`;
   - ascii mode uses `+`;
   - auto mode should prefer rounded only when `PHASE1_UTF8=1` and not forced ASCII.

4. Japanese status:
   - show `UTF-8 ready` separately from `CJK glyph renderer ready`;
   - do not claim glyph support until the display path is actually configured.

### Script/preflight changes

Add and use no-write preflight before boot testing:

```text
scripts/x200-b43-system-preflight.sh
```

Preflight should check:

- current branch and commit;
- host architecture;
- target binary architecture;
- kernel/splash presence;
- `grub-install` availability for final X200 boot media;
- framebuffer/CJK tools: `fbterm`, `jfbterm`, `kmscon`, `setfont`, `unicode_start`;
- fonts: Noto CJK, Unifont, DejaVu/Terminus console fonts;
- SSH tools: Dropbear/OpenSSH/DHCP/IP helpers;
- USB target safety, if provided;
- whether the result is `prepared_and_verified` or `blocked_with_exact_reason`.

## B44 implementation plan

B44 should package a separate Japanese rendering path.

Boot menu target:

```text
Start Phase1 Stable Safe Color UTF-8
Start Phase1 Japanese Framebuffer UTF-8
Start Phase1 Stable Safe Color UTF-8 + SSH Transfer
Start Phase1 ASCII Safe Fallback
```

The Japanese entry should pass:

```text
phase1.cjk=1 phase1.utf8=1 phase1.ascii=0 phase1.autoboot=1
```

Runtime should attempt:

1. UTF-8 environment;
2. console unicode mode if present;
3. `fbterm`/`jfbterm`/`kmscon` if packaged;
4. fallback to normal color console without claiming Japanese glyph rendering.

Evidence should separate:

```text
BASE1_B44_UTF8_ENV_READY=1
BASE1_B44_CJK_FONT_PACKAGED=0|1
BASE1_B44_CJK_RENDERER=linux-vt|fbterm|jfbterm|kmscon|none
BASE1_B44_JAPANESE_GLYPH_RENDERING=not_claimed|seen
```

## B45 implementation plan

B45 adds secure transfer/server support.

Preferred path: Dropbear in initramfs.

Default: disabled.

Explicit SSH entry should pass:

```text
phase1.ssh=1 phase1.ssh_port=2222 phase1.ssh_key_only=1
```

Packaging needs:

- `dropbear` binary or full `sshd` later;
- shared libraries;
- DHCP helper or static network instructions;
- `ip`/network inspection;
- host keys;
- authorized keys;
- evidence and status output.

Security defaults:

```text
password login off
root login off
key-only auth
print IP/port on console
fall back cleanly if network or ssh fails
```

## Hard rule before next hardware reboot

Do not request another X200 reboot until the prep script reports:

```text
prepared_and_verified
```

If any required layer is missing, report:

```text
blocked_with_exact_reason
```

## Immediate next engineering task

Create and run the B43 no-write preflight. Then implement Rust-side:

1. `PHASE1_AUTO_BOOT` handling in `src/main.rs`;
2. safe/stable blue theme and edge red theme;
3. glyph-mode selection and rounded-corner policy;
4. honest CJK renderer status.

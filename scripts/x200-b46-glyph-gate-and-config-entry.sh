#!/usr/bin/env sh
# B46 local patcher: avoid broken CJK glyph blocks and restore manual config entry.
#
# This script patches local source before build. It does not write disks.

set -eu

fail() { printf 'x200-b46-glyph-gate-and-config-entry: %s\n' "$1" >&2; exit 1; }
[ -d .git ] || fail "run from repo root"
[ -f src/boot_ui_static.rs ] || fail "missing src/boot_ui_static.rs"
[ -f scripts/x200-b42-native-stable-safe-color-utf8-usb.sh ] || fail "missing B42 writer"
command -v python3 >/dev/null 2>&1 || fail "missing python3"

python3 <<'PY'
from pathlib import Path
import re, shutil

ui = Path('src/boot_ui_static.rs')
writer = Path('scripts/x200-b42-native-stable-safe-color-utf8-usb.sh')
for p in (ui, writer):
    b = p.with_suffix(p.suffix + '.b46.bak')
    if not b.exists():
        shutil.copy2(p, b)

u = ui.read_text()
new_fn = '''fn boot_greeting_line(config: BootConfig) -> String {
    let cjk_ready = std::env::var("PHASE1_CJK_GLYPH_SUPPORT")
        .map(|raw| matches!(raw.trim().to_ascii_lowercase().as_str(), "seen" | "ready" | "1" | "true" | "yes" | "on"))
        .unwrap_or(false);
    let greeting = if cjk_ready {
        "\\u{3053}\\u{3093}\\u{306b}\\u{3061}\\u{306f}\\u{3001}\\u{30cf}\\u{30c3}\\u{30ab}\\u{30fc}\\u{ff01}"
    } else {
        "HELLO, HACKER // Japanese glyph renderer pending"
    };
    if config.color && !config.ascii_mode {
        let greeting_color = if config.bleeding_edge { BLUE } else { RED };
        format!("{BOLD}{greeting_color}{greeting}{RESET}")
    } else {
        greeting.to_string()
    }
}
'''
old_pat = r'fn boot_greeting_line\(config: BootConfig\) -> String \{.*?\n\}'
u2, n = re.subn(old_pat, new_fn.rstrip(), u, count=1, flags=re.S)
if n != 1:
    raise SystemExit('could not patch boot_greeting_line')
ui.write_text(u2)

w = writer.read_text()
if 'Configure Phase1 Boot Card' not in w:
    marker = 'menuentry "Start Phase1 ASCII Safe Fallback" {'
    entry = '''menuentry "Configure Phase1 Boot Card" {
    clear
    echo "phase1 6.0.0 ready"
    echo "B46 manual boot configuration card"
    linux /boot/phase1/vmlinuz console=tty0 rdinit=/init init=/init nomodeset quiet loglevel=0 panic=0 phase1.stable=1 phase1.safe=1 phase1.ascii=0 phase1.autoboot=0 phase1.utf8=1 phase1.config=1
    initrd /boot/phase1/phase1-b42-native-stable-safe-color-utf8.img
    boot
}

'''
    if marker not in w:
        raise SystemExit('could not find ASCII fallback marker in writer')
    w = w.replace(marker, entry + marker, 1)
if 'BASE1_B46_JAPANESE_GREETING_GATED=1' not in w:
    w = w.replace('BASE1_B42_AUTO_BOOT_DEFAULT=1\n', 'BASE1_B42_AUTO_BOOT_DEFAULT=1\nBASE1_B46_JAPANESE_GREETING_GATED=1\nBASE1_B46_CONFIG_ENTRY=Configure Phase1 Boot Card\n')
writer.write_text(w)
print('B46 glyph gate + config entry patch applied')
PY

cargo fmt --all >/dev/null 2>&1 || true
printf 'DONE: B46 local patch applied.\n'

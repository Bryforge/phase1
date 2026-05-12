#!/usr/bin/env sh
# B43 local UI policy patch helper.
#
# Applies source-level B43 policies in the local checkout before building:
# - main boot can honor PHASE1_AUTO_BOOT=1
# - safe/stable initramfs boots request blue/cyan instead of crimson
# - edge remains the red/crimson path
# - glyph-mode environment is explicitly staged
#
# The script creates .b43.bak backups and is idempotent enough to rerun.

set -eu

fail() { printf 'x200-b43-apply-ui-policy: %s\n' "$1" >&2; exit 1; }
[ -d .git ] || fail "run from repository root"
[ -f src/main.rs ] || fail "missing src/main.rs"
[ -f scripts/x200-b42-native-stable-safe-color-utf8-usb.sh ] || fail "missing B42 writer"
command -v python3 >/dev/null 2>&1 || fail "missing python3"

python3 <<'PY'
from pathlib import Path
import shutil

main = Path('src/main.rs')
writer = Path('scripts/x200-b42-native-stable-safe-color-utf8-usb.sh')

for path in (main, writer):
    backup = path.with_suffix(path.suffix + '.b43.bak')
    if not backup.exists():
        shutil.copy2(path, backup)

text = main.read_text()
changed_main = False
helper = '''
fn phase1_env_truthy(key: &str) -> bool {
    std::env::var(key)
        .map(|value| matches!(value.trim().to_ascii_lowercase().as_str(), "1" | "true" | "yes" | "on"))
        .unwrap_or(false)
}

fn phase1_auto_boot_requested() -> bool {
    phase1_env_truthy("PHASE1_AUTO_BOOT") && !phase1_env_truthy("PHASE1_ASCII")
}
'''
if 'fn phase1_auto_boot_requested()' not in text:
    text = text.replace('fn main() {', helper + '\nfn main() {', 1)
    changed_main = True

old = 'match ui::configure_boot(kernel::VERSION) {'
new = '''let boot_selection = if phase1_auto_boot_requested() {
            ops_log::log_event("boot.autoboot", "PHASE1_AUTO_BOOT=1");
            ui::BootSelection::Boot(ui::BootConfig::default())
        } else {
            ui::configure_boot(kernel::VERSION)
        };
        match boot_selection {'''
if old in text and 'let boot_selection = if phase1_auto_boot_requested()' not in text:
    text = text.replace(old, new, 1)
    changed_main = True
main.write_text(text)

w = writer.read_text()
changed_writer = False
repls = {
    'export PHASE1_THEME=crimson': 'export PHASE1_THEME=ice',
    'export PHASE1_COLOR_MODE=auto': 'export PHASE1_COLOR_MODE=auto\n  export PHASE1_GLYPH_MODE=rounded',
    'phase1.autoboot=1 phase1.utf8=1': 'phase1.autoboot=1 phase1.utf8=1 phase1.glyphs=rounded',
}
for old, new in repls.items():
    if old in w and new not in w:
        w = w.replace(old, new)
        changed_writer = True
if 'BASE1_B43_SAFE_STABLE_THEME=blue' not in w:
    marker = 'BASE1_B42_AUTO_BOOT_DEFAULT=1\n'
    w = w.replace(marker, marker + 'BASE1_B43_SAFE_STABLE_THEME=blue\nBASE1_B43_EDGE_THEME=crimson\nBASE1_B43_GLYPH_MODE=rounded\n')
    changed_writer = True
writer.write_text(w)

print(f'B43 policy patch complete: main_changed={changed_main} writer_changed={changed_writer}')
PY

cargo fmt --all >/dev/null 2>&1 || true
printf 'B43 UI policy patch helper completed. Run cargo build --release next.\n'

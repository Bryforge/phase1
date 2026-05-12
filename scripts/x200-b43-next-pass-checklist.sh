#!/usr/bin/env sh
set -eu
cd "$(dirname "$0")/.."
echo "B43 next pass checklist"
echo "1 auto-enter main runtime: pending Rust change"
echo "2 stable safe palette blue: pending Rust/UI change"
echo "3 edge palette red: pending Rust/UI change"
echo "4 rounded corners: pending glyph-mode change"
echo "5 Japanese glyph rendering: pending framebuffer/font path"
echo "6 SSH transfer: pending explicit secure boot entry"
echo
cargo build --release
sh scripts/x200-b43-system-preflight.sh "${1:-}" || true

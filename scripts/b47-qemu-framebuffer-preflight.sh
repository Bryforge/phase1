#!/usr/bin/env sh
# Phase1 B47 QEMU framebuffer renderer preflight.
#
# Usage:
#   sh scripts/b47-qemu-framebuffer-preflight.sh
#
# Purpose:
#   Verify the host can build and run the framebuffer renderer lab before the
#   lab script is invoked. This does not write disks and does not launch QEMU.

set -u

OUT_DIR="${BASE1_B47_PREFLIGHT_OUT:-build/base1-b47-qemu-framebuffer-preflight}"
REPORT="$OUT_DIR/b47-qemu-framebuffer-preflight.env"
KERNEL="${BASE1_B47_KERNEL:-build/linux/alpine-netboot/vmlinuz}"

mkdir -p "$OUT_DIR"
: > "$REPORT"

kv() { printf '%s=%s\n' "$1" "$2" | tee -a "$REPORT" >/dev/null; }
has() { command -v "$1" >/dev/null 2>&1; }
failures=""
warnings=""
add_failure() { failures="${failures}${1};"; }
add_warning() { warnings="${warnings}${1};"; }

printf 'PHASE1 B47 QEMU FRAMEBUFFER PREFLIGHT\n'
printf 'no disk writes; no QEMU launch\n\n'

arch="$(uname -m 2>/dev/null || echo unknown)"
kv BASE1_B47_PREFLIGHT_HOST_ARCH "$arch"
printf 'host arch: %s\n' "$arch"

printf '\ncommand checks:\n'
for cmd in git sh python3 cc cpio gzip file find awk head qemu-system-x86_64; do
  if has "$cmd"; then
    kv "BASE1_B47_CMD_$(printf '%s' "$cmd" | tr 'a-z-' 'A-Z_')" "yes $(command -v "$cmd")"
    printf 'ok: %s -> %s\n' "$cmd" "$(command -v "$cmd")"
  else
    kv "BASE1_B47_CMD_$(printf '%s' "$cmd" | tr 'a-z-' 'A-Z_')" no
    printf 'no: %s\n' "$cmd"
    case "$cmd" in
      qemu-system-x86_64) add_warning "missing-qemu-system-x86_64" ;;
      python3|cc|cpio|gzip|file|find|awk|head) add_failure "missing-$cmd" ;;
    esac
  fi
done

printf '\npython pillow check:\n'
if python3 - <<'PY' >/dev/null 2>&1
from PIL import Image, ImageDraw, ImageFont
PY
then
  kv BASE1_B47_PYTHON_PIL yes
  printf 'ok: PIL/Pillow available\n'
else
  kv BASE1_B47_PYTHON_PIL no
  printf 'no: PIL/Pillow unavailable\n'
  add_failure "missing-python-pillow"
fi

printf '\nkernel check:\n'
if [ -f "$KERNEL" ]; then
  kv BASE1_B47_KERNEL_PRESENT yes
  kv BASE1_B47_KERNEL "$KERNEL"
  printf 'ok: %s\n' "$KERNEL"
else
  kv BASE1_B47_KERNEL_PRESENT no
  kv BASE1_B47_KERNEL "$KERNEL"
  printf 'missing: %s\n' "$KERNEL"
  add_failure "missing-kernel"
fi

printf '\nfont check:\n'
font=""
for pattern in '*NotoSansCJK*JP*.otf' '*NotoSansMonoCJK*JP*.otf' '*Noto*Sans*CJK*JP*' '*unifont*.ttf' '*unifont*.otf' '*DejaVuSansMono*.ttf'; do
  found="$(find /usr/share/fonts /usr/share/consolefonts -type f -iname "$pattern" 2>/dev/null | head -n 1)"
  if [ -n "$found" ]; then font="$found"; break; fi
done
if [ -n "$font" ]; then
  kv BASE1_B47_FONT_CANDIDATE "$font"
  printf 'ok: %s\n' "$font"
else
  kv BASE1_B47_FONT_CANDIDATE none
  printf 'missing: CJK/Unicode font candidate\n'
  add_failure "missing-font-candidate"
fi

kv BASE1_B47_PREFLIGHT_WARNINGS "${warnings:-none}"
kv BASE1_B47_PREFLIGHT_FAILURES "${failures:-none}"

printf '\n'
if [ -n "$failures" ]; then
  kv BASE1_B47_PREFLIGHT_RESULT blocked_with_exact_reason
  printf 'RESULT: blocked_with_exact_reason\n'
  printf 'failures: %s\n' "$failures"
  printf 'warnings: %s\n' "${warnings:-none}"
  printf 'report: %s\n' "$REPORT"
  exit 1
fi

kv BASE1_B47_PREFLIGHT_RESULT ready_for_qemu_framebuffer_lab
printf 'RESULT: ready_for_qemu_framebuffer_lab\n'
printf 'warnings: %s\n' "${warnings:-none}"
printf 'report: %s\n' "$REPORT"

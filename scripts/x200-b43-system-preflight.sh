#!/usr/bin/env sh
# Phase1 B43 no-write system preflight.
#
# Purpose:
#   Verify what is ready before another hardware reboot/test.
#   This script does not write USB media and does not mutate disks.
#
# Usage:
#   sh scripts/x200-b43-system-preflight.sh
#   sh scripts/x200-b43-system-preflight.sh /dev/sdb

set -u

TARGET="${1:-}"
OUT_DIR="${BASE1_B43_PREFLIGHT_OUT:-build/base1-b43-system-preflight}"
REPORT="$OUT_DIR/b43-system-preflight.env"
KERNEL="${BASE1_B43_KERNEL:-build/linux/alpine-netboot/vmlinuz}"
SPLASH="${BASE1_B43_SPLASH:-assets/phase1-splash.png}"
PHASE1_BIN="${BASE1_B43_PHASE1_BIN:-target/release/phase1}"

mkdir -p "$OUT_DIR"
: > "$REPORT"

say() { printf '%s\n' "$*"; }
kv() { printf '%s=%s\n' "$1" "$2" | tee -a "$REPORT" >/dev/null; }
check_cmd() { command -v "$1" >/dev/null 2>&1; }
record_cmd() { if check_cmd "$1"; then kv "BASE1_B43_CMD_$(printf '%s' "$1" | tr 'a-z-' 'A-Z_')" "yes $(command -v "$1")"; else kv "BASE1_B43_CMD_$(printf '%s' "$1" | tr 'a-z-' 'A-Z_')" "no"; fi; }

failures=""
warns=""
add_failure() { failures="${failures}${1};"; }
add_warn() { warns="${warns}${1};"; }

say "PHASE1 B43 SYSTEM PREFLIGHT"
say "no writes will be performed"
say ""

branch="$(git rev-parse --abbrev-ref HEAD 2>/dev/null || echo unknown)"
commit="$(git log -1 --oneline 2>/dev/null || echo unknown)"
arch="$(uname -m 2>/dev/null || echo unknown)"
root_src="$(findmnt -no SOURCE / 2>/dev/null || echo unknown)"

kv BASE1_B43_PREFLIGHT_BRANCH "$branch"
kv BASE1_B43_PREFLIGHT_COMMIT "$commit"
kv BASE1_B43_PREFLIGHT_HOST_ARCH "$arch"
kv BASE1_B43_PREFLIGHT_ROOT_SOURCE "$root_src"
kv BASE1_B43_PREFLIGHT_TARGET "${TARGET:-none}"

say "repo: $branch / $commit"
say "host arch: $arch"
say "root source: $root_src"

if [ "$branch" != "edge/stable" ]; then add_warn "branch-not-edge-stable"; fi

say ""
say "command checks:"
for cmd in git cargo rustup sh awk grep sha256sum file findmnt lsblk mount umount mktemp sudo parted mkfs.vfat grub-install cpio gzip ldd setfont unicode_start fbterm jfbterm kmscon dropbear dropbearkey sshd ssh-keygen udhcpc dhclient ip; do
  record_cmd "$cmd"
  if check_cmd "$cmd"; then say "ok: $cmd -> $(command -v "$cmd")"; else say "no: $cmd"; fi
done

say ""
say "artifact checks:"
if [ -f "$KERNEL" ]; then kv BASE1_B43_KERNEL_PRESENT yes; kv BASE1_B43_KERNEL_SHA256 "$(sha256sum "$KERNEL" | awk '{print $1}')"; say "ok: kernel $KERNEL"; else kv BASE1_B43_KERNEL_PRESENT no; say "missing: kernel $KERNEL"; add_failure "missing-kernel"; fi
if [ -f "$SPLASH" ]; then kv BASE1_B43_SPLASH_PRESENT yes; kv BASE1_B43_SPLASH_SHA256 "$(sha256sum "$SPLASH" | awk '{print $1}')"; say "ok: splash $SPLASH"; else kv BASE1_B43_SPLASH_PRESENT no; say "missing: splash $SPLASH"; add_failure "missing-splash"; fi
if [ -x "$PHASE1_BIN" ]; then
  kv BASE1_B43_PHASE1_BIN_PRESENT yes
  bin_file="$(file "$PHASE1_BIN" 2>/dev/null || echo unknown)"
  kv BASE1_B43_PHASE1_BIN_FILE "$bin_file"
  kv BASE1_B43_PHASE1_BIN_SHA256 "$(sha256sum "$PHASE1_BIN" | awk '{print $1}')"
  say "ok: phase1 binary $PHASE1_BIN"
  say "file: $bin_file"
  case "$bin_file" in
    *x86-64*) kv BASE1_B43_PHASE1_BIN_X200_COMPAT yes ;;
    *) kv BASE1_B43_PHASE1_BIN_X200_COMPAT no; add_warn "phase1-binary-not-x86_64-for-x200" ;;
  esac
else
  kv BASE1_B43_PHASE1_BIN_PRESENT no
  say "missing: phase1 binary $PHASE1_BIN"
  add_failure "missing-phase1-binary"
fi

say ""
say "font / CJK checks:"
font_found=0
for dir in /usr/share/fonts/truetype/noto /usr/share/fonts/opentype/noto /usr/share/fonts/truetype/dejavu /usr/share/consolefonts; do
  if [ -d "$dir" ]; then
    count="$(find "$dir" -type f 2>/dev/null | wc -l | awk '{print $1}')"
    kv "BASE1_B43_FONT_DIR_$(printf '%s' "$dir" | tr '/a-z-' '_A-Z_')" "$count"
    say "font dir: $dir ($count files)"
    [ "$count" -gt 0 ] && font_found=1
  fi
done
if find /usr/share/fonts /usr/share/consolefonts -type f 2>/dev/null | grep -Ei 'noto.*cjk|cjk|unifont|ipag|takao|sazanami|terminus|psf' >/dev/null 2>&1; then
  kv BASE1_B43_CJK_OR_UNICODE_FONT_CANDIDATE yes
  say "ok: found CJK/Unicode/font candidates"
else
  kv BASE1_B43_CJK_OR_UNICODE_FONT_CANDIDATE no
  say "no: CJK/Unicode font candidate found"
  add_warn "missing-cjk-font-candidate"
fi
if check_cmd fbterm || check_cmd jfbterm || check_cmd kmscon; then kv BASE1_B43_CJK_RENDERER_CANDIDATE yes; else kv BASE1_B43_CJK_RENDERER_CANDIDATE no; add_warn "missing-framebuffer-cjk-renderer"; fi

say ""
say "SSH transfer checks:"
if check_cmd dropbear || check_cmd sshd; then kv BASE1_B43_SSH_SERVER_CANDIDATE yes; else kv BASE1_B43_SSH_SERVER_CANDIDATE no; add_warn "missing-ssh-server-candidate"; fi
if check_cmd dropbearkey || check_cmd ssh-keygen; then kv BASE1_B43_SSH_KEYGEN_CANDIDATE yes; else kv BASE1_B43_SSH_KEYGEN_CANDIDATE no; add_warn "missing-ssh-keygen-candidate"; fi
if check_cmd udhcpc || check_cmd dhclient || check_cmd ip; then kv BASE1_B43_NETWORK_TOOL_CANDIDATE yes; else kv BASE1_B43_NETWORK_TOOL_CANDIDATE no; add_warn "missing-network-tool-candidate"; fi

if [ -n "$TARGET" ]; then
  say ""
  say "target safety check: $TARGET"
  if [ -b "$TARGET" ]; then
    kv BASE1_B43_TARGET_BLOCK_DEVICE yes
    case "$root_src" in
      "$TARGET"|"$TARGET"[0-9]*|"$TARGET"p[0-9]*) kv BASE1_B43_TARGET_IS_ROOT yes; add_failure "target-is-root-device" ;;
      *) kv BASE1_B43_TARGET_IS_ROOT no ;;
    esac
    lsblk -o NAME,PATH,SIZE,MODEL,TRAN,RM,MOUNTPOINTS "$TARGET" 2>/dev/null || true
  else
    kv BASE1_B43_TARGET_BLOCK_DEVICE no
    add_failure "target-not-block-device"
  fi
fi

kv BASE1_B43_PREFLIGHT_WARNINGS "${warns:-none}"
kv BASE1_B43_PREFLIGHT_FAILURES "${failures:-none}"
if [ -n "$failures" ]; then
  kv BASE1_B43_PREFLIGHT_RESULT blocked_with_exact_reason
  say ""
  say "RESULT: blocked_with_exact_reason"
  say "failures: $failures"
  say "warnings: ${warns:-none}"
  say "report: $REPORT"
  exit 1
fi

kv BASE1_B43_PREFLIGHT_RESULT prepared_for_next_implementation
say ""
say "RESULT: prepared_for_next_implementation"
say "warnings: ${warns:-none}"
say "report: $REPORT"

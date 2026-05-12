#!/usr/bin/env sh
# Phase1 B45 next-test preparation orchestrator.
#
# Branch-aware version for black-phase1: it updates the current branch instead
# of forcing edge/stable. This prevents divergent-branch failures during rapid
# experiments.

set -eu

if [ -d "$HOME/.cargo/bin" ]; then PATH="$HOME/.cargo/bin:$PATH"; export PATH; fi
if [ -f "$HOME/.cargo/env" ]; then . "$HOME/.cargo/env" 2>/dev/null || true; fi

USB="${1:-}"
CONFIRM="${2:-}"
TARGET_BRANCH="${BASE1_TARGET_BRANCH:-$(git rev-parse --abbrev-ref HEAD 2>/dev/null || printf black-phase1)}"
OUT_DIR="${BASE1_B45_PREP_OUT:-build/base1-b45-next-test-prep}"
REPORT="$OUT_DIR/b45-next-test-prep.env"
VERIFY_LOG="$OUT_DIR/b45-final-usb-verify.log"
MAIN_ENTRY="Start Phase1 Stable Safe Color UTF-8"
MIN_ENTRY="Start Phase1 Minimal Japanese Glyph Test"
ASCII_ENTRY="Start Phase1 ASCII Safe Fallback"
INITRD_NAME="phase1-b42-native-stable-safe-color-utf8.img"

fail() { printf 'x200-b45-prepare-next-test: %s\n' "$1" >&2; exit 1; }
section() { printf '\n===== %s =====\n' "$1"; }
need() { command -v "$1" >/dev/null 2>&1 || fail "missing command: $1"; }
part1() { case "$1" in /dev/nvme[0-9]n[0-9]|/dev/mmcblk[0-9]) printf '%sp1\n' "$1" ;; /dev/sd[a-z]|/dev/vd[a-z]|/dev/hd[a-z]) printf '%s1\n' "$1" ;; *) fail "use a whole disk path, not $1" ;; esac; }

repair_ownership() {
  for path in build/base1-b42-native-stable-safe-color-utf8 build/base1-b43-system-preflight build/base1-b45-minimal-unicode "$OUT_DIR" target; do
    if [ -e "$path" ] && [ ! -w "$path" ]; then
      printf 'Repairing unwritable path: %s\n' "$path"
      sudo chown -R "$(id -u):$(id -g)" "$path"
    fi
  done
}

root_device_check() {
  root_src="$(findmnt -no SOURCE / 2>/dev/null || true)"
  printf 'root source: %s\n' "${root_src:-unknown}"
  case "$root_src" in
    "$USB"|"$USB"[0-9]*|"$USB"p[0-9]*) fail "refusing to write root filesystem device: $root_src" ;;
  esac
}

[ -n "$USB" ] || fail "usage: sh scripts/x200-b45-prepare-next-test.sh /dev/sdb YES_WRITE_USB"
[ "$CONFIRM" = "YES_WRITE_USB" ] || fail "missing YES_WRITE_USB confirmation"
[ -d .git ] || fail "run from phase1 repository root"
if [ "$(id -u)" = "0" ]; then fail "do not run this wrapper with sudo; run as normal user"; fi
for c in git sh cargo file sudo findmnt mount umount mktemp grep awk tee tail; do need "$c"; done
mkdir -p "$OUT_DIR"
: > "$VERIFY_LOG"

section "REPAIR LOCAL OWNERSHIP"
repair_ownership

section "UPDATE REPOSITORY"
printf 'target branch: %s\n' "$TARGET_BRANCH"
git fetch origin "$TARGET_BRANCH"
git pull --ff-only origin "$TARGET_BRANCH"
git log -1 --oneline
root_device_check

section "APPLY UI POLICY AND BUILD"
[ -f scripts/x200-b43-apply-ui-policy.sh ] || fail "missing B43 UI policy helper"
sh scripts/x200-b43-apply-ui-policy.sh
rustup default stable >/dev/null 2>&1 || true
cargo build --release
[ -x target/release/phase1 ] || fail "missing target/release/phase1"
file target/release/phase1
case "$(file target/release/phase1)" in
  *x86-64*) printf 'Phase1 binary architecture: x86_64 OK for X200\n' ;;
  *) fail "Phase1 binary is not x86_64; do final USB prep on X200/x86_64 builder" ;;
esac

section "B43 PREFLIGHT"
sh scripts/x200-b43-system-preflight.sh "$USB"

section "WRITE BASE USB"
[ -f scripts/x200-b43-prepare-polished-boot.sh ] || fail "missing B43 polished boot wrapper"
BASE1_TARGET_BRANCH="$TARGET_BRANCH" sh scripts/x200-b43-prepare-polished-boot.sh "$USB" YES_WRITE_USB

section "APPLY B45 MINIMAL UNICODE AUGMENTATION"
[ -f scripts/x200-b45-minimal-unicode-augment.sh ] || fail "missing B45 minimal Unicode augment script"
sh scripts/x200-b45-minimal-unicode-augment.sh "$USB" YES_WRITE_USB

section "FINAL USB READBACK VERIFY"
PART="$(part1 "$USB")"
[ -b "$PART" ] || fail "partition not found: $PART"
MNT="$(mktemp -d)"
set +e
{
  echo "partition=$PART"
  sudo mount -o ro "$PART" "$MNT" || exit 10
  echo "mounted=$MNT"
  test -f "$MNT/boot/grub/grub.cfg" || exit 11
  test -f "$MNT/boot/phase1/vmlinuz" || exit 12
  test -f "$MNT/boot/phase1/$INITRD_NAME" || exit 13
  test -f "$MNT/phase1/assets/phase1-splash.png" || exit 14
  test -f "$MNT/phase1/evidence/b42-prep.env" || exit 15
  grep -q "$MAIN_ENTRY" "$MNT/boot/grub/grub.cfg" || exit 16
  grep -q "$MIN_ENTRY" "$MNT/boot/grub/grub.cfg" || exit 17
  grep -q "$ASCII_ENTRY" "$MNT/boot/grub/grub.cfg" || exit 18
  grep -q 'phase1.ascii=0' "$MNT/boot/grub/grub.cfg" || exit 19
  grep -q 'phase1.ascii=1' "$MNT/boot/grub/grub.cfg" || exit 20
  grep -q 'phase1.b45_minimal_unicode=1' "$MNT/boot/grub/grub.cfg" || exit 21
  grep -q 'BASE1_B45_MINIMAL_FONT=' "$MNT/phase1/evidence/b42-prep.env" || exit 22
  grep -q 'BASE1_B45_MINIMAL_RENDERER=' "$MNT/phase1/evidence/b42-prep.env" || exit 23
  echo "USB menu entries:"
  grep '^menuentry ' "$MNT/boot/grub/grub.cfg" || true
  echo "USB B45 evidence:"
  grep 'BASE1_B45_' "$MNT/phase1/evidence/b42-prep.env" || true
  sudo umount "$MNT" || exit 24
} > "$VERIFY_LOG" 2>&1
VERIFY_RC=$?
rmdir "$MNT" 2>/dev/null || true
set -e
tail -n 120 "$VERIFY_LOG" || true
[ "$VERIFY_RC" -eq 0 ] || fail "final USB verification failed with exit code $VERIFY_RC; log: $VERIFY_LOG"

cat > "$REPORT" <<EOF
BASE1_B45_NEXT_TEST_TARGET=$USB
BASE1_B45_NEXT_TEST_BRANCH=$TARGET_BRANCH
BASE1_B45_NEXT_TEST_PARTITION=$PART
BASE1_B45_NEXT_TEST_MAIN_ENTRY=$MAIN_ENTRY
BASE1_B45_NEXT_TEST_MINIMAL_ENTRY=$MIN_ENTRY
BASE1_B45_NEXT_TEST_ASCII_ENTRY=$ASCII_ENTRY
BASE1_B45_NEXT_TEST_VERIFY_LOG=$VERIFY_LOG
BASE1_B45_NEXT_TEST_RESULT=prepared_and_verified_for_next_test
BASE1_B45_NEXT_TEST_JAPANESE_GLYPH_RENDERING=not_claimed
EOF

section "DONE"
printf 'RESULT: prepared_and_verified_for_next_test\n'
printf 'Boot test order:\n'
printf '  1. %s\n' "$MAIN_ENTRY"
printf '  2. %s\n' "$MIN_ENTRY"
printf '  3. %s only if fallback needed\n' "$ASCII_ENTRY"
printf 'Report: %s\n' "$REPORT"

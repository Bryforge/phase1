#!/usr/bin/env sh
# black-phase1 full rapid test cycle helper.
#
# Usage:
#   sh scripts/black-phase1-cycle.sh /dev/sdb YES_WRITE_USB "checkpoint-name"
#
# Purpose:
#   Run the full rapid branch cycle before a hardware test:
#   - ensure branch is black-phase1;
#   - pull latest black-phase1;
#   - run status;
#   - run doctor;
#   - create a checkpoint;
#   - prepare verified X200 test media;
#   - apply the B47 framebuffer boot-card augmentation;
#   - force the B47 framebuffer entry to first/default;
#   - verify the B47 framebuffer GRUB entry by readback.
#
# Do not run this whole wrapper with sudo. Called scripts use sudo internally
# when media writing is required.

set -eu

USB="${1:-}"
CONFIRM="${2:-}"
CHECKPOINT_NAME="${3:-pre-test}"
BRANCH="black-phase1"
ENTRY="Start Phase1 Framebuffer Boot Card"

fail() { printf 'black-phase1-cycle: %s\n' "$1" >&2; exit 1; }
part1() { case "$1" in /dev/nvme[0-9]n[0-9]|/dev/mmcblk[0-9]) printf '%sp1\n' "$1" ;; /dev/sd[a-z]|/dev/vd[a-z]|/dev/hd[a-z]) printf '%s1\n' "$1" ;; *) fail "use a whole disk like /dev/sdb" ;; esac; }

[ -n "$USB" ] || fail "usage: sh scripts/black-phase1-cycle.sh /dev/sdb YES_WRITE_USB checkpoint-name"
[ "$CONFIRM" = "YES_WRITE_USB" ] || fail "missing YES_WRITE_USB confirmation"
[ -d .git ] || fail "run from phase1 repository root"
command -v git >/dev/null 2>&1 || fail "missing git"

if [ "$(id -u)" = "0" ]; then
  fail "do not run with sudo; wrappers call sudo internally"
fi

current="$(git rev-parse --abbrev-ref HEAD)"
[ "$current" = "$BRANCH" ] || fail "expected branch $BRANCH, got $current"

if [ -n "$(git status --short)" ]; then
  printf 'Working tree has local changes. Commit or stash before full cycle.\n' >&2
  git status --short >&2
  exit 1
fi

printf 'black-phase1 full test cycle\n'
printf 'target    : %s\n' "$USB"
printf 'checkpoint: %s\n\n' "$CHECKPOINT_NAME"

git fetch origin "$BRANCH"
git pull --ff-only origin "$BRANCH"

git log -1 --oneline

sh scripts/black-phase1-status-report.sh
sh scripts/black-phase1-doctor.sh "$USB"
sh scripts/black-phase1-checkpoint.sh "$CHECKPOINT_NAME"
sh scripts/black-phase1-x200-test.sh "$USB" YES_WRITE_USB

printf '\n===== APPLY B47 FRAMEBUFFER BOOT ENTRY =====\n'
if [ -f scripts/x200-b47-framebuffer-boot-augment.sh ]; then
  sh scripts/x200-b47-framebuffer-boot-augment.sh "$USB" YES_WRITE_USB
else
  fail "missing scripts/x200-b47-framebuffer-boot-augment.sh"
fi

printf '\n===== FORCE B47 FRAMEBUFFER DEFAULT =====\n'
if [ -f scripts/x200-b47-force-framebuffer-default.sh ]; then
  sh scripts/x200-b47-force-framebuffer-default.sh "$USB" YES_WRITE_USB
else
  fail "missing scripts/x200-b47-force-framebuffer-default.sh"
fi

printf '\n===== VERIFY B47 FRAMEBUFFER ENTRY =====\n'
PART="$(part1 "$USB")"
MNT="$(mktemp -d)"
cleanup() { sudo umount "$MNT" 2>/dev/null || true; rmdir "$MNT" 2>/dev/null || true; }
trap cleanup EXIT INT TERM
sudo mount -o ro "$PART" "$MNT"
[ -f "$MNT/boot/grub/grub.cfg" ] || fail "missing grub.cfg after B47 augment"
first_menu="$(grep '^menuentry ' "$MNT/boot/grub/grub.cfg" | head -n 1 || true)"
printf 'First menu entry: %s\n' "$first_menu"
printf '%s\n' "$first_menu" | grep -q "$ENTRY" || fail "B47 framebuffer entry is not first/default"
grep -q "$ENTRY" "$MNT/boot/grub/grub.cfg" || fail "B47 framebuffer entry missing after augment"
grep -q 'phase1.framebuffer=1' "$MNT/boot/grub/grub.cfg" || fail "B47 framebuffer kernel flag missing"
grep -q 'BASE1_B47_ENTRY=' "$MNT/phase1/evidence/b42-prep.env" || fail "B47 evidence missing from USB"
printf 'Verified GRUB entry: %s\n' "$ENTRY"
printf 'B47 evidence:\n'
grep 'BASE1_B47_' "$MNT/phase1/evidence/b42-prep.env" || true
cleanup
trap - EXIT INT TERM

printf '\nDONE: black-phase1 full cycle completed.\n'
printf 'RESULT: prepared_and_verified_for_next_test\n'
printf 'Next X200 boot test entry/default:\n'
printf '  %s\n' "$ENTRY"
printf 'Normal fallback still available:\n'
printf '  Start Phase1 Stable Safe Color UTF-8\n'
printf '  Start Phase1 ASCII Safe Fallback\n'

#!/usr/bin/env sh
# Phase1 X200 major-result recorder and patch-share helper.
#
# Purpose:
#   Reduce repeated typing during physical X200 testing. This records a single
#   major result, commits it locally, creates a patch in ~/phase1-share, and
#   prints the Mac commands needed to apply it.
#
# Usage:
#   sh scripts/x200-record-and-share-result.sh phase1_system_console_v2_seen
#   sh scripts/x200-record-and-share-result.sh phase1_system_console_seen
#   sh scripts/x200-record-and-share-result.sh phase1_seabios_grub_seen
#
# This does not push to GitHub. The X200 can share the patch over local HTTP;
# the Mac applies and pushes it.

set -eu

RESULT="${1:-}"
SHARE_DIR="${PHASE1_SHARE_DIR:-$HOME/phase1-share}"
X200_IP="${X200_IP:-192.168.4.73}"
PATCH_NAME="x200-${RESULT}.patch"
PATCH_PATH="$SHARE_DIR/$PATCH_NAME"
RECORDER="scripts/x200-b7-record-result.sh"

fail() { printf 'x200-record-and-share-result: %s\n' "$1" >&2; exit 1; }

[ -n "$RESULT" ] || fail "usage: sh scripts/x200-record-and-share-result.sh <result>"
[ -d .git ] || fail "run this from the phase1 repository root"
[ -f "$RECORDER" ] || fail "missing $RECORDER; run: git pull --ff-only origin edge/stable"

case "$RESULT" in
  phase1_system_console_v2_seen|phase1_system_console_seen|phase1_seabios_grub_seen|phase1_seabios_multiboot_seen|phase1_mbr_bootsector_seen|phase1_bootsector_seen|phase1_kernel_framebuffer_seen|phase1_kernel_keyboard_console_seen|phase1_grub_console_seen|blocked_after_seabios_multiboot_load|blocked_after_seabios_payload|blocked_after_chainload|blocked_after_multiboot_load|seabios_usb_not_booting|failed) : ;;
  *) fail "unsupported or too-small result for this helper: $RESULT" ;;
esac

git pull --ff-only origin edge/stable
sh "$RECORDER" "$RESULT"
git add -f build/base1-b7-hardware-boot-evidence/b7-hardware-boot-evidence.env

if git diff --cached --quiet; then
  printf 'No staged evidence change; result may already be recorded.\n'
else
  git commit -m "Record X200 $RESULT"
fi

mkdir -p "$SHARE_DIR"
git format-patch -1 HEAD --stdout > "$PATCH_PATH"

printf '\nPrepared patch: %s\n' "$PATCH_PATH"
printf '\nTo serve from X200, run:\n'
printf '  cd %s && python3 -m http.server 8000\n' "$SHARE_DIR"
printf '\nOn the Mac, run one line:\n'
printf '  cd ~/phase1_library/phase1 && git am --abort 2>/dev/null || true; curl -fL -o /tmp/%s http://%s:8000/%s && git am --3way /tmp/%s && git pull --rebase origin edge/stable && git push origin edge/stable\n' "$PATCH_NAME" "$X200_IP" "$PATCH_NAME" "$PATCH_NAME"

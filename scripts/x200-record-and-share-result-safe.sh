#!/usr/bin/env sh
# Phase1 X200 major-result recorder and patch-share helper.
#
# Privacy-safe helper: this script does not hard-code private LAN IP addresses.
# Pass the X200 URL to the Mac manually, or set X200_URL after starting the
# local HTTP server.
#
# Usage:
#   sh scripts/x200-record-and-share-result-safe.sh phase1_integrated_runtime_seen
#   sh scripts/x200-record-and-share-result-safe.sh phase1_crypto_evidence_seen
#   sh scripts/x200-record-and-share-result-safe.sh phase1_supervisor_plan_seen
#
# This does not push to GitHub. The X200 creates a patch in ~/phase1-share.
# The Mac applies and pushes it.

set -eu

RESULT="${1:-}"
SHARE_DIR="${PHASE1_SHARE_DIR:-$HOME/phase1-share}"
PATCH_NAME="x200-${RESULT}.patch"
PATCH_PATH="$SHARE_DIR/$PATCH_NAME"
RECORDER="scripts/x200-b7-record-result.sh"

fail() { printf 'x200-record-and-share-result-safe: %s\n' "$1" >&2; exit 1; }

[ -n "$RESULT" ] || fail "usage: sh scripts/x200-record-and-share-result-safe.sh <result>"
[ -d .git ] || fail "run this from the phase1 repository root"
[ -f "$RECORDER" ] || fail "missing $RECORDER; run: git pull --ff-only origin edge/stable"

case "$RESULT" in
  phase1_integrated_runtime_seen|phase1_crypto_evidence_seen|phase1_evidence_hash_manifest_seen|phase1_supervisor_plan_seen|phase1_persistent_workspace_seen|phase1_runtime_workspace_seen|phase1_gnulinux_shell_seen|phase1_gnulinux_initramfs_seen|blocked_after_gnulinux_load|phase1_polished_system_seen|phase1_splash_mode_seen|phase1_system_console_v2_seen|phase1_system_console_seen|phase1_seabios_grub_seen|phase1_seabios_multiboot_seen|phase1_mbr_bootsector_seen|phase1_bootsector_seen|phase1_kernel_framebuffer_seen|phase1_kernel_keyboard_console_seen|phase1_grub_console_seen|blocked_after_seabios_multiboot_load|blocked_after_seabios_payload|blocked_after_chainload|blocked_after_multiboot_load|seabios_usb_not_booting|failed) : ;;
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

cat <<EOF

Prepared patch:
  $PATCH_PATH

Serve from X200:
  cd $SHARE_DIR && python3 -m http.server 8000

On the Mac, replace <X200_IP> with the X200 address, then run:
  cd ~/phase1_library/phase1
  git am --abort 2>/dev/null || true
  curl -fL -o /tmp/$PATCH_NAME http://<X200_IP>:8000/$PATCH_NAME
  git am --3way /tmp/$PATCH_NAME
  git pull --rebase origin edge/stable
  git push origin edge/stable

Stop the X200 server afterward with Ctrl+C.
EOF

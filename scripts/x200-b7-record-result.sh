#!/usr/bin/env sh
# Record the best observed X200 external USB boot result.
#
# Usage examples:
#   sh scripts/x200-b7-record-result.sh phase1_kernel_vga_console_seen
#   sh scripts/x200-b7-record-result.sh phase1_grub_console_seen
#   sh scripts/x200-b7-record-result.sh phase1_multiboot_kernel_seen
#   sh scripts/x200-b7-record-result.sh phase1_initramfs_shell
#   sh scripts/x200-b7-record-result.sh boot_started
#   sh scripts/x200-b7-record-result.sh blocked_after_multiboot_load
#   sh scripts/x200-b7-record-result.sh blocked_after_initrd_load
#
# This writes a small evidence file only. It does not write disks, boot
# anything, or make an installer/daily-driver claim.

set -eu

RESULT="${1:-}"
MACHINE="${BASE1_B7_MACHINE:-X200}"
ARTIFACT="${BASE1_B7_ARTIFACT:-build/phase1-uefi.img}"
OUT_DIR="${BASE1_B7_RESULT_OUT:-build/base1-b7-hardware-boot-evidence}"
REPORT="$OUT_DIR/b7-hardware-boot-evidence.env"

fail() { printf 'x200-b7-record-result: %s\n' "$1" >&2; exit 1; }

case "$RESULT" in
  phase1_kernel_vga_console_seen|phase1_grub_console_seen|phase1_multiboot_kernel_seen|phase1_initramfs_shell|boot_started|reset_after_linux16_handoff|blocked_after_multiboot_load|blocked_after_initrd_load|blocked_after_kernel_load|phase1_marker_seen|blocked|failed) : ;;
  *)
    cat <<'USAGE'
Usage:
  sh scripts/x200-b7-record-result.sh <result>

Allowed results:
  phase1_kernel_vga_console_seen
  phase1_grub_console_seen
  phase1_multiboot_kernel_seen
  phase1_initramfs_shell
  boot_started
  reset_after_linux16_handoff
  blocked_after_multiboot_load
  blocked_after_initrd_load
  blocked_after_kernel_load
  phase1_marker_seen
  blocked
  failed
USAGE
    fail "missing or unsupported result: ${RESULT:-empty}"
    ;;
esac

mkdir -p "$OUT_DIR"

cat > "$REPORT" <<EOF
BASE1_B7_HARDWARE_BOOT_MACHINE=$MACHINE
BASE1_B7_HARDWARE_BOOT_ARTIFACT=$ARTIFACT
BASE1_B7_HARDWARE_BOOT_RESULT=$RESULT
BASE1_B7_OPERATOR_CONFIRMED_MACHINE=$MACHINE
BASE1_B7_OPERATOR_CONFIRMED_RESULT=$RESULT
BASE1_B7_OPERATOR_CONFIRMED_SOURCE=physical_screen_observation
BASE1_B7_EVIDENCE_CLAIM=not_claimed
BASE1_B7_NON_CLAIM_INSTALLER=1
BASE1_B7_NON_CLAIM_INTERNAL_DISK_INSTALL=1
BASE1_B7_NON_CLAIM_RECOVERY_COMPLETE=1
BASE1_B7_NON_CLAIM_HARDENED=1
BASE1_B7_NON_CLAIM_DAILY_DRIVER=1
EOF

printf 'Recorded X200 external USB evidence result: %s\n' "$RESULT"
printf 'Evidence file: %s\n\n' "$REPORT"
cat "$REPORT"

printf '\nNext local commit command, if desired:\n'
printf '  git add -f %s && git commit -m "Record X200 %s"\n' "$REPORT" "$RESULT"

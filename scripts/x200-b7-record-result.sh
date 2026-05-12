#!/usr/bin/env sh
# Record the best observed X200 external USB boot result.
#
# Usage examples:
#   sh scripts/x200-b7-record-result.sh phase1_full_system_load_seen
#   sh scripts/x200-b7-record-result.sh phase1_real_splash_seen
#   sh scripts/x200-b7-record-result.sh reset_after_full_initrd_boot
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
  phase1_full_system_load_seen|phase1_real_splash_seen|phase1_initrd_phase1_hook_seen|phase1_initrd_break_shell_seen|reset_after_full_initrd_boot|phase1_linux_libre_full_seen|phase1_boot_splash_seen|blocked_after_huge_initrd_load|phase1_tiny_initramfs_seen|initrd16_command_returned|phase1_linux_libre_baseline_seen|linux16_command_returned|blocked_during_linux_libre_load|blocked_during_linux_load|linux_command_returned|initrd_command_returned|phase1_integrated_runtime_seen|phase1_crypto_evidence_seen|phase1_evidence_hash_manifest_seen|phase1_supervisor_plan_seen|phase1_runtime_workspace_seen|phase1_persistent_workspace_seen|phase1_gnulinux_shell_seen|phase1_gnulinux_initramfs_seen|blocked_after_gnulinux_load|phase1_polished_system_seen|phase1_splash_mode_seen|phase1_system_console_v2_seen|phase1_system_console_seen|phase1_seabios_multiboot_seen|blocked_after_seabios_multiboot_load|phase1_seabios_grub_seen|seabios_usb_not_booting|phase1_mbr_bootsector_seen|blocked_after_seabios_payload|phase1_bootsector_seen|blocked_after_chainload|phase1_keyboard_input_not_observed|phase1_kernel_keyboard_console_seen|phase1_kernel_framebuffer_seen|phase1_kernel_vga_console_seen|phase1_grub_console_seen|phase1_multiboot_kernel_seen|phase1_initramfs_shell|boot_started|reset_after_linux16_handoff|blocked_after_multiboot_load|blocked_after_initrd_load|blocked_after_kernel_load|phase1_marker_seen|blocked|failed) : ;;
  *)
    cat <<'USAGE'
Usage:
  sh scripts/x200-b7-record-result.sh <result>

Allowed results:
  phase1_full_system_load_seen
  phase1_real_splash_seen
  phase1_initrd_phase1_hook_seen
  phase1_initrd_break_shell_seen
  reset_after_full_initrd_boot
  phase1_linux_libre_full_seen
  phase1_boot_splash_seen
  blocked_after_huge_initrd_load
  phase1_tiny_initramfs_seen
  initrd16_command_returned
  phase1_linux_libre_baseline_seen
  linux16_command_returned
  blocked_during_linux_libre_load
  blocked_during_linux_load
  linux_command_returned
  initrd_command_returned
  phase1_integrated_runtime_seen
  phase1_crypto_evidence_seen
  phase1_evidence_hash_manifest_seen
  phase1_supervisor_plan_seen
  phase1_runtime_workspace_seen
  phase1_persistent_workspace_seen
  phase1_gnulinux_shell_seen
  phase1_gnulinux_initramfs_seen
  blocked_after_gnulinux_load
  phase1_polished_system_seen
  phase1_splash_mode_seen
  phase1_system_console_v2_seen
  phase1_system_console_seen
  phase1_seabios_multiboot_seen
  blocked_after_seabios_multiboot_load
  phase1_seabios_grub_seen
  seabios_usb_not_booting
  phase1_mbr_bootsector_seen
  blocked_after_seabios_payload
  phase1_bootsector_seen
  blocked_after_chainload
  phase1_keyboard_input_not_observed
  phase1_kernel_keyboard_console_seen
  phase1_kernel_framebuffer_seen
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

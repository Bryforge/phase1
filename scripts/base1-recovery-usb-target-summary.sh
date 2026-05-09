#!/usr/bin/env sh
set -eu

show_help() {
  cat <<'EOF'
Base1 recovery USB target selection summary

usage:
  sh scripts/base1-recovery-usb-target-summary.sh

This command is read-only. It prints the recovery USB target selection summary
without writing USB media, changing boot settings, writing to /boot, modifying
disks, changing firmware state, or changing host trust.
EOF
}

case "${1:-}" in
  -h|--help)
    show_help
    exit 0
    ;;
  "")
    ;;
  *)
    echo "error: unknown argument: $1" >&2
    show_help >&2
    exit 2
    ;;
esac

cat <<'EOF'
base1 recovery USB target selection summary
mode               : read-only
writes             : no
firmware           : Libreboot expected
hardware           : X200-class expected
bootloader         : GRUB first
media              : external USB planned
target_mode        : explicit device path only
trust              : no host trust escalation
maturity           : target identity previews, reports, validation bundle, and documentation

read first:
1. base1/RECOVERY_USB_TARGET_SUMMARY.md
2. base1/RECOVERY_USB_TARGET_SELECTION.md
3. base1/RECOVERY_USB_TARGET_COMMAND_INDEX.md
4. base1/RECOVERY_USB_HARDWARE_SUMMARY.md
5. base1/RECOVERY_USB_COMMAND_INDEX.md

first commands:
- sh scripts/base1-recovery-usb-target-summary.sh
- sh scripts/base1-recovery-usb-target-validate.sh
- sh scripts/base1-recovery-usb-target-dry-run.sh --dry-run --target /dev/example
- sh scripts/base1-recovery-usb-target-report.sh
- sh scripts/base1-recovery-usb-hardware-summary.sh

not claimed:
- USB media writing readiness
- bootable Base1 image readiness
- destructive installer readiness
- hidden target discovery safety
- real-hardware recovery completion
- real-hardware rollback completion
EOF

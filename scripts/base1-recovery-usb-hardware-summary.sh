#!/usr/bin/env sh
set -eu

show_help() {
  cat <<'EOF'
Base1 recovery USB hardware summary

usage:
  sh scripts/base1-recovery-usb-hardware-summary.sh

This command is read-only. It prints the recovery USB hardware validation
summary without writing USB media, changing boot settings, writing to /boot,
modifying disks, flashing firmware, or changing host trust.
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
base1 recovery USB hardware validation summary
mode       : read-only
writes     : no
firmware   : Libreboot expected
hardware   : X200-class expected
bootloader : GRUB first
media      : external USB planned
secureboot : not assumed
tpm        : not assumed
trust      : no host trust escalation
maturity   : docs, checklist, reports, and dry-runs

read first:
1. base1/RECOVERY_USB_DESIGN.md
2. base1/RECOVERY_USB_COMMAND_INDEX.md
3. base1/RECOVERY_USB_HARDWARE_CHECKLIST.md
4. base1/RECOVERY_USB_VALIDATION_REPORT.md
5. base1/RECOVERY_USB_HARDWARE_SUMMARY.md

first commands:
- sh scripts/base1-recovery-usb-hardware-summary.sh
- sh scripts/base1-recovery-usb-hardware-checklist.sh
- sh scripts/base1-recovery-usb-hardware-validate.sh
- sh scripts/base1-recovery-usb-hardware-report.sh
- sh scripts/base1-recovery-usb-validate.sh
- sh scripts/base1-recovery-usb-dry-run.sh --dry-run --target /dev/example

not claimed:
- USB media writing readiness
- bootable Base1 image readiness
- destructive installer readiness
- automatic GRUB repair
- real-hardware recovery completion
- real-hardware rollback completion
EOF

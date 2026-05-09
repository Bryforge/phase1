#!/usr/bin/env sh
set -eu

show_help() {
  cat <<'EOF'
Base1 recovery USB image provenance summary

usage:
  sh scripts/base1-recovery-usb-image-summary.sh

This command is read-only. It prints the recovery USB image provenance summary
without downloading images, writing USB media, changing boot settings, writing
to /boot, modifying disks, changing firmware state, or changing host trust.
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
base1 recovery USB image provenance summary
mode                 : read-only
writes               : no
downloads            : no
firmware             : Libreboot expected
hardware             : X200-class expected
bootloader           : GRUB first
media                : external USB planned
target_identity      : required before writing
checksum_rule        : exact match required before future writing
signature_status     : operator-confirmed
trust                : no host trust escalation
maturity             : provenance reports, validation bundle, summaries, and documentation

read first:
1. base1/RECOVERY_USB_IMAGE_SUMMARY.md
2. base1/RECOVERY_USB_IMAGE_PROVENANCE.md
3. base1/RECOVERY_USB_TARGET_SUMMARY.md
4. base1/RECOVERY_USB_TARGET_COMMAND_INDEX.md
5. base1/RECOVERY_USB_COMMAND_INDEX.md

first commands:
- sh scripts/base1-recovery-usb-image-summary.sh
- sh scripts/base1-recovery-usb-image-validate.sh
- sh scripts/base1-recovery-usb-image-report.sh
- sh scripts/base1-recovery-usb-target-summary.sh
- sh scripts/base1-recovery-usb-target-validate.sh

not claimed:
- image download readiness
- signature verification implementation
- USB media writing readiness
- bootable Base1 image readiness
- destructive installer readiness
- real-hardware recovery completion
- real-hardware rollback completion
EOF

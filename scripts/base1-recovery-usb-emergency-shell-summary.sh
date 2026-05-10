#!/usr/bin/env sh
set -eu

show_help() {
  cat <<'EOF'
Base1 recovery USB emergency shell summary

usage:
  sh scripts/base1-recovery-usb-emergency-shell-summary.sh

This command is read-only. It prints the recovery USB emergency shell behavior
summary without writing USB media, changing boot settings, writing to /boot,
modifying disks, changing firmware state, launching privileged shells, or
changing host trust.
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
base1 recovery USB emergency shell summary
mode                  : read-only
writes                : no
shell_launch          : no
firmware              : Libreboot expected
hardware              : X200-class expected
bootloader            : GRUB first
media                 : external USB planned
target_identity       : required before writing
image_provenance      : required before writing
root_boundary         : operator-visible
emergency_access      : must remain available
trust                 : no host trust escalation
maturity              : emergency shell reports, validation bundle, design, and documentation

read first:
1. base1/RECOVERY_USB_EMERGENCY_SHELL_SUMMARY.md
2. base1/RECOVERY_USB_EMERGENCY_SHELL.md
3. base1/RECOVERY_USB_IMAGE_SUMMARY.md
4. base1/RECOVERY_USB_IMAGE_COMMAND_INDEX.md
5. base1/RECOVERY_USB_COMMAND_INDEX.md

first commands:
- sh scripts/base1-recovery-usb-emergency-shell-summary.sh
- sh scripts/base1-recovery-usb-emergency-shell-validate.sh
- sh scripts/base1-recovery-usb-emergency-shell-report.sh
- sh scripts/base1-recovery-usb-image-summary.sh
- sh scripts/base1-recovery-dry-run.sh --dry-run

not claimed:
- emergency shell execution readiness
- privileged shell launch support
- USB media writing readiness
- bootable Base1 image readiness
- automatic recovery readiness
- real-hardware recovery completion
- real-hardware rollback completion
EOF

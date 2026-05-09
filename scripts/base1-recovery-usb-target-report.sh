#!/usr/bin/env sh
set -eu

show_help() {
  cat <<'EOF'
Base1 recovery USB target selection report

usage:
  sh scripts/base1-recovery-usb-target-report.sh

This command is read-only. It prints a target-device selection report template
without writing USB media, changing boot settings, writing to /boot, modifying
disks, flashing firmware, or changing host trust.
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
base1 recovery USB target selection report
mode               : read-only
writes             : no
firmware           : Libreboot expected
hardware           : X200-class expected
bootloader         : GRUB first
media              : external USB planned
trust              : no host trust escalation

target identity:
- device path: unknown
- device model/name: unknown
- device size: unknown
- removable status: unknown
- current mount status: unknown
- filesystem labels: unknown
- data preservation status: unknown
- internal disk status: unknown
- physical USB label status: unknown
- operator confirmation status: not accepted in report mode

required future confirmation phrase:
I understand this will write recovery USB media to the selected device

validation commands:
- sh scripts/base1-recovery-usb-target-dry-run.sh --dry-run --target /dev/example
- sh scripts/base1-recovery-usb-target-report.sh
- sh scripts/base1-recovery-usb-hardware-summary.sh
- sh scripts/base1-recovery-usb-hardware-validate.sh

status: target selection not completed
EOF

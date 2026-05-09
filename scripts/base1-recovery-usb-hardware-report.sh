#!/usr/bin/env sh
set -eu

show_help() {
  cat <<'EOF'
Base1 recovery USB hardware validation report

usage:
  sh scripts/base1-recovery-usb-hardware-report.sh

This command is read-only. It prints a hardware validation report template
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
base1 recovery USB hardware validation report
mode       : read-only
writes     : no
firmware   : Libreboot expected
hardware   : X200-class expected
bootloader : GRUB first
media      : external USB planned
secureboot : not assumed
tpm        : not assumed
trust      : no host trust escalation

hardware observations:
- GRUB menu reachable: unknown
- USB boot option visible: unknown
- external USB device labeled: unknown
- keyboard works in boot menu: unknown
- display readable in recovery mode: unknown
- emergency shell reachable: unknown
- normal boot path known: unknown
- recovery boot path known: unknown
- Phase1 state path known: unknown
- rollback metadata path known: unknown
- wireless limitations known: unknown
- clock drift risk known: unknown
- power and battery behavior known: unknown

validation commands:
- sh scripts/base1-recovery-usb-hardware-validate.sh
- sh scripts/base1-recovery-usb-hardware-checklist.sh
- sh scripts/base1-recovery-usb-validate.sh
- sh scripts/base1-recovery-usb-dry-run.sh --dry-run --target /dev/example

status: hardware validation not completed
EOF

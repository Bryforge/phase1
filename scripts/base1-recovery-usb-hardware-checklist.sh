#!/usr/bin/env sh
set -eu

show_help() {
  cat <<'EOF'
Base1 recovery USB hardware checklist

usage:
  sh scripts/base1-recovery-usb-hardware-checklist.sh

This command is read-only. It prints the recovery USB hardware validation
checklist without writing USB media, changing boot settings, writing to /boot,
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
base1 recovery USB hardware checklist
mode       : read-only
writes     : no
firmware   : Libreboot expected
hardware   : X200-class expected
bootloader : GRUB first
media      : external USB planned
secureboot : not assumed
tpm        : not assumed
trust      : no host trust escalation

confirm before hardware validation:
- target machine is Libreboot-backed X200-class
- USB device is physically identified
- USB device is not a data drive needing preservation
- GRUB menu is reachable
- fallback or emergency shell path is known
- normal boot path is known
- recovery boot path is known

record observations:
- GRUB menu reachable: unknown
- USB boot option visible: unknown
- external USB device labeled: unknown
- keyboard works in boot menu: unknown
- display readable in recovery mode: unknown
- emergency shell path known: unknown
- Phase1 state path known: unknown
- rollback metadata path known: unknown
- wireless limitations known: unknown
- clock drift risk known: unknown

status: checklist not completed on hardware
EOF

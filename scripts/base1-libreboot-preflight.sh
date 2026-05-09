#!/usr/bin/env sh
set -eu

show_help() {
  cat <<'EOF'
Base1 Libreboot preflight

usage:
  sh scripts/base1-libreboot-preflight.sh

This command is read-only. It does not flash firmware, change boot order,
install GRUB, write to /boot, write to disk, or modify host trust.
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
base1 libreboot preflight
firmware  : Libreboot expected
hardware  : X200-class expected
bootloader: GRUB first
secureboot: not required
tpm       : not required
recovery  : emergency shell required
usb       : recovery USB recommended
offline   : offline install path recommended
writes    : no
trust     : no host trust escalation
status    : preflight notes complete
EOF

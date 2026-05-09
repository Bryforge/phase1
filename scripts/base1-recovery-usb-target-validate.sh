#!/usr/bin/env sh
set -eu

show_help() {
  cat <<'EOF'
Base1 recovery USB target selection validation bundle

usage:
  sh scripts/base1-recovery-usb-target-validate.sh

This command is read-only. It runs target-device selection previews without
writing USB media, changing boot settings, writing to /boot, modifying disks,
changing firmware state, or changing host trust.
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

run_cmd() {
  echo
  echo "$ $*"
  "$@"
}

cat <<'EOF'
base1 recovery USB target selection validation bundle
mode               : read-only
writes             : no
firmware           : Libreboot expected
hardware           : X200-class expected
bootloader         : GRUB first
media              : external USB planned
target             : /dev/example preview
identity           : operator-confirmed
internal_disk      : must be no
confirmation       : not accepted in validation bundle
trust              : no host trust escalation
EOF

run_cmd sh scripts/base1-recovery-usb-target-dry-run.sh --dry-run --target /dev/example
run_cmd sh scripts/base1-recovery-usb-target-report.sh
run_cmd sh scripts/base1-recovery-usb-hardware-summary.sh
run_cmd sh scripts/base1-recovery-usb-hardware-validate.sh
run_cmd sh scripts/base1-recovery-usb-index.sh

echo
echo "base1 recovery USB target selection validation bundle complete"

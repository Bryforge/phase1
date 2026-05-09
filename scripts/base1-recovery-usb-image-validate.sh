#!/usr/bin/env sh
set -eu

show_help() {
  cat <<'EOF'
Base1 recovery USB image provenance validation bundle

usage:
  sh scripts/base1-recovery-usb-image-validate.sh

This command is read-only. It runs image provenance and checksum preview
commands without downloading images, writing USB media, changing boot settings,
writing to /boot, modifying disks, changing firmware state, or changing host trust.
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
base1 recovery USB image provenance validation bundle
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
EOF

run_cmd sh scripts/base1-recovery-usb-image-report.sh
run_cmd sh scripts/base1-recovery-usb-target-summary.sh
run_cmd sh scripts/base1-recovery-usb-target-validate.sh
run_cmd sh scripts/base1-recovery-usb-target-dry-run.sh --dry-run --target /dev/example
run_cmd sh scripts/base1-recovery-usb-hardware-summary.sh

echo
echo "base1 recovery USB image provenance validation bundle complete"

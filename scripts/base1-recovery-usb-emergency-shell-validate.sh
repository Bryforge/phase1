#!/usr/bin/env sh
set -eu

show_help() {
  cat <<'EOF'
Base1 recovery USB emergency shell validation bundle

usage:
  sh scripts/base1-recovery-usb-emergency-shell-validate.sh

This command is read-only. It runs emergency shell behavior preview commands
without writing USB media, changing boot settings, writing to /boot, modifying
disks, changing firmware state, launching privileged shells, or changing host trust.
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
base1 recovery USB emergency shell validation bundle
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
EOF

run_cmd sh scripts/base1-recovery-usb-emergency-shell-report.sh
run_cmd sh scripts/base1-recovery-usb-image-summary.sh
run_cmd sh scripts/base1-recovery-usb-image-validate.sh
run_cmd sh scripts/base1-recovery-usb-target-summary.sh
run_cmd sh scripts/base1-recovery-dry-run.sh --dry-run

echo
echo "base1 recovery USB emergency shell validation bundle complete"

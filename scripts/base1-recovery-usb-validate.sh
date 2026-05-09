#!/usr/bin/env sh
set -eu

show_help() {
  cat <<'EOF'
Base1 recovery USB validation bundle

usage:
  sh scripts/base1-recovery-usb-validate.sh

This command is read-only. It runs recovery USB planning previews without
writing USB media, changing boot settings, writing to /boot, modifying disks,
or changing host trust.
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
base1 recovery USB validation bundle
mode      : read-only
writes    : no
firmware  : Libreboot expected
hardware  : X200-class expected
bootloader: GRUB first
target    : /dev/example preview
trust     : no host trust escalation
EOF

run_cmd sh scripts/base1-recovery-usb-index.sh
run_cmd sh scripts/base1-recovery-usb-dry-run.sh --dry-run --target /dev/example
run_cmd sh scripts/base1-libreboot-validate.sh
run_cmd sh scripts/base1-grub-recovery-dry-run.sh --dry-run
run_cmd sh scripts/base1-recovery-dry-run.sh --dry-run
run_cmd sh scripts/base1-storage-layout-dry-run.sh --dry-run --target /dev/example

echo
echo "base1 recovery USB validation bundle complete"

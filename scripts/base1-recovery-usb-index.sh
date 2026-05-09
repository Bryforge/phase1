#!/usr/bin/env sh
set -eu

show_help() {
  cat <<'EOF'
Base1 recovery USB index

usage:
  sh scripts/base1-recovery-usb-index.sh

This command is read-only. It prints the recovery USB planning document and
command index without writing USB media, changing boot settings, writing to
/boot, modifying disks, or changing host trust.
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
base1 recovery USB command index
mode      : read-only
writes    : no
firmware  : Libreboot expected
hardware  : X200-class expected
bootloader: GRUB first
target    : explicit USB device required for dry-runs
trust     : no host trust escalation

documents:
- base1/RECOVERY_USB_DESIGN.md
- base1/RECOVERY_USB_COMMAND_INDEX.md
- base1/LIBREBOOT_DOCS_SUMMARY.md
- base1/LIBREBOOT_MILESTONE.md
- base1/LIBREBOOT_VALIDATION_REPORT.md

commands:
- sh scripts/base1-recovery-usb-index.sh
- sh scripts/base1-recovery-usb-dry-run.sh --dry-run --target /dev/example
- sh scripts/base1-libreboot-validate.sh
- sh scripts/base1-libreboot-report.sh
- sh scripts/base1-grub-recovery-dry-run.sh --dry-run
- sh scripts/base1-recovery-dry-run.sh --dry-run
- sh scripts/base1-storage-layout-dry-run.sh --dry-run --target /dev/example
EOF

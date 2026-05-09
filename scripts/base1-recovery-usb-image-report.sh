#!/usr/bin/env sh
set -eu

show_help() {
  cat <<'EOF'
Base1 recovery USB image provenance report

usage:
  sh scripts/base1-recovery-usb-image-report.sh

This command is read-only. It prints an image provenance and checksum report
template without downloading images, writing USB media, changing boot settings,
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

cat <<'EOF'
base1 recovery USB image provenance report
mode                 : read-only
writes               : no
firmware             : Libreboot expected
hardware             : X200-class expected
bootloader           : GRUB first
media                : external USB planned
target_identity      : required before writing
trust                : no host trust escalation

image provenance:
- image filename: unknown
- image source URL or local path: unknown
- image build commit: unknown
- image build date: unknown
- image builder identity: unknown
- expected SHA256 checksum: unknown
- observed SHA256 checksum: unknown
- checksum match: unknown
- signature status: unknown
- signing key identity: unknown
- target hardware profile: unknown
- target bootloader expectation: GRUB first
- recovery shell availability: unknown
- rollback metadata compatibility: unknown

verification rule:
future media writing must refuse when checksum data is missing or mismatched

validation commands:
- sh scripts/base1-recovery-usb-image-report.sh
- sh scripts/base1-recovery-usb-target-summary.sh
- sh scripts/base1-recovery-usb-target-validate.sh
- sh scripts/base1-recovery-usb-target-dry-run.sh --dry-run --target /dev/example

status: image provenance not verified
EOF

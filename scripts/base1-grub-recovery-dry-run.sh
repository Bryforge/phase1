#!/usr/bin/env sh
set -eu

show_help() {
  cat <<'EOF'
Base1 GRUB recovery dry-run

usage:
  sh scripts/base1-grub-recovery-dry-run.sh --dry-run

This command is read-only. It does not edit GRUB config, write to /boot,
change boot order, install bootloader files, flash firmware, or modify disks.
EOF
}

dry_run=0

while [ "$#" -gt 0 ]; do
  case "$1" in
    --dry-run)
      dry_run=1
      shift
      ;;
    -h|--help)
      show_help
      exit 0
      ;;
    *)
      echo "error: unknown argument: $1" >&2
      show_help >&2
      exit 2
      ;;
  esac
done

if [ "$dry_run" -ne 1 ]; then
  echo "refusing: --dry-run is required" >&2
  exit 2
fi

cat <<'EOF'
base1 grub recovery dry-run
firmware    : Libreboot expected
hardware    : X200-class expected
bootloader  : GRUB first
writes      : no
boot_order  : no change
boot_config : grub.cfg preview only
boot_path   : /boot preview only
emergency   : shell access required
recovery_usb: recommended
phase1_auto : no change
rollback    : metadata preview only
trust       : no host trust escalation
status      : GRUB recovery dry-run complete
EOF

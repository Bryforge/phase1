#!/usr/bin/env sh
set -eu

show_help() {
  cat <<'EOF'
Base1 recovery USB dry-run

usage:
  sh scripts/base1-recovery-usb-dry-run.sh --dry-run --target <usb-device>

This command is read-only. It previews recovery USB planning and does not
write USB media, partition disks, format disks, install GRUB, edit grub.cfg,
write to /boot, change boot order, flash firmware, or modify host trust.
EOF
}

dry_run=0
target=""

while [ "$#" -gt 0 ]; do
  case "$1" in
    --dry-run)
      dry_run=1
      shift
      ;;
    --target)
      if [ "$#" -lt 2 ]; then
        echo "error: --target requires a value" >&2
        exit 2
      fi
      target="$2"
      shift 2
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

if [ -z "$target" ]; then
  echo "error: --target <usb-device> is required" >&2
  exit 2
fi

case "$target" in
  /dev/*|/dev/disk/*)
    ;;
  *)
    echo "error: target must look like a device path under /dev" >&2
    exit 2
    ;;
esac

cat <<EOF
base1 recovery USB dry-run
target       : $target
writes       : no
firmware     : Libreboot expected
hardware     : X200-class expected
bootloader   : GRUB first
usb_media    : preview only
image        : not created
boot_order   : no change
emergency    : shell path preview
phase1_state : /state/phase1 preview
rollback     : metadata preview only
trust        : no host trust escalation
status       : recovery USB dry-run complete
EOF

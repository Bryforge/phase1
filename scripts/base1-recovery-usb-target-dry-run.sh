#!/usr/bin/env sh
set -eu

show_help() {
  cat <<'EOF'
Base1 recovery USB target selection dry-run

usage:
  sh scripts/base1-recovery-usb-target-dry-run.sh --dry-run --target <usb-device>

This command is read-only. It previews target-device identity checks without
writing USB media, changing boot settings, writing to /boot, modifying disks,
flashing firmware, or changing host trust.
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
base1 recovery USB target selection dry-run
target             : $target
writes             : no
mode               : read-only
firmware           : Libreboot expected
hardware           : X200-class expected
bootloader         : GRUB first
usb_media          : target identity preview only
device_path        : visible
device_model       : operator-confirmed
device_size        : operator-confirmed
removable_status   : operator-confirmed
current_state      : operator-confirmed
filesystem_labels  : operator-confirmed
data_preservation  : operator-confirmed
internal_disk      : must be no
physical_label     : operator-confirmed
confirmation       : not accepted in dry-run
trust              : no host trust escalation
status             : target selection dry-run complete
EOF

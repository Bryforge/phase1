#!/usr/bin/env sh
set -eu

show_help() {
  cat <<'EOF'
Base1 installer dry-run

usage:
  sh scripts/base1-install-dry-run.sh --dry-run --target <disk>

This command is preview-only. It does not partition, format, mount, install
bootloaders, or change host trust.
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
  echo "error: --target <disk> is required" >&2
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
base1 installer dry-run
target   : $target
writes   : no
hardware : preflight preview only
boot     : preview only
base1    : read-only layer preview
state    : writable phase1 state preview
recovery : emergency shell preview
rollback : metadata preview only
trust    : no host trust escalation
status   : dry-run complete
EOF

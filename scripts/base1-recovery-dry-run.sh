#!/usr/bin/env sh
set -eu

show_help() {
  cat <<'EOF'
Base1 recovery dry-run

usage:
  sh scripts/base1-recovery-dry-run.sh --dry-run
  sh scripts/base1-recovery-dry-run.sh --dry-run --target <disk>

This command is preview-only. It does not attach disks, change boot settings,
disable Phase1 auto-launch, export data, or modify host trust.
EOF
}

dry_run=0
target="host-default"

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

cat <<EOF
base1 recovery dry-run
target      : $target
writes      : no
boot        : recovery preview only
auto-launch : no change
shell       : emergency fallback preview
state       : /state/phase1 export preview only
rollback    : metadata preview only
trust       : no host trust escalation
status      : recovery dry-run complete
EOF

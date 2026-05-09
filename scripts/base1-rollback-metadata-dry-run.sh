#!/usr/bin/env sh
set -eu

show_help() {
  cat <<'EOF'
Base1 rollback metadata dry-run

usage:
  sh scripts/base1-rollback-metadata-dry-run.sh --dry-run
  sh scripts/base1-rollback-metadata-dry-run.sh --dry-run --target <disk>

This command is preview-only. It does not write rollback files, change boot
settings, export data, attach writable filesystems, or modify host trust.
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
base1 rollback metadata dry-run
target             : $target
writes             : no
base1_version      : foundation preview
phase1_version     : v5.0.0 preview
stable_version     : v4.4.0 preview
previous_stable    : v4.3.0 preview
boot_before        : host default preview
boot_after         : phase1 preview
state_path         : /state/phase1 preview
recovery_path      : /recovery preview
operator_confirmed : no
trust              : no host trust escalation
status             : rollback metadata dry-run complete
EOF

#!/usr/bin/env sh
set -eu

show_help() {
  cat <<'EOF'
Base1 Libreboot validation bundle

usage:
  sh scripts/base1-libreboot-validate.sh

This command is read-only. It runs the Libreboot and Base1 preview commands
that report firmware, GRUB-first recovery, storage, rollback, and installer
readiness without changing firmware, boot order, /boot, disks, or host trust.
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
base1 libreboot validation bundle
firmware : Libreboot expected
hardware : X200-class expected
boot     : GRUB first
mode     : read-only
writes   : no
trust    : no host trust escalation
EOF

run_cmd sh scripts/base1-libreboot-index.sh
run_cmd sh scripts/base1-libreboot-checklist.sh
run_cmd sh scripts/base1-libreboot-preflight.sh
run_cmd sh scripts/base1-grub-recovery-dry-run.sh --dry-run
run_cmd sh scripts/base1-recovery-dry-run.sh --dry-run
run_cmd sh scripts/base1-storage-layout-dry-run.sh --dry-run --target /dev/example
run_cmd sh scripts/base1-rollback-metadata-dry-run.sh --dry-run
run_cmd sh scripts/base1-install-dry-run.sh --dry-run --target /dev/example

echo
echo "base1 libreboot validation bundle complete"

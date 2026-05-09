#!/usr/bin/env sh
set -eu

show_help() {
  cat <<'EOF'
Base1 Libreboot index

usage:
  sh scripts/base1-libreboot-index.sh

This command is read-only. It prints the Libreboot and GRUB-first Base1
document/script index and does not change firmware, boot order, /boot, disks,
or host trust.
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
base1 libreboot command index
firmware : Libreboot
hardware : X200-class
boot     : GRUB first
writes   : no
trust    : no host trust escalation

documents:
- base1/LIBREBOOT_PROFILE.md
- base1/LIBREBOOT_PREFLIGHT.md
- base1/LIBREBOOT_GRUB_RECOVERY.md
- base1/LIBREBOOT_OPERATOR_CHECKLIST.md
- base1/LIBREBOOT_COMMAND_INDEX.md

read-only scripts:
- sh scripts/base1-libreboot-index.sh
- sh scripts/base1-libreboot-checklist.sh
- sh scripts/base1-libreboot-preflight.sh
- sh scripts/base1-grub-recovery-dry-run.sh --dry-run
- sh scripts/base1-recovery-dry-run.sh --dry-run
- sh scripts/base1-storage-layout-dry-run.sh --dry-run --target /dev/example
- sh scripts/base1-rollback-metadata-dry-run.sh --dry-run
- sh scripts/base1-install-dry-run.sh --dry-run --target /dev/example
EOF

#!/usr/bin/env sh
set -eu

show_help() {
  cat <<'EOF'
Base1 Libreboot docs

usage:
  sh scripts/base1-libreboot-docs.sh

This command is read-only. It prints the Libreboot GRUB-first documentation
path and does not change firmware, boot order, /boot, disks, or host trust.
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
base1 libreboot docs
firmware : Libreboot
hardware : X200-class
boot     : GRUB first
mode     : read-only
writes   : no
trust    : no host trust escalation

read first:
1. base1/LIBREBOOT_DOCS_SUMMARY.md
2. base1/LIBREBOOT_QUICKSTART.md
3. base1/LIBREBOOT_COMMAND_INDEX.md
4. base1/LIBREBOOT_PROFILE.md
5. base1/LIBREBOOT_PREFLIGHT.md
6. base1/LIBREBOOT_GRUB_RECOVERY.md
7. base1/LIBREBOOT_OPERATOR_CHECKLIST.md
8. base1/LIBREBOOT_VALIDATION_REPORT.md

first commands:
- sh scripts/base1-libreboot-index.sh
- sh scripts/base1-libreboot-checklist.sh
- sh scripts/base1-libreboot-preflight.sh
- sh scripts/base1-grub-recovery-dry-run.sh --dry-run
- sh scripts/base1-libreboot-validate.sh
- sh scripts/base1-libreboot-report.sh
EOF

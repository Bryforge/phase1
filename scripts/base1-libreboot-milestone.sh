#!/usr/bin/env sh
set -eu

show_help() {
  cat <<'EOF'
Base1 Libreboot milestone

usage:
  sh scripts/base1-libreboot-milestone.sh

This command is read-only. It prints the Libreboot GRUB-first milestone
checkpoint and does not change firmware, boot order, /boot, disks, or host trust.
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
base1 libreboot milestone checkpoint
firmware profile : Libreboot documented
hardware profile : X200-class documented
bootloader       : GRUB first documented
secureboot       : not assumed
tpm              : not assumed
recovery media   : external USB recommended
emergency shell  : required
phase1 posture   : safe mode default
maturity         : documentation and read-only scripts
writes           : no
trust            : no host trust escalation

completed surfaces:
- base1/LIBREBOOT_DOCS_SUMMARY.md
- base1/LIBREBOOT_QUICKSTART.md
- base1/LIBREBOOT_COMMAND_INDEX.md
- base1/LIBREBOOT_MILESTONE.md
- sh scripts/base1-libreboot-docs.sh
- sh scripts/base1-libreboot-index.sh
- sh scripts/base1-libreboot-checklist.sh
- sh scripts/base1-libreboot-preflight.sh
- sh scripts/base1-grub-recovery-dry-run.sh --dry-run
- sh scripts/base1-libreboot-validate.sh
- sh scripts/base1-libreboot-report.sh

not claimed:
- bootable Base1 image readiness
- daily-driver readiness
- destructive installer readiness
- automatic GRUB repair
- hardware recovery validation
- rollback validation on real hardware
EOF

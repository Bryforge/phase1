#!/usr/bin/env sh
set -eu

show_help() {
  cat <<'EOF'
Base1 Libreboot checklist

usage:
  sh scripts/base1-libreboot-checklist.sh

This command is read-only. It prints the Libreboot GRUB-first operator
checklist and does not change firmware, boot order, /boot, disks, or host trust.
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
base1 libreboot operator checklist
firmware       : Libreboot
hardware       : X200-class
bootloader     : GRUB first
secureboot     : not assumed
tpm            : not assumed
recovery_media : external USB recommended
emergency      : shell access required
phase1_posture : safe mode default
writes         : no
trust          : no host trust escalation

required dry-runs:
- sh scripts/base1-preflight.sh
- sh scripts/base1-libreboot-preflight.sh
- sh scripts/base1-grub-recovery-dry-run.sh --dry-run
- sh scripts/base1-recovery-dry-run.sh --dry-run
- sh scripts/base1-storage-layout-dry-run.sh --dry-run --target /dev/example
- sh scripts/base1-rollback-metadata-dry-run.sh --dry-run
- sh scripts/base1-install-dry-run.sh --dry-run --target /dev/example

ready marker:
operator can explain normal boot, recovery boot, Phase1 launch, rollback, and emergency shell paths
EOF

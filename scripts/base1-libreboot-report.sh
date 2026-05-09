#!/usr/bin/env sh
set -eu

show_help() {
  cat <<'EOF'
Base1 Libreboot validation report

usage:
  sh scripts/base1-libreboot-report.sh

This command is read-only. It prints a validation report template for
Libreboot-backed, GRUB-first Base1 systems and does not change firmware,
boot order, /boot, disks, or host trust.
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
base1 libreboot validation report
firmware profile     : Libreboot
hardware profile     : X200-class
bootloader expected  : GRUB first
secure boot          : not assumed
tpm                  : not assumed
recovery media       : external USB recommended
emergency shell      : required
phase1 posture       : safe mode default
writes               : no
trust                : no host trust escalation

validation commands:
- sh scripts/base1-libreboot-validate.sh
- sh scripts/base1-libreboot-index.sh
- sh scripts/base1-libreboot-checklist.sh
- sh scripts/base1-libreboot-preflight.sh
- sh scripts/base1-grub-recovery-dry-run.sh --dry-run

operator confirmations:
- GRUB menu reachable: unknown
- normal boot path known: unknown
- recovery boot path known: unknown
- emergency shell reachable: unknown
- Phase1 state path known: unknown
- rollback metadata path known: unknown

status: not validated yet
EOF

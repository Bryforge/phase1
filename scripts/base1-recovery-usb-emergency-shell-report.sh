#!/usr/bin/env sh
set -eu

show_help() {
  cat <<'EOF'
Base1 recovery USB emergency shell report

usage:
  sh scripts/base1-recovery-usb-emergency-shell-report.sh

This command is read-only. It prints an emergency shell behavior report
template without writing USB media, changing boot settings, writing to /boot,
modifying disks, changing firmware state, launching privileged shells, or
changing host trust.
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
base1 recovery USB emergency shell report
mode                  : read-only
writes                : no
firmware              : Libreboot expected
hardware              : X200-class expected
bootloader            : GRUB first
media                 : external USB planned
target_identity       : required before writing
image_provenance      : required before writing
trust                 : no host trust escalation

emergency shell behavior:
- emergency shell entry path: unknown
- keyboard availability: unknown
- display readability: unknown
- root/admin boundary: operator-visible
- Phase1 auto-launch status: unknown
- Phase1 state path: /state/phase1 preview
- rollback metadata path: /recovery preview
- recovery media boot path: unknown
- network availability: unknown
- offline recovery capability: unknown
- log collection path: unknown
- exit/reboot path: unknown

required behavior:
- emergency shell access must remain available
- root/admin boundary must remain visible
- automatic recovery must not hide emergency access

validation commands:
- sh scripts/base1-recovery-usb-emergency-shell-report.sh
- sh scripts/base1-recovery-usb-image-summary.sh
- sh scripts/base1-recovery-usb-image-validate.sh
- sh scripts/base1-recovery-dry-run.sh --dry-run

status: emergency shell behavior not verified
EOF

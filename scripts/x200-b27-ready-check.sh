#!/usr/bin/env sh
# Phase1 / Base1 X200 B27 readiness check.
#
# Purpose:
#   Verify that the repo is ready for a single worthwhile B27 physical test.
#   This checks tools, kernel/initrd artifacts, profiles, scripts, and privacy
#   hygiene before writing the USB.

set -eu

KERNEL="${BASE1_B27_KERNEL:-build/linux/alpine-netboot/vmlinuz}"
INITRD="${BASE1_B27_INITRD:-build/linux/alpine-netboot/initrd.img}"
OUT_DIR="${BASE1_B27_READY_OUT:-build/base1-b27-integrated-runtime}"
REPORT="$OUT_DIR/b27-ready-check.env"
MISSING=""
FAILURES=""

mkdir -p "$OUT_DIR"

note_missing() {
  printf 'missing: %s\n' "$1"
  MISSING="$MISSING $1"
}

note_fail() {
  printf 'fail: %s\n' "$1"
  FAILURES="$FAILURES $1"
}

printf 'PHASE1 B27 READY CHECK\n\n'

printf 'command check:\n'
for cmd in sh git sudo parted mkfs.vfat mount umount sync mkdir cp tee grub-install sha256sum find cpio gzip chmod date grep sed awk; do
  if command -v "$cmd" >/dev/null 2>&1; then
    printf 'ok: %s\n' "$cmd"
  else
    note_missing "$cmd"
  fi
done

printf '\nscript/profile check:\n'
for path in \
  scripts/x200-b23-preflight.sh \
  scripts/x200-b23-stage-host-gnulinux.sh \
  scripts/x200-b27-integrated-runtime-usb.sh \
  scripts/x200-record-and-share-result-safe.sh \
  scripts/x200-b7-record-result.sh \
  scripts/base1-b26-crypto-evidence.sh \
  scripts/base1-b25-supervisor-plan.sh \
  profiles/base1/x200-supervisor-concurrent-lab.env \
  docs/os/B27_INTEGRATED_RUNTIME.md
 do
  if [ -f "$path" ]; then
    printf 'ok: %s\n' "$path"
  else
    note_fail "$path"
  fi
done

printf '\nartifact check:\n'
KERNEL_PRESENT=no
INITRD_PRESENT=no
[ -f "$KERNEL" ] && KERNEL_PRESENT=yes
[ -f "$INITRD" ] && INITRD_PRESENT=yes
printf 'kernel_present=%s %s\n' "$KERNEL_PRESENT" "$KERNEL"
printf 'initrd_present=%s %s\n' "$INITRD_PRESENT" "$INITRD"
if [ "$KERNEL_PRESENT" = yes ]; then sha256sum "$KERNEL"; fi
if [ "$INITRD_PRESENT" = yes ]; then sha256sum "$INITRD"; fi
if [ "$KERNEL_PRESENT" != yes ]; then note_fail "kernel-artifact"; fi
if [ "$INITRD_PRESENT" != yes ]; then note_fail "initrd-artifact"; fi

printf '\nprivacy check:\n'
if grep -R "192\.168\." scripts docs profiles 2>/dev/null; then
  note_fail "private-lan-ip-reference"
else
  printf 'ok: no 192.168.* references in scripts/docs/profiles\n'
fi

printf '\nsyntax check:\n'
for shfile in \
  scripts/x200-b23-preflight.sh \
  scripts/x200-b23-stage-host-gnulinux.sh \
  scripts/x200-b27-integrated-runtime-usb.sh \
  scripts/x200-record-and-share-result-safe.sh \
  scripts/x200-record-and-share-result.sh \
  scripts/x200-b7-record-result.sh \
  scripts/base1-b26-crypto-evidence.sh \
  scripts/base1-b25-supervisor-plan.sh
 do
  if [ -f "$shfile" ]; then
    if sh -n "$shfile"; then
      printf 'ok: sh -n %s\n' "$shfile"
    else
      note_fail "syntax:$shfile"
    fi
  fi
done

cat > "$REPORT" <<EOF
BASE1_B27_READY_KERNEL=$KERNEL
BASE1_B27_READY_INITRD=$INITRD
BASE1_B27_READY_KERNEL_PRESENT=$KERNEL_PRESENT
BASE1_B27_READY_INITRD_PRESENT=$INITRD_PRESENT
BASE1_B27_READY_MISSING_COMMANDS=$MISSING
BASE1_B27_READY_FAILURES=$FAILURES
BASE1_B27_READY_RESULT=$([ -z "$MISSING" ] && [ -z "$FAILURES" ] && printf pass || printf blocked)
EOF

printf '\nreport: %s\n' "$REPORT"
cat "$REPORT"

if [ -n "$MISSING" ] || [ -n "$FAILURES" ]; then
  printf '\nready-check: blocked\n'
  printf 'If only kernel/initrd are missing, run:\n'
  printf '  sh scripts/x200-b23-stage-host-gnulinux.sh\n'
  printf '  sh scripts/x200-b27-ready-check.sh\n'
  exit 1
fi

printf '\nready-check: pass\n'
printf 'next USB command:\n'
printf '  sh scripts/x200-b27-integrated-runtime-usb.sh /dev/sdb YES_WRITE_USB\n'

#!/usr/bin/env sh
# Base1 B3 GNU/Linux stage wrapper.
#
# This uses an existing local GNU/Linux kernel/initrd pair as a staging point for
# the B3 kernel/initrd handoff. It does not download a distribution, install
# Base1, modify host boot settings, partition disks, or claim hardware readiness.

set -eu

ROOT_DIR=${BASE1_GNULINUX_STAGE_ROOT:-}
BOOT_DIR=${BASE1_GNULINUX_STAGE_BOOT:-}
KERNEL=${BASE1_PREVIEW_KERNEL:-}
INITRD=${BASE1_PREVIEW_INITRD:-}
OUT_DIR=${BASE1_B3_GNULINUX_STAGE_OUT:-build/base1-b3-gnulinux-stage}
MODE=prepare
TIMEOUT_SECONDS=${BASE1_B3_TIMEOUT:-30}
EXPECT=${BASE1_B3_GNULINUX_MARKER:-Linux version}
BOOT_PROFILE=${BASE1_QEMU_BOOT_PROFILE:-hardened}
EXTRA_APPEND=${BASE1_QEMU_EXTRA_APPEND:-}

usage() {
  cat <<'USAGE'
base1 B3 GNU/Linux stage
module signature: base1 B3 GNU/Linux stage

usage:
  sh scripts/base1-b3-gnulinux-stage.sh [--root <dir>|--boot <dir>|--kernel <path> --initrd <path>] [--prepare|--dry-run|--check]

options:
  --root <dir>         local GNU/Linux root directory; uses <root>/boot
  --boot <dir>         local GNU/Linux boot directory containing vmlinuz/initrd
  --kernel <path>      explicit local GNU/Linux kernel image
  --initrd <path>      explicit local GNU/Linux initrd/initramfs image
  --out <build/dir>    output bundle directory, default: build/base1-b3-gnulinux-stage
  --prepare            stage bundle only, default
  --dry-run            stage bundle and print guarded QEMU handoff plan
  --check              stage bundle and run guarded QEMU serial-marker check
  --timeout <seconds>  check timeout, default: 30
  --expect <text>      expected serial marker, default: Linux version
  --boot-profile <p>   QEMU boot profile, default: hardened; allowed: standard, hardened
  --append <text>      extra kernel command-line text for the checker
  -h, --help           show this help

stage model:
  This uses GNU/Linux as a known boot payload staging point. It does not build,
  download, install, or trust a GNU/Linux distribution by itself. It only stages
  local kernel/initrd files into the B3 handoff pipeline.

marker model:
  The default GNU/Linux stage marker is "Linux version" because stock Linux
  kernels print that string very early during boot. A later Phase1-specific
  initrd can override this with --expect "phase1 6.0.0 ready".
  This marker only confirms early allocator initialization and kernel output
  are visible to the serial capture path.

hardened profile:
  The GNU/Linux stage defaults to the hardened QEMU boot profile. This requests
  hardening-oriented kernel settings through the shared QEMU checker, including
  lockdown and debugfs disablement where supported by the staged payload.
  This is a requested boot profile, not proof, and does not prove that the
  resulting system is hardened.

non-claims:
  This is emulator-only staging evidence. It does not make Base1 a GNU/Linux
  distribution, install Base1, validate physical hardware, validate an installer,
  validate recovery, prove hardening, or prove daily-driver readiness.
USAGE
}

fail() {
  printf 'base1-b3-gnulinux-stage: %s\n' "$1" >&2
  exit 1
}

require_build_out_dir() {
  case "$1" in
    build/*) return 0 ;;
    *) return 1 ;;
  esac
}

valid_boot_profile() {
  case "$1" in
    standard|hardened) return 0 ;;
    *) return 1 ;;
  esac
}

pick_first_existing() {
  for candidate in "$@"; do
    if [ -f "$candidate" ]; then
      printf '%s\n' "$candidate"
      return 0
    fi
  done
  return 1
}

detect_kernel() {
  dir=$1
  pick_first_existing \
    "$dir"/vmlinuz \
    "$dir"/vmlinuz-* \
    "$dir"/bzImage \
    "$dir"/kernel \
    "$dir"/Image
}

detect_initrd() {
  dir=$1
  pick_first_existing \
    "$dir"/initrd.img \
    "$dir"/initrd.img-* \
    "$dir"/initramfs.img \
    "$dir"/initramfs-* \
    "$dir"/initrd \
    "$dir"/initramfs
}

while [ "$#" -gt 0 ]; do
  case "$1" in
    --root)
      [ "$#" -ge 2 ] || fail '--root requires a value'
      ROOT_DIR=$2
      shift 2
      ;;
    --boot)
      [ "$#" -ge 2 ] || fail '--boot requires a value'
      BOOT_DIR=$2
      shift 2
      ;;
    --kernel)
      [ "$#" -ge 2 ] || fail '--kernel requires a value'
      KERNEL=$2
      shift 2
      ;;
    --initrd)
      [ "$#" -ge 2 ] || fail '--initrd requires a value'
      INITRD=$2
      shift 2
      ;;
    --out)
      [ "$#" -ge 2 ] || fail '--out requires a value'
      OUT_DIR=$2
      shift 2
      ;;
    --prepare)
      MODE=prepare
      shift
      ;;
    --dry-run)
      MODE=dry-run
      shift
      ;;
    --check)
      MODE=check
      shift
      ;;
    --timeout)
      [ "$#" -ge 2 ] || fail '--timeout requires a value'
      TIMEOUT_SECONDS=$2
      shift 2
      ;;
    --expect)
      [ "$#" -ge 2 ] || fail '--expect requires a value'
      EXPECT=$2
      shift 2
      ;;
    --boot-profile)
      [ "$#" -ge 2 ] || fail '--boot-profile requires a value'
      BOOT_PROFILE=$2
      shift 2
      ;;
    --append)
      [ "$#" -ge 2 ] || fail '--append requires a value'
      EXTRA_APPEND=$2
      shift 2
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      usage >&2
      fail "unknown option: $1"
      ;;
  esac
done

case "$TIMEOUT_SECONDS" in
  ''|*[!0-9]*) fail '--timeout must be a positive integer' ;;
esac
[ "$TIMEOUT_SECONDS" -gt 0 ] || fail '--timeout must be greater than zero'

valid_boot_profile "$BOOT_PROFILE" || fail "unsupported boot profile: $BOOT_PROFILE"
require_build_out_dir "$OUT_DIR" || fail "output directory must be under build/: $OUT_DIR"

if [ -z "$BOOT_DIR" ] && [ -n "$ROOT_DIR" ]; then
  BOOT_DIR=$ROOT_DIR/boot
fi

if [ -z "$KERNEL" ] || [ -z "$INITRD" ]; then
  [ -n "$BOOT_DIR" ] || fail 'provide --kernel and --initrd, or provide --root/--boot for detection'
  [ -d "$BOOT_DIR" ] || fail "boot directory not found: $BOOT_DIR"
  if [ -z "$KERNEL" ]; then
    KERNEL=$(detect_kernel "$BOOT_DIR" || true)
  fi
  if [ -z "$INITRD" ]; then
    INITRD=$(detect_initrd "$BOOT_DIR" || true)
  fi
fi

[ -n "$KERNEL" ] || fail "could not detect GNU/Linux kernel in: $BOOT_DIR"
[ -n "$INITRD" ] || fail "could not detect GNU/Linux initrd/initramfs in: $BOOT_DIR"
[ -f "$KERNEL" ] || fail "kernel not found: $KERNEL"
[ -f "$INITRD" ] || fail "initrd not found: $INITRD"
[ -f scripts/base1-b3-kernel-handoff.sh ] || fail 'missing scripts/base1-b3-kernel-handoff.sh'

printf 'BASE1 B3 GNU/LINUX STAGE\n'
printf 'mode        : %s\n' "$MODE"
printf 'out         : %s\n' "$OUT_DIR"
printf 'boot        : %s\n' "${BOOT_DIR:-explicit}"
printf 'kernel      : %s\n' "$KERNEL"
printf 'initrd      : %s\n' "$INITRD"
printf 'expect      : %s\n' "$EXPECT"
printf 'boot_profile: %s\n' "$BOOT_PROFILE"
printf 'stage       : GNU/Linux local kernel/initrd handoff\n'
printf '\n'

case "$MODE" in
  prepare)
    sh scripts/base1-b3-kernel-handoff.sh \
      --out "$OUT_DIR" \
      --kernel "$KERNEL" \
      --initrd "$INITRD" \
      --prepare \
      --timeout "$TIMEOUT_SECONDS" \
      --expect "$EXPECT" \
      --boot-profile "$BOOT_PROFILE" \
      --append "$EXTRA_APPEND"
    ;;
  dry-run)
    sh scripts/base1-b3-kernel-handoff.sh \
      --out "$OUT_DIR" \
      --kernel "$KERNEL" \
      --initrd "$INITRD" \
      --dry-run \
      --timeout "$TIMEOUT_SECONDS" \
      --expect "$EXPECT" \
      --boot-profile "$BOOT_PROFILE" \
      --append "$EXTRA_APPEND"
    ;;
  check)
    sh scripts/base1-b3-kernel-handoff.sh \
      --out "$OUT_DIR" \
      --kernel "$KERNEL" \
      --initrd "$INITRD" \
      --check \
      --timeout "$TIMEOUT_SECONDS" \
      --expect "$EXPECT" \
      --boot-profile "$BOOT_PROFILE" \
      --append "$EXTRA_APPEND"
    ;;
  *)
    fail "internal unsupported mode: $MODE"
    ;;
esac

printf 'stage_result: %s\n' "$MODE"
printf 'boot_profile: %s\n' "$BOOT_PROFILE"
printf 'non_claims: emulator-only GNU/Linux stage; hardened profile is request-only; no installer; no hardware validation; no hardening proof; no daily-driver claim\n'

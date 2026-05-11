#!/usr/bin/env sh
# Base1 B3 kernel/initrd handoff wrapper.
#
# This ties the existing emulator preview bundle generator to the guarded QEMU
# boot checker. It does not create a kernel, install Base1, write host boot
# settings, partition disks, validate physical hardware, or claim
# hardware/installer readiness.

set -eu

OUT_DIR=${BASE1_B3_HANDOFF_OUT:-build/base1-b3-kernel-handoff}
PROFILE=${BASE1_PROFILE:-x86_64-vm-validation}
TARGET=${BASE1_HARDWARE_TARGET:-emulator-x86_64}
IMAGE_MB=${BASE1_EMULATOR_IMAGE_MB:-128}
KERNEL=${BASE1_PREVIEW_KERNEL:-}
INITRD=${BASE1_PREVIEW_INITRD:-}
MODE=prepare
TIMEOUT_SECONDS=${BASE1_B3_TIMEOUT:-30}
EXPECT=${BASE1_B3_MARKER:-phase1 6.0.0 ready}

usage() {
  cat <<'USAGE'
base1 B3 kernel/initrd handoff

usage:
  sh scripts/base1-b3-kernel-handoff.sh --kernel <path> --initrd <path> [--prepare|--dry-run|--check]

options:
  --kernel <path>     local kernel image to stage as staging/boot/vmlinuz
  --initrd <path>     local initrd image to stage as staging/boot/initrd.img
  --out <build/dir>   output bundle directory, default: build/base1-b3-kernel-handoff
  --profile <name>    profile label, default: x86_64-vm-validation
  --target <name>     target label, default: emulator-x86_64
  --image-mb <n>      sandbox raw size, default: 128
  --prepare           create the bundle only, default
  --dry-run           create bundle and print the QEMU handoff plan
  --check             create bundle and run guarded QEMU serial-marker check
  --timeout <seconds> check timeout, default: 30
  --expect <text>     expected serial marker, default: phase1 6.0.0 ready
  -h, --help          show this help

required local inputs:
  A kernel and initrd that are already safe to run in QEMU. This script only
  stages and checks them; it does not build or download them.

outputs:
  <out>/manifest.env
  <out>/staging/boot/vmlinuz
  <out>/staging/boot/initrd.img
  <out>/base1-sandbox.raw
  <out>/run-qemu-bundle.sh
  <out>/reports/qemu-boot.log when --check is used

non-claims:
  This is emulator-only B3 handoff evidence. It does not install Base1, validate physical hardware, validate recovery, validate an installer, prove hardening, or prove daily-driver readiness.
USAGE
}

fail() {
  printf 'base1-b3-kernel-handoff: %s\n' "$1" >&2
  exit 1
}

require_build_out_dir() {
  case "$1" in
    build/*) return 0 ;;
    *) return 1 ;;
  esac
}

while [ "$#" -gt 0 ]; do
  case "$1" in
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
    --profile)
      [ "$#" -ge 2 ] || fail '--profile requires a value'
      PROFILE=$2
      shift 2
      ;;
    --target)
      [ "$#" -ge 2 ] || fail '--target requires a value'
      TARGET=$2
      shift 2
      ;;
    --image-mb)
      [ "$#" -ge 2 ] || fail '--image-mb requires a value'
      IMAGE_MB=$2
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

case "$IMAGE_MB" in
  ''|*[!0-9]*) fail '--image-mb must be a positive integer' ;;
esac
[ "$IMAGE_MB" -gt 0 ] || fail '--image-mb must be greater than zero'

case "$TIMEOUT_SECONDS" in
  ''|*[!0-9]*) fail '--timeout must be a positive integer' ;;
esac
[ "$TIMEOUT_SECONDS" -gt 0 ] || fail '--timeout must be greater than zero'

require_build_out_dir "$OUT_DIR" || fail "output directory must be under build/: $OUT_DIR"
[ -n "$KERNEL" ] || fail '--kernel is required for B3 kernel handoff'
[ -n "$INITRD" ] || fail '--initrd is required for B3 kernel handoff'
[ -f "$KERNEL" ] || fail "kernel not found: $KERNEL"
[ -f "$INITRD" ] || fail "initrd not found: $INITRD"
[ -f scripts/base1-emulator-preview.sh ] || fail 'missing scripts/base1-emulator-preview.sh'
[ -f scripts/base1-qemu-boot-check.sh ] || fail 'missing scripts/base1-qemu-boot-check.sh'

printf 'BASE1 B3 KERNEL HANDOFF\n'
printf 'mode   : %s\n' "$MODE"
printf 'out    : %s\n' "$OUT_DIR"
printf 'profile: %s\n' "$PROFILE"
printf 'target : %s\n' "$TARGET"
printf 'kernel : %s\n' "$KERNEL"
printf 'initrd : %s\n' "$INITRD"
printf 'expect : %s\n' "$EXPECT"
printf '\n'

sh scripts/base1-emulator-preview.sh \
  --out "$OUT_DIR" \
  --profile "$PROFILE" \
  --target "$TARGET" \
  --image-mb "$IMAGE_MB" \
  --kernel "$KERNEL" \
  --initrd "$INITRD"

printf '\nbundle: %s\n' "$OUT_DIR"
printf 'handoff: staging/boot/vmlinuz + staging/boot/initrd.img\n'
printf 'non_claims: emulator-only; no installer; no hardware validation; no daily-driver claim\n'

case "$MODE" in
  prepare)
    printf 'result: prepared\n'
    ;;
  dry-run)
    sh scripts/base1-qemu-boot-check.sh \
      --bundle "$OUT_DIR" \
      --dry-run \
      --timeout "$TIMEOUT_SECONDS" \
      --expect "$EXPECT"
    ;;
  check)
    sh scripts/base1-qemu-boot-check.sh \
      --bundle "$OUT_DIR" \
      --execute \
      --confirm launch-qemu-base1-preview \
      --timeout "$TIMEOUT_SECONDS" \
      --expect "$EXPECT"
    ;;
  *)
    fail "internal unsupported mode: $MODE"
    ;;
esac

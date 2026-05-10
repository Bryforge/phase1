#!/usr/bin/env sh
# Base1 preview stack.
#
# Runs the safe emulator-prep flow:
#   inputs -> bundle -> doctor -> gate dry-run
#
# This script does not launch QEMU, install Base1, write real disks,
# format filesystems, mount devices, or claim Base1 is bootable.

set -eu

BUNDLE_DIR=${BASE1_EMULATOR_BUNDLE:-build/base1-emulator-preview}
KERNEL=${BASE1_PREVIEW_KERNEL:-}
INITRD=${BASE1_PREVIEW_INITRD:-}
IMAGE_MB=${BASE1_EMULATOR_IMAGE_MB:-64}
CHECK_QEMU=1

usage() {
  cat <<'USAGE'
base1 preview stack

usage:
  scripts/base1-preview-stack.sh [options]

options:
  --bundle <dir>       bundle output directory, default: build/base1-emulator-preview
  --kernel <path>      candidate kernel path
  --initrd <path>      candidate initrd path
  --image-mb <n>       sandbox raw placeholder size, default: 64
  --no-qemu-check      skip QEMU PATH check
  -h, --help           show this help

safe flow:
  1. scripts/base1-preview-inputs.sh
  2. scripts/base1-emulator-preview.sh
  3. scripts/base1-emulator-doctor.sh
  4. scripts/base1-preview-gate.sh --dry-run

non-claims:
  This stack is emulator-prep only. It does not launch the emulator,
  create a released image, install Base1, validate hardware, complete
  recovery, or prove that Base1 is bootable. It does not claim Base1 is bootable.
USAGE
}

fail() {
  printf 'base1 preview stack: %s\n' "$1" >&2
  exit 1
}

note() {
  printf 'base1 preview stack: %s\n' "$1"
}

while [ "$#" -gt 0 ]; do
  case "$1" in
    --bundle)
      [ "$#" -ge 2 ] || fail '--bundle requires a value'
      BUNDLE_DIR=$2
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
    --image-mb)
      [ "$#" -ge 2 ] || fail '--image-mb requires a value'
      IMAGE_MB=$2
      shift 2
      ;;
    --no-qemu-check)
      CHECK_QEMU=0
      shift
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      fail "unknown option: $1"
      ;;
  esac
done

[ -n "$KERNEL" ] || fail '--kernel is required'
[ -n "$INITRD" ] || fail '--initrd is required'

case "$IMAGE_MB" in
  ''|*[!0-9]*) fail '--image-mb must be a positive integer' ;;
esac
[ "$IMAGE_MB" -gt 0 ] || fail '--image-mb must be greater than zero'

INPUTS_ARGS="--bundle $BUNDLE_DIR --kernel $KERNEL --initrd $INITRD"
DOCTOR_ARGS="--bundle $BUNDLE_DIR"
if [ "$CHECK_QEMU" = "0" ]; then
  INPUTS_ARGS="$INPUTS_ARGS --no-qemu-check"
  DOCTOR_ARGS="$DOCTOR_ARGS --no-qemu-check"
fi

note 'step 1/4: checking preview inputs'
# shellcheck disable=SC2086
sh scripts/base1-preview-inputs.sh $INPUTS_ARGS

note 'step 2/4: generating emulator preview bundle'
sh scripts/base1-emulator-preview.sh \
  --out "$BUNDLE_DIR" \
  --kernel "$KERNEL" \
  --initrd "$INITRD" \
  --image-mb "$IMAGE_MB"

note 'step 3/4: running read-only bundle doctor'
# shellcheck disable=SC2086
sh scripts/base1-emulator-doctor.sh $DOCTOR_ARGS

note 'step 4/4: running guarded dry-run gate'
sh scripts/base1-preview-gate.sh --bundle "$BUNDLE_DIR" --dry-run

note 'complete: safe preview stack passed'
note 'non-claim: no emulator launched, no installer run, no hardware validation performed'

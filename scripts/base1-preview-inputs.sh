#!/usr/bin/env sh
# Base1 preview inputs checker.
#
# Read-only checker for emulator-preview inputs. It verifies candidate kernel,
# initrd, bundle, and QEMU paths and prints the next safe commands. It does not
# start QEMU, write disks, create images, mount filesystems, or claim Base1 is
# bootable.

set -eu

BUNDLE_DIR=${BASE1_EMULATOR_BUNDLE:-build/base1-emulator-preview}
KERNEL=${BASE1_PREVIEW_KERNEL:-}
INITRD=${BASE1_PREVIEW_INITRD:-}
QEMU=${BASE1_QEMU_BIN:-qemu-system-x86_64}
CHECK_QEMU=1
STATUS=pass

usage() {
  cat <<'USAGE'
base1 preview inputs checker

usage:
  scripts/base1-preview-inputs.sh [options]

options:
  --bundle <dir>       Base1 emulator preview bundle directory
  --kernel <path>      candidate kernel path
  --initrd <path>      candidate initrd path
  --qemu <path>        QEMU executable name/path, default: qemu-system-x86_64
  --no-qemu-check      skip checking whether QEMU is on PATH
  -h, --help           show this help

checks:
  candidate kernel path exists and is a file
  candidate initrd path exists and is a file
  bundle path is under build/ when supplied
  optional QEMU executable can be found

next commands:
  scripts/base1-emulator-preview.sh --out <bundle> --kernel <kernel> --initrd <initrd>
  scripts/base1-emulator-doctor.sh --bundle <bundle>
  scripts/base1-preview-gate.sh --bundle <bundle> --dry-run

non-claims:
  This checker is read-only. It does not start the emulator, install Base1,
  create an image, validate hardware, complete recovery, or prove that Base1 is
  bootable.
USAGE
}

warn() {
  printf 'WARN  %s\n' "$1"
  if [ "$STATUS" = "pass" ]; then
    STATUS=pass-with-notes
  fi
}

fail_check() {
  printf 'FAIL  %s\n' "$1"
  STATUS=failed
}

pass_check() {
  printf 'PASS  %s\n' "$1"
}

have() {
  command -v "$1" >/dev/null 2>&1
}

while [ "$#" -gt 0 ]; do
  case "$1" in
    --bundle)
      [ "$#" -ge 2 ] || { printf 'base1 preview inputs: --bundle requires a value\n' >&2; exit 2; }
      BUNDLE_DIR=$2
      shift 2
      ;;
    --kernel)
      [ "$#" -ge 2 ] || { printf 'base1 preview inputs: --kernel requires a value\n' >&2; exit 2; }
      KERNEL=$2
      shift 2
      ;;
    --initrd)
      [ "$#" -ge 2 ] || { printf 'base1 preview inputs: --initrd requires a value\n' >&2; exit 2; }
      INITRD=$2
      shift 2
      ;;
    --qemu)
      [ "$#" -ge 2 ] || { printf 'base1 preview inputs: --qemu requires a value\n' >&2; exit 2; }
      QEMU=$2
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
      printf 'base1 preview inputs: unknown option: %s\n' "$1" >&2
      exit 2
      ;;
  esac
done

printf 'BASE1 PREVIEW INPUTS\n'
printf 'status : read-only inspection\n'
printf 'bundle : %s\n' "$BUNDLE_DIR"
printf 'kernel : %s\n' "${KERNEL:-not-provided}"
printf 'initrd : %s\n' "${INITRD:-not-provided}"
printf 'qemu   : %s\n' "$QEMU"
printf '\n'

case "$BUNDLE_DIR" in
  build/*)
    pass_check "bundle path is under build/: $BUNDLE_DIR"
    ;;
  *)
    warn "bundle path is outside build/: $BUNDLE_DIR"
    ;;
esac

if [ -n "$KERNEL" ] && [ -f "$KERNEL" ]; then
  pass_check "kernel file exists: $KERNEL"
elif [ -n "$KERNEL" ]; then
  fail_check "kernel file missing: $KERNEL"
else
  fail_check "kernel path not provided"
fi

if [ -n "$INITRD" ] && [ -f "$INITRD" ]; then
  pass_check "initrd file exists: $INITRD"
elif [ -n "$INITRD" ]; then
  fail_check "initrd file missing: $INITRD"
else
  fail_check "initrd path not provided"
fi

if [ "$CHECK_QEMU" = "1" ]; then
  if have "$QEMU"; then
    pass_check "qemu executable found: $QEMU"
  else
    warn "qemu executable not found on PATH: $QEMU"
  fi
else
  warn "qemu executable check skipped"
fi

printf '\nnext safe commands:\n'
printf '  scripts/base1-emulator-preview.sh --out %s --kernel %s --initrd %s\n' "$BUNDLE_DIR" "${KERNEL:-<kernel>}" "${INITRD:-<initrd>}"
printf '  scripts/base1-emulator-doctor.sh --bundle %s\n' "$BUNDLE_DIR"
printf '  scripts/base1-preview-gate.sh --bundle %s --dry-run\n' "$BUNDLE_DIR"
printf '\nresult: %s\n' "$STATUS"
printf 'non-claims: no emulator started; no image created; no installer or hardware validation performed\n'

case "$STATUS" in
  failed) exit 1 ;;
  *) exit 0 ;;
esac

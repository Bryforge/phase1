#!/usr/bin/env sh
# Base1 emulator doctor.
#
# Read-only checker for Base1 emulator preview bundles.
# It does not launch QEMU, write disks, format filesystems, attach loop devices,
# mount filesystems, install packages, or claim Base1 is bootable.

set -eu

BUNDLE_DIR=${BASE1_EMULATOR_BUNDLE:-build/base1-emulator-preview}
CHECK_QEMU=${BASE1_DOCTOR_CHECK_QEMU:-1}
QEMU=${BASE1_QEMU_BIN:-qemu-system-x86_64}
STATUS=pass

usage() {
  cat <<'USAGE'
base1 emulator doctor

usage:
  scripts/base1-emulator-doctor.sh [options]

options:
  --bundle <dir>        Base1 emulator preview bundle directory
  --qemu <path>         QEMU executable name/path, default: qemu-system-x86_64
  --no-qemu-check       skip checking whether QEMU is on PATH
  -h, --help            show this help

checks:
  manifest.env
  README.txt
  staging/manifest.env
  staging/rootfs/opt/phase1/README.txt
  staging/rootfs/usr/local/bin/base1-phase1-run.sh
  staging/rootfs/etc/systemd/system/phase1-base1.service
  staging/boot/grub/grub.cfg
  staging/run-qemu-preview.sh
  base1-sandbox.raw
  run-qemu-bundle.sh
  optional staging/boot/vmlinuz
  optional staging/boot/initrd.img
  optional qemu executable on PATH

non-claims:
  This is a read-only bundle inspection tool. It does not launch the emulator,
  create an image, install Base1, validate hardware, complete recovery, or
  prove that Base1 is a bootable release. It does not claim Base1 is bootable.
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

check_file() {
  path=$1
  label=$2
  if [ -f "$BUNDLE_DIR/$path" ]; then
    pass_check "$label: $path"
  else
    fail_check "$label missing: $path"
  fi
}

check_optional_file() {
  path=$1
  label=$2
  if [ -f "$BUNDLE_DIR/$path" ]; then
    pass_check "$label: $path"
  else
    warn "$label missing: $path"
  fi
}

while [ "$#" -gt 0 ]; do
  case "$1" in
    --bundle)
      [ "$#" -ge 2 ] || { printf 'base1 emulator doctor: --bundle requires a value\n' >&2; exit 2; }
      BUNDLE_DIR=$2
      shift 2
      ;;
    --qemu)
      [ "$#" -ge 2 ] || { printf 'base1 emulator doctor: --qemu requires a value\n' >&2; exit 2; }
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
      printf 'base1 emulator doctor: unknown option: %s\n' "$1" >&2
      exit 2
      ;;
  esac
done

printf 'BASE1 EMULATOR DOCTOR\n'
printf 'bundle : %s\n' "$BUNDLE_DIR"
printf 'status : read-only inspection\n'
printf '\n'

case "$BUNDLE_DIR" in
  build/*)
    pass_check "bundle path is under build/"
    ;;
  *)
    warn "bundle path is outside build/; inspection only, no writes will be made"
    ;;
esac

if [ ! -d "$BUNDLE_DIR" ]; then
  fail_check "bundle directory does not exist: $BUNDLE_DIR"
  printf '\nresult: %s\n' "$STATUS"
  exit 1
fi

check_file "manifest.env" "bundle manifest"
check_file "README.txt" "bundle README"
check_file "staging/manifest.env" "staging manifest"
check_file "staging/rootfs/opt/phase1/README.txt" "rootfs placeholder"
check_file "staging/rootfs/usr/local/bin/base1-phase1-run.sh" "Phase1 launcher"
check_file "staging/rootfs/etc/systemd/system/phase1-base1.service" "systemd service"
check_file "staging/boot/grub/grub.cfg" "GRUB preview config"
check_file "staging/run-qemu-preview.sh" "staging QEMU scaffold"
check_file "base1-sandbox.raw" "sandbox raw placeholder"
check_file "run-qemu-bundle.sh" "bundle QEMU scaffold"

if [ -f "$BUNDLE_DIR/base1-rootfs-preview.tar" ]; then
  pass_check "rootfs tar preview: base1-rootfs-preview.tar"
elif [ -f "$BUNDLE_DIR/base1-rootfs-preview.tar.MISSING" ]; then
  warn "rootfs tar preview skipped; tar was unavailable"
else
  warn "rootfs tar preview not found"
fi

check_optional_file "staging/boot/vmlinuz" "kernel for direct-kernel preview"
check_optional_file "staging/boot/initrd.img" "initrd for direct-kernel preview"

if [ "$CHECK_QEMU" = "1" ]; then
  if have "$QEMU"; then
    pass_check "qemu executable found: $QEMU"
  else
    warn "qemu executable not found on PATH: $QEMU"
  fi
else
  warn "qemu executable check skipped"
fi

printf '\nresult: %s\n' "$STATUS"
printf 'non-claims: no emulator launched; no disk image created; no installer or hardware validation performed\n'

case "$STATUS" in
  failed) exit 1 ;;
  *) exit 0 ;;
esac

#!/usr/bin/env sh
# Base1 emulator preview bundle generator.
#
# This script prepares emulator-only preview artifacts under build/.
# It does not write block devices, format media, attach loop devices, change host
# boot configuration, install packages, or claim a bootable Base1 release.

set -eu

OUT_DIR=${BASE1_EMULATOR_PREVIEW_OUT:-build/base1-emulator-preview}
PROFILE=${BASE1_PROFILE:-secure-default}
TARGET=${BASE1_HARDWARE_TARGET:-emulator-x86_64}
IMAGE_MB=${BASE1_EMULATOR_IMAGE_MB:-64}
KERNEL=${BASE1_PREVIEW_KERNEL:-}
INITRD=${BASE1_PREVIEW_INITRD:-}
QEMU=${BASE1_QEMU_BIN:-qemu-system-x86_64}
MEMORY=${BASE1_QEMU_MEMORY:-1024}

usage() {
  cat <<'USAGE'
base1 emulator preview bundle

usage:
  scripts/base1-emulator-preview.sh [options]

options:
  --out <build/dir>  output directory; must be under build/
  --profile <name>   Base1 profile label, default: secure-default
  --target <name>    target label, default: emulator-x86_64
  --image-mb <n>     sandbox raw placeholder size, default: 64
  --kernel <path>    optional kernel path for direct-kernel QEMU preview
  --initrd <path>    optional initrd path for direct-kernel QEMU preview
  --qemu <path>      qemu executable name/path, default: qemu-system-x86_64
  --memory <mb>      qemu memory value, default: 1024
  -h, --help         show this help

output:
  manifest.env
  staging/
  base1-rootfs-preview.tar
  base1-sandbox.raw
  run-qemu-bundle.sh

non-claims:
  This is an emulator-only preview bundle. It is not a released Base1 image,
  installer, recovery image, hardware validation, or daily-driver system.
USAGE
}

fail() {
  printf 'base1 emulator preview: %s\n' "$1" >&2
  exit 1
}

note() {
  printf 'base1 emulator preview: %s\n' "$1"
}

require_build_out_dir() {
  case "$1" in
    build/*) return 0 ;;
    *) return 1 ;;
  esac
}

have() {
  command -v "$1" >/dev/null 2>&1
}

while [ "$#" -gt 0 ]; do
  case "$1" in
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
    --qemu)
      [ "$#" -ge 2 ] || fail '--qemu requires a value'
      QEMU=$2
      shift 2
      ;;
    --memory)
      [ "$#" -ge 2 ] || fail '--memory requires a value'
      MEMORY=$2
      shift 2
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

case "$IMAGE_MB" in
  ''|*[!0-9]*) fail "--image-mb must be a positive integer" ;;
esac
[ "$IMAGE_MB" -gt 0 ] || fail "--image-mb must be greater than zero"

require_build_out_dir "$OUT_DIR" || fail "output directory must be under build/: $OUT_DIR"

BOOT_PREVIEW_SCRIPT=scripts/base1-boot-preview.sh
[ -f "$BOOT_PREVIEW_SCRIPT" ] || fail "missing $BOOT_PREVIEW_SCRIPT"

mkdir -p "$OUT_DIR"
STAGING_DIR="$OUT_DIR/staging"

preview_args="--out $STAGING_DIR --profile $PROFILE --target $TARGET"
if [ -n "$KERNEL" ]; then
  preview_args="$preview_args --kernel $KERNEL"
fi
if [ -n "$INITRD" ]; then
  preview_args="$preview_args --initrd $INITRD"
fi

# shellcheck disable=SC2086
sh "$BOOT_PREVIEW_SCRIPT" $preview_args >/dev/null

if have tar; then
  (cd "$STAGING_DIR/rootfs" && tar -cf "../../base1-rootfs-preview.tar" .)
else
  printf 'tar not available; rootfs archive not created\n' > "$OUT_DIR/base1-rootfs-preview.tar.MISSING"
fi

RAW_IMAGE="$OUT_DIR/base1-sandbox.raw"
if have truncate; then
  truncate -s "${IMAGE_MB}M" "$RAW_IMAGE"
else
  printf 'Base1 sandbox raw placeholder; truncate not available.\n' > "$RAW_IMAGE"
fi

cat > "$OUT_DIR/manifest.env" <<EOF
BASE1_EMULATOR_PREVIEW_STATUS=staging-only
BASE1_PROFILE=$PROFILE
BASE1_HARDWARE_TARGET=$TARGET
BASE1_EMULATOR_IMAGE_MB=$IMAGE_MB
BASE1_STAGING_DIR=$STAGING_DIR
BASE1_ROOTFS_TAR=base1-rootfs-preview.tar
BASE1_SANDBOX_RAW=base1-sandbox.raw
BASE1_QEMU_SCRIPT=run-qemu-bundle.sh
BASE1_NON_CLAIM_BOOTABLE_RELEASE=1
BASE1_NON_CLAIM_INSTALLER=1
BASE1_NON_CLAIM_HARDWARE_VALIDATED=1
EOF

cat > "$OUT_DIR/README.txt" <<EOF
Base1 emulator preview bundle

This directory contains emulator-only staging artifacts created under build/.

Files:
  staging/                  inspectable boot-preview tree
  base1-rootfs-preview.tar  rootfs preview archive when tar is available
  base1-sandbox.raw         sandbox raw placeholder for emulator-only experiments
  run-qemu-bundle.sh        QEMU direct-kernel command scaffold

Boundaries:
  not a released Base1 image
  not an installer
  not a recovery image
  not hardware validated
  not daily-driver ready
EOF

cat > "$OUT_DIR/run-qemu-bundle.sh" <<EOF
#!/usr/bin/env sh
set -eu
cd "\$(dirname "\$0")"
if [ ! -f staging/boot/vmlinuz ] || [ ! -f staging/boot/initrd.img ]; then
  echo 'base1 emulator preview: missing staging/boot/vmlinuz or staging/boot/initrd.img; rerun with --kernel and --initrd' >&2
  exit 1
fi
exec "$QEMU" -m "$MEMORY" -nographic \
  -kernel staging/boot/vmlinuz \
  -initrd staging/boot/initrd.img \
  -drive file=base1-sandbox.raw,format=raw,if=virtio \
  -append 'console=ttyS0 base1.preview=1 base1.emulator=1 phase1.safe=1 phase1.host_tools=0'
EOF
chmod 755 "$OUT_DIR/run-qemu-bundle.sh"

note "created emulator preview bundle: $OUT_DIR"
note "status: staging-only"
note "sandbox raw placeholder: $RAW_IMAGE"
note "non-claim: no released image, installer, hardware validation, or daily-driver system was created"

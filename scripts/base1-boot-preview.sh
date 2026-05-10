#!/usr/bin/env sh
# Base1 boot preview generator.
#
# This script creates a local, inspectable Base1 boot-preview tree.
# It does not write block devices, create disk images, install packages, mount
# filesystems, or claim that the generated tree is a bootable release.

set -eu

OUT_DIR=${BASE1_BOOT_PREVIEW_OUT:-build/base1-boot-preview}
PROFILE=${BASE1_PROFILE:-secure-default}
TARGET=${BASE1_HARDWARE_TARGET:-generic-x86_64}
PHASE1_BIN=${PHASE1_BIN:-}
KERNEL=${BASE1_PREVIEW_KERNEL:-}
INITRD=${BASE1_PREVIEW_INITRD:-}
QEMU=${BASE1_QEMU_BIN:-qemu-system-x86_64}
MEMORY=${BASE1_QEMU_MEMORY:-1024}

usage() {
  cat <<'USAGE'
base1 boot preview generator

usage:
  scripts/base1-boot-preview.sh [options]

options:
  --out <dir>        output directory for preview files
  --profile <name>   Base1 profile label, default: secure-default
  --target <name>    target label, default: generic-x86_64
  --phase1-bin <p>   optional Phase1 binary path to reference in manifest
  --kernel <path>    optional kernel path to copy into boot/vmlinuz
  --initrd <path>    optional initrd path to copy into boot/initrd.img
  --qemu <path>      qemu executable name/path, default: qemu-system-x86_64
  --memory <mb>      qemu memory value, default: 1024
  -h, --help         show this help

output:
  manifest.env
  rootfs/opt/phase1/README.txt
  rootfs/var/lib/phase1/workspace/.keep
  rootfs/etc/systemd/system/phase1-base1.service
  rootfs/usr/local/bin/base1-phase1-run.sh
  boot/README.txt
  boot/grub/grub.cfg
  run-qemu-preview.sh

non-claims:
  This is a preview staging tree only. It is not a bootable Base1 image,
  installer, recovery image, hardware validation, or daily-driver system.
USAGE
}

fail() {
  printf 'base1 boot preview: %s\n' "$1" >&2
  exit 1
}

note() {
  printf 'base1 boot preview: %s\n' "$1"
}

safe_out_dir() {
  case "$1" in
    ''|/|/dev|/dev/*|/proc|/proc/*|/sys|/sys/*|/run|/run/*)
      return 1
      ;;
    *) return 0 ;;
  esac
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
    --phase1-bin)
      [ "$#" -ge 2 ] || fail '--phase1-bin requires a value'
      PHASE1_BIN=$2
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

safe_out_dir "$OUT_DIR" || fail "refusing unsafe output directory: $OUT_DIR"

mkdir -p "$OUT_DIR/boot/grub"
mkdir -p "$OUT_DIR/rootfs/opt/phase1"
mkdir -p "$OUT_DIR/rootfs/var/lib/phase1/workspace"
mkdir -p "$OUT_DIR/rootfs/etc/systemd/system"
mkdir -p "$OUT_DIR/rootfs/usr/local/bin"
mkdir -p "$OUT_DIR/reports"

: > "$OUT_DIR/rootfs/var/lib/phase1/workspace/.keep"

cat > "$OUT_DIR/manifest.env" <<EOF
BASE1_PREVIEW_STATUS=staging-only
BASE1_PROFILE=$PROFILE
BASE1_HARDWARE_TARGET=$TARGET
PHASE1_SAFE_MODE=1
PHASE1_ALLOW_HOST_TOOLS=0
PHASE1_STORAGE_ROOT=/var/lib/phase1/workspace
PHASE1_BIN=${PHASE1_BIN:-not-provided}
BASE1_NON_CLAIM_BOOTABLE=1
BASE1_NON_CLAIM_INSTALLER=1
BASE1_NON_CLAIM_HARDWARE_VALIDATED=1
EOF

cat > "$OUT_DIR/rootfs/opt/phase1/README.txt" <<EOF
Base1 Phase1 preview rootfs placeholder

This directory is a staging preview only.

Expected future placement:
  /opt/phase1/phase1
  /var/lib/phase1/workspace
  /usr/local/bin/base1-phase1-run.sh

Non-claims:
  not a bootable Base1 release
  not an installer
  not a recovery image
  not hardware validated
  not daily-driver ready
EOF

if [ -f scripts/base1-phase1-run.sh ]; then
  cp scripts/base1-phase1-run.sh "$OUT_DIR/rootfs/usr/local/bin/base1-phase1-run.sh"
else
  cat > "$OUT_DIR/rootfs/usr/local/bin/base1-phase1-run.sh" <<'EOF'
#!/usr/bin/env sh
printf 'base1 launcher placeholder: copy scripts/base1-phase1-run.sh here\n' >&2
exit 1
EOF
fi
chmod 755 "$OUT_DIR/rootfs/usr/local/bin/base1-phase1-run.sh"

if [ -f base1/systemd/phase1-base1.service ]; then
  cp base1/systemd/phase1-base1.service "$OUT_DIR/rootfs/etc/systemd/system/phase1-base1.service"
else
  cat > "$OUT_DIR/rootfs/etc/systemd/system/phase1-base1.service" <<'EOF'
[Unit]
Description=Phase1 Base1 preview service placeholder
After=local-fs.target

[Service]
Type=simple
User=phase1
Environment=PHASE1_SAFE_MODE=1
Environment=PHASE1_ALLOW_HOST_TOOLS=0
ExecStart=/usr/local/bin/base1-phase1-run.sh

[Install]
WantedBy=multi-user.target
EOF
fi

if [ -n "$KERNEL" ]; then
  [ -f "$KERNEL" ] || fail "kernel path is not a file: $KERNEL"
  cp "$KERNEL" "$OUT_DIR/boot/vmlinuz"
else
  cat > "$OUT_DIR/boot/vmlinuz.MISSING" <<'EOF'
Provide a Linux kernel with --kernel <path> for qemu direct-kernel preview.
EOF
fi

if [ -n "$INITRD" ]; then
  [ -f "$INITRD" ] || fail "initrd path is not a file: $INITRD"
  cp "$INITRD" "$OUT_DIR/boot/initrd.img"
else
  cat > "$OUT_DIR/boot/initrd.img.MISSING" <<'EOF'
Provide an initrd with --initrd <path> for qemu direct-kernel preview.
EOF
fi

cat > "$OUT_DIR/boot/README.txt" <<EOF
Base1 boot preview boot folder

This folder can hold copied kernel/initrd files for emulator preview work.
Generated files do not create a disk image and do not modify host boot config.
EOF

cat > "$OUT_DIR/boot/grub/grub.cfg" <<EOF
set timeout=5
set default=0

menuentry 'Base1 Phase1 preview - staging only' {
    echo 'Base1 preview only: no installer, no hardware validation, no daily-driver claim.'
    linux /boot/vmlinuz console=ttyS0 base1.profile=$PROFILE phase1.safe=1 phase1.host_tools=0
    initrd /boot/initrd.img
}
EOF

cat > "$OUT_DIR/run-qemu-preview.sh" <<EOF
#!/usr/bin/env sh
set -eu
cd "\$(dirname "\$0")"
if [ ! -f boot/vmlinuz ] || [ ! -f boot/initrd.img ]; then
  echo 'base1 qemu preview: missing boot/vmlinuz or boot/initrd.img; rerun with --kernel and --initrd' >&2
  exit 1
fi
exec "$QEMU" -m "$MEMORY" -nographic \
  -kernel boot/vmlinuz \
  -initrd boot/initrd.img \
  -append 'console=ttyS0 base1.preview=1 phase1.safe=1 phase1.host_tools=0'
EOF
chmod 755 "$OUT_DIR/run-qemu-preview.sh"

cat > "$OUT_DIR/reports/README.txt" <<EOF
Base1 preview report notes

Record emulator observations here after manual testing.
Use docs/base1/VALIDATION_REPORT_TEMPLATE.md for formal reports.
EOF

note "created preview tree: $OUT_DIR"
note "status: staging-only"
note "next: inspect manifest.env and run-qemu-preview.sh"
note "non-claim: no disk image, installer, recovery image, hardware validation, or bootable release was created"

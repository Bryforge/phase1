#!/usr/bin/env sh
# Phase1 / Base1 X200 B9 boot method matrix USB writer.
#
# Purpose:
#   Build one external USB with a broad boot-method matrix so X200 testing does
#   not require rebuilding the USB after every blocked handoff. This includes
#   Linux boot protocol variants, no-background text-only entries, Base1
#   initramfs shell entries, and marker fallbacks.
#
# Safety:
#   This erases only the selected USB disk, refuses to run without
#   YES_WRITE_USB, and refuses /dev/sda by default.
#
# Non-claims:
#   External USB evidence only. No installer claim. No internal disk install.
#   No recovery-complete claim. No hardening claim. No daily-driver claim.

set -eu

USB="${1:-/dev/sdb}"
CONFIRM="${2:-}"
PROFILE="${BASE1_B9_PROFILE:-x200-supervisor-lite}"
KERNEL="${BASE1_B9_KERNEL:-build/linux/alpine-netboot/vmlinuz}"
BASE_INITRD="${BASE1_B9_INITRD:-build/linux/alpine-netboot/initrd.img}"
OUT_DIR="${BASE1_B9_OUT:-build/base1-b9-boot-method-matrix-usb}"
OVERLAY_DIR="$OUT_DIR/phase1-initramfs-overlay"
OVERLAY_CPIO="$OUT_DIR/phase1-initramfs.cpio.gz"

fail() { printf 'x200-b9-boot-method-matrix-usb: %s\n' "$1" >&2; exit 1; }
need_cmd() { command -v "$1" >/dev/null 2>&1 || fail "missing command: $1"; }

partition_one() {
  case "$1" in
    /dev/nvme[0-9]n[0-9]|/dev/mmcblk[0-9]) printf '%sp1\n' "$1" ;;
    /dev/sd[a-z]|/dev/vd[a-z]|/dev/hd[a-z]) printf '%s1\n' "$1" ;;
    *) fail "use a whole-disk path such as /dev/sdb, not a partition" ;;
  esac
}

[ "$CONFIRM" = "YES_WRITE_USB" ] || fail "usage: sh scripts/x200-b9-boot-method-matrix-usb.sh /dev/sdb YES_WRITE_USB"
[ -b "$USB" ] || fail "not a block device: $USB"
[ "$USB" != "/dev/sda" ] || fail "refusing /dev/sda because it is commonly the internal disk"
[ -f "$KERNEL" ] || fail "missing kernel: $KERNEL. Run scripts/x200-fetch-b7-kernel-initrd.sh first."
[ -f "$BASE_INITRD" ] || fail "missing initrd: $BASE_INITRD. Run scripts/x200-fetch-b7-kernel-initrd.sh first."

ROOT_SRC="$(findmnt -no SOURCE / 2>/dev/null || true)"
case "$ROOT_SRC" in
  "$USB"|"$USB"[0-9]*|"$USB"p[0-9]*) fail "refusing to write the root filesystem device: $ROOT_SRC" ;;
esac

for cmd in sudo parted mkfs.vfat mount umount sync mkdir cp tee grub-install sha256sum find cpio gzip chmod; do
  need_cmd "$cmd"
done

PART="$(partition_one "$USB")"
MNT="$(mktemp -d)"
REPORT="$OUT_DIR/b9-boot-method-matrix-usb.env"

cleanup() { sudo umount "$MNT" 2>/dev/null || true; rmdir "$MNT" 2>/dev/null || true; }
trap cleanup EXIT INT TERM

printf 'PHASE1 BASE1 B9 BOOT METHOD MATRIX USB WRITER\n\n'
printf 'profile     : %s\n' "$PROFILE"
printf 'target disk : %s\n' "$USB"
printf 'partition   : %s\n' "$PART"
printf 'kernel      : %s\n' "$KERNEL"
printf 'base initrd : %s\n' "$BASE_INITRD"
printf 'scope       : external USB boot-method evidence only\n'
printf 'claim       : not_claimed\n\n'
printf 'This will erase the selected USB target.\n\n'

printf 'Kernel SHA256:\n'; sha256sum "$KERNEL"
printf 'Base initrd SHA256:\n'; sha256sum "$BASE_INITRD"
printf '\nCreating Phase1 overlay initramfs...\n'

rm -rf "$OVERLAY_DIR"
mkdir -p "$OVERLAY_DIR/proc" "$OVERLAY_DIR/sys" "$OVERLAY_DIR/dev" "$OVERLAY_DIR/run" "$OVERLAY_DIR/tmp" "$OVERLAY_DIR/phase1" "$OUT_DIR"
cat > "$OVERLAY_DIR/init" <<'EOF'
#!/bin/sh
PATH=/bin:/sbin:/usr/bin:/usr/sbin
export PATH
mount -t proc proc /proc 2>/dev/null || true
mount -t sysfs sysfs /sys 2>/dev/null || true
mount -t devtmpfs devtmpfs /dev 2>/dev/null || true
mount -t tmpfs tmpfs /run 2>/dev/null || true
clear 2>/dev/null || true
cat <<'BANNER'
phase1 6.0.0 ready
Base1 external USB initramfs shell
B9 candidate result: phase1_initramfs_shell

Hardware path: ThinkPad X200 / Libreboot / external USB
Scope: evidence shell only
Non-claims: no installer, no internal disk install, no recovery-complete,
no hardening proof, no hardware validation, no daily-driver claim.

Useful commands:
  cat /proc/cmdline
  dmesg | tail
  mount
  ls /dev

Type 'exit' to restart the shell.
BANNER
while true; do
  /bin/sh
  echo "Phase1 shell restarted."
done
EOF
chmod +x "$OVERLAY_DIR/init"
cat > "$OVERLAY_DIR/phase1/README.txt" <<'EOF'
Phase1 Base1 B9 external USB initramfs shell overlay.
Evidence-only. Does not install to disk.
EOF
(
  cd "$OVERLAY_DIR"
  find . -print | cpio -o -H newc 2>/dev/null | gzip -9 > "../phase1-initramfs.cpio.gz"
)
[ -s "$OVERLAY_CPIO" ] || fail "failed to create overlay initramfs: $OVERLAY_CPIO"
printf 'Overlay SHA256:\n'; sha256sum "$OVERLAY_CPIO"
printf '\nWriting USB...\n'

sudo umount "${USB}"* 2>/dev/null || true
sudo parted -s "$USB" mklabel msdos
sudo parted -s "$USB" mkpart primary fat32 1MiB 100%
sudo parted -s "$USB" set 1 boot on
sudo partprobe "$USB" 2>/dev/null || true
sync
sleep 2
[ -b "$PART" ] || fail "partition did not appear: $PART"
sudo mkfs.vfat -F 32 -n PHASE1B9 "$PART"
sudo mount "$PART" "$MNT"
sudo mkdir -p "$MNT/boot/grub" "$MNT/grub" "$MNT/boot/phase1" "$MNT/phase1-evidence"
sudo cp "$KERNEL" "$MNT/boot/phase1/vmlinuz"
sudo cp "$BASE_INITRD" "$MNT/boot/phase1/initrd.img"
sudo cp "$OVERLAY_CPIO" "$MNT/boot/phase1/phase1-initramfs.cpio.gz"

sudo tee "$MNT/boot/grub/grub.cfg" >/dev/null <<'EOF'
set timeout=30
set default=0
set pager=1

menuentry "B9-01 Phase1 initramfs shell - standard linux command" {
    echo "Phase1 Base1 B9: standard linux/initrd overlay"
    linux /boot/phase1/vmlinuz console=tty0 nomodeset vga=normal loglevel=7 debug rdinit=/init
    initrd /boot/phase1/initrd.img /boot/phase1/phase1-initramfs.cpio.gz
}

menuentry "B9-02 Phase1 initramfs shell - linux16 compatibility" {
    echo "Phase1 Base1 B9: linux16/initrd16 compatibility path"
    linux16 /boot/phase1/vmlinuz console=tty0 nomodeset vga=normal loglevel=7 debug rdinit=/init
    initrd16 /boot/phase1/initrd.img /boot/phase1/phase1-initramfs.cpio.gz
}

menuentry "B9-03 Phase1 initramfs shell - legacy IRQ/ACPI fallback" {
    echo "Phase1 Base1 B9: legacy IRQ/ACPI fallback"
    linux /boot/phase1/vmlinuz console=tty0 nomodeset vga=normal noapic nolapic acpi=off irqpoll pci=nomsi loglevel=7 debug rdinit=/init
    initrd /boot/phase1/initrd.img /boot/phase1/phase1-initramfs.cpio.gz
}

menuentry "B9-04 Phase1 initramfs shell - linux16 legacy fallback" {
    echo "Phase1 Base1 B9: linux16 legacy fallback"
    linux16 /boot/phase1/vmlinuz console=tty0 nomodeset vga=normal noapic nolapic acpi=off irqpoll pci=nomsi loglevel=7 debug rdinit=/init
    initrd16 /boot/phase1/initrd.img /boot/phase1/phase1-initramfs.cpio.gz
}

menuentry "B9-05 Kernel/initrd control - standard linux" {
    echo "Phase1 Base1 B9 control: kernel/initrd without Phase1 overlay"
    linux /boot/phase1/vmlinuz console=tty0 nomodeset vga=normal loglevel=7 debug
    initrd /boot/phase1/initrd.img
}

menuentry "B9-06 Kernel/initrd control - linux16 compatibility" {
    echo "Phase1 Base1 B9 control: linux16/initrd16 without overlay"
    linux16 /boot/phase1/vmlinuz console=tty0 nomodeset vga=normal loglevel=7 debug
    initrd16 /boot/phase1/initrd.img
}

menuentry "B9-07 Kernel only execution probe - standard linux" {
    echo "Phase1 Base1 B9 kernel-only probe. Panic is useful evidence."
    linux /boot/phase1/vmlinuz console=tty0 nomodeset vga=normal loglevel=7 debug
}

menuentry "B9-08 Kernel only execution probe - linux16 compatibility" {
    echo "Phase1 Base1 B9 linux16 kernel-only probe. Panic is useful evidence."
    linux16 /boot/phase1/vmlinuz console=tty0 nomodeset vga=normal loglevel=7 debug
}

menuentry "B9-09 Marker fallback - known good external USB proof" {
    echo "phase1 6.0.0 ready"
    echo "B6 candidate result: phase1_marker_seen"
    echo "B9 fallback marker reached from external USB"
    echo "This is not an installer, hardening, or daily-driver claim."
    sleep --interruptible 999
}
EOF

sudo cp "$MNT/boot/grub/grub.cfg" "$MNT/grub/grub.cfg"
sudo tee "$MNT/PHASE1-B9-MATRIX.txt" >/dev/null <<EOF
Phase1 Base1 B9 boot method matrix USB.
Profile: $PROFILE
Kernel: /boot/phase1/vmlinuz
Base initrd: /boot/phase1/initrd.img
Phase1 overlay: /boot/phase1/phase1-initramfs.cpio.gz

Try entries from B9-01 through B9-08.
Evidence states:
- phase1_initramfs_shell: Phase1 shell appears.
- boot_started: kernel output or kernel panic appears.
- blocked_after_initrd_load: GRUB loads initrd but no visible kernel handoff occurs.
- blocked_after_kernel_load: GRUB loads vmlinuz but no visible kernel handoff occurs.
- phase1_marker_seen: fallback marker appears.

Non-claims: no installer, no internal disk install, no recovery-complete,
no hardening, no daily-driver.
EOF

printf 'Installing GRUB BIOS bootloader to %s...\n' "$USB"
sudo grub-install --target=i386-pc --boot-directory="$MNT/boot" --recheck "$USB"
sync
sudo umount "$MNT"
rmdir "$MNT"
trap - EXIT INT TERM

mkdir -p "$OUT_DIR"
cat > "$REPORT" <<EOF
BASE1_B9_BOOT_METHOD_MATRIX_PROFILE=$PROFILE
BASE1_B9_BOOT_METHOD_MATRIX_TARGET=$USB
BASE1_B9_BOOT_METHOD_MATRIX_PARTITION=$PART
BASE1_B9_BOOT_METHOD_MATRIX_KERNEL=$KERNEL
BASE1_B9_BOOT_METHOD_MATRIX_BASE_INITRD=$BASE_INITRD
BASE1_B9_BOOT_METHOD_MATRIX_OVERLAY=$OVERLAY_CPIO
BASE1_B9_BOOT_METHOD_MATRIX_RESULT=prepared
BASE1_B9_BOOT_METHOD_MATRIX_EXPECTED_NEXT_RESULT=phase1_initramfs_shell_or_boot_started_or_blocked
BASE1_B9_BOOT_METHOD_MATRIX_CLAIM=not_claimed
EOF

printf '\nDONE: B9 boot method matrix USB prepared on %s\n' "$USB"
printf 'Boot the X200 from USB and try B9-01 through B9-08.\n'
printf 'If B9-02 or B9-04 works, linux16 compatibility was the missing path.\n'

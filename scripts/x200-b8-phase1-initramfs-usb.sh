#!/usr/bin/env sh
# Phase1 / Base1 X200 B8 Phase1 initramfs USB writer.
#
# Purpose:
#   Build one external USB that attempts to boot into a Phase1-branded
#   initramfs shell on the ThinkPad X200 Libreboot/Coreboot path.
#
# Design:
#   Uses the existing Linux kernel and Alpine netboot initrd, then appends a
#   small Phase1 overlay initramfs that provides /init. If the kernel handoff
#   works, the screen should enter the Phase1 Base1 external USB shell.
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
PROFILE="${BASE1_B8_PROFILE:-x200-supervisor-lite}"
KERNEL="${BASE1_B8_KERNEL:-build/linux/alpine-netboot/vmlinuz}"
BASE_INITRD="${BASE1_B8_INITRD:-build/linux/alpine-netboot/initrd.img}"
OUT_DIR="${BASE1_B8_OUT:-build/base1-b8-phase1-initramfs-usb}"
OVERLAY_DIR="$OUT_DIR/phase1-initramfs-overlay"
OVERLAY_CPIO="$OUT_DIR/phase1-initramfs.cpio.gz"

fail() {
  printf 'x200-b8-phase1-initramfs-usb: %s\n' "$1" >&2
  exit 1
}

need_cmd() {
  command -v "$1" >/dev/null 2>&1 || fail "missing command: $1"
}

partition_one() {
  case "$1" in
    /dev/nvme[0-9]n[0-9]|/dev/mmcblk[0-9]) printf '%sp1\n' "$1" ;;
    /dev/sd[a-z]|/dev/vd[a-z]|/dev/hd[a-z]) printf '%s1\n' "$1" ;;
    *) fail "use a whole-disk path such as /dev/sdb, not a partition" ;;
  esac
}

[ "$CONFIRM" = "YES_WRITE_USB" ] || fail "usage: sh scripts/x200-b8-phase1-initramfs-usb.sh /dev/sdb YES_WRITE_USB"
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
REPORT="$OUT_DIR/b8-phase1-initramfs-usb.env"

cleanup() {
  sudo umount "$MNT" 2>/dev/null || true
  rmdir "$MNT" 2>/dev/null || true
}
trap cleanup EXIT INT TERM

printf 'PHASE1 BASE1 B8 INITRAMFS USB WRITER\n\n'
printf 'profile     : %s\n' "$PROFILE"
printf 'target disk : %s\n' "$USB"
printf 'partition   : %s\n' "$PART"
printf 'kernel      : %s\n' "$KERNEL"
printf 'base initrd : %s\n' "$BASE_INITRD"
printf 'overlay     : %s\n' "$OVERLAY_CPIO"
printf 'scope       : external USB initramfs shell evidence only\n'
printf 'claim       : not_claimed\n\n'
printf 'This will erase the selected USB target.\n\n'

printf 'Kernel SHA256:\n'
sha256sum "$KERNEL"
printf 'Base initrd SHA256:\n'
sha256sum "$BASE_INITRD"
printf '\nCreating Phase1 initramfs overlay...\n'

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
B8 candidate result: phase1_initramfs_shell

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

if command -v setsid >/dev/null 2>&1 && command -v cttyhack >/dev/null 2>&1; then
  while true; do setsid cttyhack /bin/sh; done
fi

while true; do
  /bin/sh
  echo "Phase1 shell restarted."
done
EOF
chmod +x "$OVERLAY_DIR/init"

cat > "$OVERLAY_DIR/phase1/README.txt" <<'EOF'
Phase1 Base1 B8 external USB initramfs shell overlay.
This is evidence-only and does not install to disk.
EOF

(
  cd "$OVERLAY_DIR"
  find . -print | cpio -o -H newc 2>/dev/null | gzip -9 > "../phase1-initramfs.cpio.gz"
)

[ -s "$OVERLAY_CPIO" ] || fail "failed to create overlay initramfs: $OVERLAY_CPIO"

printf 'Overlay SHA256:\n'
sha256sum "$OVERLAY_CPIO"
printf '\nWriting USB...\n'

sudo umount "${USB}"* 2>/dev/null || true
sudo parted -s "$USB" mklabel msdos
sudo parted -s "$USB" mkpart primary fat32 1MiB 100%
sudo parted -s "$USB" set 1 boot on
sudo partprobe "$USB" 2>/dev/null || true
sync
sleep 2

[ -b "$PART" ] || fail "partition did not appear: $PART"

sudo mkfs.vfat -F 32 -n PHASE1B8 "$PART"
sudo mount "$PART" "$MNT"
sudo mkdir -p "$MNT/boot/grub" "$MNT/grub" "$MNT/boot/phase1" "$MNT/phase1-evidence"
sudo cp "$KERNEL" "$MNT/boot/phase1/vmlinuz"
sudo cp "$BASE_INITRD" "$MNT/boot/phase1/initrd.img"
sudo cp "$OVERLAY_CPIO" "$MNT/boot/phase1/phase1-initramfs.cpio.gz"

sudo tee "$MNT/boot/grub/grub.cfg" >/dev/null <<'EOF'
set timeout=20
set default=0
set pager=1

menuentry "Phase1 Base1 B8 - enter external USB initramfs shell" {
    set gfxpayload=text
    echo "Phase1 Base1 B8 external USB initramfs shell"
    echo "Target evidence: phase1_initramfs_shell"
    echo "If a Phase1 shell appears, the next result is phase1_initramfs_shell."
    linux /boot/phase1/vmlinuz console=tty0 nomodeset vga=normal loglevel=7 debug earlyprintk=vga keep_bootcon rdinit=/init
    initrd /boot/phase1/initrd.img /boot/phase1/phase1-initramfs.cpio.gz
}

menuentry "Phase1 Base1 B8 - initramfs shell, legacy fallback" {
    set gfxpayload=text
    echo "Phase1 Base1 B8 external USB initramfs shell"
    echo "Legacy fallback: noapic nolapic acpi=off irqpoll pci=nomsi"
    linux /boot/phase1/vmlinuz console=tty0 nomodeset vga=normal noapic nolapic acpi=off irqpoll pci=nomsi loglevel=7 debug earlyprintk=vga keep_bootcon rdinit=/init
    initrd /boot/phase1/initrd.img /boot/phase1/phase1-initramfs.cpio.gz
}

menuentry "Phase1 Base1 B8 - initramfs shell, minimal args" {
    echo "Phase1 Base1 B8 external USB initramfs shell"
    echo "Minimal kernel args."
    linux /boot/phase1/vmlinuz rdinit=/init
    initrd /boot/phase1/initrd.img /boot/phase1/phase1-initramfs.cpio.gz
}

menuentry "Phase1 Base1 B7 - kernel/initrd control, safe text mode" {
    set gfxpayload=text
    echo "Phase1 Base1 B7 control: kernel/initrd without Phase1 overlay"
    linux /boot/phase1/vmlinuz console=tty0 nomodeset vga=normal loglevel=7 debug earlyprintk=vga keep_bootcon
    initrd /boot/phase1/initrd.img
}

menuentry "Phase1 Base1 B7 - kernel only execution probe" {
    set gfxpayload=text
    echo "Phase1 Base1 B7 control: kernel only, no initrd"
    echo "Expected panic is useful evidence of kernel execution."
    linux /boot/phase1/vmlinuz console=tty0 nomodeset vga=normal loglevel=7 debug earlyprintk=vga keep_bootcon
}

menuentry "Phase1 Base1 B6 - marker fallback" {
    echo "phase1 6.0.0 ready"
    echo "B6 candidate result: phase1_marker_seen"
    echo "B8 fallback marker reached from external USB"
    echo "This is not an installer, hardening, or daily-driver claim."
    sleep --interruptible 999
}
EOF

sudo cp "$MNT/boot/grub/grub.cfg" "$MNT/grub/grub.cfg"

sudo tee "$MNT/PHASE1-B8-INITRAMFS.txt" >/dev/null <<EOF
Phase1 Base1 B8 external USB initramfs shell.
Profile: $PROFILE
Kernel: /boot/phase1/vmlinuz
Base initrd: /boot/phase1/initrd.img
Phase1 overlay: /boot/phase1/phase1-initramfs.cpio.gz

Evidence states:
- phase1_initramfs_shell: Phase1 shell appears.
- boot_started: kernel output or panic appears.
- blocked_after_initrd_load: GRUB loads initrd but no visible kernel handoff occurs.
- blocked_after_kernel_load: GRUB loads vmlinuz but no visible kernel handoff occurs.
- phase1_marker_seen: fallback marker appears.

Non-claims:
No installer claim.
No internal disk install claim.
No recovery-complete claim.
No hardening claim.
No daily-driver claim.
EOF

printf 'Installing GRUB BIOS bootloader to %s...\n' "$USB"
sudo grub-install --target=i386-pc --boot-directory="$MNT/boot" --recheck "$USB"

sync
sudo umount "$MNT"
rmdir "$MNT"
trap - EXIT INT TERM

mkdir -p "$OUT_DIR"
cat > "$REPORT" <<EOF
BASE1_B8_INITRAMFS_USB_PROFILE=$PROFILE
BASE1_B8_INITRAMFS_USB_TARGET=$USB
BASE1_B8_INITRAMFS_USB_PARTITION=$PART
BASE1_B8_INITRAMFS_USB_KERNEL=$KERNEL
BASE1_B8_INITRAMFS_USB_BASE_INITRD=$BASE_INITRD
BASE1_B8_INITRAMFS_USB_OVERLAY=$OVERLAY_CPIO
BASE1_B8_INITRAMFS_USB_RESULT=prepared
BASE1_B8_INITRAMFS_USB_EXPECTED_NEXT_RESULT=phase1_initramfs_shell
BASE1_B8_INITRAMFS_USB_CLAIM=not_claimed
BASE1_B8_NON_CLAIM_INSTALLER=1
BASE1_B8_NON_CLAIM_INTERNAL_DISK_INSTALL=1
BASE1_B8_NON_CLAIM_RECOVERY_COMPLETE=1
BASE1_B8_NON_CLAIM_HARDENED=1
BASE1_B8_NON_CLAIM_DAILY_DRIVER=1
EOF

printf '\nDONE: B8 Phase1 initramfs USB prepared on %s\n' "$USB"
printf 'Boot the X200 from USB and choose: Phase1 Base1 B8 - enter external USB initramfs shell\n'
printf 'If the shell appears, record: phase1_initramfs_shell\n'

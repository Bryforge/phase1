#!/usr/bin/env sh
# Phase1 / Base1 X200 B23 GNU/Linux runtime USB writer.
#
# Purpose:
#   Move Phase1 beyond a GRUB-only control surface into a GNU/Linux-backed
#   Phase1 runtime using the proven physical route:
#
#     Libreboot -> SeaBIOS payload -> USB GRUB -> GNU/Linux -> Phase1 runtime
#
# Display policy:
#   GRUB remains a short-lived loader. GNU/Linux owns the runtime console.
#   This avoids depending on the small/tearing SeaBIOS GRUB display.
#
# Required local artifacts by default:
#   build/linux/alpine-netboot/vmlinuz
#   build/linux/alpine-netboot/initrd.img
#
# Override with:
#   BASE1_B23_KERNEL=/path/to/vmlinuz
#   BASE1_B23_INITRD=/path/to/initrd.img
#
# Safety:
#   This erases only the selected USB disk, refuses to run without
#   YES_WRITE_USB, and refuses /dev/sda by default.
#
# Non-claims:
#   External USB GNU/Linux runtime only. No installer claim. No internal disk
#   install. No recovery-complete claim. No hardening claim. No daily-driver
#   claim.

set -eu

USB="${1:-/dev/sdb}"
CONFIRM="${2:-}"
PROFILE="${BASE1_B23_PROFILE:-x200-supervisor-lite}"
OUT_DIR="${BASE1_B23_OUT:-build/base1-b23-gnulinux-runtime}"
KERNEL="${BASE1_B23_KERNEL:-build/linux/alpine-netboot/vmlinuz}"
INITRD="${BASE1_B23_INITRD:-build/linux/alpine-netboot/initrd.img}"
REPORT="$OUT_DIR/b23-gnulinux-runtime-usb.env"
OVERLAY_DIR="$OUT_DIR/initramfs-overlay"
OVERLAY="$OUT_DIR/phase1-overlay.cpio.gz"

fail() { printf 'x200-b23-gnulinux-runtime-usb: %s\n' "$1" >&2; exit 1; }
need_cmd() { command -v "$1" >/dev/null 2>&1 || fail "missing command: $1"; }

partition_one() {
  case "$1" in
    /dev/nvme[0-9]n[0-9]|/dev/mmcblk[0-9]) printf '%sp1\n' "$1" ;;
    /dev/sd[a-z]|/dev/vd[a-z]|/dev/hd[a-z]) printf '%s1\n' "$1" ;;
    *) fail "use a whole-disk path such as /dev/sdb, not a partition" ;;
  esac
}

[ "$CONFIRM" = "YES_WRITE_USB" ] || fail "usage: sh scripts/x200-b23-gnulinux-runtime-usb.sh /dev/sdb YES_WRITE_USB"
[ -b "$USB" ] || fail "not a block device: $USB"
[ "$USB" != "/dev/sda" ] || fail "refusing /dev/sda because it is commonly the internal disk"
[ -f "$KERNEL" ] || fail "missing GNU/Linux kernel: $KERNEL"
[ -f "$INITRD" ] || fail "missing GNU/Linux initrd: $INITRD"

ROOT_SRC="$(findmnt -no SOURCE / 2>/dev/null || true)"
case "$ROOT_SRC" in
  "$USB"|"$USB"[0-9]*|"$USB"p[0-9]*) fail "refusing to write the root filesystem device: $ROOT_SRC" ;;
esac

for cmd in sudo parted mkfs.vfat mount umount sync mkdir cp tee grub-install sha256sum find cpio gzip chmod date; do
  need_cmd "$cmd"
done

PART1="$(partition_one "$USB")"
MNT="$(mktemp -d)"

cleanup() { sudo umount "$MNT" 2>/dev/null || true; rmdir "$MNT" 2>/dev/null || true; }
trap cleanup EXIT INT TERM

mkdir -p "$OUT_DIR"
rm -rf "$OVERLAY_DIR"
mkdir -p \
  "$OVERLAY_DIR/bin" \
  "$OVERLAY_DIR/dev" \
  "$OVERLAY_DIR/etc" \
  "$OVERLAY_DIR/proc" \
  "$OVERLAY_DIR/sys" \
  "$OVERLAY_DIR/tmp" \
  "$OVERLAY_DIR/phase1" \
  "$OVERLAY_DIR/phase1/evidence" \
  "$OVERLAY_DIR/phase1/workspace"

cat > "$OVERLAY_DIR/init" <<'EOF'
#!/bin/sh
# Phase1 B23 initramfs entrypoint.
# This is a GNU/Linux-backed Phase1 runtime shell. It does not install to disk.

PATH=/bin:/sbin:/usr/bin:/usr/sbin
export PATH

mount -t proc proc /proc 2>/dev/null || true
mount -t sysfs sys /sys 2>/dev/null || true
mount -t devtmpfs dev /dev 2>/dev/null || mount -t tmpfs dev /dev 2>/dev/null || true
mount -t tmpfs tmpfs /tmp 2>/dev/null || true
mkdir -p /phase1/evidence /phase1/workspace

cat > /phase1/evidence/b23-runtime.env <<'ENV'
BASE1_B23_GNULINUX_RUNTIME_MODE=initramfs
BASE1_B23_GNULINUX_RUNTIME_RESULT=phase1_gnulinux_shell_seen
BASE1_B23_GNULINUX_RUNTIME_CLAIM=not_claimed
BASE1_B23_NON_CLAIM_INSTALLER=1
BASE1_B23_NON_CLAIM_INTERNAL_DISK_INSTALL=1
BASE1_B23_NON_CLAIM_RECOVERY_COMPLETE=1
BASE1_B23_NON_CLAIM_HARDENED=1
BASE1_B23_NON_CLAIM_DAILY_DRIVER=1
ENV

clear 2>/dev/null || true
cat <<'BANNER'
phase1 6.0.0 ready
Base1 B23 GNU/Linux runtime
result: phase1_gnulinux_shell_seen

Route: Libreboot -> SeaBIOS -> USB GRUB -> GNU/Linux
Mode : external USB initramfs runtime
Disk : internal disk not touched
Install: no
Daily-driver claim: no

Type: help, status, evidence, shell, reboot, poweroff
BANNER

phase1_status() {
  echo ""
  echo "Phase1 status"
  echo "Runtime : GNU/Linux initramfs"
  echo "Workspace: /phase1/workspace"
  echo "Evidence : /phase1/evidence/b23-runtime.env"
  echo "Claims   : not installer, not daily-driver"
  echo ""
}

phase1_help() {
  echo ""
  echo "Commands:"
  echo "  help      show this help"
  echo "  status    show runtime status"
  echo "  evidence  print evidence file"
  echo "  shell     open /bin/sh"
  echo "  reboot    reboot machine"
  echo "  poweroff  power off machine"
  echo ""
}

while true; do
  printf 'phase1> '
  read cmd || cmd=shell
  case "$cmd" in
    help|h) phase1_help ;;
    status|s) phase1_status ;;
    evidence|e) cat /phase1/evidence/b23-runtime.env ;;
    shell|sh) /bin/sh ;;
    reboot) echo b > /proc/sysrq-trigger 2>/dev/null || reboot -f ;;
    poweroff|halt) poweroff -f 2>/dev/null || halt -f ;;
    "") : ;;
    *) echo "unknown command: $cmd"; echo "type: help" ;;
  esac
done
EOF
chmod 0755 "$OVERLAY_DIR/init"

cat > "$OVERLAY_DIR/etc/motd" <<'EOF'
phase1 6.0.0 ready - B23 GNU/Linux runtime
EOF

printf 'PHASE1 BASE1 B23 GNU/LINUX RUNTIME USB WRITER\n\n'
printf 'profile     : %s\n' "$PROFILE"
printf 'target disk : %s\n' "$USB"
printf 'grub part   : %s\n' "$PART1"
printf 'kernel      : %s\n' "$KERNEL"
printf 'initrd      : %s\n' "$INITRD"
printf 'boot path   : Libreboot -> SeaBIOS -> USB GRUB -> GNU/Linux\n'
printf 'claim       : not_claimed\n\n'
printf 'Building Phase1 initramfs overlay...\n'

( cd "$OVERLAY_DIR" && find . | cpio -H newc -o 2>/dev/null | gzip -9 > "../phase1-overlay.cpio.gz" )
[ -s "$OVERLAY" ] || fail "failed to build overlay: $OVERLAY"

printf 'Kernel SHA256:\n'
sha256sum "$KERNEL"
printf 'Initrd SHA256:\n'
sha256sum "$INITRD"
printf 'Overlay SHA256:\n'
sha256sum "$OVERLAY"
printf '\nThis will erase the selected USB target.\n\n'

sudo umount "${USB}"* 2>/dev/null || true
sudo parted -s "$USB" mklabel msdos
sudo parted -s "$USB" mkpart primary fat32 1MiB 100%
sudo parted -s "$USB" set 1 boot on
sudo partprobe "$USB" 2>/dev/null || true
sync
sleep 2
[ -b "$PART1" ] || fail "partition did not appear: $PART1"

sudo mkfs.vfat -F 32 -n PHASE1B23 "$PART1"
sudo mount "$PART1" "$MNT"
sudo mkdir -p \
  "$MNT/boot/grub" \
  "$MNT/grub" \
  "$MNT/boot/phase1" \
  "$MNT/phase1" \
  "$MNT/phase1/evidence" \
  "$MNT/phase1/help"

sudo cp "$KERNEL" "$MNT/boot/phase1/vmlinuz"
sudo cp "$INITRD" "$MNT/boot/phase1/initrd.img"
sudo cp "$OVERLAY" "$MNT/boot/phase1/phase1-overlay.cpio.gz"

sudo tee "$MNT/phase1/README.txt" >/dev/null <<EOF
Phase1 Base1 B23 GNU/Linux runtime USB
Profile: $PROFILE
Boot path: Libreboot -> Load SeaBIOS payload -> USB GRUB -> GNU/Linux

This USB boots a GNU/Linux initramfs-backed Phase1 shell.
It is not an installer and does not write the internal disk.
EOF

sudo tee "$MNT/phase1/evidence/b23-runtime-prep.env" >/dev/null <<EOF
BASE1_B23_GNULINUX_RUNTIME_PROFILE=$PROFILE
BASE1_B23_GNULINUX_RUNTIME_BOOT_PATH=Libreboot_Load_SeaBIOS_payload_to_USB_GRUB_to_GNULinux
BASE1_B23_GNULINUX_RUNTIME_EXPECTED_RESULT=phase1_gnulinux_shell_seen
BASE1_B23_GNULINUX_RUNTIME_KERNEL=/boot/phase1/vmlinuz
BASE1_B23_GNULINUX_RUNTIME_INITRD=/boot/phase1/initrd.img
BASE1_B23_GNULINUX_RUNTIME_OVERLAY=/boot/phase1/phase1-overlay.cpio.gz
BASE1_B23_GNULINUX_RUNTIME_CLAIM=not_claimed
BASE1_B23_NON_CLAIM_INSTALLER=1
BASE1_B23_NON_CLAIM_INTERNAL_DISK_INSTALL=1
BASE1_B23_NON_CLAIM_RECOVERY_COMPLETE=1
BASE1_B23_NON_CLAIM_HARDENED=1
BASE1_B23_NON_CLAIM_DAILY_DRIVER=1
EOF

sudo tee "$MNT/boot/grub/grub.cfg" >/dev/null <<'EOF'
# Phase1 Base1 B23 GNU/Linux runtime USB.
# Proven route: Libreboot -> SeaBIOS payload -> USB GRUB.
# GRUB is a loader only; GNU/Linux is the runtime surface.

set timeout=10
set default=0
set pager=1
set color_normal=white/black
set color_highlight=black/light-gray
terminal_input console
terminal_output console

menuentry "Start Phase1 GNU/Linux" {
    clear
    echo "phase1 6.0.0 ready"
    echo "Loading GNU/Linux-backed Phase1 runtime..."
    echo "Result target: phase1_gnulinux_shell_seen"
    echo "Internal disk will not be mounted by Phase1."
    linux /boot/phase1/vmlinuz console=tty0 nomodeset quiet loglevel=3
    initrd /boot/phase1/initrd.img /boot/phase1/phase1-overlay.cpio.gz
    boot
}

menuentry "Start Phase1 GNU/Linux - verbose" {
    clear
    echo "Loading verbose GNU/Linux-backed Phase1 runtime..."
    linux /boot/phase1/vmlinuz console=tty0 nomodeset loglevel=7
    initrd /boot/phase1/initrd.img /boot/phase1/phase1-overlay.cpio.gz
    boot
}

menuentry "Phase1 GRUB fallback" {
    clear
    echo "phase1 6.0.0 ready"
    echo "Base1 B23 GRUB fallback console"
    echo "Use this if GNU/Linux loading is blocked."
    echo "Result if only this works: phase1_polished_system_seen"
    echo ""
    echo "ESC returns."
    sleep --interruptible 999
}

menuentry "B22 polished console fallback" {
    clear
    echo "phase1 6.0.0 ready"
    echo "Base1 B22 polished fallback"
    echo "Route: SeaBIOS -> USB GRUB"
    echo "Internal disk: not touched"
    echo "Installer: no"
    echo "Daily-driver claim: no"
    sleep --interruptible 999
}

menuentry "GNU/Linux file check" {
    clear
    echo "B23 file check"
    echo ""
    echo "/boot/phase1"
    ls /boot/phase1
    echo ""
    echo "/phase1/evidence"
    ls /phase1/evidence
    echo ""
    echo "ESC returns."
    sleep --interruptible 999
}

menuentry "Safety" {
    clear
    echo "B23 safety boundary"
    echo ""
    echo "External USB only."
    echo "No internal disk write."
    echo "No installer claim."
    echo "No recovery-complete claim."
    echo "No hardening claim."
    echo "No daily-driver claim."
    echo ""
    echo "ESC returns."
    sleep --interruptible 999
}

menuentry "Record Help" {
    clear
    echo "If GNU/Linux Phase1 shell appears:"
    echo ""
    echo "cd ~/phase1"
    echo "git pull --ff-only origin edge/stable"
    echo "sh scripts/x200-record-and-share-result-safe.sh phase1_gnulinux_shell_seen"
    echo ""
    echo "Serve ~/phase1-share only."
    echo "Use <X200_IP>, not hard-coded IPs."
    echo ""
    echo "ESC returns."
    sleep --interruptible 999
}

menuentry "Reboot" {
    reboot
}

menuentry "Power off" {
    halt
}
EOF

sudo cp "$MNT/boot/grub/grub.cfg" "$MNT/grub/grub.cfg"

printf 'Installing GRUB BIOS bootloader to %s...\n' "$USB"
sudo grub-install --target=i386-pc --boot-directory="$MNT/boot" --recheck "$USB"
sync
sudo umount "$MNT"
rmdir "$MNT"
trap - EXIT INT TERM

cat > "$REPORT" <<EOF
BASE1_B23_GNULINUX_RUNTIME_PROFILE=$PROFILE
BASE1_B23_GNULINUX_RUNTIME_TARGET=$USB
BASE1_B23_GNULINUX_RUNTIME_PARTITION=$PART1
BASE1_B23_GNULINUX_RUNTIME_KERNEL=$KERNEL
BASE1_B23_GNULINUX_RUNTIME_INITRD=$INITRD
BASE1_B23_GNULINUX_RUNTIME_OVERLAY=$OVERLAY
BASE1_B23_GNULINUX_RUNTIME_RESULT=prepared
BASE1_B23_GNULINUX_RUNTIME_EXPECTED_NEXT_RESULT=phase1_gnulinux_shell_seen
BASE1_B23_GNULINUX_RUNTIME_BOOT_PATH=Libreboot_Load_SeaBIOS_payload_to_USB_GRUB_to_GNULinux
BASE1_B23_GNULINUX_RUNTIME_DISPLAY_POLICY=linux_console_owns_runtime_display
BASE1_B23_GNULINUX_RUNTIME_CLAIM=not_claimed
BASE1_B23_NON_CLAIM_INSTALLER=1
BASE1_B23_NON_CLAIM_INTERNAL_DISK_INSTALL=1
BASE1_B23_NON_CLAIM_RECOVERY_COMPLETE=1
BASE1_B23_NON_CLAIM_HARDENED=1
BASE1_B23_NON_CLAIM_DAILY_DRIVER=1
EOF

printf '\nDONE: B23 GNU/Linux Phase1 runtime USB prepared on %s\n' "$USB"
printf 'Boot path: Libreboot main menu -> Load SeaBIOS (payload).\n'
printf 'Choose: Start Phase1 GNU/Linux.\n'
printf 'If the Phase1 shell appears, record: phase1_gnulinux_shell_seen\n'

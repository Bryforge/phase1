#!/usr/bin/env sh
# Phase1 / Base1 X200 B29 Linux-libre baseline USB writer.
#
# Purpose:
#   Use the X200's known local GNU/Linux / Linux-libre style kernel/initrd as
#   the baseline handoff target before returning to Alpine hardened.
#
# Reasoning:
#   This Libreboot machine has historically worked best with Libreboot-friendly
#   Linux-libre style kernels. B29 therefore tests the host kernel/initrd with
#   linux16 first, then normal linux fallback.
#
# Route:
#   Libreboot -> SeaBIOS payload -> USB GRUB -> host Linux-libre baseline
#
# Safety:
#   This erases only the selected USB disk, refuses to run without
#   YES_WRITE_USB, and refuses /dev/sda by default.

set -eu

USB="${1:-/dev/sdb}"
CONFIRM="${2:-}"
PROFILE="${BASE1_B29_PROFILE:-x200-supervisor-lite}"
OUT_DIR="${BASE1_B29_OUT:-build/base1-b29-linux-libre-baseline}"
KERNEL="${BASE1_B29_KERNEL:-build/linux/alpine-netboot/vmlinuz}"
INITRD="${BASE1_B29_INITRD:-build/linux/alpine-netboot/initrd.img}"
REPORT="$OUT_DIR/b29-linux-libre-baseline-usb.env"

fail() { printf 'x200-b29-linux-libre-baseline-usb: %s\n' "$1" >&2; exit 1; }
need_cmd() { command -v "$1" >/dev/null 2>&1 || fail "missing command: $1"; }

partition_one() {
  case "$1" in
    /dev/nvme[0-9]n[0-9]|/dev/mmcblk[0-9]) printf '%sp1\n' "$1" ;;
    /dev/sd[a-z]|/dev/vd[a-z]|/dev/hd[a-z]) printf '%s1\n' "$1" ;;
    *) fail "use a whole-disk path such as /dev/sdb, not a partition" ;;
  esac
}

[ "$CONFIRM" = "YES_WRITE_USB" ] || fail "usage: sh scripts/x200-b29-linux-libre-baseline-usb.sh /dev/sdb YES_WRITE_USB"
[ -b "$USB" ] || fail "not a block device: $USB"
[ "$USB" != "/dev/sda" ] || fail "refusing /dev/sda because it is commonly the internal disk"
[ -f "$KERNEL" ] || fail "missing Linux-libre baseline kernel: $KERNEL"
[ -f "$INITRD" ] || fail "missing Linux-libre baseline initrd: $INITRD"

ROOT_SRC="$(findmnt -no SOURCE / 2>/dev/null || true)"
case "$ROOT_SRC" in
  "$USB"|"$USB"[0-9]*|"$USB"p[0-9]*) fail "refusing to write the root filesystem device: $ROOT_SRC" ;;
esac

for cmd in sudo parted mkfs.vfat mount umount sync mkdir cp tee grub-install sha256sum stat; do
  need_cmd "$cmd"
done

PART1="$(partition_one "$USB")"
MNT="$(mktemp -d)"

cleanup() { sudo umount "$MNT" 2>/dev/null || true; rmdir "$MNT" 2>/dev/null || true; }
trap cleanup EXIT INT TERM

mkdir -p "$OUT_DIR"

printf 'PHASE1 BASE1 B29 LINUX-LIBRE BASELINE USB WRITER\n\n'
printf 'profile     : %s\n' "$PROFILE"
printf 'target disk : %s\n' "$USB"
printf 'kernel      : %s\n' "$KERNEL"
printf 'initrd      : %s\n' "$INITRD"
printf 'scope       : host Linux-libre baseline handoff\n\n'
printf 'Kernel SHA256:\n'
sha256sum "$KERNEL"
printf 'Initrd SHA256:\n'
sha256sum "$INITRD"
printf '\nThis will erase the selected USB target.\n\n'

sudo umount "${USB}"* 2>/dev/null || true
sudo parted -s "$USB" mklabel msdos
sudo parted -s "$USB" mkpart primary fat32 1MiB 100%
sudo parted -s "$USB" set 1 boot on
sudo partprobe "$USB" 2>/dev/null || true
sync
sleep 2
[ -b "$PART1" ] || fail "partition did not appear: $PART1"

sudo mkfs.vfat -F 32 -n PHASE1B29 "$PART1"
sudo mount "$PART1" "$MNT"
sudo mkdir -p "$MNT/boot/grub" "$MNT/grub" "$MNT/boot/phase1" "$MNT/phase1/evidence"
sudo cp "$KERNEL" "$MNT/boot/phase1/vmlinuz"
sudo cp "$INITRD" "$MNT/boot/phase1/initrd.img"

KERNEL_SIZE="$(stat -c %s "$KERNEL" 2>/dev/null || stat -f %z "$KERNEL")"
INITRD_SIZE="$(stat -c %s "$INITRD" 2>/dev/null || stat -f %z "$INITRD")"

sudo tee "$MNT/phase1/evidence/b29-prep.env" >/dev/null <<EOF
BASE1_B29_LINUX_LIBRE_BASELINE_PROFILE=$PROFILE
BASE1_B29_LINUX_LIBRE_BASELINE_KERNEL=/boot/phase1/vmlinuz
BASE1_B29_LINUX_LIBRE_BASELINE_INITRD=/boot/phase1/initrd.img
BASE1_B29_LINUX_LIBRE_BASELINE_KERNEL_SIZE=$KERNEL_SIZE
BASE1_B29_LINUX_LIBRE_BASELINE_INITRD_SIZE=$INITRD_SIZE
BASE1_B29_LINUX_LIBRE_BASELINE_EXPECTED_RESULT=phase1_linux_libre_baseline_seen
BASE1_B29_LINUX_LIBRE_BASELINE_CLAIM=not_claimed
EOF

sudo tee "$MNT/boot/grub/grub.cfg" >/dev/null <<'EOF'
set timeout=-1
set default=0
set pager=1
set color_normal=white/black
set color_highlight=black/light-gray
terminal_input console
terminal_output console

menuentry "B29 File check" {
    clear
    echo "B29 Linux-libre baseline file check"
    echo ""
    ls -lh /boot/phase1
    echo ""
    cat /phase1/evidence/b29-prep.env
    echo ""
    sleep --interruptible 999
}

menuentry "B29 Test linux16 command only" {
    clear
    echo "B29 test: before linux16 command"
    linux16 /boot/phase1/vmlinuz root=/dev/ram0 ro console=tty0 nomodeset loglevel=7 panic=30
    echo "B29 test: linux16 command returned"
    echo "If this appears, GRUB accepted the Linux-libre baseline kernel."
    echo "No boot performed in this entry. ESC returns."
    sleep --interruptible 999
}

menuentry "B29 Test linux16 plus initrd16" {
    clear
    echo "B29 test: before linux16 command"
    linux16 /boot/phase1/vmlinuz root=/dev/ram0 ro console=tty0 nomodeset loglevel=7 panic=30
    echo "B29 test: linux16 command returned"
    echo "B29 test: before initrd16 command"
    initrd16 /boot/phase1/initrd.img
    echo "B29 test: initrd16 command returned"
    echo "No boot performed in this entry. ESC returns."
    sleep --interruptible 999
}

menuentry "B29 Boot linux16 baseline" {
    clear
    echo "B29 boot: linux16 baseline"
    echo "Target: phase1_linux_libre_baseline_seen"
    linux16 /boot/phase1/vmlinuz root=/dev/ram0 ro console=tty0 nomodeset loglevel=7 panic=30
    echo "linux16 returned; loading initrd16"
    initrd16 /boot/phase1/initrd.img
    echo "initrd16 returned; booting now"
    boot
}

menuentry "B29 Test normal linux command only" {
    clear
    echo "B29 test: before normal linux command"
    linux /boot/phase1/vmlinuz root=/dev/ram0 ro console=tty0 nomodeset loglevel=7 panic=30
    echo "B29 test: normal linux command returned"
    echo "No boot performed in this entry. ESC returns."
    sleep --interruptible 999
}

menuentry "B29 Boot normal linux baseline" {
    clear
    echo "B29 boot: normal linux baseline"
    linux /boot/phase1/vmlinuz root=/dev/ram0 ro console=tty0 nomodeset loglevel=7 panic=30
    echo "linux returned; loading initrd"
    initrd /boot/phase1/initrd.img
    echo "initrd returned; booting now"
    boot
}

menuentry "B29 GRUB fallback" {
    clear
    echo "phase1 6.0.0 ready"
    echo "B29 GRUB fallback console"
    echo "Use if Linux-libre baseline remains blocked."
    sleep --interruptible 999
}

menuentry "Reboot" { reboot }
menuentry "Power off" { halt }
EOF
sudo cp "$MNT/boot/grub/grub.cfg" "$MNT/grub/grub.cfg"

printf 'Installing GRUB BIOS bootloader to %s...\n' "$USB"
sudo grub-install --target=i386-pc --boot-directory="$MNT/boot" --recheck "$USB"
sync
sudo umount "$MNT"
rmdir "$MNT"
trap - EXIT INT TERM

cat > "$REPORT" <<EOF
BASE1_B29_LINUX_LIBRE_BASELINE_TARGET=$USB
BASE1_B29_LINUX_LIBRE_BASELINE_PARTITION=$PART1
BASE1_B29_LINUX_LIBRE_BASELINE_KERNEL=$KERNEL
BASE1_B29_LINUX_LIBRE_BASELINE_INITRD=$INITRD
BASE1_B29_LINUX_LIBRE_BASELINE_RESULT=prepared
BASE1_B29_LINUX_LIBRE_BASELINE_EXPECTED_NEXT_RESULT=phase1_linux_libre_baseline_seen
EOF

printf '\nDONE: B29 Linux-libre baseline USB prepared on %s\n' "$USB"
printf 'Boot path: Libreboot -> Load SeaBIOS payload.\n'
printf 'Use B29 Test linux16 command only first.\n'

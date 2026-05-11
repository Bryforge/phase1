#!/usr/bin/env sh
# Phase1 / Base1 X200 B7 full diagnostic USB writer.
#
# Purpose:
#   Build one external USB that contains multiple professional, evidence-oriented
#   boot attempts for the Libreboot/Coreboot ThinkPad X200 path.
#
# Safety:
#   This script erases only the selected USB disk, refuses to run without the
#   YES_WRITE_USB confirmation token, and refuses /dev/sda by default.
#
# Non-claims:
#   This is not an installer, not an internal disk install, not recovery-complete,
#   not hardened, not hardware-validated, and not daily-driver ready.

set -eu

USB="${1:-/dev/sdb}"
CONFIRM="${2:-}"
PROFILE="${BASE1_B7_PROFILE:-x200-supervisor-lite}"
KERNEL="${BASE1_B7_KERNEL:-build/linux/alpine-netboot/vmlinuz}"
INITRD="${BASE1_B7_INITRD:-build/linux/alpine-netboot/initrd.img}"
OUT_DIR="${BASE1_B7_DIAGNOSTIC_OUT:-build/base1-b7-full-diagnostic-usb}"

fail() {
  printf 'x200-b7-full-diagnostic-usb: %s\n' "$1" >&2
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

[ "$CONFIRM" = "YES_WRITE_USB" ] || fail "usage: sh scripts/x200-b7-full-diagnostic-usb.sh /dev/sdb YES_WRITE_USB"
[ -b "$USB" ] || fail "not a block device: $USB"
[ "$USB" != "/dev/sda" ] || fail "refusing /dev/sda because it is commonly the internal disk"
[ -f "$KERNEL" ] || fail "missing kernel: $KERNEL"
[ -f "$INITRD" ] || fail "missing initrd: $INITRD"

ROOT_SRC="$(findmnt -no SOURCE / 2>/dev/null || true)"
case "$ROOT_SRC" in
  "$USB"|"$USB"[0-9]*|"$USB"p[0-9]*) fail "refusing to write the root filesystem device: $ROOT_SRC" ;;
esac

for cmd in sudo parted mkfs.vfat mount umount sync mkdir cp tee grub-install sha256sum; do
  need_cmd "$cmd"
done

PART="$(partition_one "$USB")"
MNT="$(mktemp -d)"
REPORT="$OUT_DIR/b7-full-diagnostic-usb.env"

cleanup() {
  sudo umount "$MNT" 2>/dev/null || true
  rmdir "$MNT" 2>/dev/null || true
}
trap cleanup EXIT INT TERM

printf 'PHASE1 BASE1 B7 FULL DIAGNOSTIC USB WRITER\n\n'
printf 'profile     : %s\n' "$PROFILE"
printf 'target disk : %s\n' "$USB"
printf 'partition   : %s\n' "$PART"
printf 'kernel      : %s\n' "$KERNEL"
printf 'initrd      : %s\n' "$INITRD"
printf 'scope       : external USB hardware evidence only\n'
printf 'claim       : not_claimed\n\n'
printf 'This will erase the selected USB target.\n\n'

printf 'Kernel SHA256:\n'
sha256sum "$KERNEL"
printf 'Initrd SHA256:\n'
sha256sum "$INITRD"
printf '\n'

sudo umount "${USB}"* 2>/dev/null || true
sudo parted -s "$USB" mklabel msdos
sudo parted -s "$USB" mkpart primary fat32 1MiB 100%
sudo parted -s "$USB" set 1 boot on
sudo partprobe "$USB" 2>/dev/null || true
sync
sleep 2

[ -b "$PART" ] || fail "partition did not appear: $PART"

sudo mkfs.vfat -F 32 -n PHASE1B7 "$PART"
sudo mount "$PART" "$MNT"
sudo mkdir -p "$MNT/boot/grub" "$MNT/grub" "$MNT/boot/phase1" "$MNT/phase1-evidence"
sudo cp "$KERNEL" "$MNT/boot/phase1/vmlinuz"
sudo cp "$INITRD" "$MNT/boot/phase1/initrd.img"

sudo tee "$MNT/boot/grub/grub.cfg" >/dev/null <<'EOF'
set timeout=20
set default=0
set pager=1

menuentry "Phase1 Base1 B7 - kernel/initrd, safe text mode" {
    set gfxpayload=text
    echo "Phase1 Base1 B7 external USB diagnostic"
    echo "Attempt: kernel/initrd, safe text mode"
    echo "Evidence: kernel output or panic means boot_started"
    linux /boot/phase1/vmlinuz console=tty0 nomodeset vga=normal loglevel=7 debug earlyprintk=vga keep_bootcon
    initrd /boot/phase1/initrd.img
}

menuentry "Phase1 Base1 B7 - kernel/initrd, legacy IRQ and ACPI fallback" {
    set gfxpayload=text
    echo "Phase1 Base1 B7 external USB diagnostic"
    echo "Attempt: legacy IRQ and ACPI fallback"
    echo "Evidence: kernel output or panic means boot_started"
    linux /boot/phase1/vmlinuz console=tty0 nomodeset vga=normal noapic nolapic acpi=off irqpoll pci=nomsi loglevel=7 debug earlyprintk=vga keep_bootcon
    initrd /boot/phase1/initrd.img
}

menuentry "Phase1 Base1 B7 - kernel/initrd, minimal kernel arguments" {
    echo "Phase1 Base1 B7 external USB diagnostic"
    echo "Attempt: minimal kernel arguments"
    linux /boot/phase1/vmlinuz
    initrd /boot/phase1/initrd.img
}

menuentry "Phase1 Base1 B7 - kernel/initrd, conservative memory fallback" {
    set gfxpayload=text
    echo "Phase1 Base1 B7 external USB diagnostic"
    echo "Attempt: conservative memory fallback"
    linux /boot/phase1/vmlinuz console=tty0 nomodeset vga=normal mem=768M loglevel=7 debug earlyprintk=vga keep_bootcon
    initrd /boot/phase1/initrd.img
}

menuentry "Phase1 Base1 B7 - kernel only execution probe" {
    set gfxpayload=text
    echo "Phase1 Base1 B7 external USB diagnostic"
    echo "Attempt: kernel only, no initrd"
    echo "Expected panic is useful evidence of kernel execution."
    linux /boot/phase1/vmlinuz console=tty0 nomodeset vga=normal loglevel=7 debug earlyprintk=vga keep_bootcon
}

menuentry "Phase1 Base1 B7 - kernel only, legacy IRQ and ACPI fallback" {
    set gfxpayload=text
    echo "Phase1 Base1 B7 external USB diagnostic"
    echo "Attempt: kernel only with legacy fallback"
    echo "Expected panic is useful evidence of kernel execution."
    linux /boot/phase1/vmlinuz console=tty0 nomodeset vga=normal noapic nolapic acpi=off irqpoll pci=nomsi loglevel=7 debug earlyprintk=vga keep_bootcon
}

menuentry "Phase1 Base1 B6 - marker fallback" {
    echo "phase1 6.0.0 ready"
    echo "B6 candidate result: phase1_marker_seen"
    echo "B7 fallback marker reached from external USB"
    echo "This is not a kernel boot, installer, hardening, or daily-driver claim."
    sleep --interruptible 999
}
EOF

sudo cp "$MNT/boot/grub/grub.cfg" "$MNT/grub/grub.cfg"

sudo tee "$MNT/PHASE1-B7-DIAGNOSTIC.txt" >/dev/null <<EOF
Phase1 Base1 B7 full diagnostic USB.
Profile: $PROFILE
Kernel: /boot/phase1/vmlinuz
Initrd: /boot/phase1/initrd.img

Evidence states:
- boot_started: kernel output or kernel panic appears.
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
BASE1_B7_DIAGNOSTIC_USB_PROFILE=$PROFILE
BASE1_B7_DIAGNOSTIC_USB_TARGET=$USB
BASE1_B7_DIAGNOSTIC_USB_PARTITION=$PART
BASE1_B7_DIAGNOSTIC_USB_KERNEL=$KERNEL
BASE1_B7_DIAGNOSTIC_USB_INITRD=$INITRD
BASE1_B7_DIAGNOSTIC_USB_RESULT=prepared
BASE1_B7_DIAGNOSTIC_USB_CLAIM=not_claimed
BASE1_B7_NON_CLAIM_INSTALLER=1
BASE1_B7_NON_CLAIM_INTERNAL_DISK_INSTALL=1
BASE1_B7_NON_CLAIM_RECOVERY_COMPLETE=1
BASE1_B7_NON_CLAIM_HARDENED=1
BASE1_B7_NON_CLAIM_DAILY_DRIVER=1
EOF

printf '\nDONE: B7 full diagnostic USB prepared on %s\n' "$USB"
printf 'Boot the X200 from USB and try the menu entries from top to bottom.\n'
printf 'After the run, record the best observed result with scripts/x200-b7-record-result.sh.\n'

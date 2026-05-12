#!/usr/bin/env sh
# Phase1 / Base1 X200 B32 initrd break diagnostic USB writer.
#
# Purpose:
#   B31 proved splash, linux16, and initrd16 loading. The actual boot black
#   screened and returned to Libreboot, which means the failure is after GRUB
#   hands off to Linux/initrd. B32 boots the same full initrd with break/debug
#   arguments and disables automatic reboot so failures remain visible.
#
# Route:
#   Libreboot GRUB -> external USB -> linux16 -> initrd16 -> initrd break shell
#
# Safety:
#   This erases only the selected USB disk, refuses to run without
#   YES_WRITE_USB, and refuses /dev/sda by default.

set -eu

USB="${1:-/dev/sdb}"
CONFIRM="${2:-}"
PROFILE="${BASE1_B32_PROFILE:-x200-supervisor-lite}"
OUT_DIR="${BASE1_B32_OUT:-build/base1-b32-initrd-break-diagnostic}"
KERNEL="${BASE1_B32_KERNEL:-build/linux/alpine-netboot/vmlinuz}"
INITRD="${BASE1_B32_INITRD:-build/linux/alpine-netboot/initrd.img}"
REPORT="$OUT_DIR/b32-initrd-break-diagnostic-usb.env"
SPLASH="$OUT_DIR/phase1-splash.tga"

fail() { printf 'x200-b32-initrd-break-diagnostic-usb: %s\n' "$1" >&2; exit 1; }
need_cmd() { command -v "$1" >/dev/null 2>&1 || fail "missing command: $1"; }

partition_one() {
  case "$1" in
    /dev/nvme[0-9]n[0-9]|/dev/mmcblk[0-9]) printf '%sp1\n' "$1" ;;
    /dev/sd[a-z]|/dev/vd[a-z]|/dev/hd[a-z]) printf '%s1\n' "$1" ;;
    *) fail "use a whole-disk path such as /dev/sdb, not a partition" ;;
  esac
}

[ "$CONFIRM" = "YES_WRITE_USB" ] || fail "usage: sh scripts/x200-b32-initrd-break-diagnostic-usb.sh /dev/sdb YES_WRITE_USB"
[ -b "$USB" ] || fail "not a block device: $USB"
[ "$USB" != "/dev/sda" ] || fail "refusing /dev/sda because it is commonly the internal disk"
[ -f "$KERNEL" ] || fail "missing Linux-libre baseline kernel: $KERNEL"
[ -f "$INITRD" ] || fail "missing full Linux-libre baseline initrd: $INITRD"

ROOT_SRC="$(findmnt -no SOURCE / 2>/dev/null || true)"
case "$ROOT_SRC" in
  "$USB"|"$USB"[0-9]*|"$USB"p[0-9]*) fail "refusing to write the root filesystem device: $ROOT_SRC" ;;
esac

for cmd in sudo parted mkfs.vfat mount umount sync mkdir cp tee grub-install sha256sum stat date; do
  need_cmd "$cmd"
done

PART1="$(partition_one "$USB")"
MNT="$(mktemp -d)"

cleanup() { sudo umount "$MNT" 2>/dev/null || true; rmdir "$MNT" 2>/dev/null || true; }
trap cleanup EXIT INT TERM

mkdir -p "$OUT_DIR"

if command -v python3 >/dev/null 2>&1; then
  python3 - "$SPLASH" <<'PY'
import struct, sys
path = sys.argv[1]
w, h = 640, 480
header = bytearray(18)
header[2] = 2
header[12:14] = struct.pack('<H', w)
header[14:16] = struct.pack('<H', h)
header[16] = 24
header[17] = 0x20
pixels = bytearray()
for y in range(h):
    for x in range(w):
        r = 10 + (x * 25 // w)
        g = 10 + (y * 20 // h)
        b = 30 + ((x + y) * 55 // (w + h))
        cx, cy = w // 2, h // 2
        dx, dy = x - cx, y - cy
        d = (dx*dx + dy*dy) ** 0.5
        if 145 < d < 151 or 205 < d < 211:
            r, g, b = 120, 220, 255
        if abs(y - cy) < 5 and 220 < x < 420:
            r, g, b = 245, 245, 255
        if abs(x - cx) < 5 and 195 < y < 285:
            r, g, b = 245, 245, 255
        pixels.extend((b, g, r))
with open(path, 'wb') as f:
    f.write(header)
    f.write(pixels)
PY
fi

printf 'PHASE1 BASE1 B32 INITRD BREAK DIAGNOSTIC USB WRITER\n\n'
printf 'profile     : %s\n' "$PROFILE"
printf 'target disk : %s\n' "$USB"
printf 'kernel      : %s\n' "$KERNEL"
printf 'initrd      : %s\n' "$INITRD"
printf 'scope       : post-handoff Linux/initrd diagnostic\n\n'
printf 'Kernel SHA256:\n'
sha256sum "$KERNEL"
printf 'Initrd SHA256:\n'
sha256sum "$INITRD"
if [ -f "$SPLASH" ]; then printf 'Splash SHA256:\n'; sha256sum "$SPLASH"; fi
printf '\nThis will erase the selected USB target.\n\n'

sudo umount "${USB}"* 2>/dev/null || true
sudo parted -s "$USB" mklabel msdos
sudo parted -s "$USB" mkpart primary fat32 1MiB 100%
sudo parted -s "$USB" set 1 boot on
sudo partprobe "$USB" 2>/dev/null || true
sync
sleep 2
[ -b "$PART1" ] || fail "partition did not appear: $PART1"

sudo mkfs.vfat -F 32 -n PHASE1B32 "$PART1"
sudo mount "$PART1" "$MNT"
sudo mkdir -p "$MNT/boot/grub" "$MNT/grub" "$MNT/boot/phase1" "$MNT/phase1/assets" "$MNT/phase1/evidence"
sudo cp "$KERNEL" "$MNT/boot/phase1/vmlinuz"
sudo cp "$INITRD" "$MNT/boot/phase1/initrd.img"
if [ -f "$SPLASH" ]; then sudo cp "$SPLASH" "$MNT/phase1/assets/phase1-splash.tga"; fi

KERNEL_SIZE="$(stat -c %s "$KERNEL" 2>/dev/null || stat -f %z "$KERNEL")"
INITRD_SIZE="$(stat -c %s "$INITRD" 2>/dev/null || stat -f %z "$INITRD")"

sudo tee "$MNT/phase1/evidence/b32-prep.env" >/dev/null <<EOF
BASE1_B32_INITRD_BREAK_PROFILE=$PROFILE
BASE1_B32_INITRD_BREAK_KERNEL=/boot/phase1/vmlinuz
BASE1_B32_INITRD_BREAK_INITRD=/boot/phase1/initrd.img
BASE1_B32_INITRD_BREAK_KERNEL_SIZE=$KERNEL_SIZE
BASE1_B32_INITRD_BREAK_INITRD_SIZE=$INITRD_SIZE
BASE1_B32_INITRD_BREAK_EXPECTED_RESULT=phase1_initrd_break_shell_seen
BASE1_B32_INITRD_BREAK_NEGATIVE_RESULT=reset_after_full_initrd_boot
BASE1_B32_INITRD_BREAK_CLAIM=not_claimed
EOF

sudo tee "$MNT/boot/grub/grub.cfg" >/dev/null <<'EOF'
set timeout=-1
set default=0
set pager=1
set color_normal=white/black
set color_highlight=black/light-gray
terminal_input console
terminal_output console

menuentry "B32 File check" {
    clear
    echo "B32 initrd break diagnostic file check"
    echo ""
    ls -lh /boot/phase1
    echo ""
    cat /phase1/evidence/b32-prep.env
    echo ""
    sleep --interruptible 999
}

menuentry "B32 Phase1 Splash" {
    clear
    echo "Trying Phase1 splash mode..."
    insmod all_video
    insmod gfxterm
    insmod tga
    set gfxmode=auto
    set gfxpayload=keep
    terminal_output gfxterm
    if background_image /phase1/assets/phase1-splash.tga; then
        echo "phase1 6.0.0 ready"
        echo "B32 Phase1 splash active"
        echo "result: phase1_boot_splash_seen"
        sleep --interruptible 999
    else
        terminal_output console
        clear
        echo "Splash unavailable. Use text entries."
        sleep --interruptible 999
    fi
    terminal_output console
}

menuentry "B32 Break initramfs top" {
    clear
    echo "B32 break=top diagnostic"
    echo "Target: phase1_initrd_break_shell_seen"
    linux16 /boot/phase1/vmlinuz root=/dev/ram0 ro console=tty0 nomodeset loglevel=7 panic=0 break=top
    echo "linux16 returned; loading initrd16"
    initrd16 /boot/phase1/initrd.img
    echo "initrd16 returned; booting now"
    boot
}

menuentry "B32 Break initramfs modules" {
    clear
    echo "B32 break=modules diagnostic"
    linux16 /boot/phase1/vmlinuz root=/dev/ram0 ro console=tty0 nomodeset loglevel=7 panic=0 break=modules
    echo "linux16 returned; loading initrd16"
    initrd16 /boot/phase1/initrd.img
    echo "initrd16 returned; booting now"
    boot
}

menuentry "B32 Break initramfs premount" {
    clear
    echo "B32 break=premount diagnostic"
    linux16 /boot/phase1/vmlinuz root=/dev/ram0 ro console=tty0 nomodeset loglevel=7 panic=0 break=premount
    echo "linux16 returned; loading initrd16"
    initrd16 /boot/phase1/initrd.img
    echo "initrd16 returned; booting now"
    boot
}

menuentry "B32 Boot no reboot debug" {
    clear
    echo "B32 no-reboot debug boot"
    linux16 /boot/phase1/vmlinuz root=/dev/ram0 ro console=tty0 nomodeset noapic nolapic acpi=off loglevel=7 panic=0
    echo "linux16 returned; loading initrd16"
    initrd16 /boot/phase1/initrd.img
    echo "initrd16 returned; booting now"
    boot
}

menuentry "B32 Boot normal linux fallback" {
    clear
    echo "B32 normal linux fallback"
    linux /boot/phase1/vmlinuz root=/dev/ram0 ro console=tty0 nomodeset noapic nolapic acpi=off loglevel=7 panic=0
    echo "linux returned; loading initrd"
    initrd /boot/phase1/initrd.img
    echo "initrd returned; booting now"
    boot
}

menuentry "B32 GRUB fallback" {
    clear
    echo "phase1 6.0.0 ready"
    echo "B32 GRUB fallback console"
    echo "Use if initrd break diagnostics reset."
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
BASE1_B32_INITRD_BREAK_TARGET=$USB
BASE1_B32_INITRD_BREAK_PARTITION=$PART1
BASE1_B32_INITRD_BREAK_KERNEL=$KERNEL
BASE1_B32_INITRD_BREAK_INITRD=$INITRD
BASE1_B32_INITRD_BREAK_RESULT=prepared
BASE1_B32_INITRD_BREAK_EXPECTED_NEXT_RESULT=phase1_initrd_break_shell_seen
EOF

printf '\nDONE: B32 initrd break diagnostic USB prepared on %s\n' "$USB"
printf 'Boot path: Libreboot -> external USB GRUB.\n'
printf 'Try B32 Break initramfs top first.\n'

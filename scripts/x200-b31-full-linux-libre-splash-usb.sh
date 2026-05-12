#!/usr/bin/env sh
# Phase1 / Base1 X200 B31 full Linux-libre splash USB writer.
#
# Purpose:
#   Prepare the full host Linux-libre baseline test with the large initrd and
#   a Phase1 GRUB-side boot splash attempt. B29 proved linux16 accepts the
#   kernel; B31 tests full initrd boot before falling back to tiny initramfs.
#
# Route:
#   Libreboot GRUB -> external USB -> linux16 -> initrd16 -> full host initrd
#
# Safety:
#   This erases only the selected USB disk, refuses to run without
#   YES_WRITE_USB, and refuses /dev/sda by default.

set -eu

USB="${1:-/dev/sdb}"
CONFIRM="${2:-}"
PROFILE="${BASE1_B31_PROFILE:-x200-supervisor-lite}"
OUT_DIR="${BASE1_B31_OUT:-build/base1-b31-full-linux-libre-splash}"
KERNEL="${BASE1_B31_KERNEL:-build/linux/alpine-netboot/vmlinuz}"
INITRD="${BASE1_B31_INITRD:-build/linux/alpine-netboot/initrd.img}"
REPORT="$OUT_DIR/b31-full-linux-libre-splash-usb.env"
SPLASH="$OUT_DIR/phase1-splash.tga"

fail() { printf 'x200-b31-full-linux-libre-splash-usb: %s\n' "$1" >&2; exit 1; }
need_cmd() { command -v "$1" >/dev/null 2>&1 || fail "missing command: $1"; }

partition_one() {
  case "$1" in
    /dev/nvme[0-9]n[0-9]|/dev/mmcblk[0-9]) printf '%sp1\n' "$1" ;;
    /dev/sd[a-z]|/dev/vd[a-z]|/dev/hd[a-z]) printf '%s1\n' "$1" ;;
    *) fail "use a whole-disk path such as /dev/sdb, not a partition" ;;
  esac
}

[ "$CONFIRM" = "YES_WRITE_USB" ] || fail "usage: sh scripts/x200-b31-full-linux-libre-splash-usb.sh /dev/sdb YES_WRITE_USB"
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

# Generate a GRUB-side splash image when Python is available. The boot remains
# text-safe if splash mode fails.
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
        r = 8 + (x * 28 // w)
        g = 8 + (y * 26 // h)
        b = 24 + ((x + y) * 64 // (w + h))
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

printf 'PHASE1 BASE1 B31 FULL LINUX-LIBRE SPLASH USB WRITER\n\n'
printf 'profile     : %s\n' "$PROFILE"
printf 'target disk : %s\n' "$USB"
printf 'kernel      : %s\n' "$KERNEL"
printf 'initrd      : %s\n' "$INITRD"
printf 'splash      : %s\n' "$SPLASH"
printf 'scope       : full host initrd + GRUB-side splash\n\n'
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

sudo mkfs.vfat -F 32 -n PHASE1B31 "$PART1"
sudo mount "$PART1" "$MNT"
sudo mkdir -p "$MNT/boot/grub" "$MNT/grub" "$MNT/boot/phase1" "$MNT/phase1/assets" "$MNT/phase1/evidence"
sudo cp "$KERNEL" "$MNT/boot/phase1/vmlinuz"
sudo cp "$INITRD" "$MNT/boot/phase1/initrd.img"
if [ -f "$SPLASH" ]; then sudo cp "$SPLASH" "$MNT/phase1/assets/phase1-splash.tga"; fi

KERNEL_SIZE="$(stat -c %s "$KERNEL" 2>/dev/null || stat -f %z "$KERNEL")"
INITRD_SIZE="$(stat -c %s "$INITRD" 2>/dev/null || stat -f %z "$INITRD")"

sudo tee "$MNT/phase1/evidence/b31-prep.env" >/dev/null <<EOF
BASE1_B31_FULL_LINUX_LIBRE_PROFILE=$PROFILE
BASE1_B31_FULL_LINUX_LIBRE_KERNEL=/boot/phase1/vmlinuz
BASE1_B31_FULL_LINUX_LIBRE_INITRD=/boot/phase1/initrd.img
BASE1_B31_FULL_LINUX_LIBRE_KERNEL_SIZE=$KERNEL_SIZE
BASE1_B31_FULL_LINUX_LIBRE_INITRD_SIZE=$INITRD_SIZE
BASE1_B31_FULL_LINUX_LIBRE_SPLASH=/phase1/assets/phase1-splash.tga
BASE1_B31_FULL_LINUX_LIBRE_EXPECTED_RESULT=phase1_linux_libre_full_seen
BASE1_B31_FULL_LINUX_LIBRE_SPLASH_RESULT=phase1_boot_splash_seen
BASE1_B31_FULL_LINUX_LIBRE_CLAIM=not_claimed
EOF

sudo tee "$MNT/boot/grub/grub.cfg" >/dev/null <<'EOF'
set timeout=-1
set default=0
set pager=1
set color_normal=white/black
set color_highlight=black/light-gray
terminal_input console
terminal_output console

menuentry "B31 File check" {
    clear
    echo "B31 full Linux-libre file check"
    echo ""
    ls -lh /boot/phase1
    echo ""
    cat /phase1/evidence/b31-prep.env
    echo ""
    sleep --interruptible 999
}

menuentry "B31 Phase1 Splash" {
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
        echo "B31 Phase1 splash active"
        echo "result: phase1_boot_splash_seen"
        echo "ESC returns."
        sleep --interruptible 999
    else
        terminal_output console
        clear
        echo "Splash unavailable. Use text entries."
        sleep --interruptible 999
    fi
    terminal_output console
}

menuentry "B31 Test initrd16 command only" {
    clear
    echo "B31 test: before linux16 command"
    linux16 /boot/phase1/vmlinuz root=/dev/ram0 ro console=tty0 nomodeset loglevel=7 panic=30
    echo "B31 test: linux16 command returned"
    echo "B31 test: before initrd16 command"
    initrd16 /boot/phase1/initrd.img
    echo "B31 test: initrd16 command returned"
    echo "No boot performed in this entry. ESC returns."
    sleep --interruptible 999
}

menuentry "B31 Boot full initrd verbose" {
    clear
    echo "B31 boot: full initrd verbose"
    echo "Target: phase1_linux_libre_full_seen"
    linux16 /boot/phase1/vmlinuz root=/dev/ram0 ro console=tty0 nomodeset loglevel=7 panic=30
    echo "linux16 returned; loading initrd16"
    initrd16 /boot/phase1/initrd.img
    echo "initrd16 returned; booting now"
    boot
}

menuentry "B31 Boot full initrd quiet" {
    clear
    echo "B31 boot: full initrd quiet"
    linux16 /boot/phase1/vmlinuz root=/dev/ram0 ro console=tty0 nomodeset quiet loglevel=3 panic=30
    echo "linux16 returned; loading initrd16"
    initrd16 /boot/phase1/initrd.img
    echo "initrd16 returned; booting now"
    boot
}

menuentry "B31 Boot normal linux fallback" {
    clear
    echo "B31 boot: normal linux fallback"
    linux /boot/phase1/vmlinuz root=/dev/ram0 ro console=tty0 nomodeset loglevel=7 panic=30
    echo "linux returned; loading initrd"
    initrd /boot/phase1/initrd.img
    echo "initrd returned; booting now"
    boot
}

menuentry "B31 GRUB fallback" {
    clear
    echo "phase1 6.0.0 ready"
    echo "B31 GRUB fallback console"
    echo "Use if full initrd remains blocked."
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
BASE1_B31_FULL_LINUX_LIBRE_TARGET=$USB
BASE1_B31_FULL_LINUX_LIBRE_PARTITION=$PART1
BASE1_B31_FULL_LINUX_LIBRE_KERNEL=$KERNEL
BASE1_B31_FULL_LINUX_LIBRE_INITRD=$INITRD
BASE1_B31_FULL_LINUX_LIBRE_SPLASH=$SPLASH
BASE1_B31_FULL_LINUX_LIBRE_RESULT=prepared
BASE1_B31_FULL_LINUX_LIBRE_EXPECTED_NEXT_RESULT=phase1_linux_libre_full_seen
BASE1_B31_FULL_LINUX_LIBRE_OPTIONAL_SPLASH_RESULT=phase1_boot_splash_seen
EOF

printf '\nDONE: B31 full Linux-libre splash USB prepared on %s\n' "$USB"
printf 'Boot path: Libreboot -> external USB GRUB.\n'
printf 'Try B31 File check, B31 Phase1 Splash, then B31 Test initrd16 command only.\n'

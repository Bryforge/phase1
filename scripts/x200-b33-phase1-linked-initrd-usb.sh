#!/usr/bin/env sh
# Phase1 / Base1 X200 B33 Phase1-linked initrd USB writer.
#
# Purpose:
#   Link Phase1 into the full host initrd path before the next diagnostic test.
#   B31 proved linux16 and initrd16 load. B32 catches initrd break points.
#   B33 appends a small Phase1 initramfs-tools hook overlay so the host initrd
#   attempts to load Phase1 early during initramfs startup.
#
# Route:
#   Libreboot GRUB -> external USB -> linux16 -> initrd16 -> full initrd + Phase1 hook
#
# Integration strategy:
#   Add /scripts/init-top/phase1 into the initramfs. Debian/Trisquel style
#   initramfs-tools runs init-top scripts before most root-discovery logic.
#   If the host initrd uses a different layout, this overlay is harmless and
#   B32 break diagnostics remain the fallback.
#
# Safety:
#   This erases only the selected USB disk, refuses to run without
#   YES_WRITE_USB, and refuses /dev/sda by default.

set -eu

USB="${1:-/dev/sdb}"
CONFIRM="${2:-}"
PROFILE="${BASE1_B33_PROFILE:-x200-supervisor-lite}"
OUT_DIR="${BASE1_B33_OUT:-build/base1-b33-phase1-linked-initrd}"
KERNEL="${BASE1_B33_KERNEL:-build/linux/alpine-netboot/vmlinuz}"
INITRD="${BASE1_B33_INITRD:-build/linux/alpine-netboot/initrd.img}"
REPORT="$OUT_DIR/b33-phase1-linked-initrd-usb.env"
OVERLAY_DIR="$OUT_DIR/phase1-link-overlay"
OVERLAY="$OUT_DIR/phase1-link-overlay.cpio.gz"
COMBINED="$OUT_DIR/phase1-linked-initrd.img"
SPLASH="$OUT_DIR/phase1-splash.tga"

fail() { printf 'x200-b33-phase1-linked-initrd-usb: %s\n' "$1" >&2; exit 1; }
need_cmd() { command -v "$1" >/dev/null 2>&1 || fail "missing command: $1"; }

partition_one() {
  case "$1" in
    /dev/nvme[0-9]n[0-9]|/dev/mmcblk[0-9]) printf '%sp1\n' "$1" ;;
    /dev/sd[a-z]|/dev/vd[a-z]|/dev/hd[a-z]) printf '%s1\n' "$1" ;;
    *) fail "use a whole-disk path such as /dev/sdb, not a partition" ;;
  esac
}

[ "$CONFIRM" = "YES_WRITE_USB" ] || fail "usage: sh scripts/x200-b33-phase1-linked-initrd-usb.sh /dev/sdb YES_WRITE_USB"
[ -b "$USB" ] || fail "not a block device: $USB"
[ "$USB" != "/dev/sda" ] || fail "refusing /dev/sda because it is commonly the internal disk"
[ -f "$KERNEL" ] || fail "missing Linux-libre baseline kernel: $KERNEL"
[ -f "$INITRD" ] || fail "missing full Linux-libre baseline initrd: $INITRD"

ROOT_SRC="$(findmnt -no SOURCE / 2>/dev/null || true)"
case "$ROOT_SRC" in
  "$USB"|"$USB"[0-9]*|"$USB"p[0-9]*) fail "refusing to write the root filesystem device: $ROOT_SRC" ;;
esac

for cmd in sudo parted mkfs.vfat mount umount sync mkdir cp tee grub-install sha256sum stat date find cpio gzip cat chmod; do
  need_cmd "$cmd"
done

PART1="$(partition_one "$USB")"
MNT="$(mktemp -d)"

cleanup() { sudo umount "$MNT" 2>/dev/null || true; rmdir "$MNT" 2>/dev/null || true; }
trap cleanup EXIT INT TERM

mkdir -p "$OUT_DIR"
rm -rf "$OVERLAY_DIR"
mkdir -p \
  "$OVERLAY_DIR/scripts/init-top" \
  "$OVERLAY_DIR/phase1/evidence" \
  "$OVERLAY_DIR/phase1/workspace" \
  "$OVERLAY_DIR/phase1/bin"

cat > "$OVERLAY_DIR/scripts/init-top/phase1" <<'EOF'
#!/bin/sh
# Phase1 initramfs-tools init-top hook.
# Runs early if the full initrd uses Debian/Trisquel initramfs-tools.

mkdir -p /phase1/evidence /phase1/workspace 2>/dev/null || true
cat > /phase1/evidence/b33-phase1-init-top.env <<'ENV'
BASE1_B33_PHASE1_LINK_MODE=initramfs-tools-init-top
BASE1_B33_PHASE1_LINK_RESULT=phase1_initrd_phase1_hook_seen
BASE1_B33_PHASE1_LINK_CLAIM=not_claimed
ENV

# Write to console if available. Keep output short for X200 firmware consoles.
{
  echo "phase1 6.0.0 ready"
  echo "B33 Phase1 initrd hook reached"
  echo "result: phase1_initrd_phase1_hook_seen"
} > /dev/console 2>/dev/null || true

# Do not block normal initramfs flow unless explicitly requested.
if grep -qw 'phase1.break=1' /proc/cmdline 2>/dev/null; then
  echo "Phase1 break requested. Type exit to continue initramfs." > /dev/console 2>/dev/null || true
  sh </dev/console >/dev/console 2>&1 || true
fi
EOF
chmod 0755 "$OVERLAY_DIR/scripts/init-top/phase1"

cat > "$OVERLAY_DIR/phase1/bin/phase1-status" <<'EOF'
#!/bin/sh
cat /phase1/evidence/b33-phase1-init-top.env 2>/dev/null || echo "phase1 hook evidence not found"
EOF
chmod 0755 "$OVERLAY_DIR/phase1/bin/phase1-status"

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
        r = 8 + (x * 26 // w)
        g = 10 + (y * 22 // h)
        b = 28 + ((x + y) * 60 // (w + h))
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

printf 'PHASE1 BASE1 B33 PHASE1-LINKED INITRD USB WRITER\n\n'
printf 'profile     : %s\n' "$PROFILE"
printf 'target disk : %s\n' "$USB"
printf 'kernel      : %s\n' "$KERNEL"
printf 'initrd      : %s\n' "$INITRD"
printf 'scope       : full initrd plus Phase1 init-top hook overlay\n\n'

( cd "$OVERLAY_DIR" && find . | cpio -H newc -o 2>/dev/null | gzip -9 > "../phase1-link-overlay.cpio.gz" )
[ -s "$OVERLAY" ] || fail "failed to build overlay: $OVERLAY"
cat "$INITRD" "$OVERLAY" > "$COMBINED"
[ -s "$COMBINED" ] || fail "failed to build combined initrd: $COMBINED"

printf 'Kernel SHA256:\n'
sha256sum "$KERNEL"
printf 'Initrd SHA256:\n'
sha256sum "$INITRD"
printf 'Overlay SHA256:\n'
sha256sum "$OVERLAY"
printf 'Combined initrd SHA256:\n'
sha256sum "$COMBINED"
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

sudo mkfs.vfat -F 32 -n PHASE1B33 "$PART1"
sudo mount "$PART1" "$MNT"
sudo mkdir -p "$MNT/boot/grub" "$MNT/grub" "$MNT/boot/phase1" "$MNT/phase1/assets" "$MNT/phase1/evidence"
sudo cp "$KERNEL" "$MNT/boot/phase1/vmlinuz"
sudo cp "$INITRD" "$MNT/boot/phase1/initrd.img"
sudo cp "$OVERLAY" "$MNT/boot/phase1/phase1-link-overlay.cpio.gz"
sudo cp "$COMBINED" "$MNT/boot/phase1/phase1-linked-initrd.img"
if [ -f "$SPLASH" ]; then sudo cp "$SPLASH" "$MNT/phase1/assets/phase1-splash.tga"; fi

KERNEL_SIZE="$(stat -c %s "$KERNEL" 2>/dev/null || stat -f %z "$KERNEL")"
INITRD_SIZE="$(stat -c %s "$INITRD" 2>/dev/null || stat -f %z "$INITRD")"
COMBINED_SIZE="$(stat -c %s "$COMBINED" 2>/dev/null || stat -f %z "$COMBINED")"

sudo tee "$MNT/phase1/evidence/b33-prep.env" >/dev/null <<EOF
BASE1_B33_PHASE1_LINK_PROFILE=$PROFILE
BASE1_B33_PHASE1_LINK_KERNEL=/boot/phase1/vmlinuz
BASE1_B33_PHASE1_LINK_INITRD=/boot/phase1/initrd.img
BASE1_B33_PHASE1_LINK_OVERLAY=/boot/phase1/phase1-link-overlay.cpio.gz
BASE1_B33_PHASE1_LINK_COMBINED_INITRD=/boot/phase1/phase1-linked-initrd.img
BASE1_B33_PHASE1_LINK_KERNEL_SIZE=$KERNEL_SIZE
BASE1_B33_PHASE1_LINK_INITRD_SIZE=$INITRD_SIZE
BASE1_B33_PHASE1_LINK_COMBINED_INITRD_SIZE=$COMBINED_SIZE
BASE1_B33_PHASE1_LINK_EXPECTED_RESULT=phase1_initrd_phase1_hook_seen
BASE1_B33_PHASE1_LINK_CLAIM=not_claimed
EOF

sudo tee "$MNT/boot/grub/grub.cfg" >/dev/null <<'EOF'
set timeout=-1
set default=0
set pager=1
set color_normal=white/black
set color_highlight=black/light-gray
terminal_input console
terminal_output console

menuentry "B33 File check" {
    clear
    echo "B33 Phase1-linked initrd file check"
    echo ""
    ls -lh /boot/phase1
    echo ""
    cat /phase1/evidence/b33-prep.env
    echo ""
    sleep --interruptible 999
}

menuentry "B33 Phase1 Splash" {
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
        echo "B33 Phase1 splash active"
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

menuentry "B33 Test linked initrd command only" {
    clear
    echo "B33 test: before linux16 command"
    linux16 /boot/phase1/vmlinuz root=/dev/ram0 ro console=tty0 nomodeset loglevel=7 panic=0 phase1.link=1
    echo "B33 test: linux16 command returned"
    echo "B33 test: before initrd16 linked command"
    initrd16 /boot/phase1/phase1-linked-initrd.img
    echo "B33 test: linked initrd16 command returned"
    echo "No boot performed in this entry. ESC returns."
    sleep --interruptible 999
}

menuentry "B33 Boot linked initrd break" {
    clear
    echo "B33 boot: linked initrd with Phase1 break"
    echo "Target: phase1_initrd_phase1_hook_seen"
    linux16 /boot/phase1/vmlinuz root=/dev/ram0 ro console=tty0 nomodeset loglevel=7 panic=0 phase1.link=1 phase1.break=1 break=top
    echo "linux16 returned; loading linked initrd16"
    initrd16 /boot/phase1/phase1-linked-initrd.img
    echo "linked initrd16 returned; booting now"
    boot
}

menuentry "B33 Boot linked initrd normal" {
    clear
    echo "B33 boot: linked initrd normal"
    linux16 /boot/phase1/vmlinuz root=/dev/ram0 ro console=tty0 nomodeset loglevel=7 panic=0 phase1.link=1
    echo "linux16 returned; loading linked initrd16"
    initrd16 /boot/phase1/phase1-linked-initrd.img
    echo "linked initrd16 returned; booting now"
    boot
}

menuentry "B33 Boot split full initrd plus Phase1 overlay" {
    clear
    echo "B33 boot: split initrd plus Phase1 overlay"
    linux16 /boot/phase1/vmlinuz root=/dev/ram0 ro console=tty0 nomodeset loglevel=7 panic=0 phase1.link=1 phase1.break=1 break=top
    echo "linux16 returned; loading split initrd16 files"
    initrd16 /boot/phase1/initrd.img /boot/phase1/phase1-link-overlay.cpio.gz
    echo "split initrd16 returned; booting now"
    boot
}

menuentry "B33 GRUB fallback" {
    clear
    echo "phase1 6.0.0 ready"
    echo "B33 GRUB fallback console"
    echo "Use if linked initrd diagnostics reset."
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
BASE1_B33_PHASE1_LINK_TARGET=$USB
BASE1_B33_PHASE1_LINK_PARTITION=$PART1
BASE1_B33_PHASE1_LINK_KERNEL=$KERNEL
BASE1_B33_PHASE1_LINK_INITRD=$INITRD
BASE1_B33_PHASE1_LINK_OVERLAY=$OVERLAY
BASE1_B33_PHASE1_LINK_COMBINED_INITRD=$COMBINED
BASE1_B33_PHASE1_LINK_RESULT=prepared
BASE1_B33_PHASE1_LINK_EXPECTED_NEXT_RESULT=phase1_initrd_phase1_hook_seen
EOF

printf '\nDONE: B33 Phase1-linked initrd USB prepared on %s\n' "$USB"
printf 'Boot path: Libreboot -> external USB GRUB.\n'
printf 'Try B33 Test linked initrd command only first.\n'

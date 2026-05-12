#!/usr/bin/env sh
# Phase1 / Base1 X200 B34 real splash linked initrd USB writer.
#
# Purpose:
#   Use the real Phase1 README-linked boot splash asset:
#     assets/phase1-splash.png
#   while also linking Phase1 into the full host initrd path with the B33
#   initramfs-tools init-top hook.
#
# Route:
#   Libreboot GRUB -> real Phase1 splash -> linux16 -> full initrd + Phase1 hook
#
# Safety:
#   This erases only the selected USB disk, refuses to run without
#   YES_WRITE_USB, and refuses /dev/sda by default.

set -eu

USB="${1:-/dev/sdb}"
CONFIRM="${2:-}"
PROFILE="${BASE1_B34_PROFILE:-x200-supervisor-lite}"
OUT_DIR="${BASE1_B34_OUT:-build/base1-b34-real-splash-linked-initrd}"
KERNEL="${BASE1_B34_KERNEL:-build/linux/alpine-netboot/vmlinuz}"
INITRD="${BASE1_B34_INITRD:-build/linux/alpine-netboot/initrd.img}"
SPLASH_SRC="${BASE1_B34_SPLASH:-assets/phase1-splash.png}"
REPORT="$OUT_DIR/b34-real-splash-linked-initrd-usb.env"
OVERLAY_DIR="$OUT_DIR/phase1-link-overlay"
OVERLAY="$OUT_DIR/phase1-link-overlay.cpio.gz"
COMBINED="$OUT_DIR/phase1-linked-initrd.img"

fail() { printf 'x200-b34-real-splash-linked-initrd-usb: %s\n' "$1" >&2; exit 1; }
need_cmd() { command -v "$1" >/dev/null 2>&1 || fail "missing command: $1"; }

partition_one() {
  case "$1" in
    /dev/nvme[0-9]n[0-9]|/dev/mmcblk[0-9]) printf '%sp1\n' "$1" ;;
    /dev/sd[a-z]|/dev/vd[a-z]|/dev/hd[a-z]) printf '%s1\n' "$1" ;;
    *) fail "use a whole-disk path such as /dev/sdb, not a partition" ;;
  esac
}

[ "$CONFIRM" = "YES_WRITE_USB" ] || fail "usage: sh scripts/x200-b34-real-splash-linked-initrd-usb.sh /dev/sdb YES_WRITE_USB"
[ -b "$USB" ] || fail "not a block device: $USB"
[ "$USB" != "/dev/sda" ] || fail "refusing /dev/sda because it is commonly the internal disk"
[ -f "$KERNEL" ] || fail "missing Linux-libre baseline kernel: $KERNEL"
[ -f "$INITRD" ] || fail "missing full Linux-libre baseline initrd: $INITRD"
[ -f "$SPLASH_SRC" ] || fail "missing real Phase1 splash asset: $SPLASH_SRC"

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
cat > /phase1/evidence/b34-phase1-init-top.env <<'ENV'
BASE1_B34_PHASE1_LINK_MODE=initramfs-tools-init-top
BASE1_B34_PHASE1_LINK_RESULT=phase1_initrd_phase1_hook_seen
BASE1_B34_PHASE1_LINK_CLAIM=not_claimed
ENV

{
  echo "phase1 6.0.0 ready"
  echo "B34 Phase1 initrd hook reached"
  echo "result: phase1_initrd_phase1_hook_seen"
} > /dev/console 2>/dev/null || true

if grep -qw 'phase1.break=1' /proc/cmdline 2>/dev/null; then
  echo "Phase1 break requested. Type exit to continue initramfs." > /dev/console 2>/dev/null || true
  sh </dev/console >/dev/console 2>&1 || true
fi
EOF
chmod 0755 "$OVERLAY_DIR/scripts/init-top/phase1"

cat > "$OVERLAY_DIR/phase1/bin/phase1-status" <<'EOF'
#!/bin/sh
cat /phase1/evidence/b34-phase1-init-top.env 2>/dev/null || echo "phase1 hook evidence not found"
EOF
chmod 0755 "$OVERLAY_DIR/phase1/bin/phase1-status"

printf 'PHASE1 BASE1 B34 REAL SPLASH LINKED INITRD USB WRITER\n\n'
printf 'profile     : %s\n' "$PROFILE"
printf 'target disk : %s\n' "$USB"
printf 'kernel      : %s\n' "$KERNEL"
printf 'initrd      : %s\n' "$INITRD"
printf 'splash      : %s\n' "$SPLASH_SRC"
printf 'scope       : real Phase1 PNG splash plus init-top hook overlay\n\n'

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
printf 'Real splash SHA256:\n'
sha256sum "$SPLASH_SRC"
printf '\nThis will erase the selected USB target.\n\n'

sudo umount "${USB}"* 2>/dev/null || true
sudo parted -s "$USB" mklabel msdos
sudo parted -s "$USB" mkpart primary fat32 1MiB 100%
sudo parted -s "$USB" set 1 boot on
sudo partprobe "$USB" 2>/dev/null || true
sync
sleep 2
[ -b "$PART1" ] || fail "partition did not appear: $PART1"

sudo mkfs.vfat -F 32 -n PHASE1B34 "$PART1"
sudo mount "$PART1" "$MNT"
sudo mkdir -p "$MNT/boot/grub" "$MNT/grub" "$MNT/boot/phase1" "$MNT/phase1/assets" "$MNT/phase1/evidence"
sudo cp "$KERNEL" "$MNT/boot/phase1/vmlinuz"
sudo cp "$INITRD" "$MNT/boot/phase1/initrd.img"
sudo cp "$OVERLAY" "$MNT/boot/phase1/phase1-link-overlay.cpio.gz"
sudo cp "$COMBINED" "$MNT/boot/phase1/phase1-linked-initrd.img"
sudo cp "$SPLASH_SRC" "$MNT/phase1/assets/phase1-splash.png"

KERNEL_SIZE="$(stat -c %s "$KERNEL" 2>/dev/null || stat -f %z "$KERNEL")"
INITRD_SIZE="$(stat -c %s "$INITRD" 2>/dev/null || stat -f %z "$INITRD")"
COMBINED_SIZE="$(stat -c %s "$COMBINED" 2>/dev/null || stat -f %z "$COMBINED")"
SPLASH_SIZE="$(stat -c %s "$SPLASH_SRC" 2>/dev/null || stat -f %z "$SPLASH_SRC")"

sudo tee "$MNT/phase1/evidence/b34-prep.env" >/dev/null <<EOF
BASE1_B34_REAL_SPLASH_PROFILE=$PROFILE
BASE1_B34_REAL_SPLASH_KERNEL=/boot/phase1/vmlinuz
BASE1_B34_REAL_SPLASH_INITRD=/boot/phase1/initrd.img
BASE1_B34_REAL_SPLASH_OVERLAY=/boot/phase1/phase1-link-overlay.cpio.gz
BASE1_B34_REAL_SPLASH_COMBINED_INITRD=/boot/phase1/phase1-linked-initrd.img
BASE1_B34_REAL_SPLASH_ASSET=/phase1/assets/phase1-splash.png
BASE1_B34_REAL_SPLASH_KERNEL_SIZE=$KERNEL_SIZE
BASE1_B34_REAL_SPLASH_INITRD_SIZE=$INITRD_SIZE
BASE1_B34_REAL_SPLASH_COMBINED_INITRD_SIZE=$COMBINED_SIZE
BASE1_B34_REAL_SPLASH_ASSET_SIZE=$SPLASH_SIZE
BASE1_B34_REAL_SPLASH_EXPECTED_RESULT=phase1_real_splash_seen
BASE1_B34_PHASE1_LINK_EXPECTED_RESULT=phase1_initrd_phase1_hook_seen
BASE1_B34_REAL_SPLASH_CLAIM=not_claimed
EOF

sudo tee "$MNT/boot/grub/grub.cfg" >/dev/null <<'EOF'
set timeout=-1
set default=0
set pager=1
set color_normal=white/black
set color_highlight=black/light-gray
terminal_input console
terminal_output console

menuentry "B34 File check" {
    clear
    echo "B34 real Phase1 splash file check"
    echo ""
    ls -lh /boot/phase1
    echo ""
    ls -lh /phase1/assets
    echo ""
    cat /phase1/evidence/b34-prep.env
    echo ""
    sleep --interruptible 999
}

menuentry "B34 Real Phase1 Splash" {
    clear
    echo "Trying real Phase1 splash image..."
    insmod all_video
    insmod gfxterm
    insmod png
    set gfxmode=auto
    set gfxpayload=keep
    terminal_output gfxterm
    if background_image /phase1/assets/phase1-splash.png; then
        echo "phase1 6.0.0 ready"
        echo "B34 real Phase1 splash active"
        echo "result: phase1_real_splash_seen"
        sleep --interruptible 999
    else
        terminal_output console
        clear
        echo "Real PNG splash unavailable in this GRUB path."
        echo "Use text entries."
        sleep --interruptible 999
    fi
    terminal_output console
}

menuentry "B34 Test linked initrd command only" {
    clear
    echo "B34 test: before linux16 command"
    linux16 /boot/phase1/vmlinuz root=/dev/ram0 ro console=tty0 nomodeset loglevel=7 panic=0 phase1.link=1
    echo "B34 test: linux16 command returned"
    echo "B34 test: before initrd16 linked command"
    initrd16 /boot/phase1/phase1-linked-initrd.img
    echo "B34 test: linked initrd16 command returned"
    echo "No boot performed in this entry. ESC returns."
    sleep --interruptible 999
}

menuentry "B34 Boot linked initrd break" {
    clear
    echo "B34 boot: linked initrd with Phase1 break"
    echo "Target: phase1_initrd_phase1_hook_seen"
    linux16 /boot/phase1/vmlinuz root=/dev/ram0 ro console=tty0 nomodeset loglevel=7 panic=0 phase1.link=1 phase1.break=1 break=top
    echo "linux16 returned; loading linked initrd16"
    initrd16 /boot/phase1/phase1-linked-initrd.img
    echo "linked initrd16 returned; booting now"
    boot
}

menuentry "B34 Boot linked initrd normal" {
    clear
    echo "B34 boot: linked initrd normal"
    linux16 /boot/phase1/vmlinuz root=/dev/ram0 ro console=tty0 nomodeset loglevel=7 panic=0 phase1.link=1
    echo "linux16 returned; loading linked initrd16"
    initrd16 /boot/phase1/phase1-linked-initrd.img
    echo "linked initrd16 returned; booting now"
    boot
}

menuentry "B34 GRUB fallback" {
    clear
    echo "phase1 6.0.0 ready"
    echo "B34 GRUB fallback console"
    echo "Use if linked initrd diagnostics reset."
    sleep --interruptible 999
}

menuentry "Reboot" { reboot }
menuentry "Power off" { halt }
EOF
sudo cp "$MNT/boot/grub/grub.cfg" "$MNT/grub/grub.cfg"

printf 'Installing GRUB BIOS bootloader to %s...\n' "$USB"
sudo grub-install --target=i386-pc --boot-directory="$MNT/boot" "$USB"
sync
sudo umount "$MNT"
rmdir "$MNT"
trap - EXIT INT TERM

cat > "$REPORT" <<EOF
BASE1_B34_REAL_SPLASH_TARGET=$USB
BASE1_B34_REAL_SPLASH_PARTITION=$PART1
BASE1_B34_REAL_SPLASH_KERNEL=$KERNEL
BASE1_B34_REAL_SPLASH_INITRD=$INITRD
BASE1_B34_REAL_SPLASH_OVERLAY=$OVERLAY
BASE1_B34_REAL_SPLASH_COMBINED_INITRD=$COMBINED
BASE1_B34_REAL_SPLASH_ASSET=$SPLASH_SRC
BASE1_B34_REAL_SPLASH_RESULT=prepared
BASE1_B34_REAL_SPLASH_EXPECTED_NEXT_RESULT=phase1_real_splash_seen
BASE1_B34_PHASE1_LINK_EXPECTED_NEXT_RESULT=phase1_initrd_phase1_hook_seen
EOF

printf '\nDONE: B34 real Phase1 splash linked initrd USB prepared on %s\n' "$USB"
printf 'Boot path: Libreboot -> external USB GRUB.\n'
printf 'Try B34 Real Phase1 Splash, then B34 Test linked initrd command only.\n'

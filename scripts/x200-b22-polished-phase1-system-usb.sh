#!/usr/bin/env sh
# Phase1 / Base1 X200 B22 polished Phase1 system USB writer.
#
# Purpose:
#   Prepare a polished, native-feeling Phase1 boot surface using the proven
#   physical route:
#
#     Libreboot -> SeaBIOS payload -> USB GRUB
#
#   B22 keeps a guaranteed text-safe path, adds an optional Phase1 splash mode,
#   keeps all wording professional, avoids private IP references, and includes
#   Linux/OpenBSD pivot pages if deeper OS boot integration is needed.
#
# Display policy:
#   - default: polished text-safe console for the X200 SeaBIOS screen
#   - optional: gfxterm + generated Phase1 splash image
#   - automatic GRUB gfxmode=auto where supported
#   - no dependency on graphics mode for success
#
# Safety:
#   This erases only the selected USB disk, refuses to run without
#   YES_WRITE_USB, and refuses /dev/sda by default.
#
# Non-claims:
#   External USB system surface only. No installer claim. No internal disk
#   install. No recovery-complete claim. No hardening claim. No daily-driver
#   claim.

set -eu

USB="${1:-/dev/sdb}"
CONFIRM="${2:-}"
PROFILE="${BASE1_B22_PROFILE:-x200-supervisor-lite}"
OUT_DIR="${BASE1_B22_OUT:-build/base1-b22-polished-phase1-system}"
REPORT="$OUT_DIR/b22-polished-phase1-system-usb.env"

fail() { printf 'x200-b22-polished-phase1-system-usb: %s\n' "$1" >&2; exit 1; }
need_cmd() { command -v "$1" >/dev/null 2>&1 || fail "missing command: $1"; }

partition_one() {
  case "$1" in
    /dev/nvme[0-9]n[0-9]|/dev/mmcblk[0-9]) printf '%sp1\n' "$1" ;;
    /dev/sd[a-z]|/dev/vd[a-z]|/dev/hd[a-z]) printf '%s1\n' "$1" ;;
    *) fail "use a whole-disk path such as /dev/sdb, not a partition" ;;
  esac
}

[ "$CONFIRM" = "YES_WRITE_USB" ] || fail "usage: sh scripts/x200-b22-polished-phase1-system-usb.sh /dev/sdb YES_WRITE_USB"
[ -b "$USB" ] || fail "not a block device: $USB"
[ "$USB" != "/dev/sda" ] || fail "refusing /dev/sda because it is commonly the internal disk"

ROOT_SRC="$(findmnt -no SOURCE / 2>/dev/null || true)"
case "$ROOT_SRC" in
  "$USB"|"$USB"[0-9]*|"$USB"p[0-9]*) fail "refusing to write the root filesystem device: $ROOT_SRC" ;;
esac

for cmd in sudo parted mkfs.vfat mount umount sync mkdir cp tee grub-install sha256sum date; do
  need_cmd "$cmd"
done

PART1="$(partition_one "$USB")"
MNT="$(mktemp -d)"

cleanup() { sudo umount "$MNT" 2>/dev/null || true; rmdir "$MNT" 2>/dev/null || true; }
trap cleanup EXIT INT TERM

mkdir -p "$OUT_DIR"

printf 'PHASE1 BASE1 B22 POLISHED SYSTEM USB WRITER\n\n'
printf 'profile     : %s\n' "$PROFILE"
printf 'target disk : %s\n' "$USB"
printf 'grub part   : %s\n' "$PART1"
printf 'boot path   : Libreboot -> SeaBIOS -> USB GRUB\n'
printf 'display     : text-safe default, optional splash\n'
printf 'claim       : not_claimed\n\n'
printf 'This will erase the selected USB target.\n\n'

sudo umount "${USB}"* 2>/dev/null || true
sudo parted -s "$USB" mklabel msdos
sudo parted -s "$USB" mkpart primary fat32 1MiB 100%
sudo parted -s "$USB" set 1 boot on
sudo partprobe "$USB" 2>/dev/null || true
sync
sleep 2
[ -b "$PART1" ] || fail "partition did not appear: $PART1"

sudo mkfs.vfat -F 32 -n PHASE1B22 "$PART1"
sudo mount "$PART1" "$MNT"
sudo mkdir -p \
  "$MNT/boot/grub" \
  "$MNT/grub" \
  "$MNT/phase1" \
  "$MNT/phase1/evidence" \
  "$MNT/phase1/help" \
  "$MNT/phase1/assets"

# Generate an optional uncompressed TGA splash image without external graphics tools.
# GRUB may or may not support gfxterm/background_image on this path. Text mode remains
# the authoritative fallback.
if command -v python3 >/dev/null 2>&1; then
  python3 - "$MNT/phase1/assets/phase1-splash.tga" <<'PY'
import struct, sys
path = sys.argv[1]
w, h = 640, 480
# Uncompressed 24-bit TGA, bottom-left origin.
header = bytearray(18)
header[2] = 2
header[12:14] = struct.pack('<H', w)
header[14:16] = struct.pack('<H', h)
header[16] = 24
header[17] = 0x20
pixels = bytearray()
for y in range(h):
    for x in range(w):
        # dark blue/purple gradient, BGR order
        r = 12 + (x * 20 // w)
        g = 10 + (y * 18 // h)
        b = 28 + ((x + y) * 40 // (w + h))
        # simple Phase1 orbital ring / mark
        cx, cy = w // 2, h // 2
        dx, dy = x - cx, y - cy
        d = (dx*dx + dy*dy) ** 0.5
        if 138 < d < 145 or 205 < d < 211:
            r, g, b = 120, 200, 255
        if abs(y - cy) < 4 and 220 < x < 420:
            r, g, b = 240, 245, 255
        if abs(x - cx) < 4 and 195 < y < 285:
            r, g, b = 240, 245, 255
        pixels.extend((b, g, r))
with open(path, 'wb') as f:
    f.write(header)
    f.write(pixels)
PY
fi

sudo tee "$MNT/phase1/README.txt" >/dev/null <<EOF
Phase1 Base1 B22 polished system USB
Profile: $PROFILE
Boot path: Libreboot -> Load SeaBIOS payload -> USB GRUB
Display policy: text-safe default, optional splash mode

This USB is an external Phase1 system surface.
It is not an installer and does not write the internal disk.
EOF

sudo tee "$MNT/phase1/evidence/b22-polished-system.env" >/dev/null <<EOF
BASE1_B22_POLISHED_SYSTEM_PROFILE=$PROFILE
BASE1_B22_POLISHED_SYSTEM_BOOT_PATH=Libreboot_Load_SeaBIOS_payload_to_USB_GRUB
BASE1_B22_POLISHED_SYSTEM_DISPLAY_POLICY=text_safe_default_optional_splash_auto_gfxmode
BASE1_B22_POLISHED_SYSTEM_EXPECTED_RESULT=phase1_polished_system_seen
BASE1_B22_POLISHED_SYSTEM_OPTIONAL_SPLASH_RESULT=phase1_splash_mode_seen
BASE1_B22_POLISHED_SYSTEM_CLAIM=not_claimed
BASE1_B22_NON_CLAIM_INSTALLER=1
BASE1_B22_NON_CLAIM_INTERNAL_DISK_INSTALL=1
BASE1_B22_NON_CLAIM_RECOVERY_COMPLETE=1
BASE1_B22_NON_CLAIM_HARDENED=1
BASE1_B22_NON_CLAIM_DAILY_DRIVER=1
EOF

sudo tee "$MNT/phase1/help/operator-notes.txt" >/dev/null <<'EOF'
B22 operator notes

Use: Libreboot -> Load SeaBIOS payload.
Default entry: Start Phase1.
Text mode is authoritative because the X200 SeaBIOS GRUB display may stay small.
Splash mode is optional; if it fails or looks wrong, return to Start Phase1.
No internal disk is modified.
EOF

sudo tee "$MNT/boot/grub/grub.cfg" >/dev/null <<'EOF'
# Phase1 Base1 B22 polished system surface.
# Proven route: Libreboot -> SeaBIOS payload -> USB GRUB.
# Privacy: no local IP addresses are embedded.
# Display: text-safe default, optional splash/gfx mode.

set timeout=10
set default=0
set pager=1
set color_normal=white/black
set color_highlight=black/light-gray
terminal_input console
terminal_output console

function phase1_header {
    clear
    echo "phase1 6.0.0 ready"
    echo "Base1 B22 polished system"
    echo ""
}

menuentry "Start Phase1" {
    phase1_header
    echo "result: phase1_polished_system_seen"
    echo ""
    echo "Route: Libreboot -> SeaBIOS -> USB GRUB"
    echo "Mode : external USB Phase1 surface"
    echo "Disk : internal disk not touched"
    echo "Net  : none"
    echo "Install: no"
    echo "Daily-driver claim: no"
    echo ""
    echo "Status: Phase1 control surface active."
    echo ""
    echo "ESC returns."
    sleep --interruptible 999
}

menuentry "Start Phase1 Splash" {
    clear
    echo "Trying Phase1 splash mode..."
    echo "If this fails, use Start Phase1."
    insmod all_video
    insmod gfxterm
    insmod tga
    set gfxmode=auto
    set gfxpayload=keep
    terminal_output gfxterm
    if background_image /phase1/assets/phase1-splash.tga; then
        echo "phase1 6.0.0 ready"
        echo "Base1 B22 splash mode"
        echo "result: phase1_splash_mode_seen"
        echo ""
        echo "Route: SeaBIOS -> USB GRUB"
        echo "Display: gfxmode auto"
        echo ""
        echo "ESC returns."
        sleep --interruptible 999
    else
        terminal_output console
        clear
        echo "Splash unavailable."
        echo "Use Start Phase1."
        sleep --interruptible 999
    fi
    terminal_output console
}

menuentry "Status" {
    phase1_header
    echo "Confirmed route:"
    echo "SeaBIOS -> USB GRUB"
    echo ""
    echo "Confirmed surfaces:"
    echo "B20 system console"
    echo "B21 console v2"
    echo ""
    echo "B22 target: polished system"
    echo ""
    echo "ESC returns."
    sleep --interruptible 999
}

menuentry "Hardware" {
    phase1_header
    echo "Machine: ThinkPad X200"
    echo "Firmware: Libreboot/coreboot"
    echo "Payload: SeaBIOS"
    echo "Media: external USB"
    echo "Console: GRUB text"
    echo "Display: small-screen safe"
    echo ""
    echo "ESC returns."
    sleep --interruptible 999
}

menuentry "Evidence" {
    phase1_header
    echo "Record after booting Trisquel:"
    echo ""
    echo "cd ~/phase1"
    echo "git pull --ff-only origin edge/stable"
    echo "sh scripts/x200-record-and-share-result-safe.sh phase1_polished_system_seen"
    echo ""
    echo "Serve ~/phase1-share only."
    echo "Use <X200_IP>, not hard-coded IPs."
    echo ""
    echo "ESC returns."
    sleep --interruptible 999
}

menuentry "Safety" {
    phase1_header
    echo "Safety boundary"
    echo ""
    echo "External USB only."
    echo "No internal disk write."
    echo "No internal bootloader write."
    echo "No package install."
    echo "No network use."
    echo "No hardening proof."
    echo "No daily-driver claim."
    echo ""
    echo "ESC returns."
    sleep --interruptible 999
}

menuentry "Files" {
    phase1_header
    echo "/"
    ls /
    echo ""
    echo "/phase1"
    ls /phase1
    echo ""
    echo "/phase1/evidence"
    ls /phase1/evidence
    echo ""
    echo "ESC returns."
    sleep --interruptible 999
}

menuentry "Devices" {
    phase1_header
    echo "GRUB devices:"
    echo ""
    ls
    echo ""
    echo "ESC returns."
    sleep --interruptible 999
}

menuentry "OpenBSD Notes" {
    phase1_header
    echo "OpenBSD lesson applied:"
    echo ""
    echo "Visible boot path may still have"
    echo "console/evidence limits."
    echo ""
    echo "Keep claims narrow."
    echo "Record only observed states."
    echo "Keep fallback paths."
    echo ""
    echo "ESC returns."
    sleep --interruptible 999
}

menuentry "Linux Pivot" {
    phase1_header
    echo "Linux pivot option"
    echo ""
    echo "Use if Phase1-owned handoff stalls."
    echo "Target: tiny Linux/initrd shell."
    echo "Keep external USB only."
    echo "No installer claim."
    echo ""
    echo "ESC returns."
    sleep --interruptible 999
}

menuentry "OpenBSD Pivot" {
    phase1_header
    echo "OpenBSD pivot option"
    echo ""
    echo "Use only with clear console plan."
    echo "Prior OpenBSD work showed serial"
    echo "and evidence limits."
    echo ""
    echo "Keep claims narrow."
    echo "ESC returns."
    sleep --interruptible 999
}

menuentry "Handoff Lab" {
    phase1_header
    echo "Handoff lab"
    echo ""
    echo "Future Phase1-owned code must use"
    echo "SeaBIOS USB GRUB as base route."
    echo ""
    echo "Do not use for normal launch."
    echo "ESC returns."
    sleep --interruptible 999
}

menuentry "Recovery" {
    phase1_header
    echo "Recovery"
    echo ""
    echo "Remove USB to boot host OS."
    echo "Power off or reboot below."
    echo "Internal disk is unchanged."
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
BASE1_B22_POLISHED_SYSTEM_PROFILE=$PROFILE
BASE1_B22_POLISHED_SYSTEM_TARGET=$USB
BASE1_B22_POLISHED_SYSTEM_PARTITION=$PART1
BASE1_B22_POLISHED_SYSTEM_RESULT=prepared
BASE1_B22_POLISHED_SYSTEM_EXPECTED_NEXT_RESULT=phase1_polished_system_seen
BASE1_B22_POLISHED_SYSTEM_OPTIONAL_SPLASH_RESULT=phase1_splash_mode_seen
BASE1_B22_POLISHED_SYSTEM_BOOT_PATH=Libreboot_Load_SeaBIOS_payload_to_USB_GRUB
BASE1_B22_POLISHED_SYSTEM_DISPLAY_POLICY=text_safe_default_optional_splash_auto_gfxmode
BASE1_B22_POLISHED_SYSTEM_PRIVACY=uses_placeholders_no_hardcoded_lan_ips
BASE1_B22_POLISHED_SYSTEM_CLAIM=not_claimed
BASE1_B22_NON_CLAIM_INSTALLER=1
BASE1_B22_NON_CLAIM_INTERNAL_DISK_INSTALL=1
BASE1_B22_NON_CLAIM_RECOVERY_COMPLETE=1
BASE1_B22_NON_CLAIM_HARDENED=1
BASE1_B22_NON_CLAIM_DAILY_DRIVER=1
EOF

printf '\nDONE: B22 polished Phase1 system USB prepared on %s\n' "$USB"
printf 'Boot path: Libreboot main menu -> Load SeaBIOS (payload).\n'
printf 'Choose: Start Phase1. Optional: Start Phase1 Splash.\n'
printf 'If the polished system appears, record: phase1_polished_system_seen\n'

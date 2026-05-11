#!/usr/bin/env sh
# Phase1 / Base1 X200 B21 Phase1 console v2 USB writer.
#
# Purpose:
#   Prepare the next stable Phase1 system console for the X200 using the proven
#   route: Libreboot -> SeaBIOS payload -> USB GRUB. B21 explicitly assumes the
#   SeaBIOS text screen is small and may not resize, so every screen uses short
#   lines and paged menus.
#
# Display policy:
#   - console output only
#   - no gfxterm
#   - no splash
#   - no reliance on changing screen size
#   - short menu names
#   - short wrapped-safe lines
#   - one responsibility per menu page
#
# Safety:
#   This erases only the selected USB disk, refuses to run without
#   YES_WRITE_USB, and refuses /dev/sda by default.
#
# Non-claims:
#   External USB console only. No installer claim. No internal disk install.
#   No recovery-complete claim. No hardening claim. No daily-driver claim.

set -eu

USB="${1:-/dev/sdb}"
CONFIRM="${2:-}"
PROFILE="${BASE1_B21_PROFILE:-x200-supervisor-lite}"
OUT_DIR="${BASE1_B21_OUT:-build/base1-b21-phase1-console-v2}"
REPORT="$OUT_DIR/b21-phase1-console-v2-usb.env"

fail() { printf 'x200-b21-phase1-console-v2-usb: %s\n' "$1" >&2; exit 1; }
need_cmd() { command -v "$1" >/dev/null 2>&1 || fail "missing command: $1"; }

partition_one() {
  case "$1" in
    /dev/nvme[0-9]n[0-9]|/dev/mmcblk[0-9]) printf '%sp1\n' "$1" ;;
    /dev/sd[a-z]|/dev/vd[a-z]|/dev/hd[a-z]) printf '%s1\n' "$1" ;;
    *) fail "use a whole-disk path such as /dev/sdb, not a partition" ;;
  esac
}

[ "$CONFIRM" = "YES_WRITE_USB" ] || fail "usage: sh scripts/x200-b21-phase1-console-v2-usb.sh /dev/sdb YES_WRITE_USB"
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

printf 'PHASE1 BASE1 B21 CONSOLE V2 USB WRITER\n\n'
printf 'profile     : %s\n' "$PROFILE"
printf 'target disk : %s\n' "$USB"
printf 'grub part   : %s\n' "$PART1"
printf 'boot path   : Libreboot -> SeaBIOS -> USB GRUB\n'
printf 'display     : small-screen-safe text pages\n'
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

sudo mkfs.vfat -F 32 -n PHASE1B21 "$PART1"
sudo mount "$PART1" "$MNT"
sudo mkdir -p "$MNT/boot/grub" "$MNT/grub" "$MNT/phase1" "$MNT/phase1/evidence" "$MNT/phase1/help"

sudo tee "$MNT/phase1/README.txt" >/dev/null <<EOF
Phase1 Base1 B21 console v2
Profile: $PROFILE
Boot path: Libreboot -> Load SeaBIOS payload -> USB GRUB
Display policy: small-screen-safe text pages

This USB is an external Phase1 system console.
It is not an installer and does not write the internal disk.
EOF

sudo tee "$MNT/phase1/evidence/b21-console-v2.env" >/dev/null <<EOF
BASE1_B21_CONSOLE_V2_PROFILE=$PROFILE
BASE1_B21_CONSOLE_V2_BOOT_PATH=Libreboot_Load_SeaBIOS_payload_to_USB_GRUB
BASE1_B21_CONSOLE_V2_DISPLAY_POLICY=small_screen_safe_console_no_gfxterm
BASE1_B21_CONSOLE_V2_EXPECTED_RESULT=phase1_system_console_v2_seen
BASE1_B21_CONSOLE_V2_CLAIM=not_claimed
BASE1_B21_NON_CLAIM_INSTALLER=1
BASE1_B21_NON_CLAIM_INTERNAL_DISK_INSTALL=1
BASE1_B21_NON_CLAIM_RECOVERY_COMPLETE=1
BASE1_B21_NON_CLAIM_HARDENED=1
BASE1_B21_NON_CLAIM_DAILY_DRIVER=1
EOF

sudo tee "$MNT/phase1/help/operator-notes.txt" >/dev/null <<'EOF'
B21 operator notes

Use Libreboot -> Load SeaBIOS payload.
Use only the B21 menu unless debugging.
Text is intentionally short because SeaBIOS GRUB did not resize the screen.
EOF

sudo tee "$MNT/boot/grub/grub.cfg" >/dev/null <<'EOF'
# Phase1 Base1 B21 SeaBIOS USB GRUB console v2.
# Small-screen-safe: no gfxterm, no splash, short pages.

set timeout=-1
set default=0
set pager=1
set color_normal=white/black
set color_highlight=black/light-gray
terminal_input console
terminal_output console

menuentry "B21 System" {
    clear
    echo "phase1 6.0.0 ready"
    echo "Base1 B21 console v2"
    echo "result: phase1_system_console_v2_seen"
    echo ""
    echo "Route: Libreboot -> SeaBIOS -> USB GRUB"
    echo "Screen: small text mode"
    echo "Policy: no gfxterm, no splash"
    echo ""
    echo "Phase1 control surface is active."
    echo "Internal disk: not touched."
    echo "Installer: no."
    echo "Daily-driver claim: no."
    echo ""
    echo "ESC returns to menu."
    sleep --interruptible 999
}

menuentry "Status" {
    clear
    echo "Phase1 status"
    echo ""
    echo "Confirmed:"
    echo "B11 Libreboot GRUB console"
    echo "B19 SeaBIOS USB GRUB"
    echo "B20 system console"
    echo ""
    echo "Current route: SeaBIOS USB GRUB"
    echo "Current target: B21 console v2"
    echo ""
    echo "ESC returns."
    sleep --interruptible 999
}

menuentry "Safety" {
    clear
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

menuentry "Hardware" {
    clear
    echo "Hardware profile"
    echo ""
    echo "Machine: ThinkPad X200"
    echo "Firmware: Libreboot/coreboot"
    echo "Payload route: SeaBIOS"
    echo "Boot media: external USB"
    echo "Console: GRUB text"
    echo "Display: small screen safe"
    echo ""
    echo "ESC returns."
    sleep --interruptible 999
}

menuentry "Evidence" {
    clear
    echo "Evidence"
    echo ""
    echo "Record after Trisquel boot:"
    echo "cd ~/phase1"
    echo "git pull --ff-only origin edge/stable"
    echo "sh scripts/x200-record-and-share-result.sh phase1_system_console_v2_seen"
    echo ""
    echo "Then serve ~/phase1-share."
    echo "ESC returns."
    sleep --interruptible 999
}

menuentry "Files" {
    clear
    echo "USB files"
    echo ""
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
    clear
    echo "GRUB devices"
    echo ""
    ls
    echo ""
    echo "ESC returns."
    sleep --interruptible 999
}

menuentry "Env" {
    clear
    echo "GRUB environment"
    echo ""
    set
    echo ""
    echo "ESC returns."
    sleep --interruptible 999
}

menuentry "Handoff Lab" {
    clear
    echo "Handoff lab"
    echo ""
    echo "Purpose: future Phase1-owned code."
    echo "Do not use for normal launch."
    echo ""
    echo "Known blocked paths:"
    echo "Multiboot ELF from Libreboot GRUB"
    echo "Raw sector chainload from GRUB"
    echo "Raw MBR from SeaBIOS"
    echo ""
    echo "Next lab must be built from"
    echo "SeaBIOS USB GRUB only."
    echo ""
    echo "ESC returns."
    sleep --interruptible 999
}

menuentry "Recovery" {
    clear
    echo "Recovery"
    echo ""
    echo "Power off: menu item below."
    echo "Reboot: menu item below."
    echo "Remove USB to return to host OS."
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
BASE1_B21_CONSOLE_V2_PROFILE=$PROFILE
BASE1_B21_CONSOLE_V2_TARGET=$USB
BASE1_B21_CONSOLE_V2_PARTITION=$PART1
BASE1_B21_CONSOLE_V2_RESULT=prepared
BASE1_B21_CONSOLE_V2_EXPECTED_NEXT_RESULT=phase1_system_console_v2_seen
BASE1_B21_CONSOLE_V2_BOOT_PATH=Libreboot_Load_SeaBIOS_payload_to_USB_GRUB
BASE1_B21_CONSOLE_V2_DISPLAY_POLICY=small_screen_safe_console_no_gfxterm
BASE1_B21_CONSOLE_V2_CLAIM=not_claimed
BASE1_B21_NON_CLAIM_INSTALLER=1
BASE1_B21_NON_CLAIM_INTERNAL_DISK_INSTALL=1
BASE1_B21_NON_CLAIM_RECOVERY_COMPLETE=1
BASE1_B21_NON_CLAIM_HARDENED=1
BASE1_B21_NON_CLAIM_DAILY_DRIVER=1
EOF

printf '\nDONE: B21 console v2 USB prepared on %s\n' "$USB"
printf 'Boot path: Libreboot main menu -> Load SeaBIOS (payload).\n'
printf 'Choose: B21 System.\n'
printf 'If the console appears, record: phase1_system_console_v2_seen\n'

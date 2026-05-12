#!/usr/bin/env sh
# Phase1 / Base1 X200 B20 Phase1 system USB writer.
#
# Purpose:
#   Prepare a single SeaBIOS-bootable external USB that behaves like a stable
#   Phase1 system console instead of a tiny one-off evidence test. B19 proved
#   the route Libreboot -> SeaBIOS payload -> USB GRUB works. B20 uses that
#   route as the operational base.
#
# Display policy:
#   Use GRUB console text, short lines, no splash, no gfxterm, no tiny fonts,
#   no long wrapping lines. The X200 SeaBIOS GRUB menu is small, so all text is
#   intentionally compact.
#
# Safety:
#   This erases only the selected USB disk, refuses to run without
#   YES_WRITE_USB, and refuses /dev/sda by default.
#
# Non-claims:
#   External USB system console only. No installer claim. No internal disk
#   install. No recovery-complete claim. No hardening claim. No daily-driver
#   claim.

set -eu

USB="${1:-/dev/sdb}"
CONFIRM="${2:-}"
PROFILE="${BASE1_B20_PROFILE:-x200-supervisor-lite}"
OUT_DIR="${BASE1_B20_OUT:-build/base1-b20-phase1-system}"
REPORT="$OUT_DIR/b20-phase1-system-usb.env"

fail() { printf 'x200-b20-phase1-system-usb: %s\n' "$1" >&2; exit 1; }
need_cmd() { command -v "$1" >/dev/null 2>&1 || fail "missing command: $1"; }

partition_one() {
  case "$1" in
    /dev/nvme[0-9]n[0-9]|/dev/mmcblk[0-9]) printf '%sp1\n' "$1" ;;
    /dev/sd[a-z]|/dev/vd[a-z]|/dev/hd[a-z]) printf '%s1\n' "$1" ;;
    *) fail "use a whole-disk path such as /dev/sdb, not a partition" ;;
  esac
}

[ "$CONFIRM" = "YES_WRITE_USB" ] || fail "usage: sh scripts/x200-b20-phase1-system-usb.sh /dev/sdb YES_WRITE_USB"
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

printf 'PHASE1 BASE1 B20 SYSTEM USB WRITER\n\n'
printf 'profile     : %s\n' "$PROFILE"
printf 'target disk : %s\n' "$USB"
printf 'grub part   : %s\n' "$PART1"
printf 'boot path   : Libreboot -> SeaBIOS payload -> USB GRUB\n'
printf 'mode        : Phase1 system console, text-safe\n'
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

sudo mkfs.vfat -F 32 -n PHASE1B20 "$PART1"
sudo mount "$PART1" "$MNT"
sudo mkdir -p "$MNT/boot/grub" "$MNT/grub" "$MNT/phase1" "$MNT/phase1/evidence"

sudo tee "$MNT/phase1/README.txt" >/dev/null <<EOF
Phase1 Base1 B20 system USB
Profile: $PROFILE
Boot path: Libreboot -> Load SeaBIOS payload -> USB GRUB

This USB is an external Phase1 system console.
It is not an installer and does not write the internal disk.
EOF

sudo tee "$MNT/phase1/evidence/b20-system.env" >/dev/null <<EOF
BASE1_B20_SYSTEM_PROFILE=$PROFILE
BASE1_B20_SYSTEM_BOOT_PATH=Libreboot_Load_SeaBIOS_payload_to_USB_GRUB
BASE1_B20_SYSTEM_DISPLAY_POLICY=console_text_short_lines_no_gfxterm
BASE1_B20_SYSTEM_EXPECTED_RESULT=phase1_system_console_seen
BASE1_B20_SYSTEM_CLAIM=not_claimed
BASE1_B20_NON_CLAIM_INSTALLER=1
BASE1_B20_NON_CLAIM_INTERNAL_DISK_INSTALL=1
BASE1_B20_NON_CLAIM_RECOVERY_COMPLETE=1
BASE1_B20_NON_CLAIM_HARDENED=1
BASE1_B20_NON_CLAIM_DAILY_DRIVER=1
EOF

sudo tee "$MNT/boot/grub/grub.cfg" >/dev/null <<'EOF'
# Phase1 Base1 B20 SeaBIOS USB GRUB system console.
# Designed for X200 small SeaBIOS text display: short lines only.

set timeout=-1
set default=0
set pager=1
set color_normal=white/black
set color_highlight=black/light-gray
terminal_input console
terminal_output console

menuentry "Phase1 System Console" {
    clear
    echo "phase1 6.0.0 ready"
    echo "Base1 B20 Phase1 system console"
    echo "result: phase1_system_console_seen"
    echo ""
    echo "Boot path: Libreboot -> SeaBIOS -> USB GRUB"
    echo "Mode: external USB system console"
    echo "Display: text-safe, short lines, no gfxterm"
    echo ""
    echo "Status: Phase1 control surface is active."
    echo "Storage: internal disk not touched."
    echo "Network: none."
    echo "Installer: no."
    echo "Daily-driver claim: no."
    echo ""
    echo "Use ESC to return to menu."
    sleep --interruptible 999
}

menuentry "Phase1 Evidence Summary" {
    clear
    echo "Phase1 evidence summary"
    echo ""
    echo "B11: Libreboot GRUB console worked."
    echo "B19: SeaBIOS USB GRUB worked."
    echo "B20: Phase1 system console target."
    echo ""
    echo "Blocked paths:"
    echo "- Libreboot GRUB Multiboot ELF handoff."
    echo "- Libreboot GRUB raw chainloader."
    echo "- SeaBIOS raw MBR bootsector."
    echo ""
    echo "Current best boot route:"
    echo "Libreboot -> SeaBIOS -> USB GRUB."
    echo ""
    echo "Use ESC to return."
    sleep --interruptible 999
}

menuentry "Phase1 Device Listing" {
    clear
    echo "Phase1 device listing"
    echo ""
    ls
    echo ""
    echo "Known from B19: USB GRUB is booted by SeaBIOS."
    echo "Use ESC to return."
    sleep --interruptible 999
}

menuentry "Phase1 GRUB Environment" {
    clear
    echo "Phase1 GRUB environment"
    echo ""
    set
    echo ""
    echo "Use ESC to return."
    sleep --interruptible 999
}

menuentry "Phase1 USB File Listing" {
    clear
    echo "Phase1 USB file listing"
    echo ""
    echo "Root files:"
    ls /
    echo ""
    echo "Phase1 files:"
    ls /phase1
    echo ""
    echo "Evidence files:"
    ls /phase1/evidence
    echo ""
    echo "Use ESC to return."
    sleep --interruptible 999
}

menuentry "Phase1 Safety Boundary" {
    clear
    echo "Phase1 safety boundary"
    echo ""
    echo "This is external USB only."
    echo "No internal disk install."
    echo "No bootloader write to internal disk."
    echo "No package install."
    echo "No network use."
    echo "No hardening proof."
    echo "No daily-driver claim."
    echo ""
    echo "Use ESC to return."
    sleep --interruptible 999
}

menuentry "Phase1 Record Result Help" {
    clear
    echo "After returning to Trisquel, record B20 with:"
    echo ""
    echo "cd ~/phase1"
    echo "git pull --ff-only origin edge/stable"
    echo "sh scripts/x200-b7-record-result.sh phase1_system_console_seen"
    echo "git add -f build/base1-b7-hardware-boot-evidence/b7-hardware-boot-evidence.env"
    echo "git commit -m 'Record X200 B20 Phase1 system console seen'"
    echo ""
    echo "Only record once if this system console works."
    echo "Use ESC to return."
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
BASE1_B20_SYSTEM_PROFILE=$PROFILE
BASE1_B20_SYSTEM_TARGET=$USB
BASE1_B20_SYSTEM_PARTITION=$PART1
BASE1_B20_SYSTEM_RESULT=prepared
BASE1_B20_SYSTEM_EXPECTED_NEXT_RESULT=phase1_system_console_seen
BASE1_B20_SYSTEM_BOOT_PATH=Libreboot_Load_SeaBIOS_payload_to_USB_GRUB
BASE1_B20_SYSTEM_DISPLAY_POLICY=console_text_short_lines_no_gfxterm
BASE1_B20_SYSTEM_CLAIM=not_claimed
BASE1_B20_NON_CLAIM_INSTALLER=1
BASE1_B20_NON_CLAIM_INTERNAL_DISK_INSTALL=1
BASE1_B20_NON_CLAIM_RECOVERY_COMPLETE=1
BASE1_B20_NON_CLAIM_HARDENED=1
BASE1_B20_NON_CLAIM_DAILY_DRIVER=1
EOF

printf '\nDONE: B20 Phase1 system USB prepared on %s\n' "$USB"
printf 'Boot path: Libreboot main menu -> Load SeaBIOS (payload).\n'
printf 'Choose: Phase1 System Console.\n'
printf 'If the console appears, record: phase1_system_console_seen\n'

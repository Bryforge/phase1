#!/usr/bin/env sh
# Phase1 / Base1 X200 B19 SeaBIOS GRUB USB writer.
#
# Purpose:
#   B18 black-screened after Libreboot -> SeaBIOS payload, which may mean the
#   raw MBR was too minimal or SeaBIOS did not select USB correctly. B19 builds
#   a conventional BIOS/MBR USB with GRUB installed for i386-pc, then asks the
#   operator to boot it through SeaBIOS. If SeaBIOS can boot USB normally, this
#   should show a Phase1 GRUB console.
#
# Safety:
#   This erases only the selected USB disk, refuses to run without
#   YES_WRITE_USB, and refuses /dev/sda by default.
#
# Non-claims:
#   External USB evidence only. No installer claim. No internal disk install.
#   No recovery-complete claim. No hardening claim. No daily-driver claim.

set -eu

USB="${1:-/dev/sdb}"
CONFIRM="${2:-}"
PROFILE="${BASE1_B19_PROFILE:-x200-supervisor-lite}"
OUT_DIR="${BASE1_B19_OUT:-build/base1-b19-seabios-grub}"
REPORT="$OUT_DIR/b19-seabios-grub-usb.env"

fail() { printf 'x200-b19-seabios-grub-usb: %s\n' "$1" >&2; exit 1; }
need_cmd() { command -v "$1" >/dev/null 2>&1 || fail "missing command: $1"; }

partition_one() {
  case "$1" in
    /dev/nvme[0-9]n[0-9]|/dev/mmcblk[0-9]) printf '%sp1\n' "$1" ;;
    /dev/sd[a-z]|/dev/vd[a-z]|/dev/hd[a-z]) printf '%s1\n' "$1" ;;
    *) fail "use a whole-disk path such as /dev/sdb, not a partition" ;;
  esac
}

[ "$CONFIRM" = "YES_WRITE_USB" ] || fail "usage: sh scripts/x200-b19-seabios-grub-usb.sh /dev/sdb YES_WRITE_USB"
[ -b "$USB" ] || fail "not a block device: $USB"
[ "$USB" != "/dev/sda" ] || fail "refusing /dev/sda because it is commonly the internal disk"

ROOT_SRC="$(findmnt -no SOURCE / 2>/dev/null || true)"
case "$ROOT_SRC" in
  "$USB"|"$USB"[0-9]*|"$USB"p[0-9]*) fail "refusing to write the root filesystem device: $ROOT_SRC" ;;
esac

for cmd in sudo parted mkfs.vfat mount umount sync mkdir tee grub-install sha256sum; do
  need_cmd "$cmd"
done

PART1="$(partition_one "$USB")"
MNT="$(mktemp -d)"

cleanup() { sudo umount "$MNT" 2>/dev/null || true; rmdir "$MNT" 2>/dev/null || true; }
trap cleanup EXIT INT TERM

mkdir -p "$OUT_DIR"

printf 'PHASE1 BASE1 B19 SEABIOS GRUB USB WRITER\n\n'
printf 'profile     : %s\n' "$PROFILE"
printf 'target disk : %s\n' "$USB"
printf 'grub part   : %s\n' "$PART1"
printf 'boot path   : Libreboot -> Load SeaBIOS payload -> USB GRUB\n'
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

sudo mkfs.vfat -F 32 -n PHASE1B19 "$PART1"
sudo mount "$PART1" "$MNT"
sudo mkdir -p "$MNT/boot/grub" "$MNT/grub" "$MNT/phase1-evidence"

sudo tee "$MNT/boot/grub/grub.cfg" >/dev/null <<'EOF'
set timeout=30
set default=0
set pager=1

menuentry "B19-01 Phase1 SeaBIOS GRUB console" {
    clear
    echo "phase1 6.0.0 ready"
    echo "Base1 B19 SeaBIOS GRUB console"
    echo "B19 candidate result: phase1_seabios_grub_seen"
    echo ""
    echo "Boot path: Libreboot -> SeaBIOS payload -> USB GRUB"
    echo "Scope: external USB boot path proof"
    echo "Non-claims: no installer, no internal disk install, no daily-driver claim"
    echo ""
    echo "If this screen appears after choosing Load SeaBIOS payload, SeaBIOS USB GRUB works."
    echo "Press ESC to return to menu."
    sleep --interruptible 999
}

menuentry "B19-02 SeaBIOS GRUB device listing" {
    clear
    echo "Phase1 Base1 B19 SeaBIOS GRUB device listing"
    echo "Visible GRUB devices:"
    ls
    echo ""
    echo "Press ESC to return."
    sleep --interruptible 999
}

menuentry "B6 marker fallback - known good text marker" {
    clear
    echo "phase1 6.0.0 ready"
    echo "B6 candidate result: phase1_marker_seen"
    echo "B19 fallback marker reached from external USB"
    echo "This is not an installer, hardening, or daily-driver claim."
    sleep --interruptible 999
}
EOF

sudo cp "$MNT/boot/grub/grub.cfg" "$MNT/grub/grub.cfg"

sudo tee "$MNT/PHASE1-B19-SEABIOS-GRUB.txt" >/dev/null <<EOF
Phase1 Base1 B19 SeaBIOS GRUB USB.
Profile: $PROFILE
Boot path: Libreboot -> Load SeaBIOS payload -> USB GRUB

Primary evidence state:
- phase1_seabios_grub_seen: B19 SeaBIOS GRUB console appears.

Useful negative state:
- seabios_usb_not_booting: SeaBIOS payload path still shows black screen or does not reach USB GRUB.

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

cat > "$REPORT" <<EOF
BASE1_B19_SEABIOS_GRUB_PROFILE=$PROFILE
BASE1_B19_SEABIOS_GRUB_TARGET=$USB
BASE1_B19_SEABIOS_GRUB_PARTITION=$PART1
BASE1_B19_SEABIOS_GRUB_RESULT=prepared
BASE1_B19_SEABIOS_GRUB_EXPECTED_NEXT_RESULT=phase1_seabios_grub_seen
BASE1_B19_SEABIOS_GRUB_BOOT_PATH=Libreboot_Load_SeaBIOS_payload_to_USB_GRUB
BASE1_B19_SEABIOS_GRUB_CLAIM=not_claimed
BASE1_B19_NON_CLAIM_INSTALLER=1
BASE1_B19_NON_CLAIM_INTERNAL_DISK_INSTALL=1
BASE1_B19_NON_CLAIM_RECOVERY_COMPLETE=1
BASE1_B19_NON_CLAIM_HARDENED=1
BASE1_B19_NON_CLAIM_DAILY_DRIVER=1
EOF

printf '\nDONE: B19 SeaBIOS GRUB USB prepared on %s\n' "$USB"
printf 'Boot path: Libreboot main menu -> Load SeaBIOS (payload).\n'
printf 'If B19 GRUB console appears, record: phase1_seabios_grub_seen\n'

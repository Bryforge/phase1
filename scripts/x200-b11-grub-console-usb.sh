#!/usr/bin/env sh
# Phase1 / Base1 X200 B11 GRUB-native Phase1 console USB writer.
#
# Purpose:
#   Provide a professional, Phase1-branded external USB console that runs
#   entirely inside GRUB. This is useful on X200/Libreboot when Linux and
#   Multiboot handoff paths load files but block or reset before visible
#   kernel execution.
#
# Safety:
#   This erases only the selected USB disk, refuses to run without
#   YES_WRITE_USB, and refuses /dev/sda by default.
#
# Non-claims:
#   This is a GRUB-native pre-kernel console only. It is not a kernel boot,
#   not an installer, not an internal disk install, not recovery-complete,
#   not hardened, and not daily-driver ready.

set -eu

USB="${1:-/dev/sdb}"
CONFIRM="${2:-}"
PROFILE="${BASE1_B11_PROFILE:-x200-supervisor-lite}"
OUT_DIR="${BASE1_B11_OUT:-build/base1-b11-grub-console}"

fail() { printf 'x200-b11-grub-console-usb: %s\n' "$1" >&2; exit 1; }
need_cmd() { command -v "$1" >/dev/null 2>&1 || fail "missing command: $1"; }

partition_one() {
  case "$1" in
    /dev/nvme[0-9]n[0-9]|/dev/mmcblk[0-9]) printf '%sp1\n' "$1" ;;
    /dev/sd[a-z]|/dev/vd[a-z]|/dev/hd[a-z]) printf '%s1\n' "$1" ;;
    *) fail "use a whole-disk path such as /dev/sdb, not a partition" ;;
  esac
}

[ "$CONFIRM" = "YES_WRITE_USB" ] || fail "usage: sh scripts/x200-b11-grub-console-usb.sh /dev/sdb YES_WRITE_USB"
[ -b "$USB" ] || fail "not a block device: $USB"
[ "$USB" != "/dev/sda" ] || fail "refusing /dev/sda because it is commonly the internal disk"

ROOT_SRC="$(findmnt -no SOURCE / 2>/dev/null || true)"
case "$ROOT_SRC" in
  "$USB"|"$USB"[0-9]*|"$USB"p[0-9]*) fail "refusing to write the root filesystem device: $ROOT_SRC" ;;
esac

for cmd in sudo parted mkfs.vfat mount umount sync mkdir tee grub-install sha256sum; do
  need_cmd "$cmd"
done

PART="$(partition_one "$USB")"
MNT="$(mktemp -d)"
REPORT="$OUT_DIR/b11-grub-console-usb.env"

cleanup() { sudo umount "$MNT" 2>/dev/null || true; rmdir "$MNT" 2>/dev/null || true; }
trap cleanup EXIT INT TERM

printf 'PHASE1 BASE1 B11 GRUB-NATIVE CONSOLE USB WRITER\n\n'
printf 'profile     : %s\n' "$PROFILE"
printf 'target disk : %s\n' "$USB"
printf 'partition   : %s\n' "$PART"
printf 'scope       : external USB GRUB-native Phase1 console\n'
printf 'claim       : not_claimed\n\n'
printf 'This will erase the selected USB target.\n\n'

sudo umount "${USB}"* 2>/dev/null || true
sudo parted -s "$USB" mklabel msdos
sudo parted -s "$USB" mkpart primary fat32 1MiB 100%
sudo parted -s "$USB" set 1 boot on
sudo partprobe "$USB" 2>/dev/null || true
sync
sleep 2
[ -b "$PART" ] || fail "partition did not appear: $PART"

sudo mkfs.vfat -F 32 -n PHASE1B11 "$PART"
sudo mount "$PART" "$MNT"
sudo mkdir -p "$MNT/boot/grub" "$MNT/grub" "$MNT/phase1-evidence"

sudo tee "$MNT/boot/grub/grub.cfg" >/dev/null <<'EOF'
set timeout=20
set default=0
set pager=1

function phase1_header {
    clear
    echo "phase1 6.0.0 ready"
    echo "Base1 external USB GRUB-native console"
    echo "B11 candidate result: phase1_grub_console_seen"
    echo ""
    echo "Hardware path: ThinkPad X200 / Libreboot / external USB"
    echo "Scope: pre-kernel Phase1 operations console"
    echo "Non-claims: not a kernel boot, installer, hardening proof, or daily-driver claim"
    echo ""
}

menuentry "Phase1 Base1 B11 - GRUB-native operations console" {
    phase1_header
    echo "Status: external USB media reached and GRUB-native Phase1 console is active."
    echo ""
    echo "Available actions:"
    echo "  1. Evidence summary"
    echo "  2. Device listing"
    echo "  3. GRUB environment"
    echo "  4. Return to menu"
    echo ""
    echo "Record this result as: phase1_grub_console_seen"
    echo ""
    echo "Press ESC to return to the menu."
    sleep --interruptible 999
}

menuentry "Phase1 Base1 B11 - evidence summary" {
    phase1_header
    echo "Evidence reached:"
    echo "  B6: phase1 marker seen from external USB"
    echo "  B7/B8/B9: kernel/initrd/multiboot files can be loaded by GRUB"
    echo "  B11: GRUB-native Phase1 console seen on physical X200"
    echo ""
    echo "Current recommended record result: phase1_grub_console_seen"
    echo ""
    echo "Blocked paths observed so far may include:"
    echo "  blocked_after_initrd_load"
    echo "  blocked_after_multiboot_load"
    echo "  reset_after_linux16_handoff"
    echo ""
    echo "Press ESC to return."
    sleep --interruptible 999
}

menuentry "Phase1 Base1 B11 - list GRUB devices" {
    phase1_header
    echo "GRUB devices visible from this firmware path:"
    ls
    echo ""
    echo "Press ESC to return."
    sleep --interruptible 999
}

menuentry "Phase1 Base1 B11 - show root and prefix" {
    phase1_header
    echo "GRUB root and prefix values:"
    echo "root=$root"
    echo "prefix=$prefix"
    echo "cmdpath=$cmdpath"
    echo ""
    echo "Press ESC to return."
    sleep --interruptible 999
}

menuentry "Phase1 Base1 B6 - marker fallback" {
    echo "phase1 6.0.0 ready"
    echo "B6 candidate result: phase1_marker_seen"
    echo "B11 fallback marker reached from external USB"
    echo "This is not a kernel boot, installer, hardening, or daily-driver claim."
    sleep --interruptible 999
}
EOF

sudo cp "$MNT/boot/grub/grub.cfg" "$MNT/grub/grub.cfg"

sudo tee "$MNT/PHASE1-B11-GRUB-CONSOLE.txt" >/dev/null <<EOF
Phase1 Base1 B11 GRUB-native operations console.
Profile: $PROFILE

Evidence state:
- phase1_grub_console_seen: GRUB-native Phase1 console appears.
- phase1_marker_seen: fallback marker appears.

Non-claims:
No kernel boot claim.
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
BASE1_B11_GRUB_CONSOLE_PROFILE=$PROFILE
BASE1_B11_GRUB_CONSOLE_TARGET=$USB
BASE1_B11_GRUB_CONSOLE_PARTITION=$PART
BASE1_B11_GRUB_CONSOLE_RESULT=prepared
BASE1_B11_GRUB_CONSOLE_EXPECTED_NEXT_RESULT=phase1_grub_console_seen
BASE1_B11_GRUB_CONSOLE_CLAIM=not_claimed
BASE1_B11_NON_CLAIM_KERNEL_BOOT=1
BASE1_B11_NON_CLAIM_INSTALLER=1
BASE1_B11_NON_CLAIM_INTERNAL_DISK_INSTALL=1
BASE1_B11_NON_CLAIM_RECOVERY_COMPLETE=1
BASE1_B11_NON_CLAIM_HARDENED=1
BASE1_B11_NON_CLAIM_DAILY_DRIVER=1
EOF

printf '\nDONE: B11 GRUB-native Phase1 console USB prepared on %s\n' "$USB"
printf 'Boot the X200 from USB and choose: Phase1 Base1 B11 - GRUB-native operations console\n'
printf 'If the console appears, record: phase1_grub_console_seen\n'

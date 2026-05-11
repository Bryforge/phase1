#!/usr/bin/env sh
# Phase1 / Base1 X200 B17 bootsector USB writer.
#
# Purpose:
#   Build one external USB that bypasses the Multiboot ELF handoff path.
#   GRUB chainloads a tiny raw bootsector partition; the bootsector writes
#   Phase1 text directly to VGA memory. This is the next clean handoff test
#   after B11 succeeded and Multiboot ELF paths repeatedly loaded files but
#   did not produce reliable visible execution.
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
PROFILE="${BASE1_B17_PROFILE:-x200-supervisor-lite}"
OUT_DIR="${BASE1_B17_OUT:-build/base1-b17-bootsector}"
SRC="$OUT_DIR/phase1-b17-bootsector.S"
OBJ="$OUT_DIR/phase1-b17-bootsector.o"
BIN="$OUT_DIR/phase1-b17-bootsector.bin"
READBACK="$OUT_DIR/phase1-b17-bootsector-readback.bin"
REPORT="$OUT_DIR/b17-bootsector-usb.env"

fail() { printf 'x200-b17-bootsector-usb: %s\n' "$1" >&2; exit 1; }
need_cmd() { command -v "$1" >/dev/null 2>&1 || fail "missing command: $1"; }

partition_one() {
  case "$1" in
    /dev/nvme[0-9]n[0-9]|/dev/mmcblk[0-9]) printf '%sp1\n' "$1" ;;
    /dev/sd[a-z]|/dev/vd[a-z]|/dev/hd[a-z]) printf '%s1\n' "$1" ;;
    *) fail "use a whole-disk path such as /dev/sdb, not a partition" ;;
  esac
}

partition_two() {
  case "$1" in
    /dev/nvme[0-9]n[0-9]|/dev/mmcblk[0-9]) printf '%sp2\n' "$1" ;;
    /dev/sd[a-z]|/dev/vd[a-z]|/dev/hd[a-z]) printf '%s2\n' "$1" ;;
    *) fail "use a whole-disk path such as /dev/sdb, not a partition" ;;
  esac
}

[ "$CONFIRM" = "YES_WRITE_USB" ] || fail "usage: sh scripts/x200-b17-bootsector-usb.sh /dev/sdb YES_WRITE_USB"
[ -b "$USB" ] || fail "not a block device: $USB"
[ "$USB" != "/dev/sda" ] || fail "refusing /dev/sda because it is commonly the internal disk"

ROOT_SRC="$(findmnt -no SOURCE / 2>/dev/null || true)"
case "$ROOT_SRC" in
  "$USB"|"$USB"[0-9]*|"$USB"p[0-9]*) fail "refusing to write the root filesystem device: $ROOT_SRC" ;;
esac

for cmd in sudo parted mkfs.vfat mount umount sync mkdir cp tee grub-install sha256sum as objcopy dd stat cmp tail od; do
  need_cmd "$cmd"
done

PART1="$(partition_one "$USB")"
PART2="$(partition_two "$USB")"
MNT="$(mktemp -d)"

cleanup() { sudo umount "$MNT" 2>/dev/null || true; rmdir "$MNT" 2>/dev/null || true; }
trap cleanup EXIT INT TERM

mkdir -p "$OUT_DIR"

cat > "$SRC" <<'EOF'
.code16
.global _start
_start:
    cli
    xor %ax, %ax
    mov %ax, %ss
    mov $0x7C00, %sp

    /* GRUB/BIOS chainload convention places the sector at physical 0x7C00.
       Use DS=0x07C0 so labels assembled from offset 0 resolve correctly
       whether the far jump used CS:IP 0000:7C00 or 07C0:0000. */
    mov $0x07C0, %ax
    mov %ax, %ds

    sti
    mov $0x0003, %ax
    int $0x10
    cli

    mov $0xB800, %ax
    mov %ax, %es
    xor %di, %di
    mov $0x0720, %ax
    mov $2000, %cx
clear:
    stosw
    loop clear

    mov $0xB800, %ax
    mov %ax, %es
    mov $0, %di
    mov $line1, %si
    call print
    mov $160, %di
    mov $line2, %si
    call print
    mov $320, %di
    mov $line3, %si
    call print
    mov $480, %di
    mov $line4, %si
    call print
    mov $640, %di
    mov $line5, %si
    call print
    mov $960, %di
    mov $line6, %si
    call print

halt:
    hlt
    jmp halt

print:
    lodsb
    test %al, %al
    jz done
    mov $0x0F, %ah
    stosw
    jmp print
done:
    ret

line1: .asciz "phase1 6.0.0 ready"
line2: .asciz "Base1 B17 raw bootsector console"
line3: .asciz "B17 candidate result: phase1_bootsector_seen"
line4: .asciz "GRUB chainloaded a Phase1 bootsector from external USB."
line5: .asciz "Non-claims: no installer, no internal disk install, no daily-driver claim."
line6: .asciz "System halted intentionally. Power off or reboot when ready."

.org 510
.word 0xAA55
EOF

printf 'PHASE1 BASE1 B17 BOOTSECTOR USB WRITER\n\n'
printf 'profile       : %s\n' "$PROFILE"
printf 'target disk   : %s\n' "$USB"
printf 'grub part     : %s\n' "$PART1"
printf 'bootsector    : %s\n' "$PART2"
printf 'scope         : external USB chainloaded bootsector evidence\n'
printf 'claim         : not_claimed\n\n'
printf 'Building raw Phase1 bootsector...\n'

as --32 -o "$OBJ" "$SRC"
objcopy -O binary -j .text "$OBJ" "$BIN"
[ -s "$BIN" ] || fail "bootsector build failed: $BIN"
SIZE="$(stat -c %s "$BIN" 2>/dev/null || stat -f %z "$BIN")"
[ "$SIZE" = 512 ] || fail "bootsector must be exactly 512 bytes, got $SIZE"
SIG="$(tail -c 2 "$BIN" | od -An -tx1 | tr -d ' \n')"
[ "$SIG" = "55aa" ] || fail "bootsector signature must be 55aa, got $SIG"

printf 'Bootsector SHA256:\n'
sha256sum "$BIN"
printf 'Bootsector signature: %s\n' "$SIG"
printf '\nThis will erase the selected USB target.\n\n'

sudo umount "${USB}"* 2>/dev/null || true
sudo parted -s "$USB" mklabel msdos
sudo parted -s "$USB" mkpart primary fat32 1MiB 128MiB
sudo parted -s "$USB" set 1 boot on
sudo parted -s "$USB" mkpart primary 128MiB 129MiB
sudo partprobe "$USB" 2>/dev/null || true
sync
sleep 2
[ -b "$PART1" ] || fail "partition did not appear: $PART1"
[ -b "$PART2" ] || fail "partition did not appear: $PART2"

sudo mkfs.vfat -F 32 -n PHASE1B17 "$PART1"
sudo dd if="$BIN" of="$PART2" bs=512 count=1 conv=notrunc status=none
sync
sudo dd if="$PART2" of="$READBACK" bs=512 count=1 status=none
cmp "$BIN" "$READBACK" || fail "bootsector readback mismatch after writing $PART2"

sudo mount "$PART1" "$MNT"
sudo mkdir -p "$MNT/boot/grub" "$MNT/grub" "$MNT/boot/phase1" "$MNT/phase1-evidence"
sudo cp "$BIN" "$MNT/boot/phase1/phase1-b17-bootsector.bin"

sudo tee "$MNT/boot/grub/grub.cfg" >/dev/null <<'EOF'
set timeout=30
set default=0
set pager=1

menuentry "B17-01 Phase1 raw bootsector chainload - hd0 partition 2" {
    echo "Phase1 Base1 B17: chainloading raw bootsector partition"
    echo "Target evidence: phase1_bootsector_seen"
    echo "Expected screen: Base1 B17 raw bootsector console"
    chainloader (hd0,msdos2)+1
    boot
}

menuentry "B17-02 Phase1 raw bootsector chainload - hd1 partition 2" {
    echo "Phase1 Base1 B17: chainloading raw bootsector partition as hd1"
    echo "Target evidence: phase1_bootsector_seen"
    chainloader (hd1,msdos2)+1
    boot
}

menuentry "B17-03 Phase1 raw bootsector chainload - hd2 partition 2" {
    echo "Phase1 Base1 B17: chainloading raw bootsector partition as hd2"
    echo "Target evidence: phase1_bootsector_seen"
    chainloader (hd2,msdos2)+1
    boot
}

menuentry "B17-04 GRUB device listing" {
    clear
    echo "Phase1 Base1 B17 device listing"
    echo "Use this only if B17-01/02/03 do not find the raw bootsector partition."
    echo "Visible GRUB devices:"
    ls
    echo ""
    echo "Press ESC to return."
    sleep --interruptible 999
}

menuentry "B11 fallback - Phase1 GRUB-native operations console" {
    clear
    echo "phase1 6.0.0 ready"
    echo "Base1 external USB GRUB-native console"
    echo "B11 candidate result: phase1_grub_console_seen"
    echo "This fallback proves the external USB/GRUB path remains active."
    echo "It is not a kernel boot claim."
    sleep --interruptible 999
}

menuentry "B6 marker fallback - known good external USB proof" {
    echo "phase1 6.0.0 ready"
    echo "B6 candidate result: phase1_marker_seen"
    echo "B17 fallback marker reached from external USB"
    echo "This is not an installer, hardening, or daily-driver claim."
    sleep --interruptible 999
}
EOF

sudo cp "$MNT/boot/grub/grub.cfg" "$MNT/grub/grub.cfg"

sudo tee "$MNT/PHASE1-B17-BOOTSECTOR.txt" >/dev/null <<EOF
Phase1 Base1 B17 raw bootsector chainload USB.
Profile: $PROFILE
Bootsector partition: partition 2
Bootsector image copy: /boot/phase1/phase1-b17-bootsector.bin
Bootsector signature: $SIG
Readback verification: pass

Primary evidence state:
- phase1_bootsector_seen: raw bootsector console appears.

Other useful states:
- blocked_after_chainload: chainload attempted but no bootsector screen appears.
- phase1_grub_console_seen: GRUB-native fallback console appears.

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
BASE1_B17_BOOTSECTOR_PROFILE=$PROFILE
BASE1_B17_BOOTSECTOR_TARGET=$USB
BASE1_B17_BOOTSECTOR_GRUB_PARTITION=$PART1
BASE1_B17_BOOTSECTOR_RAW_PARTITION=$PART2
BASE1_B17_BOOTSECTOR_BIN=$BIN
BASE1_B17_BOOTSECTOR_SIGNATURE=$SIG
BASE1_B17_BOOTSECTOR_READBACK=pass
BASE1_B17_BOOTSECTOR_RESULT=prepared
BASE1_B17_BOOTSECTOR_EXPECTED_NEXT_RESULT=phase1_bootsector_seen
BASE1_B17_BOOTSECTOR_CLAIM=not_claimed
BASE1_B17_NON_CLAIM_INSTALLER=1
BASE1_B17_NON_CLAIM_INTERNAL_DISK_INSTALL=1
BASE1_B17_NON_CLAIM_RECOVERY_COMPLETE=1
BASE1_B17_NON_CLAIM_HARDENED=1
BASE1_B17_NON_CLAIM_DAILY_DRIVER=1
EOF

printf '\nDONE: B17 bootsector USB prepared on %s\n' "$USB"
printf 'Boot the X200 from USB and choose B17-01 first, then B17-02, then B17-03 if needed.\n'
printf 'If the raw bootsector console appears, record: phase1_bootsector_seen\n'

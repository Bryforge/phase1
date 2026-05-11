#!/usr/bin/env sh
# Phase1 / Base1 X200 B18 SeaBIOS MBR USB writer.
#
# Purpose:
#   Build an external USB whose first sector is a Phase1 MBR bootsector.
#   Libreboot's GRUB chainloader reports an unrecognized payload type, so B18
#   bypasses GRUB chainload and targets the Libreboot menu item:
#
#     Load SeaBIOS (payload)
#
#   SeaBIOS should boot the USB MBR directly as a BIOS disk.
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
PROFILE="${BASE1_B18_PROFILE:-x200-supervisor-lite}"
OUT_DIR="${BASE1_B18_OUT:-build/base1-b18-seabios-mbr}"
SRC="$OUT_DIR/phase1-b18-mbr.S"
OBJ="$OUT_DIR/phase1-b18-mbr.o"
MBR="$OUT_DIR/phase1-b18-mbr.bin"
READBACK="$OUT_DIR/phase1-b18-mbr-readback.bin"
REPORT="$OUT_DIR/b18-seabios-mbr-usb.env"

fail() { printf 'x200-b18-seabios-mbr-usb: %s\n' "$1" >&2; exit 1; }
need_cmd() { command -v "$1" >/dev/null 2>&1 || fail "missing command: $1"; }

[ "$CONFIRM" = "YES_WRITE_USB" ] || fail "usage: sh scripts/x200-b18-seabios-mbr-usb.sh /dev/sdb YES_WRITE_USB"
[ -b "$USB" ] || fail "not a block device: $USB"
[ "$USB" != "/dev/sda" ] || fail "refusing /dev/sda because it is commonly the internal disk"

ROOT_SRC="$(findmnt -no SOURCE / 2>/dev/null || true)"
case "$ROOT_SRC" in
  "$USB"|"$USB"[0-9]*|"$USB"p[0-9]*) fail "refusing to write the root filesystem device: $ROOT_SRC" ;;
esac

for cmd in sudo sync mkdir sha256sum as objcopy dd stat cmp tail od; do
  need_cmd "$cmd"
done

mkdir -p "$OUT_DIR"

cat > "$SRC" <<'EOF'
.code16
.global _start
_start:
    cli
    xor %ax, %ax
    mov %ax, %ss
    mov $0x7C00, %sp
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
line2: .asciz "Base1 B18 SeaBIOS MBR bootsector console"
line3: .asciz "B18 candidate result: phase1_mbr_bootsector_seen"
line4: .asciz "SeaBIOS booted a Phase1 MBR from external USB."
line5: .asciz "Non-claims: no installer, no internal disk install, no daily-driver claim."
line6: .asciz "System halted intentionally. Power off or reboot when ready."

.org 510
.word 0xAA55
EOF

printf 'PHASE1 BASE1 B18 SEABIOS MBR USB WRITER\n\n'
printf 'profile     : %s\n' "$PROFILE"
printf 'target disk : %s\n' "$USB"
printf 'scope       : SeaBIOS direct MBR bootsector evidence\n'
printf 'claim       : not_claimed\n\n'
printf 'Building Phase1 MBR bootsector...\n'

as --32 -o "$OBJ" "$SRC"
objcopy -O binary -j .text "$OBJ" "$MBR"
[ -s "$MBR" ] || fail "MBR build failed: $MBR"
SIZE="$(stat -c %s "$MBR" 2>/dev/null || stat -f %z "$MBR")"
[ "$SIZE" = 512 ] || fail "MBR must be exactly 512 bytes, got $SIZE"
SIG="$(tail -c 2 "$MBR" | od -An -tx1 | tr -d ' \n')"
[ "$SIG" = "55aa" ] || fail "MBR signature must be 55aa, got $SIG"

printf 'MBR SHA256:\n'
sha256sum "$MBR"
printf 'MBR signature: %s\n' "$SIG"
printf '\nThis will erase the selected USB target first sector.\n'
printf 'This USB is intended for Libreboot menu: Load SeaBIOS (payload).\n\n'

sudo dd if=/dev/zero of="$USB" bs=1M count=4 conv=fsync status=none
sudo dd if="$MBR" of="$USB" bs=512 count=1 conv=fsync,notrunc status=none
sync
sudo dd if="$USB" of="$READBACK" bs=512 count=1 status=none
cmp "$MBR" "$READBACK" || fail "MBR readback mismatch after writing $USB"

cat > "$REPORT" <<EOF
BASE1_B18_SEABIOS_MBR_PROFILE=$PROFILE
BASE1_B18_SEABIOS_MBR_TARGET=$USB
BASE1_B18_SEABIOS_MBR_BIN=$MBR
BASE1_B18_SEABIOS_MBR_SIGNATURE=$SIG
BASE1_B18_SEABIOS_MBR_READBACK=pass
BASE1_B18_SEABIOS_MBR_RESULT=prepared
BASE1_B18_SEABIOS_MBR_EXPECTED_NEXT_RESULT=phase1_mbr_bootsector_seen
BASE1_B18_SEABIOS_MBR_BOOT_PATH=Libreboot_Load_SeaBIOS_payload
BASE1_B18_SEABIOS_MBR_CLAIM=not_claimed
BASE1_B18_NON_CLAIM_INSTALLER=1
BASE1_B18_NON_CLAIM_INTERNAL_DISK_INSTALL=1
BASE1_B18_NON_CLAIM_RECOVERY_COMPLETE=1
BASE1_B18_NON_CLAIM_HARDENED=1
BASE1_B18_NON_CLAIM_DAILY_DRIVER=1
EOF

printf '\nDONE: B18 SeaBIOS MBR USB prepared on %s\n' "$USB"
printf 'Boot path: Libreboot main menu -> Load SeaBIOS (payload).\n'
printf 'If the MBR console appears, record: phase1_mbr_bootsector_seen\n'

#!/usr/bin/env sh
# Phase1 / Base1 X200 B10 multiboot VGA console USB writer.
#
# Purpose:
#   Build one external USB that boots a tiny Phase1-owned Multiboot kernel
#   directly through GRUB and writes text to VGA memory. This bypasses the
#   Linux kernel/initrd handoff path that can reset or go silent on older X200
#   Libreboot/Coreboot systems.
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
PROFILE="${BASE1_B10_PROFILE:-x200-supervisor-lite}"
OUT_DIR="${BASE1_B10_OUT:-build/base1-b10-multiboot-vga}"
SRC="$OUT_DIR/phase1-b10.S"
OBJ="$OUT_DIR/phase1-b10.o"
LDS="$OUT_DIR/phase1-b10.ld"
ELF="$OUT_DIR/phase1-b10.elf"

fail() { printf 'x200-b10-multiboot-vga-usb: %s\n' "$1" >&2; exit 1; }
need_cmd() { command -v "$1" >/dev/null 2>&1 || fail "missing command: $1"; }

partition_one() {
  case "$1" in
    /dev/nvme[0-9]n[0-9]|/dev/mmcblk[0-9]) printf '%sp1\n' "$1" ;;
    /dev/sd[a-z]|/dev/vd[a-z]|/dev/hd[a-z]) printf '%s1\n' "$1" ;;
    *) fail "use a whole-disk path such as /dev/sdb, not a partition" ;;
  esac
}

[ "$CONFIRM" = "YES_WRITE_USB" ] || fail "usage: sh scripts/x200-b10-multiboot-vga-usb.sh /dev/sdb YES_WRITE_USB"
[ -b "$USB" ] || fail "not a block device: $USB"
[ "$USB" != "/dev/sda" ] || fail "refusing /dev/sda because it is commonly the internal disk"

ROOT_SRC="$(findmnt -no SOURCE / 2>/dev/null || true)"
case "$ROOT_SRC" in
  "$USB"|"$USB"[0-9]*|"$USB"p[0-9]*) fail "refusing to write the root filesystem device: $ROOT_SRC" ;;
esac

for cmd in sudo parted mkfs.vfat mount umount sync mkdir cp tee grub-install sha256sum as ld chmod; do
  need_cmd "$cmd"
done

PART="$(partition_one "$USB")"
MNT="$(mktemp -d)"
REPORT="$OUT_DIR/b10-multiboot-vga-usb.env"

cleanup() { sudo umount "$MNT" 2>/dev/null || true; rmdir "$MNT" 2>/dev/null || true; }
trap cleanup EXIT INT TERM

mkdir -p "$OUT_DIR"

cat > "$SRC" <<'EOF'
.code32

.section .multiboot, "a"
.align 4
.long 0x1BADB002
.long 0x00000000
.long -(0x1BADB002 + 0x00000000)

.section .text, "ax"
.global _start
.type _start, @function
_start:
    cli

    mov $0xB8000, %edi
    mov $0x0720, %ax
    mov $2000, %ecx
clear_screen:
    mov %ax, (%edi)
    add $2, %edi
    loop clear_screen

    mov $(0xB8000 + 0 * 160), %edi
    mov $line1, %esi
    call print_line
    mov $(0xB8000 + 2 * 160), %edi
    mov $line2, %esi
    call print_line
    mov $(0xB8000 + 4 * 160), %edi
    mov $line3, %esi
    call print_line
    mov $(0xB8000 + 6 * 160), %edi
    mov $line4, %esi
    call print_line
    mov $(0xB8000 + 8 * 160), %edi
    mov $line5, %esi
    call print_line
    mov $(0xB8000 + 10 * 160), %edi
    mov $line6, %esi
    call print_line
    mov $(0xB8000 + 12 * 160), %edi
    mov $line7, %esi
    call print_line
    mov $(0xB8000 + 15 * 160), %edi
    mov $line8, %esi
    call print_line

halt_forever:
    hlt
    jmp halt_forever

print_line:
    lodsb
    test %al, %al
    jz print_done
    mov $0x0F, %ah
    mov %ax, (%edi)
    add $2, %edi
    jmp print_line
print_done:
    ret

.section .rodata, "a"
line1: .asciz "phase1 6.0.0 ready"
line2: .asciz "Base1 external USB multiboot VGA console"
line3: .asciz "B10 candidate result: phase1_multiboot_kernel_seen"
line4: .asciz "Hardware path: ThinkPad X200 / Libreboot / external USB"
line5: .asciz "Scope: Phase1-owned kernel evidence only"
line6: .asciz "Non-claims: no installer, no internal disk install, no daily-driver claim"
line7: .asciz "System halted intentionally. Power off or reboot when ready."
line8: .asciz "phase1> _"
EOF

cat > "$LDS" <<'EOF'
ENTRY(_start)
SECTIONS
{
  . = 1M;
  .multiboot : { *(.multiboot) }
  .text : { *(.text*) }
  .rodata : { *(.rodata*) }
  .data : { *(.data*) }
  .bss : { *(COMMON) *(.bss*) }
}
EOF

printf 'PHASE1 BASE1 B10 MULTIBOOT VGA USB WRITER\n\n'
printf 'profile     : %s\n' "$PROFILE"
printf 'target disk : %s\n' "$USB"
printf 'partition   : %s\n' "$PART"
printf 'kernel elf  : %s\n' "$ELF"
printf 'scope       : external USB Phase1-owned kernel evidence only\n'
printf 'claim       : not_claimed\n\n'
printf 'Building tiny Phase1 Multiboot kernel...\n'

as --32 -o "$OBJ" "$SRC"
ld -m elf_i386 -T "$LDS" -o "$ELF" "$OBJ"
chmod 0644 "$ELF"
[ -s "$ELF" ] || fail "failed to build ELF: $ELF"

if command -v grub-file >/dev/null 2>&1; then
  grub-file --is-x86-multiboot "$ELF" || fail "ELF failed GRUB multiboot validation"
fi

printf 'ELF SHA256:\n'
sha256sum "$ELF"
printf '\nThis will erase the selected USB target.\n\n'

sudo umount "${USB}"* 2>/dev/null || true
sudo parted -s "$USB" mklabel msdos
sudo parted -s "$USB" mkpart primary fat32 1MiB 100%
sudo parted -s "$USB" set 1 boot on
sudo partprobe "$USB" 2>/dev/null || true
sync
sleep 2
[ -b "$PART" ] || fail "partition did not appear: $PART"

sudo mkfs.vfat -F 32 -n PHASE1B10 "$PART"
sudo mount "$PART" "$MNT"
sudo mkdir -p "$MNT/boot/grub" "$MNT/grub" "$MNT/boot/phase1" "$MNT/phase1-evidence"
sudo cp "$ELF" "$MNT/boot/phase1/phase1-b10.elf"

sudo tee "$MNT/boot/grub/grub.cfg" >/dev/null <<'EOF'
set timeout=20
set default=0
set pager=1

menuentry "Phase1 Base1 B10 - multiboot VGA console" {
    echo "Phase1 Base1 B10 external USB multiboot VGA console"
    echo "Target evidence: phase1_multiboot_kernel_seen"
    multiboot /boot/phase1/phase1-b10.elf
    boot
}

menuentry "Phase1 Base1 B10 - multiboot VGA console, no gfxpayload" {
    echo "Phase1 Base1 B10 external USB multiboot VGA console"
    echo "Compatibility entry without gfxpayload changes."
    multiboot /boot/phase1/phase1-b10.elf
    boot
}

menuentry "Phase1 Base1 B6 - marker fallback" {
    echo "phase1 6.0.0 ready"
    echo "B6 candidate result: phase1_marker_seen"
    echo "B10 fallback marker reached from external USB"
    echo "This is not an installer, hardening, or daily-driver claim."
    sleep --interruptible 999
}
EOF

sudo cp "$MNT/boot/grub/grub.cfg" "$MNT/grub/grub.cfg"

sudo tee "$MNT/PHASE1-B10-MULTIBOOT.txt" >/dev/null <<EOF
Phase1 Base1 B10 external USB multiboot VGA console.
Profile: $PROFILE
Kernel: /boot/phase1/phase1-b10.elf

Evidence state:
- phase1_multiboot_kernel_seen: Phase1 VGA console appears.
- phase1_marker_seen: fallback marker appears.
- failed: no entry works.

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
BASE1_B10_MULTIBOOT_VGA_PROFILE=$PROFILE
BASE1_B10_MULTIBOOT_VGA_TARGET=$USB
BASE1_B10_MULTIBOOT_VGA_PARTITION=$PART
BASE1_B10_MULTIBOOT_VGA_ELF=$ELF
BASE1_B10_MULTIBOOT_VGA_RESULT=prepared
BASE1_B10_MULTIBOOT_VGA_EXPECTED_NEXT_RESULT=phase1_multiboot_kernel_seen
BASE1_B10_MULTIBOOT_VGA_CLAIM=not_claimed
BASE1_B10_NON_CLAIM_INSTALLER=1
BASE1_B10_NON_CLAIM_INTERNAL_DISK_INSTALL=1
BASE1_B10_NON_CLAIM_RECOVERY_COMPLETE=1
BASE1_B10_NON_CLAIM_HARDENED=1
BASE1_B10_NON_CLAIM_DAILY_DRIVER=1
EOF

printf '\nDONE: B10 multiboot VGA USB prepared on %s\n' "$USB"
printf 'Boot the X200 from USB and choose: Phase1 Base1 B10 - multiboot VGA console\n'
printf 'If the Phase1 VGA console appears, record: phase1_multiboot_kernel_seen\n'

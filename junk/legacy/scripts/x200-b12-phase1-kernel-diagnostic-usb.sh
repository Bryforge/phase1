#!/usr/bin/env sh
# Phase1 / Base1 X200 B12 Phase1-owned diagnostic kernel USB writer.
#
# Purpose:
#   Build one external USB that boots a Phase1-owned 32-bit Multiboot kernel
#   with a real stack, direct VGA text output, serial COM1 output, and a visible
#   heartbeat. This addresses earlier B10/B9 handoff ambiguity and avoids
#   repeated tiny rebuild/reboot cycles.
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
PROFILE="${BASE1_B12_PROFILE:-x200-supervisor-lite}"
OUT_DIR="${BASE1_B12_OUT:-build/base1-b12-phase1-kernel-diagnostic}"
SRC="$OUT_DIR/phase1-b12.S"
OBJ="$OUT_DIR/phase1-b12.o"
LDS="$OUT_DIR/phase1-b12.ld"
ELF="$OUT_DIR/phase1-b12.elf"
LINUX_KERNEL="${BASE1_B12_LINUX_KERNEL:-build/linux/alpine-netboot/vmlinuz}"
LINUX_INITRD="${BASE1_B12_LINUX_INITRD:-build/linux/alpine-netboot/initrd.img}"

fail() { printf 'x200-b12-phase1-kernel-diagnostic-usb: %s\n' "$1" >&2; exit 1; }
need_cmd() { command -v "$1" >/dev/null 2>&1 || fail "missing command: $1"; }

partition_one() {
  case "$1" in
    /dev/nvme[0-9]n[0-9]|/dev/mmcblk[0-9]) printf '%sp1\n' "$1" ;;
    /dev/sd[a-z]|/dev/vd[a-z]|/dev/hd[a-z]) printf '%s1\n' "$1" ;;
    *) fail "use a whole-disk path such as /dev/sdb, not a partition" ;;
  esac
}

[ "$CONFIRM" = "YES_WRITE_USB" ] || fail "usage: sh scripts/x200-b12-phase1-kernel-diagnostic-usb.sh /dev/sdb YES_WRITE_USB"
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
REPORT="$OUT_DIR/b12-phase1-kernel-diagnostic-usb.env"
HAS_LINUX=no
if [ -f "$LINUX_KERNEL" ] && [ -f "$LINUX_INITRD" ]; then
  HAS_LINUX=yes
fi

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
    mov $stack_top, %esp
    xor %ebp, %ebp

    call serial_init
    mov $serial_banner, %esi
    call serial_print

    call vga_clear

    mov $(0xB8000 + 0 * 160), %edi
    mov $line1, %esi
    call vga_print
    mov $(0xB8000 + 2 * 160), %edi
    mov $line2, %esi
    call vga_print
    mov $(0xB8000 + 4 * 160), %edi
    mov $line3, %esi
    call vga_print
    mov $(0xB8000 + 6 * 160), %edi
    mov $line4, %esi
    call vga_print
    mov $(0xB8000 + 8 * 160), %edi
    mov $line5, %esi
    call vga_print
    mov $(0xB8000 + 10 * 160), %edi
    mov $line6, %esi
    call vga_print
    mov $(0xB8000 + 12 * 160), %edi
    mov $line7, %esi
    call vga_print
    mov $(0xB8000 + 15 * 160), %edi
    mov $line8, %esi
    call vga_print
    mov $(0xB8000 + 18 * 160), %edi
    mov $line9, %esi
    call vga_print
    mov $(0xB8000 + 22 * 160), %edi
    mov $line10, %esi
    call vga_print

    mov $'0', %bl
heartbeat_loop:
    mov $(0xB8000 + 18 * 160 + 11 * 2), %edi
    mov %bl, %al
    mov $0x0A, %ah
    mov %ax, (%edi)

    mov $'.', %al
    call serial_putc

    inc %bl
    cmp $'9' + 1, %bl
    jne heartbeat_delay
    mov $'0', %bl
heartbeat_delay:
    mov $0x00FFFFFF, %ecx
1:
    loop 1b
    jmp heartbeat_loop

vga_clear:
    mov $0xB8000, %edi
    mov $0x0720, %ax
    mov $2000, %ecx
2:
    mov %ax, (%edi)
    add $2, %edi
    loop 2b
    ret

vga_print:
    lodsb
    test %al, %al
    jz 3f
    mov $0x0F, %ah
    mov %ax, (%edi)
    add $2, %edi
    jmp vga_print
3:
    ret

serial_init:
    mov $0x3F8 + 1, %dx
    mov $0x00, %al
    out %al, %dx
    mov $0x3F8 + 3, %dx
    mov $0x80, %al
    out %al, %dx
    mov $0x3F8 + 0, %dx
    mov $0x03, %al
    out %al, %dx
    mov $0x3F8 + 1, %dx
    mov $0x00, %al
    out %al, %dx
    mov $0x3F8 + 3, %dx
    mov $0x03, %al
    out %al, %dx
    mov $0x3F8 + 2, %dx
    mov $0xC7, %al
    out %al, %dx
    mov $0x3F8 + 4, %dx
    mov $0x0B, %al
    out %al, %dx
    ret

serial_print:
    lodsb
    test %al, %al
    jz 4f
    call serial_putc
    jmp serial_print
4:
    ret

serial_putc:
    push %edx
    push %eax
5:
    mov $0x3F8 + 5, %dx
    in %dx, %al
    test $0x20, %al
    jz 5b
    pop %eax
    mov $0x3F8, %dx
    out %al, %dx
    pop %edx
    ret

.section .rodata, "a"
serial_banner: .asciz "phase1 6.0.0 ready - B12 serial heartbeat\r\n"
line1: .asciz "phase1 6.0.0 ready"
line2: .asciz "Base1 external USB Phase1-owned diagnostic kernel"
line3: .asciz "B12 candidate result: phase1_kernel_vga_console_seen"
line4: .asciz "Hardware path: ThinkPad X200 / Libreboot / external USB"
line5: .asciz "Kernel path: GRUB Multiboot -> Phase1 32-bit protected-mode code"
line6: .asciz "Safety: no installer, no internal disk write, no daily-driver claim"
line7: .asciz "Diagnostics: VGA text active, stack initialized, serial COM1 heartbeat active"
line8: .asciz "If this screen is visible, Phase1-owned code is executing on hardware."
line9: .asciz "heartbeat: 0"
line10: .asciz "phase1> physical hardware evidence shell placeholder"

.section .bss, "aw"
.align 16
stack_bottom:
.skip 32768
stack_top:
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

printf 'PHASE1 BASE1 B12 DIAGNOSTIC KERNEL USB WRITER\n\n'
printf 'profile       : %s\n' "$PROFILE"
printf 'target disk   : %s\n' "$USB"
printf 'partition     : %s\n' "$PART"
printf 'phase1 kernel : %s\n' "$ELF"
printf 'linux controls: %s\n' "$HAS_LINUX"
printf 'scope         : external USB Phase1-owned kernel evidence\n'
printf 'claim         : not_claimed\n\n'
printf 'Building Phase1-owned diagnostic kernel with stack + VGA + serial...\n'

as --32 -o "$OBJ" "$SRC"
ld -m elf_i386 -T "$LDS" -o "$ELF" "$OBJ"
chmod 0644 "$ELF"
[ -s "$ELF" ] || fail "failed to build ELF: $ELF"

if command -v grub-file >/dev/null 2>&1; then
  grub-file --is-x86-multiboot "$ELF" || fail "ELF failed GRUB multiboot validation"
fi

printf 'Phase1 kernel SHA256:\n'
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

sudo mkfs.vfat -F 32 -n PHASE1B12 "$PART"
sudo mount "$PART" "$MNT"
sudo mkdir -p "$MNT/boot/grub" "$MNT/grub" "$MNT/boot/phase1" "$MNT/phase1-evidence"
sudo cp "$ELF" "$MNT/boot/phase1/phase1-b12.elf"

if [ "$HAS_LINUX" = yes ]; then
  sudo cp "$LINUX_KERNEL" "$MNT/boot/phase1/vmlinuz"
  sudo cp "$LINUX_INITRD" "$MNT/boot/phase1/initrd.img"
fi

sudo tee "$MNT/boot/grub/grub.cfg" >/dev/null <<'EOF'
set timeout=30
set default=0
set pager=1

menuentry "B12-01 Phase1-owned diagnostic kernel - VGA + serial heartbeat" {
    echo "Phase1 Base1 B12: loading Phase1-owned diagnostic kernel"
    echo "Target evidence: phase1_kernel_vga_console_seen"
    multiboot /boot/phase1/phase1-b12.elf
    boot
}

menuentry "B12-02 Phase1-owned diagnostic kernel - compatibility entry" {
    echo "Phase1 Base1 B12: compatibility multiboot entry"
    multiboot /boot/phase1/phase1-b12.elf
    boot
}

menuentry "B11-01 Phase1 GRUB-native operations console fallback" {
    clear
    echo "phase1 6.0.0 ready"
    echo "Base1 external USB GRUB-native console"
    echo "B11 candidate result: phase1_grub_console_seen"
    echo ""
    echo "This fallback proves the external USB/GRUB path remains active."
    echo "It is not a kernel boot claim."
    sleep --interruptible 999
}
EOF

if [ "$HAS_LINUX" = yes ]; then
  sudo tee -a "$MNT/boot/grub/grub.cfg" >/dev/null <<'EOF'

menuentry "B9 control - Linux kernel/initrd standard" {
    echo "Phase1 Base1 B9 control: Linux kernel/initrd standard"
    linux /boot/phase1/vmlinuz console=tty0 nomodeset vga=normal loglevel=7 debug
    initrd /boot/phase1/initrd.img
}

menuentry "B9 control - Linux kernel/initrd linux16 compatibility" {
    echo "Phase1 Base1 B9 control: linux16/initrd16 compatibility"
    linux16 /boot/phase1/vmlinuz console=tty0 nomodeset vga=normal loglevel=7 debug
    initrd16 /boot/phase1/initrd.img
}
EOF
fi

sudo tee -a "$MNT/boot/grub/grub.cfg" >/dev/null <<'EOF'

menuentry "B6 marker fallback - known good external USB proof" {
    echo "phase1 6.0.0 ready"
    echo "B6 candidate result: phase1_marker_seen"
    echo "B12 fallback marker reached from external USB"
    echo "This is not an installer, hardening, or daily-driver claim."
    sleep --interruptible 999
}
EOF

sudo cp "$MNT/boot/grub/grub.cfg" "$MNT/grub/grub.cfg"

sudo tee "$MNT/PHASE1-B12-DIAGNOSTIC.txt" >/dev/null <<EOF
Phase1 Base1 B12 external USB diagnostic kernel.
Profile: $PROFILE
Kernel: /boot/phase1/phase1-b12.elf

Primary evidence state:
- phase1_kernel_vga_console_seen: Phase1 diagnostic kernel screen appears.

Other useful states:
- phase1_grub_console_seen: GRUB-native fallback console appears.
- blocked_after_multiboot_load: GRUB loads ELF but no visible handoff occurs.
- phase1_marker_seen: marker fallback appears.

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
BASE1_B12_DIAGNOSTIC_KERNEL_PROFILE=$PROFILE
BASE1_B12_DIAGNOSTIC_KERNEL_TARGET=$USB
BASE1_B12_DIAGNOSTIC_KERNEL_PARTITION=$PART
BASE1_B12_DIAGNOSTIC_KERNEL_ELF=$ELF
BASE1_B12_DIAGNOSTIC_KERNEL_LINUX_CONTROLS=$HAS_LINUX
BASE1_B12_DIAGNOSTIC_KERNEL_RESULT=prepared
BASE1_B12_DIAGNOSTIC_KERNEL_EXPECTED_NEXT_RESULT=phase1_kernel_vga_console_seen
BASE1_B12_DIAGNOSTIC_KERNEL_CLAIM=not_claimed
BASE1_B12_NON_CLAIM_INSTALLER=1
BASE1_B12_NON_CLAIM_INTERNAL_DISK_INSTALL=1
BASE1_B12_NON_CLAIM_RECOVERY_COMPLETE=1
BASE1_B12_NON_CLAIM_HARDENED=1
BASE1_B12_NON_CLAIM_DAILY_DRIVER=1
EOF

printf '\nDONE: B12 diagnostic kernel USB prepared on %s\n' "$USB"
printf 'Boot the X200 from USB and choose B12-01 first.\n'
printf 'If the Phase1 diagnostic kernel screen appears, record: phase1_kernel_vga_console_seen\n'

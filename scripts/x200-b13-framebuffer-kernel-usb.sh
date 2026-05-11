#!/usr/bin/env sh
# Phase1 / Base1 X200 B13 framebuffer diagnostic kernel USB writer.
#
# Purpose:
#   Build one external USB that boots a Phase1-owned 32-bit Multiboot kernel
#   which requests a graphics framebuffer from GRUB and paints the screen
#   directly. This tests whether earlier B10/B12 attempts were executing while
#   invisible because the X200/Libreboot path stayed in graphics mode.
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
PROFILE="${BASE1_B13_PROFILE:-x200-supervisor-lite}"
OUT_DIR="${BASE1_B13_OUT:-build/base1-b13-framebuffer-kernel}"
SRC="$OUT_DIR/phase1-b13.S"
OBJ="$OUT_DIR/phase1-b13.o"
LDS="$OUT_DIR/phase1-b13.ld"
ELF="$OUT_DIR/phase1-b13.elf"

fail() { printf 'x200-b13-framebuffer-kernel-usb: %s\n' "$1" >&2; exit 1; }
need_cmd() { command -v "$1" >/dev/null 2>&1 || fail "missing command: $1"; }

partition_one() {
  case "$1" in
    /dev/nvme[0-9]n[0-9]|/dev/mmcblk[0-9]) printf '%sp1\n' "$1" ;;
    /dev/sd[a-z]|/dev/vd[a-z]|/dev/hd[a-z]) printf '%s1\n' "$1" ;;
    *) fail "use a whole-disk path such as /dev/sdb, not a partition" ;;
  esac
}

[ "$CONFIRM" = "YES_WRITE_USB" ] || fail "usage: sh scripts/x200-b13-framebuffer-kernel-usb.sh /dev/sdb YES_WRITE_USB"
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
REPORT="$OUT_DIR/b13-framebuffer-kernel-usb.env"

cleanup() { sudo umount "$MNT" 2>/dev/null || true; rmdir "$MNT" 2>/dev/null || true; }
trap cleanup EXIT INT TERM

mkdir -p "$OUT_DIR"

cat > "$SRC" <<'EOF'
.code32

/* Multiboot v1 header. Flags request page alignment, memory info, and video mode. */
.section .multiboot, "a"
.align 4
.long 0x1BADB002
.long 0x00000007
.long -(0x1BADB002 + 0x00000007)
.long 0
.long 0
.long 0
.long 0
.long 0
.long 0
.long 1024
.long 768
.long 32

.section .text, "ax"
.global _start
.type _start, @function
_start:
    cli
    mov $stack_top, %esp
    xor %ebp, %ebp
    mov %eax, mb_magic
    mov %ebx, mbi_ptr

    call vga_text_banner
    call serial_init
    mov $serial_banner, %esi
    call serial_print

    cmp $0x2BADB002, %eax
    jne no_framebuffer

    mov mbi_ptr, %esi
    mov (%esi), %eax
    test $0x1000, %eax
    jz no_framebuffer

    mov 88(%esi), %eax
    test %eax, %eax
    jz no_framebuffer
    mov %eax, fb_addr
    mov 96(%esi), %eax
    mov %eax, fb_pitch
    mov 100(%esi), %eax
    mov %eax, fb_width
    mov 104(%esi), %eax
    mov %eax, fb_height
    movzbl 108(%esi), %eax
    mov %eax, fb_bpp

    mov fb_bpp, %eax
    cmp $32, %eax
    je draw_fb32
    cmp $24, %eax
    je draw_fb24
    cmp $16, %eax
    je draw_fb16
    jmp no_framebuffer

draw_fb32:
    call fb32_paint
    jmp framebuffer_heartbeat32

draw_fb24:
    call fb24_paint
    jmp halt_forever

draw_fb16:
    call fb16_paint
    jmp halt_forever

no_framebuffer:
    mov $(0xB8000 + 18 * 160), %edi
    mov $nofb_line, %esi
    call vga_print
    jmp halt_forever

halt_forever:
    hlt
    jmp halt_forever

vga_text_banner:
    mov $0xB8000, %edi
    mov $0x0720, %ax
    mov $2000, %ecx
1:
    mov %ax, (%edi)
    add $2, %edi
    loop 1b
    mov $(0xB8000 + 0 * 160), %edi
    mov $vga_line1, %esi
    call vga_print
    mov $(0xB8000 + 2 * 160), %edi
    mov $vga_line2, %esi
    call vga_print
    mov $(0xB8000 + 4 * 160), %edi
    mov $vga_line3, %esi
    call vga_print
    ret

vga_print:
    lodsb
    test %al, %al
    jz 2f
    mov $0x0F, %ah
    mov %ax, (%edi)
    add $2, %edi
    jmp vga_print
2:
    ret

fb32_paint:
    mov fb_addr, %edi
    mov fb_height, %ebx
    cmp $768, %ebx
    jbe 3f
    mov $768, %ebx
3:
    test %ebx, %ebx
    jz 6f
    push %edi
    mov fb_width, %ecx
    cmp $1024, %ecx
    jbe 4f
    mov $1024, %ecx
4:
    mov $0x00401060, %eax
5:
    mov %eax, (%edi)
    add $4, %edi
    loop 5b
    pop %edi
    add fb_pitch, %edi
    dec %ebx
    jmp 3b
6:
    /* green top band */
    mov fb_addr, %edi
    mov $80, %ebx
7:
    push %edi
    mov fb_width, %ecx
    cmp $1024, %ecx
    jbe 8f
    mov $1024, %ecx
8:
    mov $0x0000AA40, %eax
9:
    mov %eax, (%edi)
    add $4, %edi
    loop 9b
    pop %edi
    add fb_pitch, %edi
    dec %ebx
    jnz 7b
    /* white vertical evidence bar */
    mov fb_addr, %edi
    add $140, %edi
    mov $220, %ebx
10:
    push %edi
    mov $22, %ecx
11:
    movl $0x00FFFFFF, (%edi)
    add $4, %edi
    loop 11b
    pop %edi
    add fb_pitch, %edi
    dec %ebx
    jnz 10b
    ret

fb24_paint:
    mov fb_addr, %edi
    mov fb_height, %ebx
    cmp $768, %ebx
    jbe 12f
    mov $768, %ebx
12:
    test %ebx, %ebx
    jz 15f
    push %edi
    mov fb_width, %ecx
    cmp $1024, %ecx
    jbe 13f
    mov $1024, %ecx
13:
    movb $0x60, %al
14:
    movb $0x60, (%edi)
    movb $0x10, 1(%edi)
    movb $0x40, 2(%edi)
    add $3, %edi
    loop 14b
    pop %edi
    add fb_pitch, %edi
    dec %ebx
    jmp 12b
15:
    ret

fb16_paint:
    mov fb_addr, %edi
    mov fb_height, %ebx
    cmp $768, %ebx
    jbe 16f
    mov $768, %ebx
16:
    test %ebx, %ebx
    jz 19f
    push %edi
    mov fb_width, %ecx
    cmp $1024, %ecx
    jbe 17f
    mov $1024, %ecx
17:
    mov $0x781F, %ax
18:
    mov %ax, (%edi)
    add $2, %edi
    loop 18b
    pop %edi
    add fb_pitch, %edi
    dec %ebx
    jmp 16b
19:
    ret

framebuffer_heartbeat32:
    mov $0, %edx
20:
    mov fb_addr, %edi
    add $12, %edi
    mov $64, %ebx
    test $1, %edx
    jz 21f
    mov $0x00FFFFFF, %eax
    jmp 22f
21:
    mov $0x00000000, %eax
22:
    push %edi
    mov $64, %ecx
23:
    mov %eax, (%edi)
    add $4, %edi
    loop 23b
    pop %edi
    add fb_pitch, %edi
    dec %ebx
    jnz 22b
    mov $'.', %al
    call serial_putc
    inc %edx
    mov $0x006FFFFF, %ecx
24:
    loop 24b
    jmp 20b

serial_init:
    mov $0x3F9, %dx
    mov $0x00, %al
    out %al, %dx
    mov $0x3FB, %dx
    mov $0x80, %al
    out %al, %dx
    mov $0x3F8, %dx
    mov $0x03, %al
    out %al, %dx
    mov $0x3F9, %dx
    mov $0x00, %al
    out %al, %dx
    mov $0x3FB, %dx
    mov $0x03, %al
    out %al, %dx
    mov $0x3FA, %dx
    mov $0xC7, %al
    out %al, %dx
    mov $0x3FC, %dx
    mov $0x0B, %al
    out %al, %dx
    ret

serial_print:
    lodsb
    test %al, %al
    jz 25f
    call serial_putc
    jmp serial_print
25:
    ret

serial_putc:
    push %edx
    push %eax
26:
    mov $0x3FD, %dx
    in %dx, %al
    test $0x20, %al
    jz 26b
    pop %eax
    mov $0x3F8, %dx
    out %al, %dx
    pop %edx
    ret

.section .rodata, "a"
serial_banner: .asciz "phase1 6.0.0 ready - B13 framebuffer diagnostic kernel\r\n"
vga_line1: .asciz "phase1 6.0.0 ready"
vga_line2: .asciz "Base1 B13 framebuffer diagnostic kernel"
vga_line3: .asciz "If the screen turns green/purple or blinks, record phase1_kernel_framebuffer_seen"
nofb_line: .asciz "No framebuffer info from Multiboot; VGA text fallback active."

.section .data, "aw"
mb_magic: .long 0
mbi_ptr: .long 0
fb_addr: .long 0
fb_pitch: .long 0
fb_width: .long 0
fb_height: .long 0
fb_bpp: .long 0

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

printf 'PHASE1 BASE1 B13 FRAMEBUFFER KERNEL USB WRITER\n\n'
printf 'profile       : %s\n' "$PROFILE"
printf 'target disk   : %s\n' "$USB"
printf 'partition     : %s\n' "$PART"
printf 'phase1 kernel : %s\n' "$ELF"
printf 'scope         : external USB Phase1 framebuffer diagnostic evidence\n'
printf 'claim         : not_claimed\n\n'
printf 'Building Phase1-owned framebuffer diagnostic kernel...\n'

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

sudo mkfs.vfat -F 32 -n PHASE1B13 "$PART"
sudo mount "$PART" "$MNT"
sudo mkdir -p "$MNT/boot/grub" "$MNT/grub" "$MNT/boot/phase1" "$MNT/phase1-evidence"
sudo cp "$ELF" "$MNT/boot/phase1/phase1-b13.elf"

sudo tee "$MNT/boot/grub/grub.cfg" >/dev/null <<'EOF'
set timeout=30
set default=0
set pager=1

menuentry "B13-01 Phase1 framebuffer diagnostic kernel" {
    echo "Phase1 Base1 B13: loading framebuffer diagnostic kernel"
    echo "Target evidence: phase1_kernel_framebuffer_seen"
    echo "Success: screen changes to a Phase1-painted green/purple framebuffer with blink."
    multiboot /boot/phase1/phase1-b13.elf
    boot
}

menuentry "B13-02 Phase1 framebuffer diagnostic kernel - compatibility entry" {
    echo "Phase1 Base1 B13: compatibility framebuffer diagnostic entry"
    echo "Target evidence: phase1_kernel_framebuffer_seen"
    multiboot /boot/phase1/phase1-b13.elf
    boot
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
    echo "B13 fallback marker reached from external USB"
    echo "This is not an installer, hardening, or daily-driver claim."
    sleep --interruptible 999
}
EOF

sudo cp "$MNT/boot/grub/grub.cfg" "$MNT/grub/grub.cfg"

sudo tee "$MNT/PHASE1-B13-FRAMEBUFFER.txt" >/dev/null <<EOF
Phase1 Base1 B13 external USB framebuffer diagnostic kernel.
Profile: $PROFILE
Kernel: /boot/phase1/phase1-b13.elf

Primary evidence state:
- phase1_kernel_framebuffer_seen: screen changes to Phase1-painted framebuffer.

Other useful states:
- phase1_grub_console_seen: GRUB-native fallback console appears.
- blocked_after_multiboot_load: GRUB loads ELF but no visible framebuffer/text handoff occurs.
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
BASE1_B13_FRAMEBUFFER_KERNEL_PROFILE=$PROFILE
BASE1_B13_FRAMEBUFFER_KERNEL_TARGET=$USB
BASE1_B13_FRAMEBUFFER_KERNEL_PARTITION=$PART
BASE1_B13_FRAMEBUFFER_KERNEL_ELF=$ELF
BASE1_B13_FRAMEBUFFER_KERNEL_RESULT=prepared
BASE1_B13_FRAMEBUFFER_KERNEL_EXPECTED_NEXT_RESULT=phase1_kernel_framebuffer_seen
BASE1_B13_FRAMEBUFFER_KERNEL_CLAIM=not_claimed
BASE1_B13_NON_CLAIM_INSTALLER=1
BASE1_B13_NON_CLAIM_INTERNAL_DISK_INSTALL=1
BASE1_B13_NON_CLAIM_RECOVERY_COMPLETE=1
BASE1_B13_NON_CLAIM_HARDENED=1
BASE1_B13_NON_CLAIM_DAILY_DRIVER=1
EOF

printf '\nDONE: B13 framebuffer diagnostic USB prepared on %s\n' "$USB"
printf 'Boot the X200 from USB and choose B13-01 first.\n'
printf 'If the screen changes to a painted/blinking framebuffer, record: phase1_kernel_framebuffer_seen\n'

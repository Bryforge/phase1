#!/usr/bin/env sh
# Phase1 / Base1 X200 B16 visible-first framebuffer console USB writer.
#
# Purpose:
#   Keep the proven B13 framebuffer path and make visibility happen before any
#   keyboard/controller work. B15 can block before drawing; B16 paints first,
#   then attempts bounded keyboard polling so physical testing moves faster.
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
PROFILE="${BASE1_B16_PROFILE:-x200-supervisor-lite}"
OUT_DIR="${BASE1_B16_OUT:-build/base1-b16-visible-first-console}"
SRC="$OUT_DIR/phase1-b16.S"
OBJ="$OUT_DIR/phase1-b16.o"
LDS="$OUT_DIR/phase1-b16.ld"
ELF="$OUT_DIR/phase1-b16.elf"

fail() { printf 'x200-b16-visible-first-console-usb: %s\n' "$1" >&2; exit 1; }
need_cmd() { command -v "$1" >/dev/null 2>&1 || fail "missing command: $1"; }

partition_one() {
  case "$1" in
    /dev/nvme[0-9]n[0-9]|/dev/mmcblk[0-9]) printf '%sp1\n' "$1" ;;
    /dev/sd[a-z]|/dev/vd[a-z]|/dev/hd[a-z]) printf '%s1\n' "$1" ;;
    *) fail "use a whole-disk path such as /dev/sdb, not a partition" ;;
  esac
}

[ "$CONFIRM" = "YES_WRITE_USB" ] || fail "usage: sh scripts/x200-b16-visible-first-console-usb.sh /dev/sdb YES_WRITE_USB"
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
REPORT="$OUT_DIR/b16-visible-first-console-usb.env"

cleanup() { sudo umount "$MNT" 2>/dev/null || true; rmdir "$MNT" 2>/dev/null || true; }
trap cleanup EXIT INT TERM

mkdir -p "$OUT_DIR"

cat > "$SRC" <<'EOF'
.code32

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

    call serial_init
    mov $serial_banner, %esi
    call serial_print

    call setup_fb_or_text
    call paint_visible_console
    sti

main_loop:
    call pulse_box
    call poll_ps2_nonblocking
    jmp main_loop

setup_fb_or_text:
    mov mb_magic, %eax
    cmp $0x2BADB002, %eax
    jne text_fallback
    mov mbi_ptr, %esi
    mov (%esi), %eax
    test $0x1000, %eax
    jz text_fallback
    mov 88(%esi), %eax
    test %eax, %eax
    jz text_fallback
    mov %eax, fb_addr
    mov 96(%esi), %eax
    mov %eax, fb_pitch
    mov 100(%esi), %eax
    mov %eax, fb_width
    mov 104(%esi), %eax
    mov %eax, fb_height
    movzbl 108(%esi), %eax
    mov %eax, fb_bpp
    cmp $32, %eax
    jne text_fallback
    ret

text_fallback:
    mov $0xB8000, %edi
    mov $0x0720, %ax
    mov $2000, %ecx
1:
    mov %ax, (%edi)
    add $2, %edi
    loop 1b
    mov $(0xB8000 + 0 * 160), %edi
    mov $fallback1, %esi
    call vga_print
    mov $(0xB8000 + 2 * 160), %edi
    mov $fallback2, %esi
    call vga_print
2:
    hlt
    jmp 2b

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

paint_visible_console:
    call clear_fb
    /* purple title bar */
    mov $0, %eax
    mov $0, %ebx
    mov $1024, %ecx
    mov $110, %esi
    mov $0x00400070, %edx
    call fill_rect

    /* bright success strip: visible immediately */
    mov $0, %eax
    mov $120, %ebx
    mov $1024, %ecx
    mov $80, %esi
    mov $0x0000AA55, %edx
    call fill_rect

    /* console panel */
    mov $24, %eax
    mov $240, %ebx
    mov $760, %ecx
    mov $260, %esi
    mov $0x00101828, %edx
    call fill_rect

    /* key area */
    mov $24, %eax
    mov $540, %ebx
    mov $760, %ecx
    mov $80, %esi
    mov $0x00000000, %edx
    call fill_rect

    /* crude text block rows */
    mov $24, %eax
    mov $24, %ebx
    mov $title_blocks, %esi
    call block_text
    mov $24, %eax
    mov $144, %ebx
    mov $result_blocks, %esi
    call block_text
    mov $40, %eax
    mov $270, %ebx
    mov $line_blocks, %esi
    call block_text
    mov $40, %eax
    mov $570, %ebx
    mov $key_blocks, %esi
    call block_text
    ret

clear_fb:
    mov fb_addr, %edi
    mov fb_height, %ebx
    cmp $768, %ebx
    jbe 4f
    mov $768, %ebx
4:
    test %ebx, %ebx
    jz 7f
    push %edi
    mov fb_width, %ecx
    cmp $1024, %ecx
    jbe 5f
    mov $1024, %ecx
5:
    mov $0x00070A10, %eax
6:
    mov %eax, (%edi)
    add $4, %edi
    loop 6b
    pop %edi
    add fb_pitch, %edi
    dec %ebx
    jmp 4b
7:
    ret

poll_ps2_nonblocking:
    in $0x64, %al
    test $1, %al
    jz 8f
    in $0x60, %al
    test $0x80, %al
    jnz 8f
    mov %al, last_scancode
    inc key_count
    call draw_key_change
8:
    ret

draw_key_change:
    /* Change key area to bright yellow/green block. This is the success sign. */
    mov $260, %eax
    mov $552, %ebx
    mov $220, %ecx
    mov $56, %esi
    mov $0x0000FF88, %edx
    call fill_rect
    mov $serial_key, %esi
    call serial_print
    ret

pulse_box:
    inc tick
    mov tick, %eax
    and $0x00080000, %eax
    jz 9f
    mov $0x00FFFFFF, %edx
    jmp 10f
9:
    mov $0x00000000, %edx
10:
    mov $850, %eax
    mov $560, %ebx
    mov $60, %ecx
    mov $60, %esi
    call fill_rect
    ret

/* x=eax, y=ebx, w=ecx, h=esi, color=edx */
fill_rect:
    push %eax
    push %ebx
    push %ecx
    push %esi
    push %edi
    mov fb_addr, %edi
    imul fb_pitch, %ebx
    add %ebx, %edi
    shl $2, %eax
    add %eax, %edi
11:
    test %esi, %esi
    jz 14f
    push %edi
    push %ecx
12:
    test %ecx, %ecx
    jz 13f
    mov %edx, (%edi)
    add $4, %edi
    dec %ecx
    jmp 12b
13:
    pop %ecx
    pop %edi
    add fb_pitch, %edi
    dec %esi
    jmp 11b
14:
    pop %edi
    pop %esi
    pop %ecx
    pop %ebx
    pop %eax
    ret

/* Simple robust block text: non-space chars become white blocks. */
block_text:
    push %eax
    push %ebx
    push %esi
15:
    lodsb
    test %al, %al
    jz 17f
    cmp $' ', %al
    je 16f
    push %eax
    push %ebx
    push %ecx
    push %edx
    push %esi
    mov $8, %ecx
    mov $18, %esi
    mov $0x00FFFFFF, %edx
    call fill_rect
    pop %esi
    pop %edx
    pop %ecx
    pop %ebx
    pop %eax
16:
    add $12, %eax
    jmp 15b
17:
    pop %esi
    pop %ebx
    pop %eax
    ret

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
    jz 18f
    call serial_putc
    jmp serial_print
18:
    ret

serial_putc:
    push %edx
    push %eax
19:
    mov $0x3FD, %dx
    in %dx, %al
    test $0x20, %al
    jz 19b
    pop %eax
    mov $0x3F8, %dx
    out %al, %dx
    pop %edx
    ret

.section .rodata, "a"
serial_banner: .asciz "phase1 6.0.0 ready - B16 visible-first console\r\n"
serial_key: .asciz "key\r\n"
fallback1: .asciz "phase1 6.0.0 ready - B16 VGA fallback"
fallback2: .asciz "No framebuffer information; external USB path is still active."
title_blocks: .asciz "phase1 ready B16 visible first console"
result_blocks: .asciz "result phase1 kernel framebuffer seen"
line_blocks: .asciz "screen first keyboard optional no hang"
key_blocks: .asciz "press keys key area changes if input works"

.section .data, "aw"
mb_magic: .long 0
mbi_ptr: .long 0
fb_addr: .long 0
fb_pitch: .long 0
fb_width: .long 0
fb_height: .long 0
fb_bpp: .long 0
tick: .long 0
key_count: .long 0
last_scancode: .byte 0

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

printf 'PHASE1 BASE1 B16 VISIBLE-FIRST CONSOLE USB WRITER\n\n'
printf 'profile       : %s\n' "$PROFILE"
printf 'target disk   : %s\n' "$USB"
printf 'partition     : %s\n' "$PART"
printf 'phase1 kernel : %s\n' "$ELF"
printf 'scope         : external USB visible-first operational evidence\n'
printf 'claim         : not_claimed\n\n'
printf 'Building Phase1-owned B16 visible-first console kernel...\n'

as --32 -o "$OBJ" "$SRC"
ld -m elf_i386 -T "$LDS" -o "$ELF" "$OBJ"
chmod 0644 "$ELF"
[ -s "$ELF" ] || fail "failed to build ELF: $ELF"

if command -v grub-file >/dev/null 2>&1; then
  grub-file --is-x86-multiboot "$ELF" || fail "ELF failed GRUB multiboot validation"
fi

printf 'Phase1 B16 kernel SHA256:\n'
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

sudo mkfs.vfat -F 32 -n PHASE1B16 "$PART"
sudo mount "$PART" "$MNT"
sudo mkdir -p "$MNT/boot/grub" "$MNT/grub" "$MNT/boot/phase1" "$MNT/phase1-evidence"
sudo cp "$ELF" "$MNT/boot/phase1/phase1-b16.elf"

sudo tee "$MNT/boot/grub/grub.cfg" >/dev/null <<'EOF'
set timeout=30
set default=0
set pager=1

menuentry "B16-01 Phase1 visible-first framebuffer console" {
    echo "Phase1 Base1 B16: visible-first framebuffer console"
    echo "Target evidence: phase1_kernel_framebuffer_seen"
    echo "Keyboard is optional; screen visibility is the first invariant."
    multiboot /boot/phase1/phase1-b16.elf
    boot
}

menuentry "B16-02 Phase1 visible-first framebuffer console - compatibility" {
    echo "Phase1 Base1 B16: compatibility visible-first console"
    multiboot /boot/phase1/phase1-b16.elf
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
    echo "B16 fallback marker reached from external USB"
    echo "This is not an installer, hardening, or daily-driver claim."
    sleep --interruptible 999
}
EOF

sudo cp "$MNT/boot/grub/grub.cfg" "$MNT/grub/grub.cfg"

sudo tee "$MNT/PHASE1-B16-VISIBLE-FIRST.txt" >/dev/null <<EOF
Phase1 Base1 B16 visible-first operational USB.
Profile: $PROFILE
Kernel: /boot/phase1/phase1-b16.elf

Evidence states:
- phase1_kernel_framebuffer_seen: framebuffer console appears.
- phase1_kernel_keyboard_console_seen: key area changes after keypress.
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
BASE1_B16_VISIBLE_FIRST_CONSOLE_PROFILE=$PROFILE
BASE1_B16_VISIBLE_FIRST_CONSOLE_TARGET=$USB
BASE1_B16_VISIBLE_FIRST_CONSOLE_PARTITION=$PART
BASE1_B16_VISIBLE_FIRST_CONSOLE_ELF=$ELF
BASE1_B16_VISIBLE_FIRST_CONSOLE_RESULT=prepared
BASE1_B16_VISIBLE_FIRST_CONSOLE_EXPECTED_NEXT_RESULT=phase1_kernel_framebuffer_seen
BASE1_B16_VISIBLE_FIRST_CONSOLE_CLAIM=not_claimed
BASE1_B16_NON_CLAIM_INSTALLER=1
BASE1_B16_NON_CLAIM_INTERNAL_DISK_INSTALL=1
BASE1_B16_NON_CLAIM_RECOVERY_COMPLETE=1
BASE1_B16_NON_CLAIM_HARDENED=1
BASE1_B16_NON_CLAIM_DAILY_DRIVER=1
EOF

printf '\nDONE: B16 visible-first USB prepared on %s\n' "$USB"
printf 'Boot the X200 from USB and choose B16-01 first.\n'
printf 'If the screen appears, continue from that stable framebuffer baseline.\n'

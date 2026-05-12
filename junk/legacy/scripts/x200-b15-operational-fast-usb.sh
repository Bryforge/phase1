#!/usr/bin/env sh
# Phase1 / Base1 X200 B15 operational fast USB writer.
#
# Purpose:
#   Build one external USB for fast physical progress on the X200. This combines
#   the proven B13 framebuffer path with a more serious B15 keyboard path that
#   initializes the PS/2 controller before polling. It also keeps B13 and B11
#   fallbacks on the same USB so the operator does not have to rebuild for every
#   small test.
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
PROFILE="${BASE1_B15_PROFILE:-x200-supervisor-lite}"
OUT_DIR="${BASE1_B15_OUT:-build/base1-b15-operational-fast-usb}"
SRC="$OUT_DIR/phase1-b15.S"
OBJ="$OUT_DIR/phase1-b15.o"
LDS="$OUT_DIR/phase1-b15.ld"
ELF="$OUT_DIR/phase1-b15.elf"

fail() { printf 'x200-b15-operational-fast-usb: %s\n' "$1" >&2; exit 1; }
need_cmd() { command -v "$1" >/dev/null 2>&1 || fail "missing command: $1"; }

partition_one() {
  case "$1" in
    /dev/nvme[0-9]n[0-9]|/dev/mmcblk[0-9]) printf '%sp1\n' "$1" ;;
    /dev/sd[a-z]|/dev/vd[a-z]|/dev/hd[a-z]) printf '%s1\n' "$1" ;;
    *) fail "use a whole-disk path such as /dev/sdb, not a partition" ;;
  esac
}

[ "$CONFIRM" = "YES_WRITE_USB" ] || fail "usage: sh scripts/x200-b15-operational-fast-usb.sh /dev/sdb YES_WRITE_USB"
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
REPORT="$OUT_DIR/b15-operational-fast-usb.env"

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

    call setup_framebuffer
    call ps2_keyboard_init
    call draw_screen
    sti

main_loop:
    call heartbeat
    call poll_keyboard
    jmp main_loop

setup_framebuffer:
    cmp $0x2BADB002, %eax
    jne fallback_text
    mov mbi_ptr, %esi
    mov (%esi), %eax
    test $0x1000, %eax
    jz fallback_text
    mov 88(%esi), %eax
    test %eax, %eax
    jz fallback_text
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
    jne fallback_text
    ret

fallback_text:
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

/* ---------------- PS/2 keyboard initialization ---------------- */
wait_input_clear:
    mov $0x10000, %ecx
4:
    in $0x64, %al
    test $0x02, %al
    jz 5f
    loop 4b
5:
    ret

wait_output_full:
    mov $0x10000, %ecx
6:
    in $0x64, %al
    test $0x01, %al
    jnz 7f
    loop 6b
7:
    ret

flush_output:
    mov $32, %ecx
8:
    in $0x64, %al
    test $0x01, %al
    jz 9f
    in $0x60, %al
    loop 8b
9:
    ret

send_cmd:
    call wait_input_clear
    out %al, $0x64
    ret

send_data:
    call wait_input_clear
    out %al, $0x60
    ret

read_data:
    call wait_output_full
    in $0x60, %al
    ret

ps2_keyboard_init:
    call flush_output

    mov $0xAD, %al       /* disable first PS/2 port */
    call send_cmd
    mov $0xA7, %al       /* disable second PS/2 port */
    call send_cmd
    call flush_output

    mov $0x20, %al       /* read controller config byte */
    call send_cmd
    call read_data
    and $0xBC, %al       /* clear IRQs + translation bit conservatively */
    mov %al, ps2_config

    mov $0x60, %al       /* write controller config byte */
    call send_cmd
    mov ps2_config, %al
    call send_data

    mov $0xAE, %al       /* enable first PS/2 port */
    call send_cmd

    mov $0xFF, %al       /* reset keyboard */
    call send_data
    call read_data       /* ACK or response */
    call read_data       /* BAT completion if present */

    mov $0xF4, %al       /* enable scanning */
    call send_data
    call read_data       /* ACK if present */

    mov $1, keyboard_initialized
    ret

poll_keyboard:
    in $0x64, %al
    test $0x01, %al
    jz 10f
    in $0x60, %al
    mov %al, last_scancode
    test $0x80, %al
    jnz 10f
    inc key_count
    call show_key_event
10:
    ret

/* ---------------- framebuffer UI ---------------- */
draw_screen:
    call fb_clear
    call draw_header
    call draw_body
    ret

fb_clear:
    mov fb_addr, %edi
    mov fb_height, %ebx
    cmp $768, %ebx
    jbe 11f
    mov $768, %ebx
11:
    test %ebx, %ebx
    jz 14f
    push %edi
    mov fb_width, %ecx
    cmp $1024, %ecx
    jbe 12f
    mov $1024, %ecx
12:
    mov $0x00080C14, %eax
13:
    mov %eax, (%edi)
    add $4, %edi
    loop 13b
    pop %edi
    add fb_pitch, %edi
    dec %ebx
    jmp 11b
14:
    ret

draw_header:
    mov $0, %eax
    mov $0, %ebx
    mov $1024, %ecx
    mov $96, %esi
    mov $0x003A0066, %edx
    call fill_rect
    mov $24, %eax
    mov $18, %ebx
    mov $title1, %esi
    call fb_text_blocks
    mov $24, %eax
    mov $48, %ebx
    mov $title2, %esi
    call fb_text_blocks
    ret

draw_body:
    mov $24, %eax
    mov $128, %ebx
    mov $line1, %esi
    call fb_text_blocks
    mov $24, %eax
    mov $160, %ebx
    mov $line2, %esi
    call fb_text_blocks
    mov $24, %eax
    mov $192, %ebx
    mov $line3, %esi
    call fb_text_blocks
    mov $24, %eax
    mov $240, %ebx
    mov $line4, %esi
    call fb_text_blocks
    mov $24, %eax
    mov $300, %ebx
    mov $keyline, %esi
    call fb_text_blocks
    ret

show_key_event:
    mov $250, %eax
    mov $296, %ebx
    mov $180, %ecx
    mov $28, %esi
    mov $0x00080C14, %edx
    call fill_rect

    movzbl last_scancode, %eax
    and $0x0F, %eax
    shl $5, %eax
    add $250, %eax
    mov $300, %ebx
    mov $24, %ecx
    mov $20, %esi
    mov $0x0000FF66, %edx
    call fill_rect

    mov $24, %eax
    mov $352, %ebx
    mov $seenline, %esi
    call fb_text_blocks
    ret

heartbeat:
    inc heartbeat_counter
    mov heartbeat_counter, %eax
    and $0x00080000, %eax
    jz 15f
    mov $0x00FFFFFF, %edx
    jmp 16f
15:
    mov $0x00000000, %edx
16:
    mov $24, %eax
    mov $420, %ebx
    mov $40, %ecx
    mov $40, %esi
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
17:
    test %esi, %esi
    jz 20f
    push %edi
    push %ecx
18:
    test %ecx, %ecx
    jz 19f
    mov %edx, (%edi)
    add $4, %edi
    dec %ecx
    jmp 18b
19:
    pop %ecx
    pop %edi
    add fb_pitch, %edi
    dec %esi
    jmp 17b
20:
    pop %edi
    pop %esi
    pop %ecx
    pop %ebx
    pop %eax
    ret

/* crude visible text: each non-space char is a block. It is intentionally robust. */
fb_text_blocks:
    push %eax
    push %ebx
    push %esi
21:
    lodsb
    test %al, %al
    jz 23f
    cmp $' ', %al
    je 22f
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
22:
    add $12, %eax
    jmp 21b
23:
    pop %esi
    pop %ebx
    pop %eax
    ret

/* ---------------- serial ---------------- */
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
    jz 24f
    call serial_putc
    jmp serial_print
24:
    ret

serial_putc:
    push %edx
    push %eax
25:
    mov $0x3FD, %dx
    in %dx, %al
    test $0x20, %al
    jz 25b
    pop %eax
    mov $0x3F8, %dx
    out %al, %dx
    pop %edx
    ret

.section .rodata, "a"
serial_banner: .asciz "phase1 6.0.0 ready - B15 operational fast console\r\n"
fallback1: .asciz "phase1 6.0.0 ready - B15 VGA text fallback"
fallback2: .asciz "No framebuffer; record VGA fallback if visible."
title1: .asciz "phase1 6.0.0 ready"
title2: .asciz "Base1 operational framebuffer console"
line1: .asciz "B15 candidate result: phase1_kernel_keyboard_console_seen"
line2: .asciz "PS2 keyboard controller initialization attempted before polling."
line3: .asciz "Press keys. If the key area changes, keyboard input is confirmed."
line4: .asciz "Non-claims: no installer, no disk install, no daily-driver claim."
keyline: .asciz "last key area:"
seenline: .asciz "keyboard event observed"

.section .data, "aw"
mb_magic: .long 0
mbi_ptr: .long 0
fb_addr: .long 0
fb_pitch: .long 0
fb_width: .long 0
fb_height: .long 0
fb_bpp: .long 0
heartbeat_counter: .long 0
key_count: .long 0
keyboard_initialized: .long 0
ps2_config: .byte 0
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

printf 'PHASE1 BASE1 B15 OPERATIONAL FAST USB WRITER\n\n'
printf 'profile       : %s\n' "$PROFILE"
printf 'target disk   : %s\n' "$USB"
printf 'partition     : %s\n' "$PART"
printf 'phase1 kernel : %s\n' "$ELF"
printf 'scope         : external USB operational console evidence\n'
printf 'claim         : not_claimed\n\n'
printf 'Building Phase1-owned B15 operational console kernel...\n'

as --32 -o "$OBJ" "$SRC"
ld -m elf_i386 -T "$LDS" -o "$ELF" "$OBJ"
chmod 0644 "$ELF"
[ -s "$ELF" ] || fail "failed to build ELF: $ELF"

if command -v grub-file >/dev/null 2>&1; then
  grub-file --is-x86-multiboot "$ELF" || fail "ELF failed GRUB multiboot validation"
fi

printf 'Phase1 B15 kernel SHA256:\n'
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

sudo mkfs.vfat -F 32 -n PHASE1B15 "$PART"
sudo mount "$PART" "$MNT"
sudo mkdir -p "$MNT/boot/grub" "$MNT/grub" "$MNT/boot/phase1" "$MNT/phase1-evidence"
sudo cp "$ELF" "$MNT/boot/phase1/phase1-b15.elf"

sudo tee "$MNT/boot/grub/grub.cfg" >/dev/null <<'EOF'
set timeout=30
set default=0
set pager=1

menuentry "B15-01 Phase1 operational framebuffer console" {
    echo "Phase1 Base1 B15: loading operational framebuffer console"
    echo "Target evidence: phase1_kernel_keyboard_console_seen"
    echo "Success: framebuffer console appears and keypresses change the key area."
    multiboot /boot/phase1/phase1-b15.elf
    boot
}

menuentry "B15-02 Phase1 operational framebuffer console - compatibility entry" {
    echo "Phase1 Base1 B15: compatibility operational console"
    multiboot /boot/phase1/phase1-b15.elf
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
    echo "B15 fallback marker reached from external USB"
    echo "This is not an installer, hardening, or daily-driver claim."
    sleep --interruptible 999
}
EOF

sudo cp "$MNT/boot/grub/grub.cfg" "$MNT/grub/grub.cfg"

sudo tee "$MNT/PHASE1-B15-OPERATIONAL.txt" >/dev/null <<EOF
Phase1 Base1 B15 operational fast USB.
Profile: $PROFILE
Kernel: /boot/phase1/phase1-b15.elf

Primary evidence state:
- phase1_kernel_keyboard_console_seen: framebuffer console appears and keypresses update the key area.

Other useful states:
- phase1_kernel_framebuffer_seen: framebuffer appears but keyboard update is not observed.
- phase1_keyboard_input_not_observed: B15 appears but no key event is observed after testing keys.
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
BASE1_B15_OPERATIONAL_FAST_USB_PROFILE=$PROFILE
BASE1_B15_OPERATIONAL_FAST_USB_TARGET=$USB
BASE1_B15_OPERATIONAL_FAST_USB_PARTITION=$PART
BASE1_B15_OPERATIONAL_FAST_USB_ELF=$ELF
BASE1_B15_OPERATIONAL_FAST_USB_RESULT=prepared
BASE1_B15_OPERATIONAL_FAST_USB_EXPECTED_NEXT_RESULT=phase1_kernel_keyboard_console_seen
BASE1_B15_OPERATIONAL_FAST_USB_CLAIM=not_claimed
BASE1_B15_NON_CLAIM_INSTALLER=1
BASE1_B15_NON_CLAIM_INTERNAL_DISK_INSTALL=1
BASE1_B15_NON_CLAIM_RECOVERY_COMPLETE=1
BASE1_B15_NON_CLAIM_HARDENED=1
BASE1_B15_NON_CLAIM_DAILY_DRIVER=1
EOF

printf '\nDONE: B15 operational fast USB prepared on %s\n' "$USB"
printf 'Boot the X200 from USB and choose B15-01 first.\n'
printf 'If keypresses update the key area, record: phase1_kernel_keyboard_console_seen\n'

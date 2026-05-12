#!/usr/bin/env sh
# Phase1 / Base1 X200 B14 framebuffer keyboard console USB writer.
#
# Purpose:
#   Build one external USB that boots a Phase1-owned 32-bit Multiboot kernel
#   with framebuffer output and PS/2 keyboard polling. B13 proved framebuffer
#   execution. B14 turns that proof into a tiny interactive Phase1-owned
#   physical console.
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
PROFILE="${BASE1_B14_PROFILE:-x200-supervisor-lite}"
OUT_DIR="${BASE1_B14_OUT:-build/base1-b14-framebuffer-keyboard-console}"
SRC="$OUT_DIR/phase1-b14.S"
OBJ="$OUT_DIR/phase1-b14.o"
LDS="$OUT_DIR/phase1-b14.ld"
ELF="$OUT_DIR/phase1-b14.elf"

fail() { printf 'x200-b14-framebuffer-keyboard-console-usb: %s\n' "$1" >&2; exit 1; }
need_cmd() { command -v "$1" >/dev/null 2>&1 || fail "missing command: $1"; }

partition_one() {
  case "$1" in
    /dev/nvme[0-9]n[0-9]|/dev/mmcblk[0-9]) printf '%sp1\n' "$1" ;;
    /dev/sd[a-z]|/dev/vd[a-z]|/dev/hd[a-z]) printf '%s1\n' "$1" ;;
    *) fail "use a whole-disk path such as /dev/sdb, not a partition" ;;
  esac
}

[ "$CONFIRM" = "YES_WRITE_USB" ] || fail "usage: sh scripts/x200-b14-framebuffer-keyboard-console-usb.sh /dev/sdb YES_WRITE_USB"
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
REPORT="$OUT_DIR/b14-framebuffer-keyboard-console-usb.env"

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

    call fb32_clear
    call draw_static_screen
    jmp console_loop

text_fallback:
    call vga_text_fallback
    jmp halt_forever

console_loop:
    call blink_cursor
    call keyboard_poll
    jmp console_loop

halt_forever:
    hlt
    jmp halt_forever

/* ---------------- framebuffer drawing ---------------- */
fb32_clear:
    mov fb_addr, %edi
    mov fb_height, %ebx
    cmp $768, %ebx
    jbe 1f
    mov $768, %ebx
1:
    test %ebx, %ebx
    jz 4f
    push %edi
    mov fb_width, %ecx
    cmp $1024, %ecx
    jbe 2f
    mov $1024, %ecx
2:
    mov $0x00101018, %eax
3:
    mov %eax, (%edi)
    add $4, %edi
    loop 3b
    pop %edi
    add fb_pitch, %edi
    dec %ebx
    jmp 1b
4:
    ret

draw_static_screen:
    /* Header bar */
    mov fb_addr, %edi
    mov $88, %ebx
5:
    push %edi
    mov fb_width, %ecx
    cmp $1024, %ecx
    jbe 6f
    mov $1024, %ecx
6:
    mov $0x00300058, %eax
7:
    mov %eax, (%edi)
    add $4, %edi
    loop 7b
    pop %edi
    add fb_pitch, %edi
    dec %ebx
    jnz 5b

    mov $16, %eax
    mov $16, %ebx
    mov $title1, %esi
    call fb_text
    mov $16, %eax
    mov $36, %ebx
    mov $title2, %esi
    call fb_text
    mov $16, %eax
    mov $64, %ebx
    mov $result_line, %esi
    call fb_text

    mov $16, %eax
    mov $120, %ebx
    mov $body1, %esi
    call fb_text
    mov $16, %eax
    mov $144, %ebx
    mov $body2, %esi
    call fb_text
    mov $16, %eax
    mov $168, %ebx
    mov $body3, %esi
    call fb_text
    mov $16, %eax
    mov $192, %ebx
    mov $body4, %esi
    call fb_text
    mov $16, %eax
    mov $236, %ebx
    mov $prompt, %esi
    call fb_text
    mov $16, %eax
    mov $280, %ebx
    mov $input_label, %esi
    call fb_text
    ret

blink_cursor:
    inc blink_counter
    mov blink_counter, %eax
    and $0x00040000, %eax
    jz 8f
    mov $0x00FFFFFF, %edx
    jmp 9f
8:
    mov $0x00000000, %edx
9:
    mov $180, %eax
    mov $280, %ebx
    mov $12, %ecx
    mov $18, %esi
    call fill_rect
    ret

keyboard_poll:
    in $0x64, %al
    test $1, %al
    jz 10f
    in $0x60, %al
    test $0x80, %al
    jnz 10f
    mov %al, last_scancode
    call show_key_event
10:
    ret

show_key_event:
    mov $16, %eax
    mov $324, %ebx
    mov $key_seen, %esi
    call fb_text
    mov $180, %eax
    mov $324, %ebx
    call draw_hex_scancode
    mov $serial_key, %esi
    call serial_print
    ret

/* x=eax y=ebx color=edx w=ecx h=esi */
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

/* x=eax y=ebx text=esi */
fb_text:
    push %eax
    push %ebx
    push %esi
15:
    lodsb
    test %al, %al
    jz 16f
    call fb_char_box
    add $10, %eax
    jmp 15b
16:
    pop %esi
    pop %ebx
    pop %eax
    ret

/* Draw readable block-glyph text: every non-space char becomes a small white box. */
fb_char_box:
    cmp $' ', %al
    je 17f
    push %eax
    push %ebx
    push %ecx
    push %edx
    push %esi
    mov $0x00FFFFFF, %edx
    mov $7, %ecx
    mov $14, %esi
    call fill_rect
    pop %esi
    pop %edx
    pop %ecx
    pop %ebx
    pop %eax
17:
    ret

draw_hex_scancode:
    push %eax
    push %ebx
    push %ecx
    push %edx
    movzbl last_scancode, %edx
    mov $0x00FFCC00, %edx
    mov $40, %ecx
    mov $18, %esi
    call fill_rect
    pop %edx
    pop %ecx
    pop %ebx
    pop %eax
    ret

/* ---------------- VGA text fallback ---------------- */
vga_text_fallback:
    mov $0xB8000, %edi
    mov $0x0720, %ax
    mov $2000, %ecx
18:
    mov %ax, (%edi)
    add $2, %edi
    loop 18b
    mov $(0xB8000 + 0 * 160), %edi
    mov $fallback1, %esi
    call vga_print
    mov $(0xB8000 + 2 * 160), %edi
    mov $fallback2, %esi
    call vga_print
    ret

vga_print:
    lodsb
    test %al, %al
    jz 19f
    mov $0x0F, %ah
    mov %ax, (%edi)
    add $2, %edi
    jmp vga_print
19:
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
    jz 20f
    call serial_putc
    jmp serial_print
20:
    ret

serial_putc:
    push %edx
    push %eax
21:
    mov $0x3FD, %dx
    in %dx, %al
    test $0x20, %al
    jz 21b
    pop %eax
    mov $0x3F8, %dx
    out %al, %dx
    pop %edx
    ret

.section .rodata, "a"
serial_banner: .asciz "phase1 6.0.0 ready - B14 framebuffer keyboard console\r\n"
serial_key: .asciz "key\r\n"
title1: .asciz "phase1 6.0.0 ready"
title2: .asciz "Base1 framebuffer keyboard console"
result_line: .asciz "B14 candidate result: phase1_kernel_keyboard_console_seen"
body1: .asciz "Hardware path: ThinkPad X200 / Libreboot / external USB"
body2: .asciz "Phase1-owned kernel code is executing on physical hardware."
body3: .asciz "Keyboard polling is active. Press keys to update the key area."
body4: .asciz "Non-claims: no installer, no internal disk write, no daily-driver claim."
prompt: .asciz "phase1> keyboard diagnostic mode"
input_label: .asciz "last key:"
key_seen: .asciz "keyboard input observed:"
fallback1: .asciz "phase1 6.0.0 ready - B14 VGA text fallback"
fallback2: .asciz "No framebuffer info from Multiboot; record phase1_kernel_vga_console_seen if visible."

.section .data, "aw"
mb_magic: .long 0
mbi_ptr: .long 0
fb_addr: .long 0
fb_pitch: .long 0
fb_width: .long 0
fb_height: .long 0
fb_bpp: .long 0
blink_counter: .long 0
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

printf 'PHASE1 BASE1 B14 FRAMEBUFFER KEYBOARD CONSOLE USB WRITER\n\n'
printf 'profile       : %s\n' "$PROFILE"
printf 'target disk   : %s\n' "$USB"
printf 'partition     : %s\n' "$PART"
printf 'phase1 kernel : %s\n' "$ELF"
printf 'scope         : external USB Phase1 keyboard console evidence\n'
printf 'claim         : not_claimed\n\n'
printf 'Building Phase1-owned framebuffer keyboard console kernel...\n'

as --32 -o "$OBJ" "$SRC"
ld -m elf_i386 -T "$LDS" -o "$ELF" "$OBJ"
chmod 0644 "$ELF"
[ -s "$ELF" ] || fail "failed to build ELF: $ELF"

if command -v grub-file >/dev/null 2>&1; then
  grub-file --is-x86-multiboot "$ELF" || fail "ELF failed GRUB multiboot validation"
fi

printf 'Phase1 keyboard console SHA256:\n'
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

sudo mkfs.vfat -F 32 -n PHASE1B14 "$PART"
sudo mount "$PART" "$MNT"
sudo mkdir -p "$MNT/boot/grub" "$MNT/grub" "$MNT/boot/phase1" "$MNT/phase1-evidence"
sudo cp "$ELF" "$MNT/boot/phase1/phase1-b14.elf"

sudo tee "$MNT/boot/grub/grub.cfg" >/dev/null <<'EOF'
set timeout=30
set default=0
set pager=1

menuentry "B14-01 Phase1 framebuffer keyboard console" {
    echo "Phase1 Base1 B14: loading framebuffer keyboard console"
    echo "Target evidence: phase1_kernel_keyboard_console_seen"
    echo "Success: framebuffer console appears and keypresses update the key area."
    multiboot /boot/phase1/phase1-b14.elf
    boot
}

menuentry "B14-02 Phase1 framebuffer keyboard console - compatibility entry" {
    echo "Phase1 Base1 B14: compatibility framebuffer keyboard console"
    multiboot /boot/phase1/phase1-b14.elf
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
    echo "B14 fallback marker reached from external USB"
    echo "This is not an installer, hardening, or daily-driver claim."
    sleep --interruptible 999
}
EOF

sudo cp "$MNT/boot/grub/grub.cfg" "$MNT/grub/grub.cfg"

sudo tee "$MNT/PHASE1-B14-KEYBOARD-CONSOLE.txt" >/dev/null <<EOF
Phase1 Base1 B14 external USB framebuffer keyboard console.
Profile: $PROFILE
Kernel: /boot/phase1/phase1-b14.elf

Primary evidence state:
- phase1_kernel_keyboard_console_seen: framebuffer console appears and keypresses update the key area.

Other useful states:
- phase1_kernel_framebuffer_seen: framebuffer appears but keyboard update is not observed.
- phase1_grub_console_seen: GRUB-native fallback console appears.
- blocked_after_multiboot_load: GRUB loads ELF but no visible framebuffer/text handoff occurs.

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
BASE1_B14_FRAMEBUFFER_KEYBOARD_CONSOLE_PROFILE=$PROFILE
BASE1_B14_FRAMEBUFFER_KEYBOARD_CONSOLE_TARGET=$USB
BASE1_B14_FRAMEBUFFER_KEYBOARD_CONSOLE_PARTITION=$PART
BASE1_B14_FRAMEBUFFER_KEYBOARD_CONSOLE_ELF=$ELF
BASE1_B14_FRAMEBUFFER_KEYBOARD_CONSOLE_RESULT=prepared
BASE1_B14_FRAMEBUFFER_KEYBOARD_CONSOLE_EXPECTED_NEXT_RESULT=phase1_kernel_keyboard_console_seen
BASE1_B14_FRAMEBUFFER_KEYBOARD_CONSOLE_CLAIM=not_claimed
BASE1_B14_NON_CLAIM_INSTALLER=1
BASE1_B14_NON_CLAIM_INTERNAL_DISK_INSTALL=1
BASE1_B14_NON_CLAIM_RECOVERY_COMPLETE=1
BASE1_B14_NON_CLAIM_HARDENED=1
BASE1_B14_NON_CLAIM_DAILY_DRIVER=1
EOF

printf '\nDONE: B14 framebuffer keyboard console USB prepared on %s\n' "$USB"
printf 'Boot the X200 from USB and choose B14-01 first.\n'
printf 'If keypresses update the key area, record: phase1_kernel_keyboard_console_seen\n'

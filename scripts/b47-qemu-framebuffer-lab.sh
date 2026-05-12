#!/usr/bin/env sh
# Phase1 B47 QEMU framebuffer renderer lab.
#
# Build a tiny initramfs that draws a pre-rendered Phase1 card directly to
# /dev/fb0, then drops to a BusyBox shell. This is for QEMU-first testing.
#
# Usage:
#   sh scripts/b47-qemu-framebuffer-lab.sh build
#   BASE1_B47_QEMU_RUN=1 sh scripts/b47-qemu-framebuffer-lab.sh run

set -eu

MODE="${1:-build}"
OUT_DIR="${BASE1_B47_OUT:-build/base1-b47-qemu-framebuffer-lab}"
ROOTFS="$OUT_DIR/rootfs"
KERNEL="${BASE1_B47_KERNEL:-build/linux/alpine-netboot/vmlinuz}"
INITRD="$OUT_DIR/phase1-b47-framebuffer-lab.img"
CARD_PPM="$OUT_DIR/phase1-b47-card.ppm"
BLITTER_C="$OUT_DIR/phase1_fb_blit.c"
BLITTER="$OUT_DIR/phase1_fb_blit"
REPORT="$OUT_DIR/b47-qemu-framebuffer-lab.env"
QEMU_RUN="${BASE1_B47_QEMU_RUN:-0}"

fail() { printf 'b47-qemu-framebuffer-lab: %s\n' "$1" >&2; exit 1; }
need() { command -v "$1" >/dev/null 2>&1 || fail "missing command: $1"; }
copy_one() { src="$1"; dstroot="$2"; [ -e "$src" ] || return 0; dst="$dstroot$src"; mkdir -p "$(dirname "$dst")"; cp -aL "$src" "$dst" 2>/dev/null || cp -L "$src" "$dst" 2>/dev/null || true; }
copy_libs() { bin="$1"; dstroot="$2"; command -v ldd >/dev/null 2>&1 || return 0; ldd "$bin" 2>/dev/null | while IFS= read -r line || [ -n "${line:-}" ]; do lib=""; case "$line" in *'=> /'*) lib="$(printf '%s\n' "$line" | awk '{print $3}')" ;; /*) lib="$(printf '%s\n' "$line" | awk '{print $1}')" ;; esac; [ -n "$lib" ] && [ -e "$lib" ] && copy_one "$lib" "$dstroot"; done; }

find_font() {
  for pattern in \
    '*NotoSansCJK*JP*.otf' '*NotoSansMonoCJK*JP*.otf' '*Noto*Sans*CJK*JP*' \
    '*unifont*.ttf' '*unifont*.otf' '*DejaVuSansMono*.ttf'
  do
    find /usr/share/fonts /usr/share/consolefonts -type f -iname "$pattern" 2>/dev/null | head -n 1
  done | awk 'NF { print; exit }'
}

for c in sh mkdir cp find awk head cpio gzip file; do need "$c"; done
[ -f "$KERNEL" ] || fail "missing kernel: $KERNEL"
command -v python3 >/dev/null 2>&1 || fail "missing python3"
command -v cc >/dev/null 2>&1 || fail "missing C compiler: cc"

mkdir -p "$OUT_DIR"
FONT="$(find_font || true)"
[ -n "$FONT" ] || fail "missing font candidate; install fonts-noto-cjk or unifont"
printf 'B47 framebuffer lab\n'
printf 'kernel: %s\n' "$KERNEL"
printf 'font  : %s\n' "$FONT"
printf 'out   : %s\n' "$OUT_DIR"

python3 - "$CARD_PPM" "$FONT" <<'PY'
import sys
from pathlib import Path

out = Path(sys.argv[1])
font_path = sys.argv[2]
try:
    from PIL import Image, ImageDraw, ImageFont
except Exception as exc:
    raise SystemExit(f"missing Pillow/PIL: {exc}. Install python3-pil or pillow")

w, h = 1024, 768
img = Image.new("RGB", (w, h), (7, 13, 20))
d = ImageDraw.Draw(img)

font_big = ImageFont.truetype(font_path, 44)
font_title = ImageFont.truetype(font_path, 28)
font_body = ImageFont.truetype(font_path, 22)
font_small = ImageFont.truetype(font_path, 18)

ice = (76, 230, 255)
blue = (30, 105, 255)
red = (255, 40, 48)
white = (230, 246, 255)
dim = (120, 160, 170)

# background grid/glow
for y in range(0, h, 32):
    d.line((0, y, w, y), fill=(8, 24, 32))
for x in range(0, w, 32):
    d.line((x, 0, x, h), fill=(8, 20, 30))

# rounded card border
x0, y0, x1, y1 = 70, 70, 954, 675
try:
    d.rounded_rectangle((x0, y0, x1, y1), radius=28, outline=ice, width=4, fill=(10, 18, 28))
    d.rounded_rectangle((x0+14, y0+14, x1-14, y1-14), radius=20, outline=blue, width=2)
except Exception:
    d.rectangle((x0, y0, x1, y1), outline=ice, width=4)

# title / greeting
hello = "こんにちは、ハッカー！"
d.text((110, 105), "Phase1 // Framebuffer Boot Card", font=font_title, fill=ice)
d.text((110, 155), hello, font=font_big, fill=red)

# status lines
lines = [
    ("version", "v6.0.0"),
    ("channel", "release"),
    ("profile", "safe"),
    ("security", "safe shield"),
    ("display", "phase1 framebuffer renderer lab"),
    ("unicode", "UTF-8 + CJK glyphs rendered as pixels"),
    ("renderer", "pre-rendered card -> /dev/fb0"),
]
y = 245
for k, v in lines:
    d.text((120, y), f"{k:<10}", font=font_body, fill=dim)
    d.text((270, y), v, font=font_body, fill=white if k != "unicode" else red)
    y += 38

# rounded glyph test as pixels
rounded = "╭─ Phase1 ─╮   ╰──────────╯"
d.text((120, 545), rounded, font=font_body, fill=ice)
d.text((120, 595), "B47 QEMU first. If this displays, move to X200 framebuffer path.", font=font_small, fill=dim)

img.save(out, "PPM")
PY

cat > "$BLITTER_C" <<'C'
#include <errno.h>
#include <fcntl.h>
#include <linux/fb.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/ioctl.h>
#include <sys/mman.h>
#include <unistd.h>

static int read_token(FILE *f, char *buf, size_t n) {
    int c;
    do { c = fgetc(f); if (c == EOF) return 0; } while (c==' '||c=='\n'||c=='\r'||c=='\t');
    if (c == '#') { while ((c=fgetc(f)) != EOF && c!='\n'){}; return read_token(f, buf, n); }
    size_t i = 0;
    do { if (i + 1 < n) buf[i++] = (char)c; c = fgetc(f); } while (c != EOF && c!=' '&&c!='\n'&&c!='\r'&&c!='\t');
    buf[i] = 0;
    return 1;
}

int main(int argc, char **argv) {
    const char *ppm_path = argc > 1 ? argv[1] : "/phase1/phase1-b47-card.ppm";
    const char *fb_path = argc > 2 ? argv[2] : "/dev/fb0";
    FILE *f = fopen(ppm_path, "rb");
    if (!f) { perror("open ppm"); return 2; }
    char tok[64];
    if (!read_token(f, tok, sizeof tok) || strcmp(tok, "P6") != 0) { fprintf(stderr, "not P6 ppm\n"); return 3; }
    read_token(f, tok, sizeof tok); int iw = atoi(tok);
    read_token(f, tok, sizeof tok); int ih = atoi(tok);
    read_token(f, tok, sizeof tok); int maxv = atoi(tok);
    if (iw <= 0 || ih <= 0 || maxv != 255) { fprintf(stderr, "bad ppm header\n"); return 4; }
    size_t pixels = (size_t)iw * (size_t)ih * 3;
    uint8_t *rgb = malloc(pixels);
    if (!rgb) { perror("malloc"); return 5; }
    if (fread(rgb, 1, pixels, f) != pixels) { fprintf(stderr, "short ppm read\n"); return 6; }
    fclose(f);

    int fd = open(fb_path, O_RDWR);
    if (fd < 0) { perror("open fb"); return 7; }
    struct fb_var_screeninfo v;
    struct fb_fix_screeninfo fix;
    if (ioctl(fd, FBIOGET_VSCREENINFO, &v) < 0) { perror("FBIOGET_VSCREENINFO"); return 8; }
    if (ioctl(fd, FBIOGET_FSCREENINFO, &fix) < 0) { perror("FBIOGET_FSCREENINFO"); return 9; }
    size_t screensz = (size_t)fix.line_length * (size_t)v.yres;
    uint8_t *fb = mmap(NULL, screensz, PROT_READ|PROT_WRITE, MAP_SHARED, fd, 0);
    if (fb == MAP_FAILED) { perror("mmap fb"); return 10; }

    memset(fb, 0, screensz);
    int offx = v.xres > (unsigned)iw ? ((int)v.xres - iw) / 2 : 0;
    int offy = v.yres > (unsigned)ih ? ((int)v.yres - ih) / 2 : 0;
    int draw_w = iw < (int)v.xres ? iw : (int)v.xres;
    int draw_h = ih < (int)v.yres ? ih : (int)v.yres;

    for (int y = 0; y < draw_h; y++) {
        for (int x = 0; x < draw_w; x++) {
            uint8_t r = rgb[(y*iw + x)*3+0];
            uint8_t g = rgb[(y*iw + x)*3+1];
            uint8_t b = rgb[(y*iw + x)*3+2];
            long loc = (long)(y + offy) * fix.line_length + (long)(x + offx) * (v.bits_per_pixel/8);
            if (loc < 0 || (size_t)loc + 4 > screensz) continue;
            uint32_t val = ((r >> (8 - v.red.length)) << v.red.offset) |
                           ((g >> (8 - v.green.length)) << v.green.offset) |
                           ((b >> (8 - v.blue.length)) << v.blue.offset);
            if (v.bits_per_pixel == 32) *(uint32_t *)(fb + loc) = val;
            else if (v.bits_per_pixel == 24) { fb[loc+0] = b; fb[loc+1] = g; fb[loc+2] = r; }
            else if (v.bits_per_pixel == 16) *(uint16_t *)(fb + loc) = (uint16_t)val;
        }
    }
    msync(fb, screensz, MS_SYNC);
    munmap(fb, screensz);
    close(fd);
    free(rgb);
    return 0;
}
C

cc -O2 -Wall -o "$BLITTER" "$BLITTER_C"
file "$BLITTER"

rm -rf "$ROOTFS"
mkdir -p "$ROOTFS/bin" "$ROOTFS/sbin" "$ROOTFS/dev" "$ROOTFS/proc" "$ROOTFS/sys" "$ROOTFS/run" "$ROOTFS/tmp" "$ROOTFS/phase1"
BUSYBOX="${BASE1_B47_BUSYBOX:-}"
if [ -z "$BUSYBOX" ]; then
  for p in /bin/busybox /usr/bin/busybox /bin/busybox.static /usr/bin/busybox.static; do
    [ -x "$p" ] && { BUSYBOX="$p"; break; }
  done
fi
[ -n "$BUSYBOX" ] || fail "missing busybox"
cp -L "$BUSYBOX" "$ROOTFS/bin/busybox"
chmod 0755 "$ROOTFS/bin/busybox"
for app in sh ash mount umount mkdir cat echo ls dmesg uname sleep reboot poweroff halt mknod stty clear printf; do ln -sf busybox "$ROOTFS/bin/$app" 2>/dev/null || true; done
cp "$CARD_PPM" "$ROOTFS/phase1/phase1-b47-card.ppm"
cp "$BLITTER" "$ROOTFS/phase1/phase1_fb_blit"
chmod 0755 "$ROOTFS/phase1/phase1_fb_blit"
copy_libs "$BLITTER" "$ROOTFS" || true

cat > "$ROOTFS/init" <<'EOF'
#!/bin/sh
PATH=/phase1:/bin:/sbin:/usr/bin:/usr/sbin
export PATH
mount -t proc proc /proc 2>/dev/null || true
mount -t sysfs sysfs /sys 2>/dev/null || true
mount -t devtmpfs devtmpfs /dev 2>/dev/null || mount -t tmpfs dev /dev 2>/dev/null || true
[ -c /dev/console ] || mknod /dev/console c 5 1 2>/dev/null || true
[ -c /dev/fb0 ] || mknod /dev/fb0 c 29 0 2>/dev/null || true
exec </dev/console >/dev/console 2>&1
clear 2>/dev/null || true
echo "Phase1 B47 framebuffer lab starting..."
echo "Attempting /dev/fb0 render of Japanese card."
if /phase1/phase1_fb_blit /phase1/phase1-b47-card.ppm /dev/fb0; then
  echo "phase1_framebuffer_card_qemu_seen candidate"
  echo "If the QEMU window shows Japanese text, record phase1_japanese_pixels_qemu_seen."
else
  echo "framebuffer blit failed; falling back to shell"
fi
echo "Type reboot or poweroff when done."
/bin/sh
EOF
chmod 0755 "$ROOTFS/init"

( cd "$ROOTFS" && find . | cpio -H newc -o 2>/dev/null | gzip -9 > "../phase1-b47-framebuffer-lab.img" )

cat > "$REPORT" <<EOF
BASE1_B47_KERNEL=$KERNEL
BASE1_B47_INITRD=$INITRD
BASE1_B47_CARD=$CARD_PPM
BASE1_B47_FONT=$FONT
BASE1_B47_BLITTER=$BLITTER
BASE1_B47_RESULT=prepared
BASE1_B47_EXPECTED_QEMU_RESULT=phase1_framebuffer_card_qemu_seen
BASE1_B47_JAPANESE_PIXELS=not_claimed
EOF

printf 'DONE: B47 framebuffer lab prepared.\n'
printf 'Initrd: %s\n' "$INITRD"
printf 'Card  : %s\n' "$CARD_PPM"
printf 'Report: %s\n' "$REPORT"

if [ "$MODE" = run ] || [ "$QEMU_RUN" = 1 ]; then
  need qemu-system-x86_64
  printf 'Launching QEMU...\n'
  qemu-system-x86_64 \
    -m 512M \
    -kernel "$KERNEL" \
    -initrd "$INITRD" \
    -append "console=tty0 rdinit=/init video=1024x768-32 nomodeset" \
    -display gtk \
    -no-reboot
fi

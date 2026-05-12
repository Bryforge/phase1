#!/usr/bin/env sh
# Phase1 B47 X200 framebuffer boot augmentation.
#
# Purpose:
#   Add the QEMU-proven framebuffer boot card path to an already prepared X200
#   Phase1 USB. This adds a separate experimental GRUB entry; it does not
#   replace the normal terminal boot path.
#
# Usage:
#   sh scripts/x200-b47-framebuffer-boot-augment.sh /dev/sdb YES_WRITE_USB

set -eu

USB="${1:-}"
CONFIRM="${2:-}"
INITRD_NAME="phase1-b42-native-stable-safe-color-utf8.img"
OUT_DIR="${BASE1_B47_X200_OUT:-build/base1-b47-x200-framebuffer-boot}"
WORK="$OUT_DIR/work"
ROOTFS="$WORK/rootfs"
REPORT="$OUT_DIR/b47-x200-framebuffer-boot.env"
CARD_PPM="$OUT_DIR/phase1-b47-x200-card.ppm"
BLITTER_C="$OUT_DIR/phase1_fb_blit.c"
BLITTER="$OUT_DIR/phase1_fb_blit"
ENTRY="Start Phase1 Framebuffer Boot Card"

fail() { printf 'x200-b47-framebuffer-boot-augment: %s\n' "$1" >&2; exit 1; }
need() { command -v "$1" >/dev/null 2>&1 || fail "missing command: $1"; }
part1() { case "$1" in /dev/nvme[0-9]n[0-9]|/dev/mmcblk[0-9]) printf '%sp1\n' "$1" ;; /dev/sd[a-z]|/dev/vd[a-z]|/dev/hd[a-z]) printf '%s1\n' "$1" ;; *) fail "use a whole disk like /dev/sdb" ;; esac; }
copy_one() { src="$1"; dstroot="$2"; [ -e "$src" ] || return 0; dst="$dstroot$src"; mkdir -p "$(dirname "$dst")"; cp -aL "$src" "$dst" 2>/dev/null || cp -L "$src" "$dst" 2>/dev/null || true; }
copy_libs() { bin="$1"; dstroot="$2"; command -v ldd >/dev/null 2>&1 || return 0; ldd "$bin" 2>/dev/null | while IFS= read -r line || [ -n "${line:-}" ]; do lib=""; case "$line" in *'=> /'*) lib="$(printf '%s\n' "$line" | awk '{print $3}')" ;; /*) lib="$(printf '%s\n' "$line" | awk '{print $1}')" ;; esac; [ -n "$lib" ] && [ -e "$lib" ] && copy_one "$lib" "$dstroot"; done; }
copy_common_loaders() { dstroot="$1"; for loader in /lib64/ld-linux-x86-64.so.2 /lib/x86_64-linux-gnu/ld-linux-x86-64.so.2 /lib/ld-linux.so.2 /lib/i386-linux-gnu/ld-linux.so.2; do [ -e "$loader" ] && copy_one "$loader" "$dstroot"; done; }
find_font() {
  for pattern in \
    '*NotoSansCJK*JP*.otf' '*NotoSansMonoCJK*JP*.otf' '*Noto*Sans*CJK*JP*' \
    '*unifont*.ttf' '*unifont*.otf' '*DejaVuSansMono*.ttf'
  do
    find /usr/share/fonts /usr/share/consolefonts -type f -iname "$pattern" 2>/dev/null | head -n 1
  done | awk 'NF { print; exit }'
}

[ -n "$USB" ] || fail "usage: sh scripts/x200-b47-framebuffer-boot-augment.sh /dev/sdb YES_WRITE_USB"
[ "$CONFIRM" = "YES_WRITE_USB" ] || fail "missing YES_WRITE_USB confirmation"
for c in sudo mount umount mkdir cp grep awk find cpio gzip tee mktemp findmnt head python3 cc file; do need "$c"; done
[ -b "$USB" ] || fail "not a block device: $USB"
ROOT_SRC="$(findmnt -no SOURCE / 2>/dev/null || true)"
case "$ROOT_SRC" in "$USB"|"$USB"[0-9]*|"$USB"p[0-9]*) fail "refusing root filesystem device: $ROOT_SRC" ;; esac

FONT="$(find_font || true)"
[ -n "$FONT" ] || fail "missing CJK/Unicode font candidate; install fonts-noto-cjk or unifont"
PART="$(part1 "$USB")"
[ -b "$PART" ] || fail "partition not found: $PART"
MNT="$(mktemp -d)"
cleanup() { sudo umount "$MNT" 2>/dev/null || true; rmdir "$MNT" 2>/dev/null || true; }
trap cleanup EXIT INT TERM

mkdir -p "$OUT_DIR"
printf 'B47 X200 framebuffer boot augment\n'
printf 'target: %s\n' "$USB"
printf 'font  : %s\n' "$FONT"

python3 - "$CARD_PPM" "$FONT" <<'PY'
import sys
from pathlib import Path
try:
    from PIL import Image, ImageDraw, ImageFont
except Exception as exc:
    raise SystemExit(f"missing Pillow/PIL: {exc}")
out = Path(sys.argv[1]); font_path = sys.argv[2]
w, h = 1024, 768
img = Image.new("RGB", (w, h), (7, 13, 20))
d = ImageDraw.Draw(img)
font_big = ImageFont.truetype(font_path, 44)
font_title = ImageFont.truetype(font_path, 28)
font_body = ImageFont.truetype(font_path, 22)
font_small = ImageFont.truetype(font_path, 18)
ice=(76,230,255); blue=(30,105,255); red=(255,40,48); white=(230,246,255); dim=(120,160,170)
for y in range(0,h,32): d.line((0,y,w,y), fill=(8,24,32))
for x in range(0,w,32): d.line((x,0,x,h), fill=(8,20,30))
try:
    d.rounded_rectangle((70,70,954,675), radius=28, outline=ice, width=4, fill=(10,18,28))
    d.rounded_rectangle((84,84,940,661), radius=20, outline=blue, width=2)
except Exception:
    d.rectangle((70,70,954,675), outline=ice, width=4)
d.text((110,105), "Phase1 // Framebuffer Boot Card", font=font_title, fill=ice)
d.text((110,155), "こんにちは、ハッカー！", font=font_big, fill=red)
rows=[("version","v6.0.0"),("target","ThinkPad X200"),("channel","release"),("profile","safe"),("security","safe shield"),("display","phase1 framebuffer renderer"),("unicode","Japanese glyphs rendered as pixels"),("fallback","terminal + ASCII paths preserved")]
y=240
for k,v in rows:
    d.text((120,y), f"{k:<10}", font=font_body, fill=dim)
    d.text((275,y), v, font=font_body, fill=red if k=="unicode" else white)
    y += 35
d.text((120,545), "╭─ Phase1 ─╮   ╰──────────╯", font=font_body, fill=ice)
d.text((120,595), "B47 X200 framebuffer test. Record success only if this displays on hardware.", font=font_small, fill=dim)
img.save(out, "PPM")
PY

cat > "$BLITTER_C" <<'C'
#include <fcntl.h>
#include <linux/fb.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/ioctl.h>
#include <sys/mman.h>
#include <unistd.h>
static int read_token(FILE *f, char *buf, size_t n){int c;do{c=fgetc(f);if(c==EOF)return 0;}while(c==' '||c=='\n'||c=='\r'||c=='\t');if(c=='#'){while((c=fgetc(f))!=EOF&&c!='\n'){};return read_token(f,buf,n);}size_t i=0;do{if(i+1<n)buf[i++]=(char)c;c=fgetc(f);}while(c!=EOF&&c!=' '&&c!='\n'&&c!='\r'&&c!='\t');buf[i]=0;return 1;}
int main(int argc,char**argv){const char*ppm=argc>1?argv[1]:"/phase1/phase1-b47-x200-card.ppm";const char*fbp=argc>2?argv[2]:"/dev/fb0";FILE*f=fopen(ppm,"rb");if(!f){perror("open ppm");return 2;}char tok[64];if(!read_token(f,tok,sizeof tok)||strcmp(tok,"P6")){fprintf(stderr,"not P6 ppm\n");return 3;}read_token(f,tok,sizeof tok);int iw=atoi(tok);read_token(f,tok,sizeof tok);int ih=atoi(tok);read_token(f,tok,sizeof tok);int maxv=atoi(tok);if(iw<=0||ih<=0||maxv!=255){fprintf(stderr,"bad ppm\n");return 4;}size_t pixels=(size_t)iw*(size_t)ih*3;uint8_t*rgb=malloc(pixels);if(!rgb){perror("malloc");return 5;}if(fread(rgb,1,pixels,f)!=pixels){fprintf(stderr,"short ppm\n");return 6;}fclose(f);int fd=open(fbp,O_RDWR);if(fd<0){perror("open fb");return 7;}struct fb_var_screeninfo v;struct fb_fix_screeninfo fix;if(ioctl(fd,FBIOGET_VSCREENINFO,&v)<0){perror("FBIOGET_VSCREENINFO");return 8;}if(ioctl(fd,FBIOGET_FSCREENINFO,&fix)<0){perror("FBIOGET_FSCREENINFO");return 9;}size_t screensz=(size_t)fix.line_length*(size_t)v.yres;uint8_t*fb=mmap(NULL,screensz,PROT_READ|PROT_WRITE,MAP_SHARED,fd,0);if(fb==MAP_FAILED){perror("mmap fb");return 10;}memset(fb,0,screensz);int offx=v.xres>(unsigned)iw?((int)v.xres-iw)/2:0;int offy=v.yres>(unsigned)ih?((int)v.yres-ih)/2:0;int dw=iw<(int)v.xres?iw:(int)v.xres;int dh=ih<(int)v.yres?ih:(int)v.yres;for(int y=0;y<dh;y++){for(int x=0;x<dw;x++){uint8_t r=rgb[(y*iw+x)*3],g=rgb[(y*iw+x)*3+1],b=rgb[(y*iw+x)*3+2];long loc=(long)(y+offy)*fix.line_length+(long)(x+offx)*(v.bits_per_pixel/8);if(loc<0||(size_t)loc+4>screensz)continue;uint32_t val=((r>>(8-v.red.length))<<v.red.offset)|((g>>(8-v.green.length))<<v.green.offset)|((b>>(8-v.blue.length))<<v.blue.offset);if(v.bits_per_pixel==32)*(uint32_t*)(fb+loc)=val;else if(v.bits_per_pixel==24){fb[loc]=b;fb[loc+1]=g;fb[loc+2]=r;}else if(v.bits_per_pixel==16)*(uint16_t*)(fb+loc)=(uint16_t)val;}}msync(fb,screensz,MS_SYNC);munmap(fb,screensz);close(fd);free(rgb);return 0;}
C
if cc -O2 -static -Wall -o "$BLITTER" "$BLITTER_C" 2>/dev/null; then BLITTER_LINK=static; else cc -O2 -Wall -o "$BLITTER" "$BLITTER_C"; BLITTER_LINK=dynamic; fi
file "$BLITTER"

sudo mount "$PART" "$MNT"
INITRD="$MNT/boot/phase1/$INITRD_NAME"
[ -f "$INITRD" ] || fail "missing initramfs on USB: /boot/phase1/$INITRD_NAME"
[ -f "$MNT/boot/grub/grub.cfg" ] || fail "missing grub.cfg on USB"
rm -rf "$WORK"
mkdir -p "$ROOTFS"
cp "$INITRD" "$WORK/original.img.gz"
( cd "$ROOTFS" && gzip -dc "../original.img.gz" | cpio -idmu 2>/dev/null ) || fail "could not extract initramfs"
mkdir -p "$ROOTFS/phase1/evidence"
cp "$CARD_PPM" "$ROOTFS/phase1/phase1-b47-x200-card.ppm"
cp "$BLITTER" "$ROOTFS/phase1/phase1_fb_blit"
chmod 0755 "$ROOTFS/phase1/phase1_fb_blit"
if [ "$BLITTER_LINK" = dynamic ]; then copy_libs "$BLITTER" "$ROOTFS" || true; copy_common_loaders "$ROOTFS" || true; fi
cat > "$ROOTFS/phase1/evidence/b47-x200-framebuffer.env" <<EOF
BASE1_B47_FRAMEBUFFER_CARD_QEMU=seen
BASE1_B47_JAPANESE_PIXELS_QEMU=seen
BASE1_B47_FRAMEBUFFER_CARD_X200=not_claimed
BASE1_B47_JAPANESE_PIXELS_X200=not_claimed
BASE1_B47_BLITTER_LINK=$BLITTER_LINK
EOF
if ! grep -q 'B47 X200 framebuffer card' "$ROOTFS/init"; then
  tmp="$ROOTFS/init.b47"
  awk '
    /^exec <\/dev\/console/ && !done {
      print "# B47 X200 framebuffer card";
      print "if echo \" $CMDLINE \" | grep -q \" phase1.framebuffer=1 \"; then";
      print "  echo \"B47: attempting Phase1 framebuffer boot card on /dev/fb0\"";
      print "  cat /proc/fb 2>/dev/null || true";
      print "  /phase1/phase1_fb_blit /phase1/phase1-b47-x200-card.ppm /dev/fb0 && echo \"phase1_framebuffer_card_x200_seen candidate\" || echo \"B47 framebuffer card failed; terminal fallback remains available\"";
      print "  echo \"If Japanese pixels display on X200, record phase1_japanese_pixels_x200_seen.\"";
      print "fi";
      done=1;
    }
    { print }
  ' "$ROOTFS/init" > "$tmp"
  mv "$tmp" "$ROOTFS/init"
  chmod 0755 "$ROOTFS/init"
fi
( cd "$ROOTFS" && find . | cpio -H newc -o 2>/dev/null | gzip -9 > "../$INITRD_NAME" ) || fail "could not repack initramfs"
sudo cp "$WORK/$INITRD_NAME" "$INITRD"
CFG="$MNT/boot/grub/grub.cfg"
if ! grep -q "$ENTRY" "$CFG"; then
  TMP_LOCAL="$WORK/grub.cfg.b47"
  awk -v initrd="$INITRD_NAME" -v entry="$ENTRY" '
    /menuentry "Start Phase1 Stable Safe Color UTF-8"/ && !done {
      print "menuentry \"" entry "\" {";
      print "    clear";
      print "    echo \"phase1 6.0.0 ready\"";
      print "    echo \"B47 X200 framebuffer boot card\"";
      print "    linux /boot/phase1/vmlinuz console=tty0 rdinit=/init init=/init vga=791 video=1024x768-32 quiet loglevel=0 panic=0 phase1.stable=1 phase1.safe=1 phase1.ascii=0 phase1.autoboot=0 phase1.utf8=1 phase1.framebuffer=1";
      print "    initrd /boot/phase1/" initrd;
      print "    boot";
      print "}";
      print "";
      done=1;
    }
    { print }
  ' "$CFG" > "$TMP_LOCAL"
  sudo cp "$TMP_LOCAL" "$CFG"
fi
cat > "$WORK/b47-prep-append.env" <<EOF
BASE1_B47_FRAMEBUFFER_CARD_QEMU=seen
BASE1_B47_JAPANESE_PIXELS_QEMU=seen
BASE1_B47_FRAMEBUFFER_CARD_X200=not_claimed
BASE1_B47_JAPANESE_PIXELS_X200=not_claimed
BASE1_B47_ENTRY=$ENTRY
EOF
sudo sh -c "cat '$WORK/b47-prep-append.env' >> '$MNT/phase1/evidence/b42-prep.env'"
grep -q "$ENTRY" "$CFG" || fail "B47 framebuffer entry was not written"
sync
sudo umount "$MNT"
rmdir "$MNT"
trap - EXIT INT TERM
cat > "$REPORT" <<EOF
BASE1_B47_TARGET=$USB
BASE1_B47_PARTITION=$PART
BASE1_B47_ENTRY=$ENTRY
BASE1_B47_BLITTER_LINK=$BLITTER_LINK
BASE1_B47_RESULT=augmented
BASE1_B47_FRAMEBUFFER_CARD_X200=not_claimed
BASE1_B47_JAPANESE_PIXELS_X200=not_claimed
EOF
printf 'DONE: B47 X200 framebuffer boot entry added.\n'
printf 'Entry: %s\n' "$ENTRY"
printf 'Blitter link: %s\n' "$BLITTER_LINK"
printf 'Report: %s\n' "$REPORT"

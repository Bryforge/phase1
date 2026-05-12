#!/usr/bin/env sh
# Phase1 B44 Unicode/font augmentation.
#
# Purpose:
#   Add best-effort Unicode, font, and renderer assets to an already prepared
#   Phase1 USB. This does not change the proven boot protocol. It augments the
#   initramfs and GRUB menu after the normal B43/B42 writer has prepared media.
#
# Usage:
#   sh scripts/x200-b44-unicode-font-augment.sh /dev/sdb YES_WRITE_USB
#
# Notes:
#   Full Japanese rendering still depends on a renderer that can draw CJK fonts.
#   This script packages everything available on the builder and records evidence.

set -eu

USB="${1:-}"
CONFIRM="${2:-}"
INITRD_NAME="phase1-b42-native-stable-safe-color-utf8.img"
OUT_DIR="${BASE1_B44_OUT:-build/base1-b44-unicode-font-augment}"
WORK="$OUT_DIR/work"
ROOTFS="$WORK/rootfs"
REPORT="$OUT_DIR/b44-unicode-font-augment.env"
MAIN_ENTRY="Start Phase1 Stable Safe Color UTF-8"
FONT_ENTRY="Start Phase1 Unicode Font Lab"
CJK_ENTRY="Start Phase1 Japanese Framebuffer UTF-8"
ASCII_ENTRY="Start Phase1 ASCII Safe Fallback"

fail() { printf 'x200-b44-unicode-font-augment: %s\n' "$1" >&2; exit 1; }
need() { command -v "$1" >/dev/null 2>&1 || fail "missing command: $1"; }
part1() { case "$1" in /dev/nvme[0-9]n[0-9]|/dev/mmcblk[0-9]) printf '%sp1\n' "$1" ;; /dev/sd[a-z]|/dev/vd[a-z]|/dev/hd[a-z]) printf '%s1\n' "$1" ;; *) fail "use a whole disk like /dev/sdb" ;; esac; }
copy_one() { src="$1"; dstroot="$2"; [ -e "$src" ] || return 0; dst="$dstroot$src"; mkdir -p "$(dirname "$dst")"; cp -aL "$src" "$dst" 2>/dev/null || cp -L "$src" "$dst" 2>/dev/null || true; }
copy_libs() { bin="$1"; dstroot="$2"; command -v ldd >/dev/null 2>&1 || return 0; ldd "$bin" 2>/dev/null | while IFS= read -r line || [ -n "$line" ]; do lib=""; case "$line" in *'=> /'*) lib="$(printf '%s\n' "$line" | awk '{print $3}')" ;; /*) lib="$(printf '%s\n' "$line" | awk '{print $1}')" ;; esac; [ -n "$lib" ] && [ -e "$lib" ] && copy_one "$lib" "$dstroot"; done; }

[ -n "$USB" ] || fail "usage: sh scripts/x200-b44-unicode-font-augment.sh /dev/sdb YES_WRITE_USB"
[ "$CONFIRM" = "YES_WRITE_USB" ] || fail "missing YES_WRITE_USB confirmation"
for c in sudo mount umount mkdir cp grep awk find cpio gzip gunzip tee sed sha256sum mktemp; do need "$c"; done
[ -b "$USB" ] || fail "not a block device: $USB"
ROOT_SRC="$(findmnt -no SOURCE / 2>/dev/null || true)"
case "$ROOT_SRC" in "$USB"|"$USB"[0-9]*|"$USB"p[0-9]*) fail "refusing root filesystem device: $ROOT_SRC" ;; esac

PART="$(part1 "$USB")"
[ -b "$PART" ] || fail "partition not found: $PART"
MNT="$(mktemp -d)"
cleanup() { sudo umount "$MNT" 2>/dev/null || true; rmdir "$MNT" 2>/dev/null || true; }
trap cleanup EXIT INT TERM

mkdir -p "$OUT_DIR"
sudo mount "$PART" "$MNT"
INITRD="$MNT/boot/phase1/$INITRD_NAME"
[ -f "$INITRD" ] || fail "missing initramfs on USB: /boot/phase1/$INITRD_NAME"
[ -f "$MNT/boot/grub/grub.cfg" ] || fail "missing grub.cfg on USB"

rm -rf "$WORK"
mkdir -p "$ROOTFS"
cp "$INITRD" "$WORK/original.img.gz"
( cd "$ROOTFS" && gzip -dc "$WORK/original.img.gz" | cpio -idmu 2>/dev/null )

font_count=0
renderer=none
cjk_font=0

for dir in /usr/share/fonts /usr/share/fontconfig /usr/share/consolefonts /usr/share/consoletrans /etc/fonts; do
  if [ -d "$dir" ]; then
    mkdir -p "$ROOTFS$dir"
    before="$(find "$ROOTFS$dir" -type f 2>/dev/null | wc -l | awk '{print $1}')"
    cp -aL "$dir/." "$ROOTFS$dir/" 2>/dev/null || true
    after="$(find "$ROOTFS$dir" -type f 2>/dev/null | wc -l | awk '{print $1}')"
    font_count=$((font_count + after - before))
  fi
done

if find "$ROOTFS/usr/share/fonts" "$ROOTFS/usr/share/consolefonts" -type f 2>/dev/null | grep -Ei 'noto.*cjk|cjk|unifont|ipag|takao|sazanami' >/dev/null 2>&1; then
  cjk_font=1
fi

for tool in /usr/bin/fbterm /usr/bin/jfbterm /usr/bin/kmscon /usr/bin/bterm /usr/bin/setfont /bin/setfont /usr/bin/unicode_start /bin/unicode_start /usr/bin/kbd_mode /bin/kbd_mode /usr/bin/fc-match /usr/bin/fc-cache; do
  if [ -x "$tool" ]; then
    copy_one "$tool" "$ROOTFS"
    copy_libs "$tool" "$ROOTFS" || true
    case "$tool" in
      *fbterm) [ "$renderer" = none ] && renderer=fbterm ;;
      *jfbterm) [ "$renderer" = none ] && renderer=jfbterm ;;
      *kmscon) [ "$renderer" = none ] && renderer=kmscon ;;
      *bterm) [ "$renderer" = none ] && renderer=bterm ;;
    esac
  fi
done

mkdir -p "$ROOTFS/phase1/evidence" "$ROOTFS/phase1/i18n/ja" "$ROOTFS/phase1/fonts"
cat > "$ROOTFS/phase1/i18n/ja/boot-test.txt" <<'EOF'
フェーズ1 起動完了
日本語 UTF-8 テスト
安全・非公開・強力
EOF
cat > "$ROOTFS/phase1/i18n/unicode-test.txt" <<'EOF'
Unicode check: α β γ λ π Ω — ✓ ★ → ← ↑ ↓
Box/rounded: ╭────╮ ╰────╯ ┌────┐ └────┘
Japanese: フェーズ1 日本語 安全 非公開 強力
Emoji may require color-font renderer: 🚀 🔥 ✅
EOF

cat > "$ROOTFS/phase1/evidence/b44-unicode-font.env" <<EOF
BASE1_B44_UTF8_ENV_READY=1
BASE1_B44_FONT_PACKAGED=1
BASE1_B44_FONT_SOURCE_COUNT=$font_count
BASE1_B44_CJK_FONT_CANDIDATE=$cjk_font
BASE1_B44_RENDERER_CANDIDATE=$renderer
BASE1_B44_JAPANESE_GLYPH_RENDERING=not_claimed
BASE1_B44_UNICODE_TEST=/phase1/i18n/unicode-test.txt
BASE1_B44_JAPANESE_TEST=/phase1/i18n/ja/boot-test.txt
EOF

if ! grep -q 'B44 unicode font augmentation' "$ROOTFS/init"; then
  tmp="$ROOTFS/init.b44"
  awk '
    /^exec <\/dev\/console/ && !done {
      print "# B44 unicode font augmentation";
      print "export PHASE1_FONT_PACK=unicode-full";
      print "export PHASE1_CJK_GLYPH_SUPPORT=attempt";
      print "export PHASE1_CJK_RENDERER=\"${PHASE1_CJK_RENDERER:-auto}\"";
      print "[ -x /usr/bin/unicode_start ] && /usr/bin/unicode_start >/dev/null 2>&1 || true";
      print "[ -x /bin/unicode_start ] && /bin/unicode_start >/dev/null 2>&1 || true";
      print "if [ -x /usr/bin/setfont ] || [ -x /bin/setfont ]; then";
      print "  SF=/usr/bin/setfont; [ -x /bin/setfont ] && SF=/bin/setfont";
      print "  for f in /usr/share/consolefonts/Uni2-Terminus16.psf.gz /usr/share/consolefonts/Lat2-Terminus16.psf.gz /usr/share/consolefonts/Lat15-Terminus16.psf.gz; do";
      print "    [ -f \"$f\" ] && { $SF \"$f\" >/dev/null 2>&1 || true; break; }";
      print "  done";
      print "fi";
      done=1;
    }
    { print }
  ' "$ROOTFS/init" > "$tmp"
  mv "$tmp" "$ROOTFS/init"
  chmod 0755 "$ROOTFS/init"
fi

( cd "$ROOTFS" && find . | cpio -H newc -o 2>/dev/null | gzip -9 > "$WORK/$INITRD_NAME" )
cp "$WORK/$INITRD_NAME" "$INITRD"

# Patch GRUB menu with Unicode lab and Japanese framebuffer entries if absent.
CFG="$MNT/boot/grub/grub.cfg"
if ! grep -q "$FONT_ENTRY" "$CFG"; then
  tmp="$CFG.b44"
  awk -v initrd="$INITRD_NAME" -v font_entry="$FONT_ENTRY" -v cjk_entry="$CJK_ENTRY" '
    /menuentry "Start Phase1 ASCII Safe Fallback"/ && !done {
      print "menuentry \"" font_entry "\" {";
      print "    clear";
      print "    echo \"phase1 6.0.0 ready\"";
      print "    echo \"B44 Unicode Font Lab\"";
      print "    linux /boot/phase1/vmlinuz console=tty0 rdinit=/init init=/init nomodeset quiet loglevel=0 panic=0 phase1.stable=1 phase1.safe=1 phase1.ascii=0 phase1.autoboot=0 phase1.utf8=1 phase1.fontlab=1";
      print "    initrd /boot/phase1/" initrd;
      print "    boot";
      print "}";
      print "";
      print "menuentry \"" cjk_entry "\" {";
      print "    clear";
      print "    echo \"phase1 6.0.0 ready\"";
      print "    echo \"B44 Japanese framebuffer UTF-8 attempt\"";
      print "    linux /boot/phase1/vmlinuz console=tty0 rdinit=/init init=/init vga=791 video=1024x768-16 quiet loglevel=0 panic=0 phase1.stable=1 phase1.safe=1 phase1.ascii=0 phase1.autoboot=1 phase1.utf8=1 phase1.cjk=1";
      print "    initrd /boot/phase1/" initrd;
      print "    boot";
      print "}";
      print "";
      done=1;
    }
    { print }
  ' "$CFG" > "$tmp"
  sudo mv "$tmp" "$CFG"
fi

cat >> "$MNT/phase1/evidence/b42-prep.env" <<EOF
BASE1_B44_UTF8_ENV_READY=1
BASE1_B44_FONT_PACKAGED=1
BASE1_B44_FONT_SOURCE_COUNT=$font_count
BASE1_B44_CJK_FONT_CANDIDATE=$cjk_font
BASE1_B44_RENDERER_CANDIDATE=$renderer
BASE1_B44_JAPANESE_GLYPH_RENDERING=not_claimed
BASE1_B44_FONT_ENTRY=$FONT_ENTRY
BASE1_B44_CJK_ENTRY=$CJK_ENTRY
EOF

sync
sudo umount "$MNT"
rmdir "$MNT"
trap - EXIT INT TERM

cat > "$REPORT" <<EOF
BASE1_B44_TARGET=$USB
BASE1_B44_PARTITION=$PART
BASE1_B44_INITRD=$INITRD_NAME
BASE1_B44_FONT_PACKAGED=1
BASE1_B44_FONT_SOURCE_COUNT=$font_count
BASE1_B44_CJK_FONT_CANDIDATE=$cjk_font
BASE1_B44_RENDERER_CANDIDATE=$renderer
BASE1_B44_JAPANESE_GLYPH_RENDERING=not_claimed
BASE1_B44_RESULT=augmented
EOF

printf 'DONE: B44 Unicode/font augmentation complete.\n'
printf 'Font files copied: %s\n' "$font_count"
printf 'CJK font candidate: %s\n' "$cjk_font"
printf 'Renderer candidate: %s\n' "$renderer"
printf 'Report: %s\n' "$REPORT"

#!/usr/bin/env bash
# Build and optionally run the Phase1/Base1 visual boot preview in QEMU.
#
# This is a showcase-only boot splash path. It creates local build artifacts in
# build/base1-qemu-visual-boot-preview and build/base1-qemu-visual-boot-preview.img.
# It does not install Base1, modify host boot settings, partition disks, or claim
# boot readiness.

set -euo pipefail

usage() {
  cat <<'USAGE'
usage: sh scripts/base1-qemu-visual-boot-preview.sh --build [--run] [--fullscreen]

Builds a local UEFI FAT image that displays a small centered Phase1 symbol-only
splash through GRUB inside QEMU. This is visual boot preview only.

Options:
  --build       Build the local preview image under build/.
  --run         Launch QEMU after building or using an existing image.
  --fullscreen  Launch QEMU fullscreen when used with --run.
  --help        Show this help.

Non-claims:
  This does not make Base1 bootable, installer-ready, recovery-complete,
  hardened, hardware-validated, release-candidate ready, or daily-driver ready.
USAGE
}

fail() {
  printf 'base1-qemu-visual-boot-preview: %s\n' "$1" >&2
  exit 1
}

have() {
  command -v "$1" >/dev/null 2>&1
}

BUILD=no
RUN=no
FULLSCREEN=no

while [ "$#" -gt 0 ]; do
  case "$1" in
    --build)
      BUILD=yes
      shift
      ;;
    --run)
      RUN=yes
      shift
      ;;
    --fullscreen)
      FULLSCREEN=yes
      shift
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      usage >&2
      fail "unknown argument: $1"
      ;;
  esac
done

[ "$BUILD" = yes ] || [ "$RUN" = yes ] || {
  usage >&2
  fail "choose --build, --run, or both"
}

ROOT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
WORK_DIR="$ROOT_DIR/build/base1-qemu-visual-boot-preview"
IMG="$ROOT_DIR/build/base1-qemu-visual-boot-preview.img"
QEMU_SPLASH="$WORK_DIR/boot/grub/phase1-qemu-splash.png"
QEMU_SPLASH_WIDTH=1024
QEMU_SPLASH_HEIGHT=768
QEMU_SPLASH_SYMBOL_SIZE=176

generate_qemu_symbol_splash() {
  have python3 || fail "missing python3; try: brew install python"

  python3 - "$QEMU_SPLASH" "$QEMU_SPLASH_WIDTH" "$QEMU_SPLASH_HEIGHT" "$QEMU_SPLASH_SYMBOL_SIZE" <<'PY'
import math
import struct
import sys
import zlib

out_path = sys.argv[1]
width = int(sys.argv[2])
height = int(sys.argv[3])
symbol_size = int(sys.argv[4])
scale = 2
canvas_w = width * scale
canvas_h = height * scale
cx = canvas_w // 2
cy = canvas_h // 2
unit = symbol_size * scale / 176.0

pixels = [(0, 0, 0)] * (canvas_w * canvas_h)


def blend_pixel(x, y, color, alpha=1.0):
    if x < 0 or y < 0 or x >= canvas_w or y >= canvas_h:
        return
    idx = y * canvas_w + x
    old = pixels[idx]
    pixels[idx] = tuple(
        max(0, min(255, int(old[i] * (1.0 - alpha) + color[i] * alpha))) for i in range(3)
    )


def fill_circle(x, y, radius, color, alpha=1.0):
    r = max(1, int(radius))
    x0 = max(0, int(x) - r)
    x1 = min(canvas_w - 1, int(x) + r)
    y0 = max(0, int(y) - r)
    y1 = min(canvas_h - 1, int(y) + r)
    rr = radius * radius
    for py in range(y0, y1 + 1):
        dy = py - y
        for px in range(x0, x1 + 1):
            dx = px - x
            d2 = dx * dx + dy * dy
            if d2 <= rr:
                edge = max(0.0, min(1.0, (rr - d2) / max(1.0, radius * 2.0)))
                blend_pixel(px, py, color, alpha * max(0.25, min(1.0, edge)))


def draw_line(x0, y0, x1, y1, thickness, color, alpha=1.0):
    steps = max(1, int(math.hypot(x1 - x0, y1 - y0)))
    radius = thickness / 2.0
    for step in range(steps + 1):
        t = step / steps
        fill_circle(x0 + (x1 - x0) * t, y0 + (y1 - y0) * t, radius, color, alpha)


def rotated_ellipse_point(angle, rx, ry, rotation):
    x = math.cos(angle) * rx
    y = math.sin(angle) * ry
    r = math.radians(rotation)
    return (
        cx + x * math.cos(r) - y * math.sin(r),
        cy + x * math.sin(r) + y * math.cos(r),
    )


def draw_ellipse(rx, ry, rotation, thickness, color, alpha=1.0):
    previous = rotated_ellipse_point(0.0, rx, ry, rotation)
    samples = 420
    for idx in range(1, samples + 1):
        point = rotated_ellipse_point((math.tau * idx) / samples, rx, ry, rotation)
        draw_line(previous[0], previous[1], point[0], point[1], thickness, color, alpha)
        previous = point

cyan = (55, 226, 255)
magenta = (255, 51, 214)
blue = (78, 126, 255)
white = (232, 246, 255)
shadow = (10, 28, 46)

# Keep the mark intentionally small for a boot splash: centered, symbol-only,
# and bounded to roughly 18% of a 1024px-wide QEMU viewport.
draw_ellipse(104 * unit, 38 * unit, -24, 3.2 * unit, cyan, 0.90)
draw_ellipse(92 * unit, 32 * unit, 28, 2.8 * unit, magenta, 0.82)
draw_ellipse(54 * unit, 54 * unit, 0, 3.0 * unit, blue, 0.65)

# Stylized Phase1 numeric core. No word mark is rendered here.
draw_line(cx - 28 * unit, cy - 42 * unit, cx + 2 * unit, cy - 66 * unit, 13 * unit, shadow, 0.70)
draw_line(cx + 2 * unit, cy - 66 * unit, cx + 2 * unit, cy + 63 * unit, 15 * unit, shadow, 0.70)
draw_line(cx - 36 * unit, cy + 66 * unit, cx + 40 * unit, cy + 66 * unit, 13 * unit, shadow, 0.70)

draw_line(cx - 28 * unit, cy - 44 * unit, cx + 2 * unit, cy - 68 * unit, 9 * unit, white, 1.0)
draw_line(cx + 2 * unit, cy - 68 * unit, cx + 2 * unit, cy + 64 * unit, 11 * unit, cyan, 1.0)
draw_line(cx - 36 * unit, cy + 66 * unit, cx + 40 * unit, cy + 66 * unit, 9 * unit, white, 1.0)

# Tiny center node for the orbital symbol.
fill_circle(cx + 2 * unit, cy - 6 * unit, 6 * unit, magenta, 0.95)

# Downsample for simple antialiasing.
rows = []
for y in range(height):
    row = bytearray([0])
    for x in range(width):
        samples = []
        for sy in range(scale):
            for sx in range(scale):
                samples.append(pixels[(y * scale + sy) * canvas_w + (x * scale + sx)])
        row.extend(sum(sample[channel] for sample in samples) // len(samples) for channel in range(3))
    rows.append(bytes(row))
raw = b"".join(rows)

def chunk(kind, data):
    body = kind + data
    return struct.pack(">I", len(data)) + body + struct.pack(">I", zlib.crc32(body) & 0xFFFFFFFF)

png = b"\x89PNG\r\n\x1a\n"
png += chunk(b"IHDR", struct.pack(">IIBBBBB", width, height, 8, 2, 0, 0, 0))
png += chunk(b"IDAT", zlib.compress(raw, 9))
png += chunk(b"IEND", b"")
with open(out_path, "wb") as handle:
    handle.write(png)
PY
}

build_preview() {
  have x86_64-elf-grub-mkstandalone || fail "missing x86_64-elf-grub-mkstandalone; try: brew install x86_64-elf-grub"
  have mformat || fail "missing mformat; try: brew install mtools"
  have mmd || fail "missing mmd; try: brew install mtools"
  have mcopy || fail "missing mcopy; try: brew install mtools"
  have dd || fail "missing dd"

  mkdir -p "$WORK_DIR/EFI/BOOT" "$WORK_DIR/boot/grub/fonts"
  generate_qemu_symbol_splash

  GRUB_MKFONT=$(command -v x86_64-elf-grub-mkfont || command -v grub-mkfont || true)
  FONT_SRC="/System/Library/Fonts/Supplemental/Arial.ttf"

  if [ -n "$GRUB_MKFONT" ]; then
    if [ ! -f "$FONT_SRC" ]; then
      FONT_SRC=$(find /System/Library/Fonts /Library/Fonts -name '*.ttf' 2>/dev/null | head -n 1 || true)
    fi
    if [ -n "$FONT_SRC" ] && [ -f "$FONT_SRC" ]; then
      "$GRUB_MKFONT" -s 28 -o "$WORK_DIR/boot/grub/fonts/phase1.pf2" "$FONT_SRC"
    fi
  fi

  cat > "$WORK_DIR/grub.cfg" <<'EOF'
set timeout=0
set default=0

insmod all_video
insmod gfxterm
insmod png
insmod fat
insmod search
insmod search_fs_file
insmod font

if loadfont /boot/grub/fonts/phase1.pf2; then
  true
fi

set gfxmode=1024x768,auto
set gfxpayload=keep
terminal_output gfxterm

set color_normal=black/black
set color_highlight=black/black
set menu_color_normal=black/black
set menu_color_highlight=black/black

search --file /boot/grub/phase1-qemu-splash.png --set=root
background_image /boot/grub/phase1-qemu-splash.png

menuentry " " {
    clear
    background_image /boot/grub/phase1-qemu-splash.png
    sleep --interruptible 9999
}
EOF

  x86_64-elf-grub-mkstandalone \
    -O x86_64-efi \
    -o "$WORK_DIR/EFI/BOOT/BOOTX64.EFI" \
    --modules="part_gpt part_msdos fat all_video gfxterm png font search search_fs_file" \
    "boot/grub/grub.cfg=$WORK_DIR/grub.cfg"

  dd if=/dev/zero of="$IMG" bs=1m count=128
  mformat -i "$IMG" -F ::
  mmd -i "$IMG" ::/EFI ::/EFI/BOOT ::/boot ::/boot/grub ::/boot/grub/fonts
  mcopy -i "$IMG" "$WORK_DIR/EFI/BOOT/BOOTX64.EFI" ::/EFI/BOOT/BOOTX64.EFI
  mcopy -i "$IMG" "$QEMU_SPLASH" ::/boot/grub/phase1-qemu-splash.png
  if [ -f "$WORK_DIR/boot/grub/fonts/phase1.pf2" ]; then
    mcopy -i "$IMG" "$WORK_DIR/boot/grub/fonts/phase1.pf2" ::/boot/grub/fonts/phase1.pf2
  fi

  printf 'base1_qemu_visual_boot_preview: built %s\n' "$IMG"
  printf 'splash: small centered phase1 symbol-only image\n'
  printf 'writes: build-directory-only\n'
  printf 'boot_readiness_claim: no\n'
}

run_preview() {
  have qemu-system-x86_64 || fail "missing qemu-system-x86_64; try: brew install qemu"
  [ -f "$IMG" ] || fail "missing preview image: $IMG; run with --build first"

  QEMU_PREFIX=$(brew --prefix qemu 2>/dev/null || true)
  [ -n "$QEMU_PREFIX" ] || fail "could not locate qemu Homebrew prefix"
  OVMF_CODE="$QEMU_PREFIX/share/qemu/edk2-x86_64-code.fd"
  [ -f "$OVMF_CODE" ] || fail "missing UEFI firmware: $OVMF_CODE"

  FULLSCREEN_ARG=""
  if [ "$FULLSCREEN" = yes ]; then
    FULLSCREEN_ARG="-full-screen"
  fi

  # shellcheck disable=SC2086
  qemu-system-x86_64 \
    -machine q35,accel=tcg \
    -m 4096 \
    -smp 4 \
    -drive if=pflash,format=raw,unit=0,readonly=on,file="$OVMF_CODE" \
    -drive if=none,id=phase1usb,format=raw,file="$IMG" \
    -device qemu-xhci \
    -device usb-storage,drive=phase1usb,bootindex=1 \
    -boot menu=off \
    -vga std \
    -display cocoa,zoom-to-fit=on \
    $FULLSCREEN_ARG \
    -net none
}

if [ "$BUILD" = yes ]; then
  build_preview
fi

if [ "$RUN" = yes ]; then
  run_preview
fi

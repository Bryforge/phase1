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

Builds a local UEFI FAT image that displays assets/phase1-splash.png through GRUB
inside QEMU. This is visual boot preview only.

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
SPLASH="$ROOT_DIR/assets/phase1-splash.png"

build_preview() {
  have x86_64-elf-grub-mkstandalone || fail "missing x86_64-elf-grub-mkstandalone; try: brew install x86_64-elf-grub"
  have mformat || fail "missing mformat; try: brew install mtools"
  have mmd || fail "missing mmd; try: brew install mtools"
  have mcopy || fail "missing mcopy; try: brew install mtools"
  have dd || fail "missing dd"
  [ -f "$SPLASH" ] || fail "missing splash asset: $SPLASH"

  mkdir -p "$WORK_DIR/EFI/BOOT" "$WORK_DIR/boot/grub/fonts"

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

  cp "$SPLASH" "$WORK_DIR/boot/grub/phase1-splash.png"

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

search --file /boot/grub/phase1-splash.png --set=root
background_image /boot/grub/phase1-splash.png

menuentry " " {
    clear
    background_image /boot/grub/phase1-splash.png
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
  mcopy -i "$IMG" "$WORK_DIR/boot/grub/phase1-splash.png" ::/boot/grub/phase1-splash.png
  if [ -f "$WORK_DIR/boot/grub/fonts/phase1.pf2" ]; then
    mcopy -i "$IMG" "$WORK_DIR/boot/grub/fonts/phase1.pf2" ::/boot/grub/fonts/phase1.pf2
  fi

  printf 'base1_qemu_visual_boot_preview: built %s\n' "$IMG"
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

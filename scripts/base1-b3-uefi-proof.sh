#!/usr/bin/env bash
# Base1 B3 UEFI proof-of-life path.
#
# Builds a local UEFI FAT image that boots in QEMU through OVMF/GRUB,
# displays the fitted Phase1 word-mark splash, emits a serial readiness marker,
# and writes local evidence logs. This is emulator-only evidence.

set -euo pipefail

usage() {
  cat <<'USAGE'
usage: sh scripts/base1-b3-uefi-proof.sh --build [--run|--check] [--fullscreen]

Builds a local B3 UEFI proof image under build/. The image is bootable in QEMU,
displays the fitted Phase1 word-mark splash, and emits a serial proof marker.

Options:
  --build       Build build/base1-b3-uefi-proof.img.
  --run         Launch QEMU visibly after building or using an existing image.
  --check       Launch QEMU headless, capture serial output, and validate marker.
  --fullscreen  Launch QEMU fullscreen when used with --run.
  --timeout N   Check timeout in seconds, default: 12.
  --help        Show this help.

Marker:
  phase1 6.0.0 ready

Display behavior:
  Visible QEMU runs show the splash plus readable GRUB proof text. The build
  prefers GRUB's unicode.pf2 font, then falls back to a generated monospaced
  font. Visible runs also send the serial console to null so QEMU does not draw
  a duplicate serial text console over the graphical boot screen.

Non-claims:
  This is QEMU/OVMF proof-of-life only. It does not make Base1 installer-ready,
  hardware-validated, recovery-complete, hardened, release-candidate ready, or
  daily-driver ready.
USAGE
}

fail() {
  printf 'base1-b3-uefi-proof: %s\n' "$1" >&2
  exit 1
}

have() {
  command -v "$1" >/dev/null 2>&1
}

BUILD=no
RUN=no
CHECK=no
FULLSCREEN=no
TIMEOUT_SECONDS=${BASE1_B3_TIMEOUT:-12}
MARKER=${BASE1_B3_MARKER:-phase1 6.0.0 ready}

while [ "$#" -gt 0 ]; do
  case "$1" in
    --build) BUILD=yes; shift ;;
    --run) RUN=yes; shift ;;
    --check) CHECK=yes; shift ;;
    --fullscreen) FULLSCREEN=yes; shift ;;
    --timeout)
      [ "$#" -ge 2 ] || fail '--timeout requires a value'
      TIMEOUT_SECONDS=$2
      shift 2
      ;;
    -h|--help) usage; exit 0 ;;
    *) usage >&2; fail "unknown argument: $1" ;;
  esac
done

[ "$BUILD" = yes ] || [ "$RUN" = yes ] || [ "$CHECK" = yes ] || {
  usage >&2
  fail "choose --build, --run, --check, or a combination"
}

case "$TIMEOUT_SECONDS" in
  ''|*[!0-9]*) fail '--timeout must be a positive integer' ;;
esac
[ "$TIMEOUT_SECONDS" -gt 0 ] || fail '--timeout must be greater than zero'

ROOT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
WORK_DIR="$ROOT_DIR/build/base1-b3-uefi-proof"
IMG="$ROOT_DIR/build/base1-b3-uefi-proof.img"
SPLASH_SOURCE="$ROOT_DIR/assets/phase1_word.png"
SPLASH_FIT="$WORK_DIR/boot/grub/phase1-qemu-splash-fit.png"
SPLASH="$WORK_DIR/boot/grub/phase1-qemu-splash.png"
GRUB_FONT="$WORK_DIR/boot/grub/fonts/phase1.pf2"
REPORTS_DIR="$WORK_DIR/reports"
SERIAL_LOG="$REPORTS_DIR/b3-serial.log"
SUMMARY="$REPORTS_DIR/b3-summary.env"
SPLASH_WIDTH=1024
SPLASH_HEIGHT=768
SPLASH_MAX_EDGE=560

qemu_share() {
  if have brew; then
    brew --prefix qemu 2>/dev/null | sed 's:$:/share/qemu:'
  fi
}

grub_prefix() {
  if have brew; then
    brew --prefix x86_64-elf-grub 2>/dev/null || true
  fi
}

ovmf_code() {
  local share
  share=$(qemu_share || true)
  if [ -n "$share" ] && [ -f "$share/edk2-x86_64-code.fd" ]; then
    printf '%s\n' "$share/edk2-x86_64-code.fd"
    return 0
  fi
  find /opt/homebrew /usr/local -name 'edk2-x86_64-code.fd' -o -name 'OVMF_CODE.fd' 2>/dev/null | head -n 1
}

timeout_bin() {
  if have timeout; then
    printf 'timeout\n'
  elif have gtimeout; then
    printf 'gtimeout\n'
  else
    printf '\n'
  fi
}

generate_splash() {
  have sips || fail 'missing sips; macOS sips is used to fit the splash'
  [ -f "$SPLASH_SOURCE" ] || fail "missing splash asset: $SPLASH_SOURCE"
  sips -Z "$SPLASH_MAX_EDGE" "$SPLASH_SOURCE" --out "$SPLASH_FIT" >/dev/null 2>&1
  sips --padToHeightWidth "$SPLASH_HEIGHT" "$SPLASH_WIDTH" --padColor 000000 \
    "$SPLASH_FIT" --out "$SPLASH" >/dev/null 2>&1
}

copy_grub_unicode_font() {
  local prefix found
  prefix=$(grub_prefix || true)
  if [ -n "$prefix" ]; then
    found=$(find "$prefix" -name 'unicode.pf2' 2>/dev/null | head -n 1 || true)
    if [ -n "$found" ] && [ -f "$found" ]; then
      cp "$found" "$GRUB_FONT"
      return 0
    fi
  fi
  found=$(find /opt/homebrew /usr/local -path '*grub*' -name 'unicode.pf2' 2>/dev/null | head -n 1 || true)
  if [ -n "$found" ] && [ -f "$found" ]; then
    cp "$found" "$GRUB_FONT"
    return 0
  fi
  return 1
}

generate_font() {
  local mkfont font_src
  if copy_grub_unicode_font; then
    return 0
  fi

  mkfont=$(command -v x86_64-elf-grub-mkfont || command -v grub-mkfont || true)
  [ -n "$mkfont" ] || fail 'missing grub-mkfont and unicode.pf2; install x86_64-elf-grub'

  for font_src in \
    "/System/Library/Fonts/Monaco.ttf" \
    "/System/Library/Fonts/Menlo.ttc" \
    "/System/Library/Fonts/Supplemental/Courier New.ttf" \
    "/System/Library/Fonts/Supplemental/Arial.ttf"
  do
    if [ -f "$font_src" ]; then
      if "$mkfont" -s 24 -o "$GRUB_FONT" "$font_src" >/dev/null 2>&1; then
        [ -s "$GRUB_FONT" ] && return 0
      fi
    fi
  done

  fail 'could not prepare a GRUB font for readable gfxterm output'
}

write_grub_config() {
  cat > "$WORK_DIR/grub.cfg" <<EOF
set timeout=0
set default=0

insmod all_video
insmod png
insmod fat
insmod search
insmod search_fs_file
insmod font
insmod gfxterm
insmod serial
insmod terminal

serial --unit=0 --speed=115200 --word=8 --parity=no --stop=1

set gfxmode=1024x768,auto
set gfxpayload=keep

# Load a known-good GRUB font before enabling gfxterm. Missing or mismatched
# fonts make ASCII/menu text render as repeated box/glitch glyphs on macOS QEMU.
if loadfont /boot/grub/fonts/phase1.pf2; then
  true
fi

terminal_input console serial
terminal_output gfxterm serial

set color_normal=white/black
set color_highlight=black/light-cyan

search --file /boot/grub/phase1-qemu-splash.png --set=root
background_image /boot/grub/phase1-qemu-splash.png

menuentry "Phase1 / Base1 B3 UEFI proof" {
    clear
    background_image /boot/grub/phase1-qemu-splash.png
    echo "base1 b3 uefi proof start"
    echo "$MARKER"
    echo "emulator-only evidence; no installer; no hardware-validation claim"
    sleep --interruptible 9999
}
EOF
}

build_image() {
  have x86_64-elf-grub-mkstandalone || fail 'missing x86_64-elf-grub-mkstandalone; try: brew install x86_64-elf-grub'
  have mformat || fail 'missing mformat; try: brew install mtools'
  have mmd || fail 'missing mmd; try: brew install mtools'
  have mcopy || fail 'missing mcopy; try: brew install mtools'
  have truncate || fail 'missing truncate'

  rm -rf "$WORK_DIR"
  mkdir -p "$WORK_DIR/EFI/BOOT" "$WORK_DIR/boot/grub/fonts" "$REPORTS_DIR"
  generate_splash
  generate_font
  write_grub_config

  x86_64-elf-grub-mkstandalone \
    -O x86_64-efi \
    -o "$WORK_DIR/EFI/BOOT/BOOTX64.EFI" \
    --modules="part_gpt part_msdos fat all_video gfxterm png font search search_fs_file serial terminal" \
    "boot/grub/grub.cfg=$WORK_DIR/grub.cfg"

  rm -f "$IMG"
  truncate -s 128m "$IMG"
  mformat -i "$IMG" -F ::
  mmd -i "$IMG" ::/EFI ::/EFI/BOOT ::/boot ::/boot/grub ::/boot/grub/fonts
  mcopy -i "$IMG" "$WORK_DIR/EFI/BOOT/BOOTX64.EFI" ::/EFI/BOOT/BOOTX64.EFI
  mcopy -i "$IMG" "$SPLASH" ::/boot/grub/phase1-qemu-splash.png
  if [ -f "$GRUB_FONT" ]; then
    mcopy -i "$IMG" "$GRUB_FONT" ::/boot/grub/fonts/phase1.pf2
  fi

  printf 'base1_b3_uefi_proof: built %s\n' "$IMG"
  printf 'marker: %s\n' "$MARKER"
  printf 'splash: assets/phase1_word.png fitted to %sx%s max edge %s\n' "$SPLASH_WIDTH" "$SPLASH_HEIGHT" "$SPLASH_MAX_EDGE"
  printf 'display: readable GRUB overlay with unicode font; visible serial disabled\n'
  printf 'boot_readiness_claim: no\n'
}

qemu_common_args() {
  local ovmf
  ovmf=$(ovmf_code)
  [ -n "$ovmf" ] && [ -f "$ovmf" ] || fail 'missing UEFI firmware edk2-x86_64-code.fd; try: brew install qemu'
  [ -f "$IMG" ] || fail "missing image: $IMG; run --build first"
  printf '%s\n' "$ovmf"
}

run_visible() {
  have qemu-system-x86_64 || fail 'missing qemu-system-x86_64; try: brew install qemu'
  local ovmf fullscreen_arg
  ovmf=$(qemu_common_args)
  fullscreen_arg=""
  if [ "$FULLSCREEN" = yes ]; then
    fullscreen_arg="-full-screen"
  fi
  # shellcheck disable=SC2086
  qemu-system-x86_64 \
    -machine q35,accel=tcg \
    -m 4096 \
    -smp 4 \
    -drive if=pflash,format=raw,unit=0,readonly=on,file="$ovmf" \
    -drive if=none,id=phase1usb,format=raw,file="$IMG" \
    -device qemu-xhci \
    -device usb-storage,drive=phase1usb,bootindex=1 \
    -boot menu=off \
    -vga std \
    -display cocoa,zoom-to-fit=on \
    -serial null \
    $fullscreen_arg \
    -net none
}

run_check() {
  have qemu-system-x86_64 || fail 'missing qemu-system-x86_64; try: brew install qemu'
  local ovmf tbin rc result
  ovmf=$(qemu_common_args)
  tbin=$(timeout_bin)
  [ -n "$tbin" ] || fail 'check mode requires timeout or gtimeout so QEMU cannot run unbounded'
  mkdir -p "$REPORTS_DIR"
  : > "$SERIAL_LOG"

  set +e
  "$tbin" "$TIMEOUT_SECONDS" qemu-system-x86_64 \
    -machine q35,accel=tcg \
    -m 1024 \
    -smp 2 \
    -drive if=pflash,format=raw,unit=0,readonly=on,file="$ovmf" \
    -drive if=none,id=phase1usb,format=raw,file="$IMG" \
    -device qemu-xhci \
    -device usb-storage,drive=phase1usb,bootindex=1 \
    -boot menu=off \
    -display none \
    -serial "file:$SERIAL_LOG" \
    -net none
  rc=$?
  set -e

  if grep -F "$MARKER" "$SERIAL_LOG" >/dev/null 2>&1; then
    result=pass
  else
    result=failed
  fi

  cat > "$SUMMARY" <<EOF
BASE1_B3_UEFI_PROOF_RESULT=$result
BASE1_B3_UEFI_PROOF_EXIT_CODE=$rc
BASE1_B3_UEFI_PROOF_MARKER=$MARKER
BASE1_B3_UEFI_PROOF_SERIAL_LOG=reports/b3-serial.log
BASE1_B3_NON_CLAIM_INSTALLER=1
BASE1_B3_NON_CLAIM_HARDWARE=1
BASE1_B3_NON_CLAIM_DAILY_DRIVER=1
EOF

  printf 'result: %s\n' "$result"
  printf 'serial_log: %s\n' "$SERIAL_LOG"
  printf 'summary: %s\n' "$SUMMARY"
  printf 'non_claims: emulator-only; no installer; no hardware validation; no daily-driver claim\n'
  [ "$result" = pass ] || exit 1
}

if [ "$BUILD" = yes ]; then
  build_image
fi
if [ "$RUN" = yes ]; then
  run_visible
fi
if [ "$CHECK" = yes ]; then
  run_check
fi

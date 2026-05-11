#!/usr/bin/env sh
# Fetch the B7 kernel/initrd artifacts onto the X200 from a local HTTP server.
#
# On the Mac, serve the artifact directory first:
#   cd ~/phase1_library/phase1/build/linux/alpine-netboot
#   python3 -m http.server 8010
#
# On the X200, run:
#   sh scripts/x200-fetch-b7-kernel-initrd.sh http://MAC_IP:8010
#
# This does not write disks, boot anything, or make a bootability claim.

set -eu

SRC="${1:-}"
OUT_DIR="${BASE1_B7_NETBOOT_OUT:-build/linux/alpine-netboot}"
KERNEL="$OUT_DIR/vmlinuz"
INITRD="$OUT_DIR/initrd.img"

fail() {
  printf 'x200-fetch-b7-kernel-initrd: %s\n' "$1" >&2
  exit 1
}

usage() {
  cat <<'USAGE'
Usage:
  sh scripts/x200-fetch-b7-kernel-initrd.sh http://MAC_IP:8010

Mac side first:
  cd ~/phase1_library/phase1/build/linux/alpine-netboot
  python3 -m http.server 8010

Then on X200:
  cd ~/phase1
  git pull --ff-only origin edge/stable
  sh scripts/x200-fetch-b7-kernel-initrd.sh http://MAC_IP:8010

Expected files created:
  build/linux/alpine-netboot/vmlinuz
  build/linux/alpine-netboot/initrd.img
USAGE
}

[ -n "$SRC" ] || { usage; fail "missing source URL"; }

case "$SRC" in
  http://*|https://*) BASE_URL="$SRC" ;;
  *) BASE_URL="http://$SRC" ;;
esac

BASE_URL="${BASE_URL%/}"

if command -v curl >/dev/null 2>&1; then
  fetch() { curl -fL --retry 2 -o "$1" "$2"; }
elif command -v wget >/dev/null 2>&1; then
  fetch() { wget -O "$1" "$2"; }
else
  fail "missing curl or wget. On Trisquel: sudo apt update && sudo apt install -y wget"
fi

mkdir -p "$OUT_DIR"

TMP_KERNEL="$KERNEL.tmp"
TMP_INITRD="$INITRD.tmp"
rm -f "$TMP_KERNEL" "$TMP_INITRD"

printf 'Fetching B7 kernel/initrd from: %s\n\n' "$BASE_URL"
fetch "$TMP_KERNEL" "$BASE_URL/vmlinuz"
fetch "$TMP_INITRD" "$BASE_URL/initrd.img"

[ -s "$TMP_KERNEL" ] || fail "downloaded kernel is empty: $TMP_KERNEL"
[ -s "$TMP_INITRD" ] || fail "downloaded initrd is empty: $TMP_INITRD"

mv "$TMP_KERNEL" "$KERNEL"
mv "$TMP_INITRD" "$INITRD"

printf '\nFetched files:\n'
ls -lh "$KERNEL" "$INITRD"

if command -v sha256sum >/dev/null 2>&1; then
  printf '\nSHA256:\n'
  sha256sum "$KERNEL" "$INITRD"
fi

printf '\nNext command:\n'
printf '  sh scripts/x200-b7-kernel-initrd-usb.sh /dev/sdb YES_WRITE_USB\n'

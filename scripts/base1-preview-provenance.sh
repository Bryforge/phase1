#!/usr/bin/env sh
# Base1 preview provenance.
#
# Records checksums for a Base1 emulator-preview bundle under build/.
# It does not launch an emulator, install Base1, write real devices, format
# media, attach devices, or claim Base1 is bootable.

set -eu

BUNDLE_DIR=${BASE1_EMULATOR_BUNDLE:-build/base1-emulator-preview}
REPORT_DIR=reports
STATUS=pass

usage() {
  cat <<'USAGE'
base1 preview provenance

usage:
  scripts/base1-preview-provenance.sh [options]

options:
  --bundle <dir>   Base1 emulator preview bundle directory under build/
  -h, --help       show this help

output:
  <bundle>/reports/provenance.env
  <bundle>/reports/SHA256SUMS

tracked files:
  manifest.env
  README.txt
  base1-sandbox.raw
  base1-rootfs-preview.tar when present
  run-qemu-bundle.sh
  staging/manifest.env
  staging/boot/grub/grub.cfg
  staging/boot/vmlinuz when present
  staging/boot/initrd.img when present

non-claims:
  This records preview-bundle provenance only. It does not launch the emulator,
  install Base1, create a released image, validate hardware, complete recovery,
  or prove that Base1 is bootable. It does not claim Base1 is bootable.
USAGE
}

fail() {
  printf 'base1 preview provenance: %s\n' "$1" >&2
  exit 1
}

note() {
  printf 'base1 preview provenance: %s\n' "$1"
}

have() {
  command -v "$1" >/dev/null 2>&1
}

hash_file() {
  file=$1
  if have shasum; then
    shasum -a 256 "$file" | awk '{print $1}'
  elif have sha256sum; then
    sha256sum "$file" | awk '{print $1}'
  elif have openssl; then
    openssl dgst -sha256 "$file" | awk '{print $NF}'
  else
    fail 'no SHA-256 tool found: need shasum, sha256sum, or openssl'
  fi
}

file_size() {
  wc -c < "$1" | tr -d ' '
}

track_file() {
  rel=$1
  file="$BUNDLE_DIR/$rel"
  if [ -f "$file" ]; then
    sum=$(hash_file "$file")
    bytes=$(file_size "$file")
    printf '%s  %s\n' "$sum" "$rel" >> "$SUMS_TMP"
    printf 'file=%s sha256=%s bytes=%s\n' "$rel" "$sum" "$bytes" >> "$PROV_TMP"
    note "tracked $rel"
  else
    printf 'missing=%s\n' "$rel" >> "$PROV_TMP"
    if [ "$STATUS" = "pass" ]; then
      STATUS=pass-with-notes
    fi
    note "missing optional/expected file: $rel"
  fi
}

while [ "$#" -gt 0 ]; do
  case "$1" in
    --bundle)
      [ "$#" -ge 2 ] || fail '--bundle requires a value'
      BUNDLE_DIR=$2
      shift 2
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      fail "unknown option: $1"
      ;;
  esac
done

case "$BUNDLE_DIR" in
  build/*) ;;
  *) fail "bundle must be under build/: $BUNDLE_DIR" ;;
esac

[ -d "$BUNDLE_DIR" ] || fail "bundle directory does not exist: $BUNDLE_DIR"
[ -f "$BUNDLE_DIR/manifest.env" ] || fail "bundle manifest missing: $BUNDLE_DIR/manifest.env"

mkdir -p "$BUNDLE_DIR/$REPORT_DIR"
PROV_TMP="$BUNDLE_DIR/$REPORT_DIR/provenance.env.tmp"
SUMS_TMP="$BUNDLE_DIR/$REPORT_DIR/SHA256SUMS.tmp"
: > "$PROV_TMP"
: > "$SUMS_TMP"

cat > "$PROV_TMP" <<EOF
BASE1_PREVIEW_PROVENANCE_STATUS=preview-only
BASE1_PREVIEW_BUNDLE=$BUNDLE_DIR
BASE1_PREVIEW_REPORT_DIR=$REPORT_DIR
BASE1_NON_CLAIM_BOOTABLE=1
BASE1_NON_CLAIM_RELEASED_IMAGE=1
BASE1_NON_CLAIM_HARDWARE_VALIDATED=1
EOF

for rel in \
  manifest.env \
  README.txt \
  base1-sandbox.raw \
  base1-rootfs-preview.tar \
  run-qemu-bundle.sh \
  staging/manifest.env \
  staging/boot/grub/grub.cfg \
  staging/boot/vmlinuz \
  staging/boot/initrd.img
  do
    track_file "$rel"
  done

printf 'result=%s\n' "$STATUS" >> "$PROV_TMP"
printf 'non_claims=no emulator launched; no installer run; no hardware validation; no bootable Base1 release claim\n' >> "$PROV_TMP"

mv "$PROV_TMP" "$BUNDLE_DIR/$REPORT_DIR/provenance.env"
mv "$SUMS_TMP" "$BUNDLE_DIR/$REPORT_DIR/SHA256SUMS"

note "wrote $BUNDLE_DIR/$REPORT_DIR/provenance.env"
note "wrote $BUNDLE_DIR/$REPORT_DIR/SHA256SUMS"
note "result: $STATUS"
note "non-claim: provenance was recorded for a preview bundle only"

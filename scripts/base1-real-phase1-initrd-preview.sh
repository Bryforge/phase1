#!/usr/bin/env sh
set -eu

OUT="build/base1-real-boot/initramfs-real-phase1.img"
BASE_INITRD="build/base1-real-boot/initramfs-virt"
PHASE1_BIN="target/x86_64-unknown-linux-musl/release/phase1"
WORK="build/base1-real-phase1-initrd-work"

while [ "$#" -gt 0 ]; do
  case "$1" in
    --base-initrd)
      BASE_INITRD="$2"
      shift 2
      ;;
    --phase1-bin)
      PHASE1_BIN="$2"
      shift 2
      ;;
    --out)
      OUT="$2"
      shift 2
      ;;
    *)
      echo "unknown arg: $1" >&2
      exit 2
      ;;
  esac
done

case "$OUT" in
  build/*) ;;
  *)
    echo "refusing output outside build/: $OUT" >&2
    exit 1
    ;;
esac

[ -f "$BASE_INITRD" ] || {
  echo "missing base initrd: $BASE_INITRD" >&2
  exit 1
}

[ -x "$PHASE1_BIN" ] || {
  echo "missing executable Phase1 binary: $PHASE1_BIN" >&2
  echo "build with: RUSTFLAGS=\"-C linker=rust-lld\" cargo build -p phase1 --bin phase1 --release --target x86_64-unknown-linux-musl" >&2
  exit 1
}

rm -rf "$WORK"
mkdir -p "$WORK"
mkdir -p "$(dirname "$OUT")"

(
  cd "$WORK"
  gzip -dc "../../$BASE_INITRD" | cpio -id >/tmp/base1-real-phase1-initrd-unpack.log 2>&1

  mkdir -p opt/phase1
  cp "../../$PHASE1_BIN" opt/phase1/phase1
  chmod +x opt/phase1/phase1

  mv init init.alpine

  cat > init <<'EOF'
#!/bin/sh
echo "base1 init wrapper reached"
echo "base1 launching real Phase1 binary"

/opt/phase1/phase1 --help
phase1_status="$?"

echo "base1 real Phase1 binary returned status ${phase1_status}"
echo "base1 handoff: exec alpine init"
exec /init.alpine "$@"
EOF

  chmod +x init

  find . | cpio -o -H newc 2>/tmp/base1-real-phase1-initrd-pack.log | gzip -9 > "../../$OUT"
)

echo "base1 real Phase1 initrd preview: wrote $OUT"
echo "base1 real Phase1 initrd preview: embedded $PHASE1_BIN"
echo "non-claim: preview initramfs only; no installer, hardware validation, or release image"

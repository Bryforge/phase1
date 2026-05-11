#!/usr/bin/env sh
# Phase1 / Base1 B26 crypto evidence manifest tool.
#
# Purpose:
#   Create a deterministic SHA-256 manifest for Phase1 evidence files.
#   This is the first cryptography layer: hashes first, signatures later.
#
# Usage:
#   sh scripts/base1-b26-crypto-evidence.sh
#   BASE1_B26_EVIDENCE_DIR=/phase1/evidence sh scripts/base1-b26-crypto-evidence.sh
#
# Scope:
#   Hashing only. No private keys. No signing claim. No secure boot claim.

set -eu

EVIDENCE_DIR="${BASE1_B26_EVIDENCE_DIR:-build}"
OUT_DIR="${BASE1_B26_OUT:-build/base1-b26-crypto-evidence}"
MANIFEST="$OUT_DIR/phase1-evidence-manifest.sha256"
REPORT="$OUT_DIR/crypto-evidence.env"

fail() { printf 'base1-b26-crypto-evidence: %s\n' "$1" >&2; exit 1; }
need_cmd() { command -v "$1" >/dev/null 2>&1 || fail "missing command: $1"; }

case "$OUT_DIR" in
  build/*|/phase1/*) : ;;
  *) fail "output directory must be under build/ or /phase1/: $OUT_DIR" ;;
esac

[ -d "$EVIDENCE_DIR" ] || fail "evidence directory not found: $EVIDENCE_DIR"

for cmd in find sort sha256sum mkdir date wc; do
  need_cmd "$cmd"
done

mkdir -p "$OUT_DIR"
: > "$MANIFEST"

# Hash regular files only. Exclude the output directory if it is inside evidence dir
# to avoid self-referential manifests.
find "$EVIDENCE_DIR" -type f 2>/dev/null \
  | sort \
  | while IFS= read -r file; do
      case "$file" in
        "$OUT_DIR"/*) continue ;;
      esac
      sha256sum "$file"
    done > "$MANIFEST"

COUNT="$(wc -l < "$MANIFEST" | tr -d ' ')"
MANIFEST_SHA="$(sha256sum "$MANIFEST" | awk '{print $1}')"

cat > "$REPORT" <<EOF
BASE1_B26_CRYPTO_EVIDENCE_MODE=sha256-manifest
BASE1_B26_CRYPTO_EVIDENCE_DIR=$EVIDENCE_DIR
BASE1_B26_CRYPTO_EVIDENCE_OUT=$OUT_DIR
BASE1_B26_CRYPTO_EVIDENCE_MANIFEST=$MANIFEST
BASE1_B26_CRYPTO_EVIDENCE_FILE_COUNT=$COUNT
BASE1_B26_CRYPTO_EVIDENCE_MANIFEST_SHA256=$MANIFEST_SHA
BASE1_B26_CRYPTO_EVIDENCE_RESULT=phase1_evidence_hash_manifest_seen
BASE1_B26_CRYPTO_EVIDENCE_SIGNING=not_enabled
BASE1_B26_CRYPTO_EVIDENCE_KEY_STORAGE=not_selected
BASE1_B26_CRYPTO_EVIDENCE_CLAIM=not_claimed
BASE1_B26_NON_CLAIM_SECURE_BOOT=1
BASE1_B26_NON_CLAIM_MEASURED_BOOT=1
BASE1_B26_NON_CLAIM_PRODUCTION_KEY_MANAGEMENT=1
BASE1_B26_NON_CLAIM_HARDENED=1
BASE1_B26_NON_CLAIM_DAILY_DRIVER=1
EOF

printf 'phase1 B26 crypto evidence\n\n'
printf 'evidence_dir : %s\n' "$EVIDENCE_DIR"
printf 'manifest     : %s\n' "$MANIFEST"
printf 'files        : %s\n' "$COUNT"
printf 'manifest_sha : %s\n\n' "$MANIFEST_SHA"
cat "$REPORT"

#!/usr/bin/env sh
# Base1 test inventory verifier.
#
# This script is read-only. It compares the reporter output from
# scripts/base1-test-inventory.sh against docs/base1/TEST_INVENTORY.md so
# Base1 test coverage stays visible before any reorganization work.

set -eu

DOC=${BASE1_TEST_INVENTORY_DOC:-docs/base1/TEST_INVENTORY.md}
REPORTER=${BASE1_TEST_INVENTORY_REPORTER:-scripts/base1-test-inventory.sh}
missing=0
reported=0

info() {
  printf 'base1-test-inventory-verify: %s\n' "$1"
}

warn() {
  printf 'base1-test-inventory-verify warning: %s\n' "$1" >&2
}

[ -f "$DOC" ] || {
  warn "missing inventory doc: $DOC"
  exit 1
}

[ -f "$REPORTER" ] || {
  warn "missing reporter: $REPORTER"
  exit 1
}

info 'mode: read-only'
info "doc: $DOC"
info "reporter: $REPORTER"

# shellcheck disable=SC2039
for file in $(sh "$REPORTER" | awk '/^tests\/.*\.rs$/ { print $1 }' | sort -u); do
  reported=$((reported + 1))
  if ! grep -F "$file" "$DOC" >/dev/null 2>&1; then
    warn "reported test missing from docs inventory: $file"
    missing=$((missing + 1))
  fi
done

info "tests-reported: $reported"
info "missing-from-doc: $missing"

if [ "$missing" -ne 0 ]; then
  exit 1
fi

info 'verification complete; no files were changed'

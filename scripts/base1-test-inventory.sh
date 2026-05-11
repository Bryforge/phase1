#!/usr/bin/env sh
# Base1 test inventory reporter.
#
# This script is read-only. It lists Base1-related Rust integration tests so
# repository organization work can keep test coverage visible before any moves.

set -eu

info() {
  printf 'base1-test-inventory: %s\n' "$1"
}

list_pattern() {
  label=$1
  pattern=$2
  printf '\n## %s\n' "$label"
  found=0
  for file in $pattern; do
    if [ -f "$file" ]; then
      printf '%s\n' "$file"
      found=$((found + 1))
    fi
  done
  printf 'count: %s\n' "$found"
}

info 'mode: read-only'
info 'scope: tests/base1_*.rs tests/quality_base1_*.rs tests/*base1*.rs'

list_pattern 'Base1 integration tests' 'tests/base1_*.rs'
list_pattern 'Base1 quality tests' 'tests/quality_base1_*.rs'
list_pattern 'Other Base1-named tests' 'tests/*base1*.rs'

info 'inventory complete; no files were changed'

#!/usr/bin/env sh
set -eu

# Phase1 learning helper.
# Keeps all learning local and sanitized in phase1.learn by default.

cmd="${1:-status}"
shift || true

case "$cmd" in
  status|profile|suggest|ask|note|teach|observe|import-history|learn-history|forget|export|help|-h|--help)
    cargo run --bin phase1-learn -- "$cmd" "$@"
    ;;
  bootstrap)
    cargo run --bin phase1-learn -- import-history "${1:-phase1.history}"
    cargo run --bin phase1-learn -- suggest
    ;;
  *)
    echo "usage: scripts/phase1-learn.sh [status|profile|suggest|ask|note|teach|observe|import-history|forget|export|bootstrap] [args...]" >&2
    exit 2
    ;;
esac

#!/usr/bin/env sh
# Hardened Phase1 launcher for Base1.
#
# This wrapper sets the Base1 compatibility environment and refuses unsafe root
# launches unless explicitly allowed for development.

set -eu

BASE1_PROFILE=${BASE1_PROFILE:-secure-default}
BASE1_HARDWARE_TARGET=${BASE1_HARDWARE_TARGET:-generic}
BASE1_PHASE1_CONTRACT=${BASE1_PHASE1_CONTRACT:-0.1}
PHASE1_INSTALL_DIR=${PHASE1_INSTALL_DIR:-/opt/phase1}
PHASE1_BIN=${PHASE1_BIN:-$PHASE1_INSTALL_DIR/phase1}
PHASE1_STORAGE_ROOT=${PHASE1_STORAGE_ROOT:-/var/lib/phase1/workspace}
PHASE1_SAFE_MODE=${PHASE1_SAFE_MODE:-1}
PHASE1_ALLOW_HOST_TOOLS=${PHASE1_ALLOW_HOST_TOOLS:-0}
BASE1_ALLOW_ROOT_PHASE1=${BASE1_ALLOW_ROOT_PHASE1:-0}

warn() {
  printf 'base1 launcher warning: %s\n' "$1" >&2
}

fail() {
  printf 'base1 launcher: %s\n' "$1" >&2
  exit 1
}

if [ "$(id -u)" = "0" ] && [ "$BASE1_ALLOW_ROOT_PHASE1" != "1" ]; then
  fail "refusing to launch Phase1 as root; run as the dedicated phase1 user or set BASE1_ALLOW_ROOT_PHASE1=1 only for development"
fi

if [ "$PHASE1_SAFE_MODE" != "1" ]; then
  warn "PHASE1_SAFE_MODE is not 1; secure-default expects safe mode on"
fi

if [ "$PHASE1_ALLOW_HOST_TOOLS" != "0" ]; then
  warn "PHASE1_ALLOW_HOST_TOOLS is not 0; host tools should be allowed only in explicit maintenance mode"
fi

if [ ! -d "$PHASE1_STORAGE_ROOT" ]; then
  fail "workspace is missing: $PHASE1_STORAGE_ROOT"
fi

if [ ! -w "$PHASE1_STORAGE_ROOT" ]; then
  fail "workspace is not writable by current user: $PHASE1_STORAGE_ROOT"
fi

if [ ! -x "$PHASE1_BIN" ]; then
  fail "Phase1 executable not found or not executable: $PHASE1_BIN"
fi

export BASE1_PROFILE
export BASE1_HARDWARE_TARGET
export BASE1_PHASE1_CONTRACT
export PHASE1_STORAGE_ROOT
export PHASE1_SAFE_MODE
export PHASE1_ALLOW_HOST_TOOLS

exec "$PHASE1_BIN" "$@"

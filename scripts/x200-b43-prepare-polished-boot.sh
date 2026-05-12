#!/usr/bin/env sh
# Phase1 B43 polished boot preparation wrapper.
#
# This is the next-step orchestration script. It performs local policy patching,
# builds the native binary, runs preflight, then delegates to the verified B42
# media writer/verification automation.
#
# Use on the x86_64 X200/final builder for the actual boot USB:
#   sh scripts/x200-b43-prepare-polished-boot.sh /dev/sdb YES_WRITE_USB
#
# Use on Raspberry Pi only for preflight/edit checks, not final X200 media,
# unless an x86_64 Phase1 binary and i386-pc grub-install are available.

set -eu

USB="${1:-}"
CONFIRM="${2:-}"

fail() { printf 'x200-b43-prepare-polished-boot: %s\n' "$1" >&2; exit 1; }
section() { printf '\n===== %s =====\n' "$1"; }
need() { command -v "$1" >/dev/null 2>&1 || fail "missing command: $1"; }

[ -n "$USB" ] || fail "usage: sh scripts/x200-b43-prepare-polished-boot.sh /dev/sdb YES_WRITE_USB"
[ "$CONFIRM" = "YES_WRITE_USB" ] || fail "missing YES_WRITE_USB confirmation"
[ -d .git ] || fail "run from phase1 repository root"
for c in git sh cargo file; do need "$c"; done

section "UPDATE REPOSITORY"
git fetch origin edge/stable
git pull --ff-only origin edge/stable
git log -1 --oneline

section "APPLY B43 LOCAL UI POLICY"
[ -f scripts/x200-b43-apply-ui-policy.sh ] || fail "missing scripts/x200-b43-apply-ui-policy.sh"
sh scripts/x200-b43-apply-ui-policy.sh

section "BUILD NATIVE PHASE1"
cargo build --release
[ -x target/release/phase1 ] || fail "missing target/release/phase1 after build"
file target/release/phase1

section "ARCHITECTURE WARNING CHECK"
case "$(file target/release/phase1)" in
  *x86-64*) printf 'x86_64 Phase1 binary: ok for X200\n' ;;
  *)
    printf 'WARNING: built Phase1 binary is not x86_64.\n'
    printf 'This is acceptable for preflight on this host, but not for final X200 boot media.\n'
    ;;
esac

section "B43 PREFLIGHT"
sh scripts/x200-b43-system-preflight.sh "$USB"

section "WRITE AND VERIFY B43/B42 USB"
printf 'Delegating to verified media automation.\n'
sh scripts/x200-b40-prepare-native-phase1-boot.sh "$USB" YES_WRITE_USB

section "B43 POLISHED BOOT PREP COMPLETE"
printf 'Expected default boot entry:\n'
printf '  Start Phase1 Stable Safe Color UTF-8\n'
printf 'Expected fallback-only entry:\n'
printf '  Start Phase1 ASCII Safe Fallback\n'
printf 'Expected policy markers:\n'
printf '  PHASE1_AUTO_BOOT=1\n'
printf '  PHASE1_ASCII=0 main, PHASE1_ASCII=1 fallback\n'
printf '  safe/stable blue/ice request\n'
printf '  edge crimson request\n'
printf '  rounded glyph request\n'

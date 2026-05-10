#!/usr/bin/env sh
# Base1 guarded QEMU boot checker.
#
# This script is the first explicit QEMU execution hook for Base1 preview
# bundles. It defaults to dry-run. Use --execute plus the confirmation phrase
# to actually run the bundle's run-qemu-bundle.sh, capture serial output, and
# check for a Phase1 readiness marker.
#
# It only writes reports inside the selected build/ bundle. It does not install
# Base1, write host devices, format media, mount filesystems, or validate real
# hardware.

set -eu

BUNDLE_DIR=${BASE1_EMULATOR_BUNDLE:-build/base1-emulator-preview}
MODE=dry-run
CONFIRM=${BASE1_QEMU_BOOT_CONFIRM:-}
TIMEOUT_SECONDS=${BASE1_QEMU_BOOT_TIMEOUT:-30}
EXPECT=${BASE1_QEMU_EXPECT_MARKER:-phase1 6.0.0 ready}

usage() {
  cat <<'USAGE'
base1 qemu boot check

usage:
  scripts/base1-qemu-boot-check.sh [options]

options:
  --bundle <dir>       Base1 emulator preview bundle under build/
  --dry-run            inspect the bundle and print the QEMU handoff plan
  --execute            run the bundle QEMU scaffold and capture serial output
  --confirm <phrase>   required with --execute; phrase: launch-qemu-base1-preview
  --timeout <seconds>  max runtime before terminating QEMU, default: 30
  --expect <text>      serial marker required for PASS, default: phase1 6.0.0 ready
  -h, --help           show this help

outputs:
  <bundle>/reports/qemu-boot.log
  <bundle>/reports/qemu-boot-summary.env

result model:
  dry-run       no emulator launched
  pass          marker found in serial log
  failed        marker missing or bundle invalid

non-claims:
  This is emulator-only boot evidence. It does not install Base1, validate real
  hardware, validate recovery, validate an installer, or prove daily-driver
  readiness. A pass only means the selected bundle emitted the expected marker
  during this QEMU run.
USAGE
}

fail() {
  printf 'base1 qemu boot check: %s\n' "$1" >&2
  exit 1
}

note() {
  printf 'base1 qemu boot check: %s\n' "$1"
}

require_build_bundle() {
  case "$1" in
    build/*) return 0 ;;
    *) return 1 ;;
  esac
}

while [ "$#" -gt 0 ]; do
  case "$1" in
    --bundle)
      [ "$#" -ge 2 ] || fail '--bundle requires a value'
      BUNDLE_DIR=$2
      shift 2
      ;;
    --dry-run)
      MODE=dry-run
      shift
      ;;
    --execute)
      MODE=execute
      shift
      ;;
    --confirm)
      [ "$#" -ge 2 ] || fail '--confirm requires a value'
      CONFIRM=$2
      shift 2
      ;;
    --timeout)
      [ "$#" -ge 2 ] || fail '--timeout requires a value'
      TIMEOUT_SECONDS=$2
      shift 2
      ;;
    --expect)
      [ "$#" -ge 2 ] || fail '--expect requires a value'
      EXPECT=$2
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

case "$TIMEOUT_SECONDS" in
  ''|*[!0-9]*) fail '--timeout must be a positive integer' ;;
esac
[ "$TIMEOUT_SECONDS" -gt 0 ] || fail '--timeout must be greater than zero'

require_build_bundle "$BUNDLE_DIR" || fail "bundle must be under build/: $BUNDLE_DIR"
[ -d "$BUNDLE_DIR" ] || fail "bundle directory does not exist: $BUNDLE_DIR"
[ -f "$BUNDLE_DIR/run-qemu-bundle.sh" ] || fail "missing bundle QEMU scaffold: $BUNDLE_DIR/run-qemu-bundle.sh"
[ -f "$BUNDLE_DIR/staging/boot/vmlinuz" ] || fail "missing kernel: $BUNDLE_DIR/staging/boot/vmlinuz"
[ -f "$BUNDLE_DIR/staging/boot/initrd.img" ] || fail "missing initrd: $BUNDLE_DIR/staging/boot/initrd.img"

REPORTS_DIR="$BUNDLE_DIR/reports"
LOG="$REPORTS_DIR/qemu-boot.log"
SUMMARY="$REPORTS_DIR/qemu-boot-summary.env"

printf 'BASE1 QEMU BOOT CHECK\n'
printf 'bundle : %s\n' "$BUNDLE_DIR"
printf 'mode   : %s\n' "$MODE"
printf 'expect : %s\n' "$EXPECT"
printf 'timeout: %ss\n' "$TIMEOUT_SECONDS"

if [ "$MODE" = "dry-run" ]; then
  printf '\nplan: sh %s/run-qemu-bundle.sh\n' "$BUNDLE_DIR"
  printf 'result: dry-run\n'
  printf 'non-claims: no emulator launched; no installer run; no hardware validation performed\n'
  exit 0
fi

[ "$CONFIRM" = "launch-qemu-base1-preview" ] || fail "--execute requires --confirm launch-qemu-base1-preview"

if command -v timeout >/dev/null 2>&1; then
  TIMEOUT_BIN=timeout
elif command -v gtimeout >/dev/null 2>&1; then
  TIMEOUT_BIN=gtimeout
else
  fail 'execute mode requires timeout or gtimeout so QEMU cannot run unbounded'
fi

mkdir -p "$REPORTS_DIR"
: > "$LOG"

note "launching QEMU through bundle scaffold with $TIMEOUT_BIN; log: $LOG"

set +e
"$TIMEOUT_BIN" "$TIMEOUT_SECONDS" sh "$BUNDLE_DIR/run-qemu-bundle.sh" > "$LOG" 2>&1
rc=$?
set -e

if grep -F "$EXPECT" "$LOG" >/dev/null 2>&1; then
  result=pass
else
  result=failed
fi

cat > "$SUMMARY" <<EOF
BASE1_QEMU_BOOT_RESULT=$result
BASE1_QEMU_BOOT_EXIT_CODE=$rc
BASE1_QEMU_BOOT_EXPECT=$EXPECT
BASE1_QEMU_BOOT_LOG=reports/qemu-boot.log
BASE1_NON_CLAIM_HARDWARE_VALIDATED=1
BASE1_NON_CLAIM_INSTALLER=1
BASE1_NON_CLAIM_DAILY_DRIVER=1
EOF

printf '\nqemu-exit-code: %s\n' "$rc"
printf 'result: %s\n' "$result"
printf 'log: %s\n' "$LOG"
printf 'summary: %s\n' "$SUMMARY"
printf 'non-claims: emulator-only evidence; no installer; no hardware validation; no daily-driver claim\n'

[ "$result" = "pass" ] || exit 1

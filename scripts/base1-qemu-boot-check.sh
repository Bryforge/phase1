#!/usr/bin/env sh
# Base1 guarded QEMU boot checker.
#
# This script is the first explicit QEMU execution hook for Base1 preview
# bundles. It defaults to dry-run. Use --execute plus the confirmation phrase
# to actually run QEMU against the selected preview bundle, capture serial
# output, and check for a Phase1 readiness marker.
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
QEMU_BIN=${BASE1_QEMU:-qemu-system-x86_64}
MEMORY_MB=${BASE1_QEMU_MEMORY_MB:-1024}
BOOT_PROFILE=${BASE1_QEMU_BOOT_PROFILE:-standard}
EXTRA_APPEND=${BASE1_QEMU_EXTRA_APPEND:-}

usage() {
  cat <<'USAGE'
base1 qemu boot check

usage:
  scripts/base1-qemu-boot-check.sh [options]

options:
  --bundle <dir>       Base1 emulator preview bundle under build/
  --dry-run            inspect the bundle and print the QEMU handoff plan
  --execute            run QEMU with serial capture and inspect output
  --confirm <phrase>   required with --execute; phrase: launch-qemu-base1-preview
  --timeout <seconds>  max runtime before terminating QEMU, default: 30
  --expect <text>      serial marker required for PASS, default: phase1 6.0.0 ready
  --boot-profile <p>   kernel command-line profile: standard or hardened
  --append <text>      extra kernel command-line text appended after profile settings
  -h, --help           show this help

profiles:
  standard   serial console, safe preview flags, host tools disabled
  hardened   standard profile plus hardening-oriented Linux kernel parameters

outputs:
  <bundle>/reports/qemu-boot.log
  <bundle>/reports/qemu-boot-summary.env

result model:
  dry-run       no emulator launched
  pass          marker found in serial log
  failed        marker missing or bundle invalid

non-claims:
  This is emulator-only boot evidence. The hardened profile requests Linux
  hardening-oriented kernel parameters, but it does not prove the system is
  hardened. This script does not install Base1, validate real hardware, validate
  recovery, validate an installer, or prove daily-driver readiness. A pass only
  means the selected bundle emitted the expected marker during this QEMU run.
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

append_for_profile() {
  case "$1" in
    standard)
      printf '%s\n' 'console=ttyS0,115200 earlyprintk=serial,ttyS0,115200 loglevel=8 base1.preview=1 base1.emulator=1 phase1.safe=1 phase1.host_tools=0'
      ;;
    hardened)
      printf '%s\n' 'console=ttyS0,115200 earlyprintk=serial,ttyS0,115200 loglevel=8 base1.preview=1 base1.emulator=1 phase1.safe=1 phase1.host_tools=0 module.sig_enforce=1 lockdown=confidentiality lsm=landlock,lockdown,yama,integrity,apparmor,bpf random.trust_cpu=off slab_nomerge init_on_alloc=1 init_on_free=1 page_alloc.shuffle=1 pti=on vsyscall=none debugfs=off oops=panic panic=10 quiet'
      ;;
    *)
      return 1
      ;;
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
    --boot-profile)
      [ "$#" -ge 2 ] || fail '--boot-profile requires a value'
      BOOT_PROFILE=$2
      shift 2
      ;;
    --append)
      [ "$#" -ge 2 ] || fail '--append requires a value'
      EXTRA_APPEND=$2
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

case "$MEMORY_MB" in
  ''|*[!0-9]*) fail 'BASE1_QEMU_MEMORY_MB must be a positive integer' ;;
esac
[ "$MEMORY_MB" -gt 0 ] || fail 'BASE1_QEMU_MEMORY_MB must be greater than zero'

APPEND=$(append_for_profile "$BOOT_PROFILE") || fail "unsupported boot profile: $BOOT_PROFILE"
if [ -n "$EXTRA_APPEND" ]; then
  APPEND="$APPEND $EXTRA_APPEND"
fi

require_build_bundle "$BUNDLE_DIR" || fail "bundle must be under build/: $BUNDLE_DIR"
[ -d "$BUNDLE_DIR" ] || fail "bundle directory does not exist: $BUNDLE_DIR"
[ -f "$BUNDLE_DIR/run-qemu-bundle.sh" ] || fail "missing bundle QEMU scaffold: $BUNDLE_DIR/run-qemu-bundle.sh"
[ -f "$BUNDLE_DIR/staging/boot/vmlinuz" ] || fail "missing kernel: $BUNDLE_DIR/staging/boot/vmlinuz"
[ -f "$BUNDLE_DIR/staging/boot/initrd.img" ] || fail "missing initrd: $BUNDLE_DIR/staging/boot/initrd.img"
[ -f "$BUNDLE_DIR/base1-sandbox.raw" ] || fail "missing sandbox disk: $BUNDLE_DIR/base1-sandbox.raw"

REPORTS_DIR="$BUNDLE_DIR/reports"
LOG="$REPORTS_DIR/qemu-boot.log"
SUMMARY="$REPORTS_DIR/qemu-boot-summary.env"
KERNEL="$BUNDLE_DIR/staging/boot/vmlinuz"
INITRD="$BUNDLE_DIR/staging/boot/initrd.img"
SANDBOX="$BUNDLE_DIR/base1-sandbox.raw"

printf 'BASE1 QEMU BOOT CHECK\n'
printf 'bundle : %s\n' "$BUNDLE_DIR"
printf 'mode   : %s\n' "$MODE"
printf 'expect : %s\n' "$EXPECT"
printf 'timeout: %ss\n' "$TIMEOUT_SECONDS"
printf 'qemu   : %s\n' "$QEMU_BIN"
printf 'profile: %s\n' "$BOOT_PROFILE"
printf 'append : %s\n' "$APPEND"

if [ "$MODE" = "dry-run" ]; then
  printf '\nplan: sh %s/run-qemu-bundle.sh\n' "$BUNDLE_DIR"
  printf 'serial-capture-plan: %s -display none -serial file:%s\n' "$QEMU_BIN" "$LOG"
  printf 'result: dry-run\n'
  printf 'non-claims: no emulator launched; no installer run; no hardware validation performed; hardened profile is request-only\n'
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

command -v "$QEMU_BIN" >/dev/null 2>&1 || fail "missing QEMU executable: $QEMU_BIN"

mkdir -p "$REPORTS_DIR"
: > "$LOG"

note "launching QEMU with serial capture through $TIMEOUT_BIN; log: $LOG"

set +e
"$TIMEOUT_BIN" "$TIMEOUT_SECONDS" "$QEMU_BIN" \
  -m "$MEMORY_MB" \
  -display none \
  -serial "file:$LOG" \
  -no-reboot \
  -kernel "$KERNEL" \
  -initrd "$INITRD" \
  -drive "file=$SANDBOX,format=raw,if=virtio" \
  -append "$APPEND"
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
BASE1_QEMU_BOOT_PROFILE=$BOOT_PROFILE
BASE1_QEMU_BOOT_APPEND=$APPEND
BASE1_NON_CLAIM_HARDWARE_VALIDATED=1
BASE1_NON_CLAIM_INSTALLER=1
BASE1_NON_CLAIM_HARDENED=1
BASE1_NON_CLAIM_DAILY_DRIVER=1
EOF

printf '\nqemu-exit-code: %s\n' "$rc"
printf 'result: %s\n' "$result"
printf 'log: %s\n' "$LOG"
printf 'summary: %s\n' "$SUMMARY"
printf 'non-claims: emulator-only evidence; no installer; no hardware validation; no hardening proof; no daily-driver claim\n'

[ "$result" = "pass" ] || exit 1

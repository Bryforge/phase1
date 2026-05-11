#!/usr/bin/env sh
# Base1 B3 OpenBSD stage wrapper.
#
# This prepares a local OpenBSD boot artifact as a B3 staging point. OpenBSD is
# not forced through the GNU/Linux kernel/initrd handoff path because its boot
# artifacts and installer flow differ. This script is local-only and evidence-
# oriented: it does not download OpenBSD, install Base1, modify host boot
# settings, partition disks, or claim hardware readiness.

set -eu

MODE=prepare
OUT_DIR=${BASE1_B3_OPENBSD_STAGE_OUT:-build/base1-b3-openbsd-stage}
OPENBSD_ISO=${BASE1_OPENBSD_ISO:-}
OPENBSD_IMG=${BASE1_OPENBSD_IMG:-}
TIMEOUT_SECONDS=${BASE1_B3_TIMEOUT:-45}
EXPECT=${BASE1_B3_OPENBSD_MARKER:-OpenBSD}
QEMU_BIN=${BASE1_QEMU:-qemu-system-x86_64}
MEMORY_MB=${BASE1_QEMU_MEMORY_MB:-1024}
BOOT_MODE=auto
CHECK_MODE=${BASE1_B3_OPENBSD_CHECK_MODE:-marker}

usage() {
  cat <<'USAGE'
base1 B3 OpenBSD stage

usage:
  sh scripts/base1-b3-openbsd-stage.sh [--iso <path>|--img <path>] [--prepare|--dry-run|--check]

options:
  --iso <path>         local OpenBSD install ISO, for example install*.iso
  --img <path>         local OpenBSD boot image, for example install*.img or miniroot*.img
  --out <build/dir>    output evidence directory, default: build/base1-b3-openbsd-stage
  --prepare            create evidence directory and stage metadata only, default
  --dry-run            print the guarded QEMU OpenBSD boot plan
  --check              run guarded QEMU serial-marker check
  --check-mode <mode>  marker or launch, default: marker
  --timeout <seconds>  check timeout, default: 45
  --expect <text>      expected serial marker, default: OpenBSD
  --memory-mb <n>      QEMU memory size, default: 1024
  -h, --help           show this help

stage model:
  This uses an existing local OpenBSD boot artifact as a B3 staging point. It is
  separate from the GNU/Linux kernel/initrd handoff because OpenBSD uses its own
  bootloader, kernel, ramdisk, and installer media shape.

check modes:
  marker   pass only when the expected marker appears in the captured serial log
  launch   pass when QEMU launches in a bounded run, even if the OpenBSD console
           is not yet routed to serial; this is launch evidence, not boot proof

outputs:
  <out>/openbsd-stage.env
  <out>/reports/openbsd-qemu-boot.log       when --check is used
  <out>/reports/openbsd-qemu-summary.env    when --check is used

non-claims:
  This is emulator-only OpenBSD staging evidence. It does not make Base1 an
  OpenBSD distribution, install Base1, modify host boot settings, validate
  physical hardware, validate an installer, validate recovery, prove hardening,
  or prove daily-driver readiness.
USAGE
}

fail() {
  printf 'base1-b3-openbsd-stage: %s\n' "$1" >&2
  exit 1
}

require_build_out_dir() {
  case "$1" in
    build/*) return 0 ;;
    *) return 1 ;;
  esac
}

valid_check_mode() {
  case "$1" in
    marker|launch) return 0 ;;
    *) return 1 ;;
  esac
}

while [ "$#" -gt 0 ]; do
  case "$1" in
    --iso)
      [ "$#" -ge 2 ] || fail '--iso requires a value'
      OPENBSD_ISO=$2
      BOOT_MODE=iso
      shift 2
      ;;
    --img)
      [ "$#" -ge 2 ] || fail '--img requires a value'
      OPENBSD_IMG=$2
      BOOT_MODE=img
      shift 2
      ;;
    --out)
      [ "$#" -ge 2 ] || fail '--out requires a value'
      OUT_DIR=$2
      shift 2
      ;;
    --prepare)
      MODE=prepare
      shift
      ;;
    --dry-run)
      MODE=dry-run
      shift
      ;;
    --check)
      MODE=check
      shift
      ;;
    --check-mode)
      [ "$#" -ge 2 ] || fail '--check-mode requires a value'
      CHECK_MODE=$2
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
    --memory-mb)
      [ "$#" -ge 2 ] || fail '--memory-mb requires a value'
      MEMORY_MB=$2
      shift 2
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      usage >&2
      fail "unknown option: $1"
      ;;
  esac
done

case "$TIMEOUT_SECONDS" in
  ''|*[!0-9]*) fail '--timeout must be a positive integer' ;;
esac
[ "$TIMEOUT_SECONDS" -gt 0 ] || fail '--timeout must be greater than zero'

case "$MEMORY_MB" in
  ''|*[!0-9]*) fail '--memory-mb must be a positive integer' ;;
esac
[ "$MEMORY_MB" -gt 0 ] || fail '--memory-mb must be greater than zero'

valid_check_mode "$CHECK_MODE" || fail "unsupported check mode: $CHECK_MODE"
require_build_out_dir "$OUT_DIR" || fail "output directory must be under build/: $OUT_DIR"

case "$BOOT_MODE" in
  iso)
    [ -f "$OPENBSD_ISO" ] || fail "OpenBSD ISO not found: $OPENBSD_ISO"
    ARTIFACT=$OPENBSD_ISO
    ;;
  img)
    [ -f "$OPENBSD_IMG" ] || fail "OpenBSD image not found: $OPENBSD_IMG"
    ARTIFACT=$OPENBSD_IMG
    ;;
  auto)
    fail 'provide --iso <path> or --img <path> for the OpenBSD stage'
    ;;
  *)
    fail "internal unsupported boot mode: $BOOT_MODE"
    ;;
esac

REPORTS_DIR="$OUT_DIR/reports"
LOG="$REPORTS_DIR/openbsd-qemu-boot.log"
SUMMARY="$REPORTS_DIR/openbsd-qemu-summary.env"
STAGE_ENV="$OUT_DIR/openbsd-stage.env"

mkdir -p "$OUT_DIR"

cat > "$STAGE_ENV" <<EOF
BASE1_B3_OPENBSD_STAGE_MODE=$MODE
BASE1_B3_OPENBSD_BOOT_MODE=$BOOT_MODE
BASE1_B3_OPENBSD_CHECK_MODE=$CHECK_MODE
BASE1_B3_OPENBSD_ARTIFACT=$ARTIFACT
BASE1_B3_OPENBSD_EXPECT=$EXPECT
BASE1_B3_OPENBSD_LOG=reports/openbsd-qemu-boot.log
BASE1_B3_OPENBSD_SUMMARY=reports/openbsd-qemu-summary.env
BASE1_B3_OPENBSD_CLAIM=not_claimed
BASE1_B3_OPENBSD_NON_CLAIM_INSTALLER=1
BASE1_B3_OPENBSD_NON_CLAIM_HARDWARE=1
BASE1_B3_OPENBSD_NON_CLAIM_RECOVERY=1
BASE1_B3_OPENBSD_NON_CLAIM_HARDENED=1
BASE1_B3_OPENBSD_NON_CLAIM_DAILY_DRIVER=1
EOF

printf 'BASE1 B3 OPENBSD STAGE\n'
printf 'mode      : %s\n' "$MODE"
printf 'out       : %s\n' "$OUT_DIR"
printf 'boot_mode : %s\n' "$BOOT_MODE"
printf 'check_mode: %s\n' "$CHECK_MODE"
printf 'artifact  : %s\n' "$ARTIFACT"
printf 'expect    : %s\n' "$EXPECT"
printf 'stage_env : %s\n' "$STAGE_ENV"
printf '\n'

if [ "$MODE" = prepare ]; then
  printf 'result: prepared\n'
  printf 'non_claims: emulator-only OpenBSD stage; no installer; no hardware validation; no hardening proof; no daily-driver claim\n'
  exit 0
fi

case "$BOOT_MODE" in
  iso)
    QEMU_ARGS="-m $MEMORY_MB -display none -serial file:$LOG -no-reboot -cdrom $ARTIFACT -boot d"
    ;;
  img)
    QEMU_ARGS="-m $MEMORY_MB -display none -serial file:$LOG -no-reboot -drive file=$ARTIFACT,format=raw,if=ide -boot c"
    ;;
esac

if [ "$MODE" = dry-run ]; then
  printf 'plan: %s %s\n' "$QEMU_BIN" "$QEMU_ARGS"
  printf 'serial-capture-plan: %s\n' "$LOG"
  printf 'result: dry-run\n'
  printf 'non_claims: no emulator launched; no installer run; no hardware validation performed\n'
  exit 0
fi

[ "$MODE" = check ] || fail "internal unsupported mode: $MODE"

if command -v timeout >/dev/null 2>&1; then
  TIMEOUT_BIN=timeout
elif command -v gtimeout >/dev/null 2>&1; then
  TIMEOUT_BIN=gtimeout
else
  fail 'check mode requires timeout or gtimeout so QEMU cannot run unbounded'
fi

command -v "$QEMU_BIN" >/dev/null 2>&1 || fail "missing QEMU executable: $QEMU_BIN"
mkdir -p "$REPORTS_DIR"
: > "$LOG"

set +e
# shellcheck disable=SC2086
"$TIMEOUT_BIN" "$TIMEOUT_SECONDS" "$QEMU_BIN" $QEMU_ARGS
rc=$?
set -e

case "$CHECK_MODE" in
  marker)
    if grep -F "$EXPECT" "$LOG" >/dev/null 2>&1; then
      result=pass
    else
      result=failed
    fi
    ;;
  launch)
    if [ "$rc" -eq 124 ] || [ "$rc" -eq 0 ]; then
      result=pass
    else
      result=failed
    fi
    ;;
esac

cat > "$SUMMARY" <<EOF
BASE1_B3_OPENBSD_RESULT=$result
BASE1_B3_OPENBSD_EXIT_CODE=$rc
BASE1_B3_OPENBSD_EXPECT=$EXPECT
BASE1_B3_OPENBSD_BOOT_MODE=$BOOT_MODE
BASE1_B3_OPENBSD_CHECK_MODE=$CHECK_MODE
BASE1_B3_OPENBSD_ARTIFACT=$ARTIFACT
BASE1_B3_OPENBSD_LOG=reports/openbsd-qemu-boot.log
BASE1_B3_OPENBSD_CLAIM=not_claimed
BASE1_B3_OPENBSD_NON_CLAIM_INSTALLER=1
BASE1_B3_OPENBSD_NON_CLAIM_HARDWARE=1
BASE1_B3_OPENBSD_NON_CLAIM_RECOVERY=1
BASE1_B3_OPENBSD_NON_CLAIM_HARDENED=1
BASE1_B3_OPENBSD_NON_CLAIM_DAILY_DRIVER=1
EOF

printf 'qemu-exit-code: %s\n' "$rc"
printf 'result: %s\n' "$result"
printf 'log: %s\n' "$LOG"
printf 'summary: %s\n' "$SUMMARY"
printf 'non_claims: emulator-only OpenBSD evidence; no installer; no hardware validation; no hardening proof; no daily-driver claim\n'

[ "$result" = pass ] || exit 1

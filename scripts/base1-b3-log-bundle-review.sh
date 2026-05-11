#!/usr/bin/env sh
# Base1 B3 log bundle review scaffold.
#
# Reviews the local B2/B3 evidence summaries and logs that are expected before a
# stronger B3 VM-validation claim can be considered. This script does not launch
# emulators, install Base1, mutate disks, modify host boot settings, prove
# hardening, validate hardware, or claim daily-driver readiness.

set -eu

MODE=dry-run
OUT_DIR=${BASE1_B3_REVIEW_OUT:-build/base1-b3-vm-validation}
WRITE_REPORT=no

B2_SUMMARY=${BASE1_B2_TEST_SUITE_SUMMARY:-build/base1-b2-test-suite/b2-test-suite-summary.env}
UEFI_SUMMARY=${BASE1_B3_UEFI_SUMMARY:-build/base1-b3-uefi-proof/reports/b3-summary.env}
UEFI_LOG=${BASE1_B3_UEFI_LOG:-build/base1-b3-uefi-proof/reports/b3-serial.log}
HANDOFF_SUMMARY=${BASE1_B3_HANDOFF_SUMMARY:-build/base1-b3-kernel-handoff/reports/qemu-boot-summary.env}
HANDOFF_LOG=${BASE1_B3_HANDOFF_LOG:-build/base1-b3-kernel-handoff/reports/qemu-boot.log}
GNULINUX_SUMMARY=${BASE1_B3_GNULINUX_SUMMARY:-build/base1-b3-gnulinux-stage/reports/qemu-boot-summary.env}
GNULINUX_LOG=${BASE1_B3_GNULINUX_LOG:-build/base1-b3-gnulinux-stage/reports/qemu-boot.log}
OPENBSD_SUMMARY=${BASE1_B3_OPENBSD_SUMMARY:-build/base1-b3-openbsd-stage/reports/openbsd-qemu-summary.env}
OPENBSD_LOG=${BASE1_B3_OPENBSD_LOG:-build/base1-b3-openbsd-stage/reports/openbsd-qemu-boot.log}
OPENBSD_LIMITATION=${BASE1_B3_OPENBSD_LIMITATION_DOC:-docs/os/B3_OPENBSD_SERIAL_LIMITATION.md}

usage() {
  cat <<'USAGE'
base1 B3 log bundle review

usage:
  sh scripts/base1-b3-log-bundle-review.sh [--dry-run|--review] [--write-report]

options:
  --dry-run        print expected evidence paths without requiring them, default
  --review         check expected local evidence files and write review result
  --out <dir>      output directory, default: build/base1-b3-vm-validation
  --write-report   write <out>/b3-log-bundle-review.env, implied by --review
  -h, --help       show this help

required local evidence:
  build/base1-b2-test-suite/b2-test-suite-summary.env
  build/base1-b3-uefi-proof/reports/b3-summary.env
  build/base1-b3-uefi-proof/reports/b3-serial.log
  build/base1-b3-kernel-handoff/reports/qemu-boot-summary.env
  build/base1-b3-kernel-handoff/reports/qemu-boot.log
  build/base1-b3-gnulinux-stage/reports/qemu-boot-summary.env
  build/base1-b3-gnulinux-stage/reports/qemu-boot.log
  build/base1-b3-openbsd-stage/reports/openbsd-qemu-summary.env
  build/base1-b3-openbsd-stage/reports/openbsd-qemu-boot.log
  docs/os/B3_OPENBSD_SERIAL_LIMITATION.md

outputs:
  <out>/b3-log-bundle-review.env

non-claims:
  This is a local evidence review scaffold only. It does not make Base1
  bootable, installer-ready, recovery-complete, hardened, hardware-validated,
  release-candidate ready, or daily-driver ready.
USAGE
}

fail() {
  printf 'base1-b3-log-bundle-review: %s\n' "$1" >&2
  exit 1
}

require_build_out_dir() {
  case "$1" in
    build/*) return 0 ;;
    *) return 1 ;;
  esac
}

while [ "$#" -gt 0 ]; do
  case "$1" in
    --dry-run)
      MODE=dry-run
      shift
      ;;
    --review)
      MODE=review
      WRITE_REPORT=yes
      shift
      ;;
    --write-report)
      WRITE_REPORT=yes
      shift
      ;;
    --out)
      [ "$#" -ge 2 ] || fail '--out requires a value'
      OUT_DIR=$2
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

require_build_out_dir "$OUT_DIR" || fail "output directory must be under build/: $OUT_DIR"

REPORT="$OUT_DIR/b3-log-bundle-review.env"

present() {
  if [ -f "$1" ]; then
    printf 'yes'
  else
    printf 'no'
  fi
}

has_pass_marker() {
  [ -f "$1" ] || return 1
  grep -E '([Rr][Ee][Ss][Uu][Ll][Tt]|stage_result)[=:][[:space:]]*pass|result:[[:space:]]*pass' "$1" >/dev/null 2>&1
}

print_paths() {
  printf 'b2_summary: %s\n' "$B2_SUMMARY"
  printf 'uefi_summary: %s\n' "$UEFI_SUMMARY"
  printf 'uefi_log: %s\n' "$UEFI_LOG"
  printf 'handoff_summary: %s\n' "$HANDOFF_SUMMARY"
  printf 'handoff_log: %s\n' "$HANDOFF_LOG"
  printf 'gnulinux_summary: %s\n' "$GNULINUX_SUMMARY"
  printf 'gnulinux_log: %s\n' "$GNULINUX_LOG"
  printf 'openbsd_summary: %s\n' "$OPENBSD_SUMMARY"
  printf 'openbsd_log: %s\n' "$OPENBSD_LOG"
  printf 'openbsd_limitation: %s\n' "$OPENBSD_LIMITATION"
}

write_report() {
  mkdir -p "$OUT_DIR"
  cat > "$REPORT" <<EOF
BASE1_B3_LOG_REVIEW_MODE=$MODE
BASE1_B3_LOG_REVIEW_RESULT=$result
BASE1_B3_LOG_REVIEW_CLAIM=not_claimed
BASE1_B3_LOG_REVIEW_B2_SUMMARY=$B2_SUMMARY
BASE1_B3_LOG_REVIEW_B2_SUMMARY_PRESENT=$(present "$B2_SUMMARY")
BASE1_B3_LOG_REVIEW_UEFI_SUMMARY=$UEFI_SUMMARY
BASE1_B3_LOG_REVIEW_UEFI_SUMMARY_PRESENT=$(present "$UEFI_SUMMARY")
BASE1_B3_LOG_REVIEW_UEFI_LOG=$UEFI_LOG
BASE1_B3_LOG_REVIEW_UEFI_LOG_PRESENT=$(present "$UEFI_LOG")
BASE1_B3_LOG_REVIEW_HANDOFF_SUMMARY=$HANDOFF_SUMMARY
BASE1_B3_LOG_REVIEW_HANDOFF_SUMMARY_PRESENT=$(present "$HANDOFF_SUMMARY")
BASE1_B3_LOG_REVIEW_HANDOFF_LOG=$HANDOFF_LOG
BASE1_B3_LOG_REVIEW_HANDOFF_LOG_PRESENT=$(present "$HANDOFF_LOG")
BASE1_B3_LOG_REVIEW_GNULINUX_SUMMARY=$GNULINUX_SUMMARY
BASE1_B3_LOG_REVIEW_GNULINUX_SUMMARY_PRESENT=$(present "$GNULINUX_SUMMARY")
BASE1_B3_LOG_REVIEW_GNULINUX_LOG=$GNULINUX_LOG
BASE1_B3_LOG_REVIEW_GNULINUX_LOG_PRESENT=$(present "$GNULINUX_LOG")
BASE1_B3_LOG_REVIEW_OPENBSD_SUMMARY=$OPENBSD_SUMMARY
BASE1_B3_LOG_REVIEW_OPENBSD_SUMMARY_PRESENT=$(present "$OPENBSD_SUMMARY")
BASE1_B3_LOG_REVIEW_OPENBSD_LOG=$OPENBSD_LOG
BASE1_B3_LOG_REVIEW_OPENBSD_LOG_PRESENT=$(present "$OPENBSD_LOG")
BASE1_B3_LOG_REVIEW_OPENBSD_LIMITATION=$OPENBSD_LIMITATION
BASE1_B3_LOG_REVIEW_OPENBSD_LIMITATION_PRESENT=$(present "$OPENBSD_LIMITATION")
BASE1_B3_NON_CLAIM_BOOTABLE_PHYSICAL=1
BASE1_B3_NON_CLAIM_INSTALLER=1
BASE1_B3_NON_CLAIM_RECOVERY=1
BASE1_B3_NON_CLAIM_HARDENED=1
BASE1_B3_NON_CLAIM_HARDWARE=1
BASE1_B3_NON_CLAIM_RELEASE_CANDIDATE=1
BASE1_B3_NON_CLAIM_DAILY_DRIVER=1
EOF
}

printf 'BASE1 B3 LOG BUNDLE REVIEW\n'
printf 'mode  : %s\n' "$MODE"
printf 'out   : %s\n' "$OUT_DIR"
printf 'report: %s\n' "$REPORT"
printf '\n'

print_paths
printf '\n'

result=not_run

if [ "$MODE" = dry-run ]; then
  if [ "$WRITE_REPORT" = yes ]; then
    write_report
    printf 'written_report: %s\n' "$REPORT"
  fi
  printf 'result: dry-run\n'
  printf 'non_claims: no evidence reviewed; no B3 claim; no installer/hardware/hardening/daily-driver claim\n'
  exit 0
fi

[ "$MODE" = review ] || fail "internal unsupported mode: $MODE"

result=pass
failed=0

check_file() {
  label=$1
  path=$2
  if [ -f "$path" ]; then
    printf 'present: %s -> %s\n' "$label" "$path"
  else
    printf 'missing: %s -> %s\n' "$label" "$path"
    failed=$((failed + 1))
    result=failed
  fi
}

check_pass_summary() {
  label=$1
  path=$2
  check_file "$label" "$path"
  if [ -f "$path" ]; then
    if has_pass_marker "$path"; then
      printf 'pass_marker: %s\n' "$label"
    else
      printf 'missing_pass_marker: %s -> %s\n' "$label" "$path"
      failed=$((failed + 1))
      result=failed
    fi
  fi
}

check_pass_summary b2_summary "$B2_SUMMARY"
check_pass_summary uefi_summary "$UEFI_SUMMARY"
check_file uefi_log "$UEFI_LOG"
check_pass_summary handoff_summary "$HANDOFF_SUMMARY"
check_file handoff_log "$HANDOFF_LOG"
check_pass_summary gnulinux_summary "$GNULINUX_SUMMARY"
check_file gnulinux_log "$GNULINUX_LOG"
check_pass_summary openbsd_summary "$OPENBSD_SUMMARY"
check_file openbsd_log "$OPENBSD_LOG"
check_file openbsd_limitation "$OPENBSD_LIMITATION"

write_report

printf '\nresult: %s\n' "$result"
printf 'failed_checks: %s\n' "$failed"
printf 'summary: %s\n' "$REPORT"
printf 'non_claims: local B3 log review only; no boot-ready claim; no installer claim; no hardware validation; no hardening proof; no daily-driver claim\n'

[ "$result" = pass ] || exit 1

#!/usr/bin/env sh
# Base1 B2 focused test-suite checker.
#
# Runs or prints the focused B2 dry-run assembly validation commands and writes
# a local build/report artifact. This is a validation helper only. It does not
# build an installer, boot an OS, mutate disks, alter host boot settings, or
# claim VM/hardware readiness.

set -eu

MODE=dry-run
OUT_DIR=${BASE1_B2_TEST_SUITE_OUT:-build/base1-b2-test-suite}
WRITE_REPORT=no

usage() {
  cat <<'USAGE'
base1 B2 focused test-suite checker

usage:
  sh scripts/base1-b2-test-suite-check.sh [--dry-run|--check] [--write-report]

options:
  --dry-run        print the focused B2 commands without running them, default
  --check          run the focused B2 commands and record pass/fail evidence
  --out <dir>      output directory, default: build/base1-b2-test-suite
  --write-report   write <out>/b2-test-suite-summary.env, implied by --check
  -h, --help       show this help

focused commands:
  cargo test -p phase1 --test b2_dry_run_assembly_plan_docs
  cargo test -p phase1 --test base1_b2_assembly_dry_run_script
  cargo test -p phase1 --test b2_dry_run_assembly_limitations_docs
  cargo test -p phase1 --test b2_dry_run_assembly_validation_docs
  cargo test -p phase1 --test b2_dry_run_assembly_output_review_docs
  cargo test -p phase1 --test boot_readiness_status_docs
  cargo test -p phase1 --test boot_readiness_race_plan_docs
  cargo test -p phase1 --test x86_64_boot_support_roadmap_docs
  cargo test -p phase1 --test readme_navigation_reorganization_links

outputs:
  <out>/b2-test-suite-summary.env
  <out>/b2-test-suite.log when --check is used

non-claims:
  This records local B2 focused test-suite evidence only. It does not make
  Base1 bootable, installer-ready, recovery-complete, hardened, VM-validated,
  hardware-validated, release-candidate ready, or daily-driver ready.
USAGE
}

fail() {
  printf 'base1-b2-test-suite-check: %s\n' "$1" >&2
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
    --check)
      MODE=check
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

COMMANDS='cargo test -p phase1 --test b2_dry_run_assembly_plan_docs
cargo test -p phase1 --test base1_b2_assembly_dry_run_script
cargo test -p phase1 --test b2_dry_run_assembly_limitations_docs
cargo test -p phase1 --test b2_dry_run_assembly_validation_docs
cargo test -p phase1 --test b2_dry_run_assembly_output_review_docs
cargo test -p phase1 --test boot_readiness_status_docs
cargo test -p phase1 --test boot_readiness_race_plan_docs
cargo test -p phase1 --test x86_64_boot_support_roadmap_docs
cargo test -p phase1 --test readme_navigation_reorganization_links'

SUMMARY="$OUT_DIR/b2-test-suite-summary.env"
LOG="$OUT_DIR/b2-test-suite.log"

printf 'BASE1 B2 FOCUSED TEST SUITE\n'
printf 'mode  : %s\n' "$MODE"
printf 'out   : %s\n' "$OUT_DIR"
printf 'report: %s\n' "$SUMMARY"
printf '\n'

if [ "$MODE" = dry-run ]; then
  printf '%s\n' "$COMMANDS" | while IFS= read -r cmd; do
    printf 'plan: %s\n' "$cmd"
  done

  if [ "$WRITE_REPORT" = yes ]; then
    mkdir -p "$OUT_DIR"
    cat > "$SUMMARY" <<EOF
BASE1_B2_TEST_SUITE_MODE=dry-run
BASE1_B2_TEST_SUITE_RESULT=not_run
BASE1_B2_TEST_SUITE_CLAIM=not_claimed
BASE1_B2_TEST_SUITE_LOG=b2-test-suite.log
BASE1_B2_NON_CLAIM_BOOTABLE=1
BASE1_B2_NON_CLAIM_INSTALLER=1
BASE1_B2_NON_CLAIM_VM_VALIDATED=1
BASE1_B2_NON_CLAIM_HARDWARE=1
BASE1_B2_NON_CLAIM_DAILY_DRIVER=1
EOF
    printf 'written_report: %s\n' "$SUMMARY"
  fi

  printf 'result: dry-run\n'
  printf 'non_claims: no tests executed; no bootability claim; no installer claim; no VM/hardware validation claim\n'
  exit 0
fi

[ "$MODE" = check ] || fail "internal unsupported mode: $MODE"
command -v cargo >/dev/null 2>&1 || fail 'cargo is required for --check'

mkdir -p "$OUT_DIR"
: > "$LOG"

result=pass
count=0
failed=0

printf '%s\n' "$COMMANDS" | while IFS= read -r cmd; do
  count=$((count + 1))
  printf '\n>>> %s\n' "$cmd" | tee -a "$LOG"
  set +e
  sh -c "$cmd" >> "$LOG" 2>&1
  rc=$?
  set -e
  printf 'exit_code: %s\n' "$rc" | tee -a "$LOG"
  if [ "$rc" -ne 0 ]; then
    failed=$((failed + 1))
    result=failed
  fi
  printf '%s %s %s\n' "$count" "$failed" "$result" > "$OUT_DIR/.b2-test-suite-state"
done

if [ -f "$OUT_DIR/.b2-test-suite-state" ]; then
  # shellcheck disable=SC2034
  read count failed result < "$OUT_DIR/.b2-test-suite-state"
else
  result=failed
  count=0
  failed=1
fi
rm -f "$OUT_DIR/.b2-test-suite-state"

cat > "$SUMMARY" <<EOF
BASE1_B2_TEST_SUITE_MODE=check
BASE1_B2_TEST_SUITE_RESULT=$result
BASE1_B2_TEST_SUITE_COMMAND_COUNT=$count
BASE1_B2_TEST_SUITE_FAILED_COUNT=$failed
BASE1_B2_TEST_SUITE_CLAIM=not_claimed
BASE1_B2_TEST_SUITE_LOG=b2-test-suite.log
BASE1_B2_NON_CLAIM_BOOTABLE=1
BASE1_B2_NON_CLAIM_INSTALLER=1
BASE1_B2_NON_CLAIM_VM_VALIDATED=1
BASE1_B2_NON_CLAIM_HARDWARE=1
BASE1_B2_NON_CLAIM_DAILY_DRIVER=1
EOF

printf '\nresult: %s\n' "$result"
printf 'summary: %s\n' "$SUMMARY"
printf 'log: %s\n' "$LOG"
printf 'non_claims: focused B2 test evidence only; no bootability claim; no installer claim; no VM/hardware validation claim\n'

[ "$result" = pass ] || exit 1

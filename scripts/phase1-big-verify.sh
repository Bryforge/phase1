#!/usr/bin/env sh
# Phase1 big verification runner.
#
# One command to prepare for a larger verification pass after pulling new code.
# It can optionally run safe auto-fixes, run broad repo validation, collect B2/B3
# evidence status, write a build/ report, and optionally commit/push tracked
# fixes or reports that the caller staged.
#
# Safe default:
#   sh scripts/phase1-big-verify.sh
#
# Common large pass:
#   sh scripts/phase1-big-verify.sh --pull --fix --full --base1 --b3 --status
#
# Push only after review:
#   sh scripts/phase1-big-verify.sh --pull --fix --full --base1 --b3 --status --commit --push

set -u

MODE=quick
DO_PULL=no
DO_FIX=no
DO_FULL=no
DO_BASE1=no
DO_B2=no
DO_B3=no
DO_STATUS=no
DO_WIKI=no
DO_COMMIT=no
DO_PUSH=no
CONTINUE_ON_FAIL=yes
REMOTE=${PHASE1_BIG_VERIFY_REMOTE:-origin}
BRANCH=${PHASE1_BIG_VERIFY_BRANCH:-edge/stable}
OUT_DIR=${PHASE1_BIG_VERIFY_OUT:-build/phase1-big-verify}
REPORT="$OUT_DIR/report.md"
SUMMARY="$OUT_DIR/summary.env"
LOG="$OUT_DIR/phase1-big-verify.log"
START_UTC=$(date -u '+%Y-%m-%dT%H:%M:%SZ')
FAILED=0
RAN=0

usage() {
  cat <<'USAGE'
Phase1 big verification runner

usage:
  sh scripts/phase1-big-verify.sh [options]

options:
  --pull             git pull origin edge/stable before checks
  --fix              run safe auto-fixes first: cargo fmt and docs generator when present
  --quick            run quick quality gate, default
  --full             run full quality gate instead of quick
  --base1            run Base1 doc/reorg/link/test-inventory checks
  --b2               run B2 focused test-suite checker
  --b3               run B3 VM scaffold/log review checks using existing local evidence
  --status           verify public status files and status docs
  --wiki             check wiki source script syntax and source directory presence
  --commit           commit tracked changes made by --fix or status/doc updates
  --push             commit and push to origin edge/stable
  --stop-on-fail     stop at first failed command
  --continue         continue after failures and summarize them, default
  --out <dir>        output directory under build/, default: build/phase1-big-verify
  --remote <name>    git remote for pull/push, default: origin
  --branch <name>    git branch for pull/push, default: edge/stable
  -h, --help         show this help

recommended first big pass:
  git pull origin edge/stable
  sh scripts/phase1-big-verify.sh --fix --full --base1 --b2 --b3 --status

recommended push pass after reviewing results:
  sh scripts/phase1-big-verify.sh --fix --full --base1 --b2 --b3 --status --commit --push

non-claims:
  This script validates repository and local emulator evidence only. It does not
  install Base1, write disks, change boot settings, prove hardening, validate
  recovery, validate physical hardware, or make a daily-driver claim.
USAGE
}

fail() {
  printf 'phase1-big-verify: %s\n' "$1" >&2
  exit 1
}

while [ "$#" -gt 0 ]; do
  case "$1" in
    --pull) DO_PULL=yes; shift ;;
    --fix) DO_FIX=yes; shift ;;
    --quick) MODE=quick; shift ;;
    --full) MODE=full; DO_FULL=yes; shift ;;
    --base1) DO_BASE1=yes; shift ;;
    --b2) DO_B2=yes; shift ;;
    --b3) DO_B3=yes; shift ;;
    --status) DO_STATUS=yes; shift ;;
    --wiki) DO_WIKI=yes; shift ;;
    --commit) DO_COMMIT=yes; shift ;;
    --push) DO_COMMIT=yes; DO_PUSH=yes; shift ;;
    --stop-on-fail) CONTINUE_ON_FAIL=no; shift ;;
    --continue) CONTINUE_ON_FAIL=yes; shift ;;
    --out)
      [ "$#" -ge 2 ] || fail '--out requires a value'
      OUT_DIR=$2
      REPORT="$OUT_DIR/report.md"
      SUMMARY="$OUT_DIR/summary.env"
      LOG="$OUT_DIR/phase1-big-verify.log"
      shift 2
      ;;
    --remote)
      [ "$#" -ge 2 ] || fail '--remote requires a value'
      REMOTE=$2
      shift 2
      ;;
    --branch)
      [ "$#" -ge 2 ] || fail '--branch requires a value'
      BRANCH=$2
      shift 2
      ;;
    -h|--help) usage; exit 0 ;;
    *) usage >&2; fail "unknown option: $1" ;;
  esac
done

case "$OUT_DIR" in
  build/*) ;;
  *) fail "output directory must be under build/: $OUT_DIR" ;;
esac

[ -d .git ] || fail 'run from the phase1 repository root'
mkdir -p "$OUT_DIR"
: > "$LOG"

run_step() {
  label=$1
  shift
  RAN=$((RAN + 1))
  printf '\n### %s\n' "$label" | tee -a "$LOG"
  printf '$ %s\n' "$*" | tee -a "$LOG"
  set +e
  "$@" >> "$LOG" 2>&1
  rc=$?
  set -e
  printf 'exit_code: %s\n' "$rc" | tee -a "$LOG"
  if [ "$rc" -ne 0 ]; then
    FAILED=$((FAILED + 1))
    printf 'FAILED: %s\n' "$label" | tee -a "$LOG"
    [ "$CONTINUE_ON_FAIL" = yes ] || exit "$rc"
  else
    printf 'PASS: %s\n' "$label" | tee -a "$LOG"
  fi
  return 0
}

run_shell_step() {
  label=$1
  cmd=$2
  RAN=$((RAN + 1))
  printf '\n### %s\n' "$label" | tee -a "$LOG"
  printf '$ %s\n' "$cmd" | tee -a "$LOG"
  set +e
  sh -c "$cmd" >> "$LOG" 2>&1
  rc=$?
  set -e
  printf 'exit_code: %s\n' "$rc" | tee -a "$LOG"
  if [ "$rc" -ne 0 ]; then
    FAILED=$((FAILED + 1))
    printf 'FAILED: %s\n' "$label" | tee -a "$LOG"
    [ "$CONTINUE_ON_FAIL" = yes ] || exit "$rc"
  else
    printf 'PASS: %s\n' "$label" | tee -a "$LOG"
  fi
  return 0
}

need_script() {
  [ -f "$1" ] || fail "missing script: $1"
}

printf 'PHASE1 BIG VERIFY\n'
printf 'start_utc : %s\n' "$START_UTC"
printf 'branch    : %s\n' "$BRANCH"
printf 'mode      : %s\n' "$MODE"
printf 'out       : %s\n' "$OUT_DIR"
printf 'log       : %s\n' "$LOG"
printf '\n'

if [ "$DO_PULL" = yes ]; then
  run_step "git pull" git pull "$REMOTE" "$BRANCH"
fi

if [ "$DO_FIX" = yes ]; then
  if command -v cargo >/dev/null 2>&1; then
    run_step "cargo fmt auto-fix" cargo fmt --all
  else
    run_shell_step "cargo fmt unavailable" "echo cargo not found"
  fi

  if [ -f scripts/update-docs.py ]; then
    if command -v python3 >/dev/null 2>&1; then
      run_step "update generated docs" python3 scripts/update-docs.py
    else
      run_shell_step "update-docs skipped" "echo python3 not found"
    fi
  fi
fi

if [ "$MODE" = full ]; then
  need_script scripts/quality-check.sh
  run_step "quality full" sh scripts/quality-check.sh full
else
  need_script scripts/quality-check.sh
  run_step "quality quick" sh scripts/quality-check.sh quick
fi

if [ "$DO_BASE1" = yes ]; then
  run_step "base1 docs gate" sh scripts/quality-check.sh base1-docs
  run_step "base1 reorg gate" sh scripts/quality-check.sh base1-reorg
  run_step "base1 link check" sh scripts/base1-link-check.sh
fi

if [ "$DO_B2" = yes ]; then
  need_script scripts/base1-b2-test-suite-check.sh
  run_step "B2 focused test-suite" sh scripts/base1-b2-test-suite-check.sh --check
fi

if [ "$DO_B3" = yes ]; then
  need_script scripts/base1-b3-vm-validate.sh
  run_step "B3 VM validation scaffold" sh scripts/base1-b3-vm-validate.sh --dry-run --write-report
  if [ -f scripts/base1-b3-log-bundle-review.sh ]; then
    run_step "B3 log bundle review" sh scripts/base1-b3-log-bundle-review.sh --review
  fi
  if [ -f scripts/base1-b3-x200-upload-report.sh ]; then
    run_step "B3 X200 report refresh" sh scripts/base1-b3-x200-upload-report.sh
  fi
fi

if [ "$DO_STATUS" = yes ]; then
  run_shell_step "status json parse" "python3 -m json.tool site/status.json >/dev/null"
  run_shell_step "status badge parse" "python3 -m json.tool site/status-badge.json >/dev/null"
  run_shell_step "status docs mention roadmap" "grep -E 'Overall estimated roadmap completion|Non-claims' docs/status/PROJECT_STATUS.md >/dev/null"
fi

if [ "$DO_WIKI" = yes ]; then
  [ -d docs/wiki ] && run_shell_step "wiki source present" "test -d docs/wiki && test -f docs/wiki/Home.md" || run_shell_step "wiki source missing" "test -d docs/wiki"
  [ -f scripts/publish-wiki.sh ] && run_step "publish wiki script syntax" sh -n scripts/publish-wiki.sh
fi

END_UTC=$(date -u '+%Y-%m-%dT%H:%M:%SZ')
GIT_HEAD=$(git rev-parse HEAD 2>/dev/null || printf unknown)
GIT_STATUS=$(git status --short 2>/dev/null || true)

cat > "$SUMMARY" <<EOF
PHASE1_BIG_VERIFY_START_UTC=$START_UTC
PHASE1_BIG_VERIFY_END_UTC=$END_UTC
PHASE1_BIG_VERIFY_RESULT=$([ "$FAILED" -eq 0 ] && printf pass || printf failed)
PHASE1_BIG_VERIFY_STEPS=$RAN
PHASE1_BIG_VERIFY_FAILED=$FAILED
PHASE1_BIG_VERIFY_HEAD=$GIT_HEAD
PHASE1_BIG_VERIFY_LOG=$LOG
PHASE1_BIG_VERIFY_NON_CLAIM_INSTALLER=1
PHASE1_BIG_VERIFY_NON_CLAIM_HARDWARE=1
PHASE1_BIG_VERIFY_NON_CLAIM_HARDENING=1
PHASE1_BIG_VERIFY_NON_CLAIM_RECOVERY=1
PHASE1_BIG_VERIFY_NON_CLAIM_DAILY_DRIVER=1
EOF

cat > "$REPORT" <<EOF
# Phase1 big verification report

Start UTC: $START_UTC
End UTC: $END_UTC
Head: $GIT_HEAD
Result: $([ "$FAILED" -eq 0 ] && printf pass || printf failed)
Steps run: $RAN
Failed steps: $FAILED

Log: $LOG
Summary: $SUMMARY

## Git status after run

\`\`\`text
$GIT_STATUS
\`\`\`

## Non-claims

This report records repository and local emulator-evidence validation only. It does not claim installer readiness, physical hardware validation, recovery validation, hardening proof, release-candidate readiness, or daily-driver readiness.
EOF

printf '\nPHASE1 BIG VERIFY RESULT: %s\n' "$([ "$FAILED" -eq 0 ] && printf pass || printf failed)"
printf 'steps: %s\n' "$RAN"
printf 'failed: %s\n' "$FAILED"
printf 'summary: %s\n' "$SUMMARY"
printf 'report: %s\n' "$REPORT"
printf 'log: %s\n' "$LOG"

if [ "$DO_COMMIT" = yes ]; then
  if [ "$FAILED" -ne 0 ]; then
    printf 'phase1-big-verify: refusing commit because verification failed\n' >&2
    exit 1
  fi
  git add -u
  if git diff --cached --quiet; then
    printf 'phase1-big-verify: no tracked changes to commit\n'
  else
    git commit -m "Run Phase1 big verification updates"
  fi
fi

if [ "$DO_PUSH" = yes ]; then
  if [ "$FAILED" -ne 0 ]; then
    printf 'phase1-big-verify: refusing push because verification failed\n' >&2
    exit 1
  fi
  git push "$REMOTE" "$BRANCH"
fi

[ "$FAILED" -eq 0 ] || exit 1

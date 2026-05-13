#!/usr/bin/env sh
# Upload a repository-safe Phase1 big verification report.
#
# This reads the latest local build/phase1-big-verify output and writes a
# tracked Markdown report under docs/reports/ so the results can be inspected
# from GitHub. It intentionally uploads a sanitized report, not raw build logs.
#
# Usage:
#   sh scripts/phase1-big-verify-upload-report.sh
#   sh scripts/phase1-big-verify-upload-report.sh --push
#
# Typical flow:
#   sh scripts/phase1-big-verify.sh --fix --full --base1 --b2 --b3 --status --wiki
#   sh scripts/phase1-big-verify-upload-report.sh --push

set -eu

SOURCE_DIR=${PHASE1_BIG_VERIFY_OUT:-build/phase1-big-verify}
SOURCE_SUMMARY=${PHASE1_BIG_VERIFY_SUMMARY:-$SOURCE_DIR/summary.env}
SOURCE_REPORT=${PHASE1_BIG_VERIFY_REPORT:-$SOURCE_DIR/report.md}
SOURCE_LOG=${PHASE1_BIG_VERIFY_LOG:-$SOURCE_DIR/phase1-big-verify.log}
DEST=${PHASE1_BIG_VERIFY_UPLOAD_REPORT:-docs/reports/PHASE1_BIG_VERIFY_LATEST.md}
REMOTE=${PHASE1_BIG_VERIFY_REMOTE:-origin}
BRANCH=${PHASE1_BIG_VERIFY_BRANCH:-edge/stable}
DO_COMMIT=no
DO_PUSH=no
ALLOW_DIRTY=no
INCLUDE_LOG_TAIL_LINES=${PHASE1_BIG_VERIFY_LOG_TAIL_LINES:-220}
INCLUDE_FAILED_SECTION_LINES=${PHASE1_BIG_VERIFY_FAILED_SECTION_LINES:-180}

usage() {
  cat <<'USAGE'
Phase1 big verification report uploader

usage:
  sh scripts/phase1-big-verify-upload-report.sh [options]

options:
  --source <dir>          source build dir, default: build/phase1-big-verify
  --dest <path>           tracked report path, default: docs/reports/PHASE1_BIG_VERIFY_LATEST.md
  --commit                commit the tracked report
  --push                  commit and push the tracked report
  --remote <name>         git remote, default: origin
  --branch <name>         git branch, default: edge/stable
  --allow-dirty           allow unrelated local tracked changes while committing
  --log-tail-lines <n>    number of log tail lines to include, default: 220
  -h, --help              show this help

notes:
  - This uploads a sanitized Markdown report, not raw build logs.
  - It can upload failed verification results for remote inspection.
  - It preserves non-claims: no installer, hardware, hardening, recovery, or daily-driver claim.
USAGE
}

fail() {
  printf 'phase1-big-verify-upload-report: %s\n' "$1" >&2
  exit 1
}

while [ "$#" -gt 0 ]; do
  case "$1" in
    --source)
      [ "$#" -ge 2 ] || fail '--source requires a value'
      SOURCE_DIR=$2
      SOURCE_SUMMARY=$SOURCE_DIR/summary.env
      SOURCE_REPORT=$SOURCE_DIR/report.md
      SOURCE_LOG=$SOURCE_DIR/phase1-big-verify.log
      shift 2
      ;;
    --dest)
      [ "$#" -ge 2 ] || fail '--dest requires a value'
      DEST=$2
      shift 2
      ;;
    --commit)
      DO_COMMIT=yes
      shift
      ;;
    --push)
      DO_COMMIT=yes
      DO_PUSH=yes
      shift
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
    --allow-dirty)
      ALLOW_DIRTY=yes
      shift
      ;;
    --log-tail-lines)
      [ "$#" -ge 2 ] || fail '--log-tail-lines requires a value'
      INCLUDE_LOG_TAIL_LINES=$2
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

case "$DEST" in
  docs/*) ;;
  *) fail "destination must be under docs/: $DEST" ;;
esac
case "$INCLUDE_LOG_TAIL_LINES" in
  ''|*[!0-9]*) fail '--log-tail-lines must be a number' ;;
esac
case "$INCLUDE_FAILED_SECTION_LINES" in
  ''|*[!0-9]*) fail 'PHASE1_BIG_VERIFY_FAILED_SECTION_LINES must be a number' ;;
esac

[ -d .git ] || fail 'run from the phase1 repository root'
[ -f "$SOURCE_SUMMARY" ] || fail "missing summary: $SOURCE_SUMMARY"
[ -f "$SOURCE_REPORT" ] || fail "missing report: $SOURCE_REPORT"
[ -f "$SOURCE_LOG" ] || fail "missing log: $SOURCE_LOG"

mkdir -p "$(dirname "$DEST")"

summary_value() {
  key=$1
  default=${2:-unknown}
  value=$(grep -E "^${key}=" "$SOURCE_SUMMARY" 2>/dev/null | tail -n 1 | sed "s/^${key}=//" || true)
  if [ -n "$value" ]; then
    printf '%s' "$value" | tr '\n\r|' '   '
  else
    printf '%s' "$default"
  fi
}

safe_text() {
  printf '%s' "$1" | tr '\n\r|' '   '
}

NOW_UTC=$(date -u '+%Y-%m-%dT%H:%M:%SZ')
HEAD_SHA=$(git rev-parse HEAD 2>/dev/null || printf unknown)
HEAD_BRANCH=$(git rev-parse --abbrev-ref HEAD 2>/dev/null || printf unknown)
RESULT=$(summary_value PHASE1_BIG_VERIFY_RESULT unknown)
STEPS=$(summary_value PHASE1_BIG_VERIFY_STEPS unknown)
FAILED=$(summary_value PHASE1_BIG_VERIFY_FAILED unknown)
START_UTC=$(summary_value PHASE1_BIG_VERIFY_START_UTC unknown)
END_UTC=$(summary_value PHASE1_BIG_VERIFY_END_UTC unknown)
HOST_NAME=$(hostname 2>/dev/null || printf unknown)
UNAME_TEXT=$(uname -a 2>/dev/null || printf unknown)

TMP_FAILED=$(mktemp)
TMP_TAIL=$(mktemp)
trap 'rm -f "$TMP_FAILED" "$TMP_TAIL"' EXIT INT TERM

grep -n '^FAILED:' "$SOURCE_LOG" > "$TMP_FAILED" 2>/dev/null || true
tail -n "$INCLUDE_LOG_TAIL_LINES" "$SOURCE_LOG" > "$TMP_TAIL"

cat > "$DEST" <<EOF
# Phase1 big verification latest report

Generated UTC: $NOW_UTC
Source branch: $(safe_text "$HEAD_BRANCH")
Source commit: $(safe_text "$HEAD_SHA")
Host: $(safe_text "$HOST_NAME")
Host kernel: $(safe_text "$UNAME_TEXT")

## Result

| Field | Value |
| --- | --- |
| Result | $RESULT |
| Steps | $STEPS |
| Failed steps | $FAILED |
| Verification start UTC | $START_UTC |
| Verification end UTC | $END_UTC |
| Source summary | $SOURCE_SUMMARY |
| Source report | $SOURCE_REPORT |
| Source log | $SOURCE_LOG |

## Failed step summary

\`\`\`text
EOF

if [ -s "$TMP_FAILED" ]; then
  cat "$TMP_FAILED" >> "$DEST"
else
  printf 'No failed steps recorded.\n' >> "$DEST"
fi

cat >> "$DEST" <<EOF
\`\`\`

## Verification report snapshot

EOF

sed -n '1,220p' "$SOURCE_REPORT" >> "$DEST"

cat >> "$DEST" <<EOF

## Log tail

Last $INCLUDE_LOG_TAIL_LINES lines from the local verifier log:

\`\`\`text
EOF

cat "$TMP_TAIL" >> "$DEST"

cat >> "$DEST" <<EOF
\`\`\`

## Non-claims

This uploaded report records repository and local emulator-evidence validation only. It does not claim installer readiness, physical hardware validation, recovery validation, hardening proof, release-candidate readiness, or daily-driver readiness.
EOF

printf 'phase1-big-verify-upload-report: wrote %s\n' "$DEST"
git status --short -- "$DEST"

if [ "$DO_COMMIT" = yes ]; then
  if [ "$ALLOW_DIRTY" != yes ]; then
    dirty=$(git status --short | grep -v "^.. $DEST$" | grep -v "^?? $DEST$" || true)
    if [ -n "$dirty" ]; then
      printf 'phase1-big-verify-upload-report: refusing commit with unrelated tracked changes/untracked files:\n%s\n' "$dirty" >&2
      printf 'Use --allow-dirty only after reviewing those changes.\n' >&2
      exit 1
    fi
  fi

  git add "$DEST"
  if git diff --cached --quiet -- "$DEST"; then
    printf 'phase1-big-verify-upload-report: no report changes to commit\n'
  else
    git commit -m "Upload Phase1 big verification report"
  fi
fi

if [ "$DO_PUSH" = yes ]; then
  git push "$REMOTE" "$BRANCH"
fi

printf 'phase1-big-verify-upload-report: complete\n'
printf 'report: %s\n' "$DEST"
[ "$DO_PUSH" = yes ] && printf 'uploaded: %s %s\n' "$REMOTE" "$BRANCH"

#!/usr/bin/env sh
# Run cargo all-targets and upload a compact diagnostics report.
#
# This is intended for remote inspection from GitHub when the big verifier's
# wrapper gates fail. It uploads only a Markdown summary under docs/reports/.

set -eu

REPORT=${PHASE1_CARGO_DIAG_REPORT:-docs/reports/CARGO_ALL_TARGETS_FAILURE_DIAGNOSTICS.md}
LOG=${PHASE1_CARGO_DIAG_LOG:-build/phase1-big-verify/cargo-test-all-targets.log}
REMOTE=${PHASE1_BIG_VERIFY_REMOTE:-origin}
BRANCH=${PHASE1_BIG_VERIFY_BRANCH:-edge/stable}
DO_PUSH=no
ALLOW_DIRTY=no

usage() {
  cat <<'USAGE'
Phase1 cargo diagnostics uploader

usage:
  sh scripts/phase1-upload-cargo-diagnostics.sh [--push] [--allow-dirty]

options:
  --push          commit and push the diagnostics report
  --allow-dirty   allow unrelated local changes while committing the report
  -h, --help      show this help
USAGE
}

while [ "$#" -gt 0 ]; do
  case "$1" in
    --push) DO_PUSH=yes; shift ;;
    --allow-dirty) ALLOW_DIRTY=yes; shift ;;
    -h|--help) usage; exit 0 ;;
    *) usage >&2; echo "phase1-upload-cargo-diagnostics: unknown option: $1" >&2; exit 1 ;;
  esac
done

[ -d .git ] || { echo "phase1-upload-cargo-diagnostics: run from repository root" >&2; exit 1; }
mkdir -p "$(dirname "$REPORT")" "$(dirname "$LOG")"

set +e
cargo test --all-targets 2>&1 | tee "$LOG"
RESULT=$?
set -e

{
  echo '# Cargo all-targets failure diagnostics'
  echo
  echo "Generated UTC: $(date -u '+%Y-%m-%dT%H:%M:%SZ')"
  echo "Commit: $(git rev-parse HEAD)"
  echo "Result code: $RESULT"
  echo
  echo '## Failure index'
  echo '```text'
  grep -n -E 'failures:|test result: FAILED|error: test failed|panicked|assertion failed|FAILED|missing|unplanned|outdated|Illegal option|compatibility|dirty|generated|root' "$LOG" | tail -320 || true
  echo '```'
  echo
  echo '## Final output'
  echo '```text'
  tail -420 "$LOG"
  echo '```'
  echo
  echo '## Non-claims'
  echo
  echo 'This diagnostics report records repository test output only. It does not claim installer readiness, physical hardware validation, recovery validation, hardening proof, release-candidate readiness, or daily-driver readiness.'
} > "$REPORT"

echo "phase1-upload-cargo-diagnostics: wrote $REPORT"

if [ "$DO_PUSH" = yes ]; then
  if [ "$ALLOW_DIRTY" != yes ]; then
    dirty=$(git status --short | grep -v "^.. $REPORT$" | grep -v "^?? $REPORT$" || true)
    if [ -n "$dirty" ]; then
      echo "phase1-upload-cargo-diagnostics: refusing commit with unrelated changes:" >&2
      echo "$dirty" >&2
      echo "Use --allow-dirty only after reviewing those changes." >&2
      exit 1
    fi
  fi
  git add "$REPORT"
  if git diff --cached --quiet -- "$REPORT"; then
    echo "phase1-upload-cargo-diagnostics: no report changes to commit"
  else
    git commit -m "Upload cargo all-targets failure diagnostics"
  fi
  git push "$REMOTE" "$BRANCH"
fi

exit "$RESULT"

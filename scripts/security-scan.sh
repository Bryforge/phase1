#!/usr/bin/env bash
set -euo pipefail

# Phase1 lightweight local secret scanner.
# This intentionally avoids third-party dependencies so it can run in CI and on
# small developer machines. It catches high-confidence credential patterns and
# blocks accidental commits of private keys, provider tokens, and env secrets.

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

status=0

scan_files() {
  git ls-files \
    ':!:Cargo.lock' \
    ':!:target/**' \
    ':!:phase1.workspace/**' \
    ':!:plugins/*.local.py' \
    ':!:phase1.conf' \
    ':!:phase1.state' \
    ':!:phase1.history' \
    ':!:phase1.log' \
    ':!:phase1.log.1'
}

report_match() {
  local rule="$1"
  local pattern="$2"
  local matches
  matches="$(scan_files | xargs grep -nI -E "$pattern" 2>/dev/null || true)"
  if [[ -n "$matches" ]]; then
    printf 'security-scan: %s\n%s\n' "$rule" "$matches" >&2
    status=1
  fi
}

report_match 'private key material found' '-----BEGIN (RSA |DSA |EC |OPENSSH |PGP )?PRIVATE KEY-----'
report_match 'GitHub token-like value found' 'gh[pousr]_[A-Za-z0-9_]{36,}|github_pat_[A-Za-z0-9_]{20,}'
report_match 'GitLab token-like value found' 'glpat-[A-Za-z0-9_-]{20,}'
report_match 'OpenAI key-like value found' 'sk-[A-Za-z0-9]{32,}'
report_match 'Slack token-like value found' 'xox[baprs]-[A-Za-z0-9-]{20,}'
report_match 'AWS access key-like value found' 'AKIA[0-9A-Z]{16}'
report_match 'hard-coded credential assignment found' '(^|[^A-Za-z0-9_])(password|passwd|secret|token|api[_-]?key|client[_-]?secret)=[^[:space:]"'"'<>]{8,}'

if [[ "$status" -eq 0 ]]; then
  printf 'security-scan: no high-confidence secrets detected\n'
fi

exit "$status"

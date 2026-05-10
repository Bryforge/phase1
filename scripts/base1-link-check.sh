#!/usr/bin/env sh
# Base1 local Markdown link checker.
#
# This script is read-only. It checks local Markdown links in Base1 docs and
# reports missing relative targets. External URLs and pure anchors are skipped.

set -eu

failures=0
checked=0

info() {
  printf 'base1-link-check: %s\n' "$1"
}

warn() {
  printf 'base1-link-check warning: %s\n' "$1" >&2
}

check_target() {
  source_file=$1
  raw_target=$2

  # Drop optional Markdown title text after the path.
  target=$(printf '%s\n' "$raw_target" | sed 's/[[:space:]].*$//')
  # Drop angle brackets sometimes used around URLs/paths.
  target=$(printf '%s\n' "$target" | sed 's/^<//; s/>$//')

  case "$target" in
    ''|'#'*) return 0 ;;
    http://*|https://*|mailto:*|tel:*|ftp://*) return 0 ;;
    '```'*|'`'*) return 0 ;;
  esac

  # Ignore image/data URLs and badge-like inline assets that are external.
  case "$target" in
    data:*|javascript:*) return 0 ;;
  esac

  # Remove anchor suffix for local file existence checks.
  path_part=${target%%#*}
  [ -n "$path_part" ] || return 0

  case "$path_part" in
    /*)
      candidate=".${path_part}"
      ;;
    *)
      candidate="$(dirname "$source_file")/$path_part"
      ;;
  esac

  if [ ! -e "$candidate" ]; then
    warn "missing local link target: $source_file -> $target"
    failures=$((failures + 1))
  fi
}

check_file() {
  file=$1
  [ -f "$file" ] || return 0
  checked=$((checked + 1))

  # Extract simple Markdown inline links: [text](target). This intentionally
  # skips reference-style links for now; those remain covered by explicit
  # integrity checks until a fuller parser is needed.
  sed -n 's/.*]([^)]*)//p' "$file" >/dev/null 2>&1 || true
  sed -n 's/.*](\([^)]*\)).*/\1/p' "$file" |
  while IFS= read -r target; do
    check_target "$file" "$target"
  done
}

collect_files() {
  for file in \
    README.md \
    base1/*.md \
    docs/base1/*.md \
    docs/base1/releases/*.md \
    docs/base1/real-device/*.md \
    docs/base1/real-device/reports/*.md \
    docs/os/BASE1_*.md \
    RELEASE_BASE1_*.md \
    DEVELOPMENT_CHECKPOINT_BASE1_*.md
  do
    [ -f "$file" ] && printf '%s\n' "$file"
  done
}

info 'mode: read-only'
info 'external-links: skipped'
info 'anchors: file-only check'

collect_files | sort -u | while IFS= read -r file; do
  check_file "$file"
done

# The loop above runs in a subshell on many POSIX shells, so rerun a compact
# failure scan that preserves the result in this shell.
failures=0
checked=0
for file in $(collect_files | sort -u); do
  checked=$((checked + 1))
  for target in $(sed -n 's/.*](\([^)]*\)).*/\1/p' "$file" | sed 's/[[:space:]].*$//'); do
    case "$target" in
      ''|'#'*|http://*|https://*|mailto:*|tel:*|ftp://*|data:*|javascript:*)
        continue
        ;;
    esac
    path_part=${target%%#*}
    [ -n "$path_part" ] || continue
    case "$path_part" in
      /*) candidate=".${path_part}" ;;
      *) candidate="$(dirname "$file")/$path_part" ;;
    esac
    if [ ! -e "$candidate" ]; then
      warn "missing local link target: $file -> $target"
      failures=$((failures + 1))
    fi
  done
done

info "files-checked: $checked"
info "missing-targets: $failures"

if [ "$failures" -ne 0 ]; then
  exit 1
fi

info 'link check complete; no host changes were made'

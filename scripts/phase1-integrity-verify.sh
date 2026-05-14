#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'HELP'
phase1-integrity-verify.sh

Read-only SHA-256 integrity report helper.

Usage:
  sh scripts/phase1-integrity-verify.sh --manifest <file>
  sh scripts/phase1-integrity-verify.sh --file <path>

Manifest format:
  <sha256>  <relative-path>

Safety:
  - read-only
  - no repair
  - no deletion
  - no manifest rewrite
  - local files only
HELP
}

if [ "$#" -eq 0 ]; then
  usage
  exit 0
fi

mode=""
target=""
while [ "$#" -gt 0 ]; do
  case "$1" in
    --manifest|--file)
      mode="$1"
      shift
      if [ "$#" -eq 0 ]; then
        echo "error: missing value for $mode" >&2
        exit 2
      fi
      target="$1"
      ;;
    --help|-h)
      usage
      exit 0
      ;;
    *)
      echo "error: unknown argument: $1" >&2
      exit 2
      ;;
  esac
  shift
done

sha_tool=""
if command -v sha256sum >/dev/null 2>&1; then
  sha_tool="sha256sum"
elif command -v shasum >/dev/null 2>&1; then
  sha_tool="shasum -a 256"
else
  echo "error: no SHA-256 tool found" >&2
  exit 3
fi

hash_file() {
  # shellcheck disable=SC2086
  $sha_tool "$1" | awk '{print $1}'
}

case "$mode" in
  --file)
    if [ ! -f "$target" ]; then
      echo "path: $target"
      echo "result: missing"
      exit 1
    fi
    echo "path: $target"
    echo "sha256: $(hash_file "$target")"
    echo "result: ok"
    ;;
  --manifest)
    if [ ! -f "$target" ]; then
      echo "manifest: $target"
      echo "result: manifest-missing"
      exit 1
    fi
    echo "manifest: $target"
    failures=0
    checked=0
    while IFS= read -r line || [ -n "$line" ]; do
      case "$line" in
        ''|'#'*) continue ;;
      esac
      expected=$(printf '%s\n' "$line" | awk '{print $1}')
      path=$(printf '%s\n' "$line" | cut -d' ' -f3-)
      if ! printf '%s' "$expected" | grep -Eq '^[0-9a-fA-F]{64}$'; then
        echo "result: manifest-invalid"
        echo "line: $line"
        exit 2
      fi
      if [ ! -f "$path" ]; then
        echo "missing: $path"
        failures=$((failures + 1))
        continue
      fi
      observed=$(hash_file "$path")
      checked=$((checked + 1))
      if [ "$observed" = "$expected" ]; then
        echo "ok: $path"
      else
        echo "changed: $path"
        failures=$((failures + 1))
      fi
    done < "$target"
    echo "checked: $checked"
    echo "failures: $failures"
    if [ "$failures" -eq 0 ]; then
      echo "result: ok"
    else
      echo "result: changed"
      exit 1
    fi
    ;;
  *)
    usage
    exit 2
    ;;
esac

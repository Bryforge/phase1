#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'USAGE'
Base1 real-device read-only validation bundle

Usage:
  scripts/base1-real-device-readonly-validation-bundle.sh --dry-run --target /dev/<device>

Runs read-only previews only:
  - real-device read-only preview
  - real-device read-only report
  - docs presence checks

Non-claims:
  - not installer-ready
  - not hardware-validated
  - not daily-driver ready
  - no destructive disk writes
  - no real-device write path
USAGE
}

dry_run=0
target=""

while [ "$#" -gt 0 ]; do
  case "$1" in
    --dry-run)
      dry_run=1
      shift
      ;;
    --target)
      target="${2:-}"
      shift 2
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "error: unknown argument: $1" >&2
      usage >&2
      exit 2
      ;;
  esac
done

if [ "$dry_run" -ne 1 ]; then
  echo "error: --dry-run is required" >&2
  exit 2
fi

if [ -z "$target" ]; then
  echo "error: --target /dev/<device> is required" >&2
  exit 2
fi

case "$target" in
  /dev/*) ;;
  *)
    echo "error: target must be under /dev/" >&2
    exit 2
    ;;
esac

echo "Base1 real-device read-only validation bundle"
echo "status: dry-run only"
echo "target: $target"
echo "writes: disabled"
echo "mutation: disabled"
echo "installer: disabled"
echo "hardware-validation-claim: false"
echo "daily-driver-claim: false"
echo

echo "checking docs:"
test -f docs/base1/real-device/READONLY_VALIDATION_PLAN.md
echo "- READONLY_VALIDATION_PLAN.md: present"
test -f docs/base1/real-device/READONLY_REPORT_TEMPLATE.md
echo "- READONLY_REPORT_TEMPLATE.md: present"
echo

echo "running read-only doctor:"
scripts/base1-real-device-readonly-doctor.sh --dry-run | sed -n '1,80p'
echo

echo "running read-only preview:"
scripts/base1-real-device-readonly-preview.sh --dry-run --target "$target" | sed -n '1,40p'
echo

echo "running read-only report generator:"
scripts/base1-real-device-readonly-report.sh --dry-run --target "$target" | sed -n '1,60p'
echo

echo "non-claims:"
echo "- not installer-ready"
echo "- not hardware-validated"
echo "- not daily-driver ready"
echo "- no destructive disk writes"
echo "- no real-device write path"

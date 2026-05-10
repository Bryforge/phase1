#!/usr/bin/env bash
set -eu

usage() {
  echo "usage: scripts/base1-real-device-readonly-doctor.sh --dry-run"
}

dry_run=0
for arg in "$@"; do
  case "$arg" in
    --dry-run) dry_run=1 ;;
    --help|-h) usage; exit 0 ;;
    *) echo "error: unknown argument: $arg" >&2; usage >&2; exit 2 ;;
  esac
done

if [ "$dry_run" -ne 1 ]; then
  echo "error: --dry-run is required" >&2
  usage >&2
  exit 2
fi

echo "Base1 real-device read-only doctor"
echo "status: dry-run only"
echo "scope: documentation, scripts, and local tool availability only"
echo "writes: disabled"
echo "mutation: disabled"
echo "installer: disabled"
echo "hardware-validation-claim: false"
echo "daily-driver-claim: false"
echo

check_file() {
  path="$1"
  if [ -f "$path" ]; then
    echo "- $path: present"
  else
    echo "- $path: missing"
  fi
}

check_executable() {
  path="$1"
  if [ -x "$path" ]; then
    echo "- $path: executable"
  elif [ -f "$path" ]; then
    echo "- $path: present-not-executable"
  else
    echo "- $path: missing"
  fi
}

check_tool() {
  tool="$1"
  if command -v "$tool" >/dev/null 2>&1; then
    echo "- $tool: available"
  else
    echo "- $tool: unavailable"
  fi
}

echo "required docs:"
check_file docs/base1/real-device/README.md
check_file docs/base1/real-device/READONLY_VALIDATION_PLAN.md
check_file docs/base1/real-device/READONLY_REPORT_TEMPLATE.md
check_file docs/base1/real-device/READONLY_VALIDATION_BUNDLE_REPORT.md
check_file docs/base1/real-device/RUNBOOK.md
check_file docs/base1/real-device/CHECKLIST.md
check_file docs/base1/real-device/STATUS_SUMMARY.md
echo

echo "required scripts:"
check_executable scripts/base1-real-device-readonly-preview.sh
check_executable scripts/base1-real-device-readonly-report.sh
check_executable scripts/base1-real-device-readonly-validation-bundle.sh
echo

echo "local tools:"
check_tool git
check_tool cargo
check_tool uname
check_tool sed
check_tool lsblk
check_tool diskutil
echo

echo "non-claims:"
echo "- not installer-ready"
echo "- not hardware-validated"
echo "- not daily-driver ready"
echo "- no destructive disk writes"
echo "- no real-device write path"

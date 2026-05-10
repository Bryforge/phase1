#!/usr/bin/env bash
set -euo pipefail

target=""
dry_run="false"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --target)
      target="${2:-}"
      shift 2
      ;;
    --dry-run)
      dry_run="true"
      shift
      ;;
    -h|--help)
      cat <<'HELP'
Base1 real-device read-only preview

Usage:
  scripts/base1-real-device-readonly-preview.sh --target /dev/<device> --dry-run

Guardrails:
  --dry-run is required
  target must be a /dev/ path
  no disk writes
  no partitioning
  no formatting
  no mounting
  no firmware writes
  no installer execution
HELP
      exit 0
      ;;
    *)
      echo "unknown argument: $1" >&2
      exit 2
      ;;
  esac
done

if [[ "$dry_run" != "true" ]]; then
  echo "--dry-run is required" >&2
  exit 2
fi

if [[ -z "$target" ]]; then
  echo "--target /dev/<device> is required" >&2
  exit 2
fi

if [[ "$target" != /dev/* ]]; then
  echo "target must be a /dev/ path" >&2
  exit 2
fi

cat <<REPORT
Base1 real-device read-only preview
status: dry-run only
target: $target
writes: disabled
mutation: disabled
installer: disabled
partitioning: disabled
formatting: disabled
mounting: disabled
firmware-writes: disabled
hardware-validation-claim: false
daily-driver-claim: false

Non-claims:
- not installer-ready
- not hardware-validated
- not daily-driver ready
- no destructive disk writes
- no real-device write path
REPORT

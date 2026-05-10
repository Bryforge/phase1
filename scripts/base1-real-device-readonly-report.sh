#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'USAGE'
Base1 real-device read-only report generator

Usage:
  scripts/base1-real-device-readonly-report.sh --dry-run --target /dev/<device>

Rules:
  - prints a report to stdout only
  - performs no disk writes
  - performs no partitioning
  - performs no formatting
  - performs no mounting
  - performs no installer action
  - performs no firmware action
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

collection_date="$(date -u +"%Y-%m-%dT%H:%M:%SZ")"
host_platform="$(uname -a 2>/dev/null || echo unknown)"

cat <<REPORT
# Base1 Real-Device Read-Only Validation Report

Status: draft-read-only
Date: $collection_date
Scope: read-only real-device evidence capture

## Target Identity

- Operator:
- Device path: $target
- Device model:
- Device serial:
- Device size:
- Transport:
- Host platform: $host_platform
- Collection date: $collection_date

## Evidence Source

- scripts/base1-real-device-readonly-report.sh --dry-run --target $target
- scripts/base1-real-device-readonly-preview.sh --dry-run --target $target
- read-only lsblk identity output when available
- read-only diskutil info identity output when available
- operator-entered boot environment notes
- QEMU evidence references

## Read-Only Observations

- Boot environment:
- Firmware/platform notes:
- Storage layout notes:
- Device identity notes:
- QEMU evidence reference:

## Guardrails Confirmed

- No disk writes
- No partitioning
- No formatting
- No installer execution
- No firmware flashing
- No bootloader installation
- No automatic target selection
- No destructive repair commands

## Result Label

- needs-follow-up

## Promotion Rule

This report may only promote read-only real-device observation evidence.

It does not promote Phase1 to installer-ready, hardware-validated, or daily-driver status.

## Non-Claims

- Not installer-ready
- Not hardware-validated
- Not daily-driver ready
- No destructive disk writes
- No real-device write path
REPORT

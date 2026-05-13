#!/usr/bin/env sh
# Base1 B3 X200 evidence report uploader.
#
# Collects local B3 emulator evidence summaries from build/, writes a sanitized
# repository-safe report under docs/os/, optionally commits it, and optionally
# pushes it to edge/stable.
#
# Safe default:
#   sh scripts/base1-b3-x200-upload-report.sh
#
# Upload after review:
#   sh scripts/base1-b3-x200-upload-report.sh --push
#
# Refresh local evidence first, then upload:
#   sh scripts/base1-b3-x200-upload-report.sh --refresh --push
#
# Boundaries:
#   - does not install Base1;
#   - does not write disks or host boot settings;
#   - does not upload raw build logs by default;
#   - does not claim physical hardware validation, recovery validation,
#     hardening, release-candidate readiness, or daily-driver readiness.

set -eu

REPORT=${BASE1_B3_X200_REPORT:-docs/os/B3_X200_EMULATOR_EVIDENCE_REPORT.md}
REMOTE=${BASE1_B3_X200_REMOTE:-origin}
BRANCH=${BASE1_B3_X200_BRANCH:-edge/stable}
TIMEOUT_SECONDS=${BASE1_B3_TIMEOUT:-60}
REFRESH=no
COMMIT=no
PUSH=no

VM_SCAFFOLD=${BASE1_B3_VM_SCAFFOLD:-build/base1-b3-vm-validation/b3-validation-scaffold.env}
UEFI_SUMMARY=${BASE1_B3_UEFI_SUMMARY:-build/base1-b3-uefi-proof/reports/b3-summary.env}
UEFI_LOG=${BASE1_B3_UEFI_LOG:-build/base1-b3-uefi-proof/reports/b3-serial.log}
HANDOFF_SUMMARY=${BASE1_B3_HANDOFF_SUMMARY:-build/base1-b3-kernel-handoff/reports/qemu-boot-summary.env}
HANDOFF_LOG=${BASE1_B3_HANDOFF_LOG:-build/base1-b3-kernel-handoff/reports/qemu-boot.log}
GNULINUX_SUMMARY=${BASE1_B3_GNULINUX_SUMMARY:-build/base1-b3-gnulinux-stage/reports/qemu-boot-summary.env}
GNULINUX_LOG=${BASE1_B3_GNULINUX_LOG:-build/base1-b3-gnulinux-stage/reports/qemu-boot.log}

usage() {
  cat <<'USAGE'
base1 B3 X200 evidence report uploader

usage:
  sh scripts/base1-b3-x200-upload-report.sh [options]

options:
  --refresh          rerun local X200 B3 checks before writing the report
  --timeout <sec>    timeout used with --refresh, default: 60
  --report <path>    report path, default: docs/os/B3_X200_EMULATOR_EVIDENCE_REPORT.md
  --commit           commit the report if changed
  --push             commit and push the report to origin edge/stable
  --remote <name>    git remote for --push, default: origin
  --branch <name>    git branch for --push, default: edge/stable
  -h, --help         show this help

examples:
  sh scripts/base1-b3-x200-upload-report.sh
  sh scripts/base1-b3-x200-upload-report.sh --commit
  sh scripts/base1-b3-x200-upload-report.sh --push
  sh scripts/base1-b3-x200-upload-report.sh --refresh --push

non-claims:
  The uploaded report records emulator evidence only. It does not claim Base1 is
  installer-ready, hardware-validated, recovery-complete, hardened,
  release-candidate ready, or daily-driver ready.
USAGE
}

fail() {
  printf 'base1-b3-x200-upload-report: %s\n' "$1" >&2
  exit 1
}

need_file() {
  [ -f "$1" ] || fail "missing required file: $1"
}

present() {
  if [ -f "$1" ]; then
    printf 'yes'
  else
    printf 'no'
  fi
}

env_value() {
  file=$1
  key=$2
  default=${3:-unknown}
  if [ -f "$file" ]; then
    value=$(grep -E "^${key}=" "$file" 2>/dev/null | tail -n 1 | sed "s/^${key}=//" || true)
    if [ -n "$value" ]; then
      printf '%s' "$value" | tr '\n\r' '  '
      return 0
    fi
  fi
  printf '%s' "$default"
}

safe_text() {
  printf '%s' "$1" | sed 's/`/'"'"'/g'
}

while [ "$#" -gt 0 ]; do
  case "$1" in
    --refresh)
      REFRESH=yes
      shift
      ;;
    --timeout)
      [ "$#" -ge 2 ] || fail '--timeout requires a value'
      TIMEOUT_SECONDS=$2
      shift 2
      ;;
    --report)
      [ "$#" -ge 2 ] || fail '--report requires a value'
      REPORT=$2
      shift 2
      ;;
    --commit)
      COMMIT=yes
      shift
      ;;
    --push)
      COMMIT=yes
      PUSH=yes
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

case "$TIMEOUT_SECONDS" in
  ''|*[!0-9]*) fail '--timeout must be a positive integer' ;;
esac
[ "$TIMEOUT_SECONDS" -gt 0 ] || fail '--timeout must be greater than zero'

case "$REPORT" in
  docs/*) ;;
  *) fail "report path must be under docs/: $REPORT" ;;
esac

[ -d .git ] || fail 'run from the phase1 repository root'
need_file scripts/base1-b3-vm-validate.sh

if [ "$REFRESH" = yes ]; then
  need_file scripts/base1-b3-local-linux-fastpath.sh
  need_file scripts/base1-b3-uefi-proof.sh
  need_file scripts/base1-b3-kernel-handoff.sh

  printf 'base1-b3-x200-upload-report: refreshing GNU/Linux fastpath evidence\n'
  sh scripts/base1-b3-local-linux-fastpath.sh --check --timeout "$TIMEOUT_SECONDS"

  printf 'base1-b3-x200-upload-report: refreshing UEFI proof evidence\n'
  bash scripts/base1-b3-uefi-proof.sh --build --check --timeout "$TIMEOUT_SECONDS"

  printf 'base1-b3-x200-upload-report: refreshing VM validation scaffold\n'
  sh scripts/base1-b3-vm-validate.sh --dry-run --write-report >/dev/null
fi

need_file "$VM_SCAFFOLD"
need_file "$UEFI_SUMMARY"
need_file "$HANDOFF_SUMMARY"
need_file "$GNULINUX_SUMMARY"

mkdir -p "$(dirname "$REPORT")"

NOW_UTC=$(date -u '+%Y-%m-%dT%H:%M:%SZ')
GIT_COMMIT=$(git rev-parse HEAD 2>/dev/null || printf unknown)
GIT_BRANCH=$(git rev-parse --abbrev-ref HEAD 2>/dev/null || printf unknown)
HOST_NAME=$(hostname 2>/dev/null || printf unknown)
UNAME_TEXT=$(uname -a 2>/dev/null || printf unknown)

B3_STATE=$(env_value "$VM_SCAFFOLD" BASE1_B3_EVIDENCE_STATE unknown)
B3_COUNT=$(env_value "$VM_SCAFFOLD" BASE1_B3_EVIDENCE_SUMMARY_COUNT unknown)
B3_CLAIM=$(env_value "$VM_SCAFFOLD" BASE1_B3_VALIDATION_CLAIM not_claimed)
UEFI_PRESENT=$(env_value "$VM_SCAFFOLD" BASE1_B3_UEFI_SUMMARY_PRESENT unknown)
HANDOFF_PRESENT=$(env_value "$VM_SCAFFOLD" BASE1_B3_HANDOFF_SUMMARY_PRESENT unknown)
GNULINUX_PRESENT=$(env_value "$VM_SCAFFOLD" BASE1_B3_GNULINUX_SUMMARY_PRESENT unknown)
OPENBSD_PRESENT=$(env_value "$VM_SCAFFOLD" BASE1_B3_OPENBSD_SUMMARY_PRESENT unknown)

UEFI_RESULT=$(env_value "$UEFI_SUMMARY" BASE1_B3_UEFI_PROOF_RESULT unknown)
UEFI_EXIT=$(env_value "$UEFI_SUMMARY" BASE1_B3_UEFI_PROOF_EXIT_CODE unknown)
UEFI_MARKER=$(env_value "$UEFI_SUMMARY" BASE1_B3_UEFI_PROOF_MARKER unknown)
HANDOFF_RESULT=$(env_value "$HANDOFF_SUMMARY" BASE1_QEMU_BOOT_RESULT unknown)
HANDOFF_EXIT=$(env_value "$HANDOFF_SUMMARY" BASE1_QEMU_BOOT_EXIT_CODE unknown)
HANDOFF_EXPECT=$(env_value "$HANDOFF_SUMMARY" BASE1_QEMU_BOOT_EXPECT unknown)
GNULINUX_RESULT=$(env_value "$GNULINUX_SUMMARY" BASE1_QEMU_BOOT_RESULT unknown)
GNULINUX_EXIT=$(env_value "$GNULINUX_SUMMARY" BASE1_QEMU_BOOT_EXIT_CODE unknown)
GNULINUX_EXPECT=$(env_value "$GNULINUX_SUMMARY" BASE1_QEMU_BOOT_EXPECT unknown)

cat > "$REPORT" <<EOF
# B3 X200 emulator evidence report

Status: local X200 emulator evidence report
Generated UTC: \\`$NOW_UTC\\`
Source branch: \\`$(safe_text "$GIT_BRANCH")\\`
Source commit: \\`$(safe_text "$GIT_COMMIT")\\`
Host: \\`$(safe_text "$HOST_NAME")\\`
Host kernel: \\`$(safe_text "$UNAME_TEXT")\\`

## Summary

This report records local X200 B3 emulator evidence generated from the repository build outputs.

The report is intentionally conservative. It records emulator evidence only and keeps the B3 validation claim as \\`not_claimed\\` until reviewed release-facing validation is complete.

| Evidence item | Present | Result | Exit code | Marker / expectation |
| --- | --- | --- | --- | --- |
| B3 VM scaffold | yes | $B3_STATE | n/a | claim: $B3_CLAIM |
| B3 UEFI proof | $UEFI_PRESENT | $UEFI_RESULT | $UEFI_EXIT | $UEFI_MARKER |
| B3 kernel/initrd handoff | $HANDOFF_PRESENT | $HANDOFF_RESULT | $HANDOFF_EXIT | $HANDOFF_EXPECT |
| B3 GNU/Linux stage | $GNULINUX_PRESENT | $GNULINUX_RESULT | $GNULINUX_EXIT | $GNULINUX_EXPECT |
| B3 OpenBSD stage | $OPENBSD_PRESENT | optional/not used | n/a | optional stage |

Evidence summary count: \\`$B3_COUNT\\`

## Local evidence paths

- VM validation scaffold: \\`$VM_SCAFFOLD\\`
- UEFI proof summary: \\`$UEFI_SUMMARY\\`
- UEFI proof log: \\`$UEFI_LOG\\`
- Kernel/initrd handoff summary: \\`$HANDOFF_SUMMARY\\`
- Kernel/initrd handoff log: \\`$HANDOFF_LOG\\`
- GNU/Linux stage summary: \\`$GNULINUX_SUMMARY\\`
- GNU/Linux stage log: \\`$GNULINUX_LOG\\`

Raw build logs remain under \\`build/\\` and are not committed by this report.

## Interpretation boundary

The evidence above supports this limited statement:

- B3 emulator evidence for GNU/Linux local kernel/initrd staging, UEFI proof, and kernel/initrd handoff is present on the X200 test host.

It does not support these claims:

- Base1 is installed.
- Base1 is hardware-validated.
- Base1 recovery is validated.
- Base1 hardening is proven.
- Base1 is release-candidate ready.
- Base1 is daily-driver ready.
- Phase1/Base1 is a production operating system.

## Next step

Promote a reviewed B3 validation report only after the evidence bundle is reviewed and the non-claim boundaries above remain intact.
EOF

printf 'base1-b3-x200-upload-report: wrote %s\n' "$REPORT"

git status --short -- "$REPORT"

if [ "$COMMIT" = yes ]; then
  git add "$REPORT"
  if git diff --cached --quiet -- "$REPORT"; then
    printf 'base1-b3-x200-upload-report: no report changes to commit\n'
  else
    git commit -m "Add B3 X200 emulator evidence report"
  fi
fi

if [ "$PUSH" = yes ]; then
  git push "$REMOTE" "$BRANCH"
fi

printf 'base1-b3-x200-upload-report: complete\n'
printf 'report: %s\n' "$REPORT"
[ "$PUSH" = yes ] && printf 'uploaded: %s %s\n' "$REMOTE" "$BRANCH"

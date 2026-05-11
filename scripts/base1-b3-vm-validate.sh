#!/usr/bin/env sh
# Base1 B3 VM validation driver.
#
# This script aggregates local B3 evidence paths and prints or writes a validation
# scaffold. It loads the selected Base1 profile from profiles/base1/*.env so B3
# VM validation uses the same profile vocabulary as direct-first and supervisor
# delivery paths. It does not launch QEMU, fetch kernels, install Base1, mutate
# disks, validate hardware, validate recovery, prove hardening, or claim full B3
# completion by itself.

set -eu

MODE=dry-run
PROFILE=${BASE1_B3_PROFILE:-x86_64-vm-validation}
PROFILE_DIR=${BASE1_PROFILE_DIR:-profiles/base1}
REPORT=${BASE1_B3_REPORT:-build/base1-b3-vm-validation/b3-validation-scaffold.env}
UEFI_DIR=${BASE1_B3_UEFI_DIR:-build/base1-b3-uefi-proof}
HANDOFF_DIR=${BASE1_B3_HANDOFF_DIR:-build/base1-b3-kernel-handoff}
GNULINUX_DIR=${BASE1_B3_GNULINUX_DIR:-build/base1-b3-gnulinux-stage}
OPENBSD_DIR=${BASE1_B3_OPENBSD_DIR:-build/base1-b3-openbsd-stage}
EXPECT=${BASE1_B3_MARKER:-phase1 6.0.0 ready}
WRITE_REPORT=no

usage() {
  cat <<'USAGE'
base1 B3 VM validation scaffold

usage:
  sh scripts/base1-b3-vm-validate.sh --dry-run --profile x86_64-vm-validation [options]

options:
  --dry-run              inspect evidence and print a validation scaffold
  --profile <profile>    validation profile, default: x86_64-vm-validation
  --profile-dir <dir>    profile directory, default: profiles/base1
  --report <build/file>  report scaffold path under build/
  --write-report         write the scaffold report under build/
  --uefi-dir <dir>       UEFI proof evidence dir, default: build/base1-b3-uefi-proof
  --handoff-dir <dir>    kernel/initrd handoff evidence dir, default: build/base1-b3-kernel-handoff
  --gnulinux-dir <dir>   GNU/Linux stage evidence dir, default: build/base1-b3-gnulinux-stage
  --openbsd-dir <dir>    OpenBSD stage evidence dir, default: build/base1-b3-openbsd-stage
  --expect <text>        expected marker, default: phase1 6.0.0 ready
  -h, --help             show this help

profile source:
  profiles/base1/x86_64-vm-validation.env
  profiles/base1/x200-supervisor-lite.env
  profiles/base1/workstation-supervisor.env

result model:
  scaffold-only         no claim; report scaffold generated or printed
  evidence-present      one or more expected evidence summaries/logs exist
  evidence-incomplete   required evidence is missing for a B3 validation claim

non-claims:
  This does not make Base1 bootable, installer-ready, recovery-complete,
  hardened, hypervisor-ready, hardware-validated, release-candidate ready, or
  daily-driver ready.
USAGE
}

fail() {
  printf 'base1-b3-vm-validate: %s\n' "$1" >&2
  exit 1
}

require_build_path() {
  case "$1" in
    build/*) return 0 ;;
    *) return 1 ;;
  esac
}

while [ "$#" -gt 0 ]; do
  case "$1" in
    --dry-run)
      MODE=dry-run
      shift
      ;;
    --profile)
      [ "$#" -ge 2 ] || fail '--profile requires a value'
      PROFILE=$2
      shift 2
      ;;
    --profile-dir)
      [ "$#" -ge 2 ] || fail '--profile-dir requires a value'
      PROFILE_DIR=$2
      shift 2
      ;;
    --report)
      [ "$#" -ge 2 ] || fail '--report requires a value'
      REPORT=$2
      shift 2
      ;;
    --write-report)
      WRITE_REPORT=yes
      shift
      ;;
    --uefi-dir)
      [ "$#" -ge 2 ] || fail '--uefi-dir requires a value'
      UEFI_DIR=$2
      shift 2
      ;;
    --handoff-dir)
      [ "$#" -ge 2 ] || fail '--handoff-dir requires a value'
      HANDOFF_DIR=$2
      shift 2
      ;;
    --gnulinux-dir)
      [ "$#" -ge 2 ] || fail '--gnulinux-dir requires a value'
      GNULINUX_DIR=$2
      shift 2
      ;;
    --openbsd-dir)
      [ "$#" -ge 2 ] || fail '--openbsd-dir requires a value'
      OPENBSD_DIR=$2
      shift 2
      ;;
    --expect)
      [ "$#" -ge 2 ] || fail '--expect requires a value'
      EXPECT=$2
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

[ "$MODE" = dry-run ] || fail 'only --dry-run mode is currently supported'
require_build_path "$REPORT" || fail "report path must be under build/: $REPORT"
require_build_path "$UEFI_DIR" || fail "UEFI evidence dir must be under build/: $UEFI_DIR"
require_build_path "$HANDOFF_DIR" || fail "handoff evidence dir must be under build/: $HANDOFF_DIR"
require_build_path "$GNULINUX_DIR" || fail "GNU/Linux evidence dir must be under build/: $GNULINUX_DIR"
require_build_path "$OPENBSD_DIR" || fail "OpenBSD evidence dir must be under build/: $OPENBSD_DIR"

PROFILE_FILE="$PROFILE_DIR/$PROFILE.env"
[ -f "$PROFILE_FILE" ] || fail "profile file not found: $PROFILE_FILE"

# shellcheck disable=SC1090
. "$PROFILE_FILE"

[ "${BASE1_PROFILE_NAME:-}" = "$PROFILE" ] || fail "profile name mismatch in $PROFILE_FILE"
[ "${BASE1_PROFILE_CLAIM:-}" = not_claimed ] || fail "profile must keep BASE1_PROFILE_CLAIM=not_claimed"

case "${BASE1_PROFILE_CLASS:-}" in
  vm-validation|low-resource|workstation) : ;;
  *) fail "unsupported B3 profile class: ${BASE1_PROFILE_CLASS:-unknown}" ;;
esac

case "${BASE1_PROFILE_ALLOWED_DELIVERY_MODES:-}" in
  *supervisor*) : ;;
  *) fail "profile must allow a supervisor delivery mode for B3 validation: $PROFILE" ;;
esac

for required_non_claim in \
  BASE1_PROFILE_NON_CLAIM_BOOTABLE \
  BASE1_PROFILE_NON_CLAIM_INSTALLER \
  BASE1_PROFILE_NON_CLAIM_HARDENED \
  BASE1_PROFILE_NON_CLAIM_HYPERVISOR \
  BASE1_PROFILE_NON_CLAIM_HARDWARE \
  BASE1_PROFILE_NON_CLAIM_DAILY_DRIVER
 do
  value=$(eval "printf '%s' \"\${$required_non_claim:-}\"")
  [ "$value" = 1 ] || fail "profile missing required non-claim: $required_non_claim=1"
done

UEFI_SUMMARY="$UEFI_DIR/reports/b3-summary.env"
UEFI_LOG="$UEFI_DIR/reports/b3-serial.log"
HANDOFF_SUMMARY="$HANDOFF_DIR/reports/qemu-boot-summary.env"
HANDOFF_LOG="$HANDOFF_DIR/reports/qemu-boot.log"
GNULINUX_SUMMARY="$GNULINUX_DIR/reports/qemu-boot-summary.env"
GNULINUX_LOG="$GNULINUX_DIR/reports/qemu-boot.log"
OPENBSD_SUMMARY="$OPENBSD_DIR/reports/openbsd-qemu-summary.env"
OPENBSD_LOG="$OPENBSD_DIR/reports/openbsd-qemu-boot.log"

present_count=0
[ -f "$UEFI_SUMMARY" ] && present_count=$((present_count + 1))
[ -f "$HANDOFF_SUMMARY" ] && present_count=$((present_count + 1))
[ -f "$GNULINUX_SUMMARY" ] && present_count=$((present_count + 1))
[ -f "$OPENBSD_SUMMARY" ] && present_count=$((present_count + 1))

if [ "$present_count" -gt 0 ]; then
  evidence_state=evidence-present
else
  evidence_state=evidence-incomplete
fi

render_report() {
  cat <<EOF
BASE1_B3_VM_VALIDATION_MODE=scaffold-only
BASE1_B3_VM_VALIDATION_PROFILE=$PROFILE
BASE1_B3_VM_VALIDATION_PROFILE_FILE=$PROFILE_FILE
BASE1_B3_VM_VALIDATION_PROFILE_CLASS=${BASE1_PROFILE_CLASS:-}
BASE1_B3_VM_VALIDATION_PROFILE_TARGET_RAM_MB=${BASE1_PROFILE_TARGET_RAM_MB:-}
BASE1_B3_VM_VALIDATION_PROFILE_DEFAULT_MODE=${BASE1_PROFILE_DEFAULT_DELIVERY_MODE:-}
BASE1_B3_VM_VALIDATION_PROFILE_ALLOWED_MODES=${BASE1_PROFILE_ALLOWED_DELIVERY_MODES:-}
BASE1_B3_VM_VALIDATION_PROFILE_MAX_CONCURRENCY=${BASE1_PROFILE_MAX_CONCURRENCY:-}
BASE1_B3_VM_VALIDATION_PROFILE_VM_MEMORY_MB=${BASE1_PROFILE_VM_MEMORY_MB:-}
BASE1_B3_VM_VALIDATION_PROFILE_OPENBSD_MEMORY_MB=${BASE1_PROFILE_OPENBSD_MEMORY_MB:-}
BASE1_B3_VM_VALIDATION_PROFILE_STORAGE_TIER_POLICY=${BASE1_PROFILE_STORAGE_TIER_POLICY:-}
BASE1_B3_EXPECT_MARKER=$EXPECT
BASE1_B3_EVIDENCE_STATE=$evidence_state
BASE1_B3_EVIDENCE_SUMMARY_COUNT=$present_count
BASE1_B3_UEFI_SUMMARY=$UEFI_SUMMARY
BASE1_B3_UEFI_LOG=$UEFI_LOG
BASE1_B3_UEFI_SUMMARY_PRESENT=$([ -f "$UEFI_SUMMARY" ] && printf yes || printf no)
BASE1_B3_HANDOFF_SUMMARY=$HANDOFF_SUMMARY
BASE1_B3_HANDOFF_LOG=$HANDOFF_LOG
BASE1_B3_HANDOFF_SUMMARY_PRESENT=$([ -f "$HANDOFF_SUMMARY" ] && printf yes || printf no)
BASE1_B3_GNULINUX_SUMMARY=$GNULINUX_SUMMARY
BASE1_B3_GNULINUX_LOG=$GNULINUX_LOG
BASE1_B3_GNULINUX_SUMMARY_PRESENT=$([ -f "$GNULINUX_SUMMARY" ] && printf yes || printf no)
BASE1_B3_OPENBSD_SUMMARY=$OPENBSD_SUMMARY
BASE1_B3_OPENBSD_LOG=$OPENBSD_LOG
BASE1_B3_OPENBSD_SUMMARY_PRESENT=$([ -f "$OPENBSD_SUMMARY" ] && printf yes || printf no)
BASE1_B3_VALIDATION_CLAIM=not_claimed
BASE1_B3_NON_CLAIM_BOOTABLE_PHYSICAL=1
BASE1_B3_NON_CLAIM_INSTALLER=1
BASE1_B3_NON_CLAIM_RECOVERY=1
BASE1_B3_NON_CLAIM_HARDENED=1
BASE1_B3_NON_CLAIM_HYPERVISOR=1
BASE1_B3_NON_CLAIM_HARDWARE=1
BASE1_B3_NON_CLAIM_RELEASE_CANDIDATE=1
BASE1_B3_NON_CLAIM_DAILY_DRIVER=1
EOF
}

printf 'BASE1 B3 VM VALIDATION SCAFFOLD\n'
printf 'mode       : %s\n' "$MODE"
printf 'profile    : %s\n' "$PROFILE"
printf 'profile_file: %s\n' "$PROFILE_FILE"
printf 'profile_cls : %s\n' "${BASE1_PROFILE_CLASS:-}"
printf 'report     : %s\n' "$REPORT"
printf 'evidence   : %s\n' "$evidence_state"
printf 'claim      : not_claimed\n'
printf '\n'

render_report

if [ "$WRITE_REPORT" = yes ]; then
  mkdir -p "$(dirname "$REPORT")"
  render_report > "$REPORT"
  printf '\nwritten_report: %s\n' "$REPORT"
fi

printf '\nnext_required_evidence:\n'
printf '  - B2 test suite pass record\n'
printf '  - B3 UEFI proof summary/log\n'
printf '  - B3 kernel/initrd handoff summary/log\n'
printf '  - B3 GNU/Linux stage summary/log when used\n'
printf '  - B3 OpenBSD stage summary/log when used\n'
printf '  - validation report promoted from scaffold to reviewed evidence\n'
printf 'non_claims: no installer; no recovery validation; no hardening; no hypervisor claim; no hardware validation; no daily-driver claim\n'

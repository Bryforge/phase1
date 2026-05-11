#!/usr/bin/env sh
# Base1 supervisor orchestration planner.
#
# This plans the Base1 supervisor/control-plane route without launching kernels,
# installing Base1, mutating disks, or modifying host boot settings.
# It does not modify host boot settings, prove hardening, claim hypervisor
# readiness, validate hardware, or claim daily-driver readiness.

set -eu

MODE=dry-run
PROFILE=${BASE1_SUPERVISOR_PROFILE:-x200-supervisor-lite}
PROFILE_DIR=${BASE1_PROFILE_DIR:-profiles/base1}
OUT_DIR=${BASE1_SUPERVISOR_PLAN_OUT:-build/base1-supervisor-orchestration}
WRITE_REPORT=no

usage() {
  cat <<'USAGE'
base1 supervisor orchestration planner

usage:
  sh scripts/base1-supervisor-orchestration-plan.sh [--dry-run|--prepare] [--profile <name>] [--write-report]

options:
  --dry-run            print supervisor orchestration plan, default
  --prepare            write local supervisor orchestration plan report
  --profile <name>     profile name, default: x200-supervisor-lite
  --profile-dir <dir>  profile directory, default: profiles/base1
  --out <build/dir>    output directory, default: build/base1-supervisor-orchestration
  --write-report       write <out>/supervisor-orchestration-plan.env
  -h, --help           show this help

model:
  direct-first remains first-class.
  supervisor-lite uses one active staged kernel plus Base1 control plane.
  supervisor-concurrent is allowed only where the selected profile permits it.

non-claims:
  This does not boot kernels, launch QEMU, install Base1, mutate disks,
  modify host boot settings, prove hardening, claim hypervisor readiness,
  validate hardware, or claim daily-driver readiness.
USAGE
}

fail() {
  printf 'base1-supervisor-orchestration-plan: %s\n' "$1" >&2
  exit 1
}

require_build_out_dir() {
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
    --prepare)
      MODE=prepare
      WRITE_REPORT=yes
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
    --out)
      [ "$#" -ge 2 ] || fail '--out requires a value'
      OUT_DIR=$2
      shift 2
      ;;
    --write-report)
      WRITE_REPORT=yes
      shift
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

case "$MODE" in
  dry-run|prepare) : ;;
  *) fail "unsupported mode: $MODE" ;;
esac

require_build_out_dir "$OUT_DIR" || fail "output directory must be under build/: $OUT_DIR"

PROFILE_FILE="$PROFILE_DIR/$PROFILE.env"
[ -f "$PROFILE_FILE" ] || fail "profile file not found: $PROFILE_FILE"

# shellcheck disable=SC1090
. "$PROFILE_FILE"

[ "${BASE1_PROFILE_NAME:-}" = "$PROFILE" ] || fail "profile name mismatch in $PROFILE_FILE"
[ "${BASE1_PROFILE_CLAIM:-}" = not_claimed ] || fail "profile must keep BASE1_PROFILE_CLAIM=not_claimed"

case "${BASE1_PROFILE_DEFAULT_DELIVERY_MODE:-}" in
  direct-first|supervisor-lite|supervisor-concurrent) : ;;
  *) fail "unsupported default delivery mode: ${BASE1_PROFILE_DEFAULT_DELIVERY_MODE:-unknown}" ;;
esac

case "${BASE1_PROFILE_ALLOWED_DELIVERY_MODES:-}" in
  *supervisor*) : ;;
  *) fail "profile must allow supervisor route: $PROFILE" ;;
esac

REPORT="$OUT_DIR/supervisor-orchestration-plan.env"

ACTIVE_KERNELS=1
case "${BASE1_PROFILE_DEFAULT_DELIVERY_MODE:-}" in
  supervisor-concurrent)
    ACTIVE_KERNELS=${BASE1_PROFILE_MAX_CONCURRENCY:-1}
    ;;
esac

cat_report() {
  cat <<EOF_REPORT
BASE1_SUPERVISOR_PLAN_MODE=$MODE
BASE1_SUPERVISOR_PROFILE=$PROFILE
BASE1_SUPERVISOR_PROFILE_FILE=$PROFILE_FILE
BASE1_SUPERVISOR_PROFILE_CLASS=${BASE1_PROFILE_CLASS:-}
BASE1_SUPERVISOR_DEFAULT_DELIVERY_MODE=${BASE1_PROFILE_DEFAULT_DELIVERY_MODE:-}
BASE1_SUPERVISOR_ALLOWED_DELIVERY_MODES=${BASE1_PROFILE_ALLOWED_DELIVERY_MODES:-}
BASE1_SUPERVISOR_TARGET_RAM_MB=${BASE1_PROFILE_TARGET_RAM_MB:-}
BASE1_SUPERVISOR_MAX_CONCURRENCY=${BASE1_PROFILE_MAX_CONCURRENCY:-}
BASE1_SUPERVISOR_ACTIVE_KERNELS=$ACTIVE_KERNELS
BASE1_SUPERVISOR_STORAGE_TIER_POLICY=${BASE1_PROFILE_STORAGE_TIER_POLICY:-}
BASE1_SUPERVISOR_CONTROL_PLANE=planned
BASE1_SUPERVISOR_POLICY_BUS=planned
BASE1_SUPERVISOR_EVIDENCE_BUS=planned
BASE1_SUPERVISOR_RECOVERY_HOOKS=planned
BASE1_SUPERVISOR_CLAIM=not_claimed
BASE1_SUPERVISOR_NON_CLAIM_BOOTABLE=1
BASE1_SUPERVISOR_NON_CLAIM_INSTALLER=1
BASE1_SUPERVISOR_NON_CLAIM_RECOVERY=1
BASE1_SUPERVISOR_NON_CLAIM_HARDENED=1
BASE1_SUPERVISOR_NON_CLAIM_HYPERVISOR=1
BASE1_SUPERVISOR_NON_CLAIM_HARDWARE=1
BASE1_SUPERVISOR_NON_CLAIM_DAILY_DRIVER=1
EOF_REPORT
}

printf 'BASE1 SUPERVISOR ORCHESTRATION PLAN\n'
printf 'mode          : %s\n' "$MODE"
printf 'profile       : %s\n' "$PROFILE"
printf 'profile_file  : %s\n' "$PROFILE_FILE"
printf 'profile_class : %s\n' "${BASE1_PROFILE_CLASS:-}"
printf 'default_mode  : %s\n' "${BASE1_PROFILE_DEFAULT_DELIVERY_MODE:-}"
printf 'allowed_modes : %s\n' "${BASE1_PROFILE_ALLOWED_DELIVERY_MODES:-}"
printf 'target_ram_mb : %s\n' "${BASE1_PROFILE_TARGET_RAM_MB:-}"
printf 'active_kernels: %s\n' "$ACTIVE_KERNELS"
printf 'storage_policy: %s\n' "${BASE1_PROFILE_STORAGE_TIER_POLICY:-}"
printf '\n'

printf 'control_plane: planned\n'
printf 'policy_bus: planned\n'
printf 'evidence_bus: planned\n'
printf 'recovery_hooks: planned\n'
printf 'direct_first: preserved\n'
printf 'supervisor_lite: preferred for X200-class systems\n'
printf 'supervisor_concurrent: profile-gated\n'
printf '\n'

cat_report

if [ "$WRITE_REPORT" = yes ]; then
  mkdir -p "$OUT_DIR"
  cat_report > "$REPORT"
  printf '\nwritten_report: %s\n' "$REPORT"
fi

case "$MODE" in
  dry-run) printf '\nresult: dry-run\n' ;;
  prepare) printf '\nresult: prepared\n' ;;
esac

printf 'non_claims: no boot-ready claim; no installer claim; no recovery claim; no hardening proof; no hypervisor claim; no hardware validation; no daily-driver claim\n'

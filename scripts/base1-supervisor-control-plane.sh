#!/usr/bin/env sh
# Base1 supervisor control-plane scaffold.
#
# Non-mutating command dispatcher for the supervisor route.
# It does not boot kernels, launch QEMU, install Base1, mutate disks,
# modify host boot settings, prove hardening, claim hypervisor readiness,
# validate hardware, or claim daily-driver readiness.

set -eu

COMMAND=status
PROFILE=${BASE1_SUPERVISOR_PROFILE:-x200-supervisor-lite}
PROFILE_DIR=${BASE1_PROFILE_DIR:-profiles/base1}
OUT_DIR=${BASE1_SUPERVISOR_CONTROL_OUT:-build/base1-supervisor-control-plane}
WRITE_REPORT=no

fail() {
  printf 'base1-supervisor-control-plane: %s\n' "$1" >&2
  exit 1
}

require_build_out_dir() {
  case "$1" in
    build/*) return 0 ;;
    *) return 1 ;;
  esac
}

usage() {
  cat <<'USAGE'
base1 supervisor control plane

usage:
  sh scripts/base1-supervisor-control-plane.sh <command> [--profile <name>] [--write-report]

commands:
  status
  plan
  stage-artifact
  validate-artifact
  launch-preview
  capture-evidence
  request-recovery
  stop

options:
  --profile <name>     profile name, default: x200-supervisor-lite
  --profile-dir <dir>  profile directory, default: profiles/base1
  --out <build/dir>    output directory, default: build/base1-supervisor-control-plane
  --write-report       write <out>/supervisor-control-plane.env
  -h, --help           show this help

non-claims:
  This is a local control-plane scaffold only. It does not make Base1 bootable,
  installer-ready, recovery-complete, hardened, hypervisor-ready,
  hardware-validated, release-candidate ready, or daily-driver ready.
USAGE
}

if [ "$#" -gt 0 ]; then
  case "$1" in
    status|plan|stage-artifact|validate-artifact|launch-preview|capture-evidence|request-recovery|stop)
      COMMAND=$1
      shift
      ;;
  esac
fi

while [ "$#" -gt 0 ]; do
  case "$1" in
    --profile)
      [ "$#" -ge 2 ] || fail "--profile requires a value"
      PROFILE=$2
      shift 2
      ;;
    --profile-dir)
      [ "$#" -ge 2 ] || fail "--profile-dir requires a value"
      PROFILE_DIR=$2
      shift 2
      ;;
    --out)
      [ "$#" -ge 2 ] || fail "--out requires a value"
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
      fail "unknown option or command: $1"
      ;;
  esac
done

require_build_out_dir "$OUT_DIR" || fail "output directory must be under build/: $OUT_DIR"

PROFILE_FILE="$PROFILE_DIR/$PROFILE.env"
[ -f "$PROFILE_FILE" ] || fail "profile file not found: $PROFILE_FILE"

# shellcheck disable=SC1090
. "$PROFILE_FILE"

[ "${BASE1_PROFILE_NAME:-}" = "$PROFILE" ] || fail "profile name mismatch in $PROFILE_FILE"
[ "${BASE1_PROFILE_CLAIM:-}" = not_claimed ] || fail "profile must keep BASE1_PROFILE_CLAIM=not_claimed"

case "${BASE1_PROFILE_ALLOWED_DELIVERY_MODES:-}" in
  *supervisor*) : ;;
  *) fail "profile must allow supervisor route: $PROFILE" ;;
esac

REPORT="$OUT_DIR/supervisor-control-plane.env"

case "$COMMAND" in
  status) ACTION="report profile and control-plane readiness" ;;
  plan) ACTION="render supervisor plan from selected profile" ;;
  stage-artifact) ACTION="plan artifact staging without mutation" ;;
  validate-artifact) ACTION="plan artifact validation without execution" ;;
  launch-preview) ACTION="plan guarded preview launch only" ;;
  capture-evidence) ACTION="plan evidence capture paths" ;;
  request-recovery) ACTION="plan recovery request path" ;;
  stop) ACTION="plan controlled stop path" ;;
  *) fail "unsupported command: $COMMAND" ;;
esac

cat_report() {
  cat <<EOF_REPORT
BASE1_SUPERVISOR_CONTROL_COMMAND=$COMMAND
BASE1_SUPERVISOR_CONTROL_ACTION=$ACTION
BASE1_SUPERVISOR_CONTROL_PROFILE=$PROFILE
BASE1_SUPERVISOR_CONTROL_PROFILE_FILE=$PROFILE_FILE
BASE1_SUPERVISOR_CONTROL_PROFILE_CLASS=${BASE1_PROFILE_CLASS:-}
BASE1_SUPERVISOR_CONTROL_ALLOWED_MODES=${BASE1_PROFILE_ALLOWED_DELIVERY_MODES:-}
BASE1_SUPERVISOR_CONTROL_TARGET_RAM_MB=${BASE1_PROFILE_TARGET_RAM_MB:-}
BASE1_SUPERVISOR_CONTROL_MAX_CONCURRENCY=${BASE1_PROFILE_MAX_CONCURRENCY:-}
BASE1_SUPERVISOR_CONTROL_STORAGE_TIER_POLICY=${BASE1_PROFILE_STORAGE_TIER_POLICY:-}
BASE1_SUPERVISOR_CONTROL_CLAIM=not_claimed
BASE1_SUPERVISOR_CONTROL_NON_CLAIM_BOOTABLE=1
BASE1_SUPERVISOR_CONTROL_NON_CLAIM_INSTALLER=1
BASE1_SUPERVISOR_CONTROL_NON_CLAIM_RECOVERY=1
BASE1_SUPERVISOR_CONTROL_NON_CLAIM_HARDENED=1
BASE1_SUPERVISOR_CONTROL_NON_CLAIM_HYPERVISOR=1
BASE1_SUPERVISOR_CONTROL_NON_CLAIM_HARDWARE=1
BASE1_SUPERVISOR_CONTROL_NON_CLAIM_DAILY_DRIVER=1
EOF_REPORT
}

printf 'BASE1 SUPERVISOR CONTROL PLANE\n'
printf 'command       : %s\n' "$COMMAND"
printf 'action        : %s\n' "$ACTION"
printf 'profile       : %s\n' "$PROFILE"
printf 'profile_file  : %s\n' "$PROFILE_FILE"
printf 'storage_policy: %s\n' "${BASE1_PROFILE_STORAGE_TIER_POLICY:-}"
printf 'claim         : not_claimed\n'
printf '\n'

cat_report

if [ "$WRITE_REPORT" = yes ]; then
  mkdir -p "$OUT_DIR"
  cat_report > "$REPORT"
  printf '\nwritten_report: %s\n' "$REPORT"
fi

printf '\nresult: planned\n'
printf 'non_claims: no boot-ready claim; no installer claim; no recovery claim; no hardening proof; no hypervisor claim; no hardware validation; no daily-driver claim\n'

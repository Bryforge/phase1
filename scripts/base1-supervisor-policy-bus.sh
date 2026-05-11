#!/usr/bin/env sh
# Base1 supervisor policy bus scaffold.
#
# This evaluates supervisor control-plane requests against the selected Base1
# profile before any future staging, launch-preview, evidence, or recovery work.
# It does not boot kernels, launch QEMU, install Base1, mutate disks, modify host boot settings, prove hardening, claim hypervisor readiness, validate hardware, or claim daily-driver readiness.

set -eu

COMMAND=status
REQUESTED_MODE=
PROFILE=${BASE1_SUPERVISOR_PROFILE:-x200-supervisor-lite}
PROFILE_DIR=${BASE1_PROFILE_DIR:-profiles/base1}
OUT_DIR=${BASE1_SUPERVISOR_POLICY_OUT:-build/base1-supervisor-policy-bus}
WRITE_REPORT=no

fail() {
  printf "base1-supervisor-policy-bus: %s\n" "$1" >&2
  exit 1
}

require_build_out_dir() {
  case "$1" in
    build/*) return 0 ;;
    *) return 1 ;;
  esac
}

usage() {
  printf "%s\n" \
    "base1 supervisor policy bus" \
    "" \
    "usage:" \
    "  sh scripts/base1-supervisor-policy-bus.sh [command] [--profile <name>] [--delivery-mode <mode>] [--write-report]" \
    "" \
    "commands:" \
    "  status" \
    "  plan" \
    "  stage-artifact" \
    "  validate-artifact" \
    "  launch-preview" \
    "  capture-evidence" \
    "  request-recovery" \
    "  stop" \
    "" \
    "options:" \
    "  --command <name>          command to evaluate" \
    "  --delivery-mode <mode>    direct-first, supervisor-lite, or supervisor-concurrent" \
    "  --profile <name>          profile name, default: x200-supervisor-lite" \
    "  --profile-dir <dir>       profile directory, default: profiles/base1" \
    "  --out <build/dir>         output directory, default: build/base1-supervisor-policy-bus" \
    "  --write-report            write <out>/supervisor-policy-bus.env" \
    "  -h, --help                show this help" \
    "" \
    "decisions:" \
    "  allow" \
    "  deny" \
    "  plan-only" \
    "  evidence-required" \
    "  profile-upgrade-required" \
    "" \
    "non-claims:" \
    "  This is a local policy scaffold only. It does not make Base1 bootable," \
    "  installer-ready, recovery-complete, hardened, hypervisor-ready," \
    "  hardware-validated, release-candidate ready, or daily-driver ready."
}

while [ "$#" -gt 0 ]; do
  case "$1" in
    status|plan|stage-artifact|validate-artifact|launch-preview|capture-evidence|request-recovery|stop)
      COMMAND=$1
      shift
      ;;
    --command)
      [ "$#" -ge 2 ] || fail "--command requires a value"
      COMMAND=$2
      shift 2
      ;;
    --delivery-mode)
      [ "$#" -ge 2 ] || fail "--delivery-mode requires a value"
      REQUESTED_MODE=$2
      shift 2
      ;;
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

case "$COMMAND" in
  status|plan|stage-artifact|validate-artifact|launch-preview|capture-evidence|request-recovery|stop) : ;;
  *) fail "unsupported command: $COMMAND" ;;
esac

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

if [ -z "$REQUESTED_MODE" ]; then
  REQUESTED_MODE=${BASE1_PROFILE_DEFAULT_DELIVERY_MODE:-direct-first}
fi

case "$REQUESTED_MODE" in
  direct-first|supervisor-lite|supervisor-concurrent) : ;;
  *) fail "unsupported delivery mode: $REQUESTED_MODE" ;;
esac

MODE_ALLOWED=no
case ",${BASE1_PROFILE_ALLOWED_DELIVERY_MODES:-}," in
  *,"$REQUESTED_MODE",*) MODE_ALLOWED=yes ;;
esac

DECISION=allow
REASON="profile allows requested command and delivery mode"

if [ "$MODE_ALLOWED" = no ]; then
  DECISION=deny
  REASON="profile-upgrade-required: requested delivery mode is not allowed by selected profile"
else
  case "$COMMAND" in
    status|plan)
      DECISION=allow
      REASON="profile allows read-only planning command"
      ;;
    stage-artifact|validate-artifact|capture-evidence|request-recovery|stop)
      DECISION=plan-only
      REASON="command remains scaffolded until implementation evidence exists"
      ;;
    launch-preview)
      DECISION=evidence-required
      REASON="launch-preview requires reviewed B3 evidence before stronger claims"
      ;;
  esac
fi

REPORT="$OUT_DIR/supervisor-policy-bus.env"

cat_report() {
  cat <<EOF_REPORT
BASE1_SUPERVISOR_POLICY_COMMAND=$COMMAND
BASE1_SUPERVISOR_POLICY_REQUESTED_MODE=$REQUESTED_MODE
BASE1_SUPERVISOR_POLICY_DECISION=$DECISION
BASE1_SUPERVISOR_POLICY_REASON=$REASON
BASE1_SUPERVISOR_POLICY_PROFILE=$PROFILE
BASE1_SUPERVISOR_POLICY_PROFILE_FILE=$PROFILE_FILE
BASE1_SUPERVISOR_POLICY_PROFILE_CLASS=${BASE1_PROFILE_CLASS:-}
BASE1_SUPERVISOR_POLICY_ALLOWED_MODES=${BASE1_PROFILE_ALLOWED_DELIVERY_MODES:-}
BASE1_SUPERVISOR_POLICY_TARGET_RAM_MB=${BASE1_PROFILE_TARGET_RAM_MB:-}
BASE1_SUPERVISOR_POLICY_MAX_CONCURRENCY=${BASE1_PROFILE_MAX_CONCURRENCY:-}
BASE1_SUPERVISOR_POLICY_STORAGE_TIER_POLICY=${BASE1_PROFILE_STORAGE_TIER_POLICY:-}
BASE1_SUPERVISOR_POLICY_CLAIM=not_claimed
BASE1_SUPERVISOR_POLICY_NON_CLAIM_BOOTABLE=1
BASE1_SUPERVISOR_POLICY_NON_CLAIM_INSTALLER=1
BASE1_SUPERVISOR_POLICY_NON_CLAIM_RECOVERY=1
BASE1_SUPERVISOR_POLICY_NON_CLAIM_HARDENED=1
BASE1_SUPERVISOR_POLICY_NON_CLAIM_HYPERVISOR=1
BASE1_SUPERVISOR_POLICY_NON_CLAIM_HARDWARE=1
BASE1_SUPERVISOR_POLICY_NON_CLAIM_DAILY_DRIVER=1
EOF_REPORT
}

printf "BASE1 SUPERVISOR POLICY BUS\n"
printf "command       : %s\n" "$COMMAND"
printf "delivery_mode : %s\n" "$REQUESTED_MODE"
printf "decision      : %s\n" "$DECISION"
printf "reason        : %s\n" "$REASON"
printf "profile       : %s\n" "$PROFILE"
printf "profile_file  : %s\n" "$PROFILE_FILE"
printf "storage_policy: %s\n" "${BASE1_PROFILE_STORAGE_TIER_POLICY:-}"
printf "claim         : not_claimed\n"
printf "\n"

cat_report

if [ "$WRITE_REPORT" = yes ]; then
  mkdir -p "$OUT_DIR"
  cat_report > "$REPORT"
  printf "\nwritten_report: %s\n" "$REPORT"
fi

printf "\nresult: planned\n"
printf "non_claims: no boot-ready claim; no installer claim; no recovery claim; no hardening proof; no hypervisor claim; no hardware validation; no daily-driver claim\n"

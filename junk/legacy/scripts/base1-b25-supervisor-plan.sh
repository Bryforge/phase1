#!/usr/bin/env sh
# Phase1 / Base1 B25 supervisor orchestration planner.
#
# Purpose:
#   Produce a local evidence-bound supervisor plan for concurrent Phase1 lanes.
#   This is intended to run in the GNU/Linux-backed Phase1 runtime once B23/B24
#   are visible.
#
# Scope:
#   Planning only. It does not start hypervisors, boot other operating systems,
#   mount internal disks, or fetch network resources.

set -eu

PROFILE="${BASE1_B25_PROFILE:-x200-supervisor-concurrent-lab}"
PROFILE_DIR="${BASE1_PROFILE_DIR:-profiles/base1}"
OUT_DIR="${BASE1_B25_OUT:-build/base1-b25-supervisor-plan}"
REPORT="$OUT_DIR/supervisor-plan.env"

fail() { printf 'base1-b25-supervisor-plan: %s\n' "$1" >&2; exit 1; }

case "$OUT_DIR" in
  build/*|/phase1/*) : ;;
  *) fail "output directory must be under build/ or /phase1/: $OUT_DIR" ;;
esac

PROFILE_FILE="$PROFILE_DIR/$PROFILE.env"
[ -f "$PROFILE_FILE" ] || fail "profile file not found: $PROFILE_FILE"

# shellcheck disable=SC1090
. "$PROFILE_FILE"

[ "${BASE1_PROFILE_NAME:-}" = "$PROFILE" ] || fail "profile name mismatch in $PROFILE_FILE"
[ "${BASE1_PROFILE_CLAIM:-}" = not_claimed ] || fail "profile must keep BASE1_PROFILE_CLAIM=not_claimed"

mkdir -p "$OUT_DIR"

cat > "$REPORT" <<EOF
BASE1_B25_SUPERVISOR_PLAN_PROFILE=$PROFILE
BASE1_B25_SUPERVISOR_PLAN_PROFILE_FILE=$PROFILE_FILE
BASE1_B25_SUPERVISOR_PLAN_MODE=planning
BASE1_B25_SUPERVISOR_PLAN_DEFAULT_DELIVERY_MODE=${BASE1_PROFILE_DEFAULT_DELIVERY_MODE:-}
BASE1_B25_SUPERVISOR_PLAN_ALLOWED_DELIVERY_MODES=${BASE1_PROFILE_ALLOWED_DELIVERY_MODES:-}
BASE1_B25_SUPERVISOR_PLAN_DEFAULT_CONCURRENCY=${BASE1_PROFILE_DEFAULT_CONCURRENCY:-}
BASE1_B25_SUPERVISOR_PLAN_MAX_CONCURRENCY=${BASE1_PROFILE_MAX_CONCURRENCY:-}
BASE1_B25_SUPERVISOR_LANE_1=linux-runtime
BASE1_B25_SUPERVISOR_LANE_2=workspace
BASE1_B25_SUPERVISOR_LANE_3=openbsd-plan
BASE1_B25_SUPERVISOR_LANE_4=crypto-plan
BASE1_B25_SUPERVISOR_LANE_POLICY=plan_only_no_auto_boot
BASE1_B25_SUPERVISOR_EVIDENCE_PATH=$OUT_DIR
BASE1_B25_SUPERVISOR_EXPECTED_RESULT=phase1_supervisor_plan_seen
BASE1_B25_SUPERVISOR_CLAIM=not_claimed
BASE1_B25_NON_CLAIM_HYPERVISOR_READY=1
BASE1_B25_NON_CLAIM_MULTI_OS_BOOTED=1
BASE1_B25_NON_CLAIM_INSTALLER=1
BASE1_B25_NON_CLAIM_INTERNAL_DISK_WRITE=1
BASE1_B25_NON_CLAIM_DAILY_DRIVER=1
EOF

printf 'phase1 B25 supervisor plan\n\n'
printf 'profile: %s\n' "$PROFILE"
printf 'report : %s\n\n' "$REPORT"
cat "$REPORT"

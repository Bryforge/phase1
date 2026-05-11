#!/usr/bin/env sh
# Base1 dual-path delivery mode planner.
#
# This chooses between direct first-kernel delivery and supervisor orchestration
# delivery without fragmenting Base1. It loads shared profile contracts from
# profiles/base1/*.env so direct-first, supervisor-lite, concurrent supervisor,
# and workstation supervisor modes use one profile vocabulary. It creates only
# build-directory planning artifacts. It does not boot kernels, launch QEMU,
# install Base1, mutate disks, modify host boot settings, validate hardware,
# prove hardening, or claim daily-driver readiness.

set -eu

MODE=dry-run
DELIVERY_MODE=${BASE1_DELIVERY_MODE:-}
PROFILE=${BASE1_DELIVERY_PROFILE:-x200-supervisor-lite}
PROFILE_DIR=${BASE1_PROFILE_DIR:-profiles/base1}
OUT_DIR=${BASE1_DELIVERY_OUT:-build/base1-delivery-mode}
WRITE_REPORT=no

usage() {
  cat <<'USAGE'
base1 dual-path delivery mode planner

usage:
  sh scripts/base1-delivery-mode-plan.sh [--dry-run|--prepare] [--mode <name>] [--profile <name>] [--write-report]

options:
  --dry-run             print the dual-path delivery plan, default
  --prepare             create a build-only delivery-mode report
  --mode <name>         direct-first, supervisor-lite, supervisor-concurrent, or workstation-supervisor
  --profile <name>      x200-supervisor-lite, x86_64-vm-validation, or workstation-supervisor
  --profile-dir <dir>   profile directory, default: profiles/base1
  --out <build/dir>     output directory, default: build/base1-delivery-mode
  --write-report        write <out>/delivery-mode-plan.env
  -h, --help            show this help

delivery modes:
  direct-first            fastest first-kernel/single-kernel route
  supervisor-lite         one active staged kernel plus Base1 control plane
  supervisor-concurrent   multiple staged kernels under orchestration
  workstation-supervisor  larger-memory workflow and parallel validation profile

profile source:
  profiles/base1/x200-supervisor-lite.env
  profiles/base1/x86_64-vm-validation.env
  profiles/base1/workstation-supervisor.env

shared contract:
  Both paths must share profile names, policy vocabulary, boot artifact IDs,
  log paths, storage-tier assumptions, evidence states, and non-claims.

non-claims:
  This planner does not make Base1 bootable, installer-ready, recovery-complete,
  hardened, hypervisor-ready, hardware-validated, release-candidate ready, or
  daily-driver ready.
USAGE
}

fail() {
  printf 'base1-delivery-mode-plan: %s\n' "$1" >&2
  exit 1
}

require_build_out_dir() {
  case "$1" in
    build/*) return 0 ;;
    *) return 1 ;;
  esac
}

csv_contains() {
  list=$1
  value=$2
  case ",$list," in
    *",$value,"*) return 0 ;;
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
    --mode)
      [ "$#" -ge 2 ] || fail '--mode requires a value'
      DELIVERY_MODE=$2
      shift 2
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

require_build_out_dir "$OUT_DIR" || fail "output directory must be under build/: $OUT_DIR"

PROFILE_FILE="$PROFILE_DIR/$PROFILE.env"
[ -f "$PROFILE_FILE" ] || fail "profile file not found: $PROFILE_FILE"

# shellcheck disable=SC1090
. "$PROFILE_FILE"

[ "${BASE1_PROFILE_NAME:-}" = "$PROFILE" ] || fail "profile name mismatch in $PROFILE_FILE"
[ "${BASE1_PROFILE_CLAIM:-}" = not_claimed ] || fail "profile must keep BASE1_PROFILE_CLAIM=not_claimed"

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

if [ -z "$DELIVERY_MODE" ]; then
  DELIVERY_MODE=${BASE1_PROFILE_DEFAULT_DELIVERY_MODE:-direct-first}
fi

csv_contains "${BASE1_PROFILE_ALLOWED_DELIVERY_MODES:-}" "$DELIVERY_MODE" || \
  fail "delivery mode $DELIVERY_MODE is not allowed by profile $PROFILE"

case "$DELIVERY_MODE" in
  direct-first)
    MODE_FAMILY=direct
    MODE_INTENT="minimal first-kernel delivery route"
    SELECTED_CONCURRENCY=${BASE1_PROFILE_DEFAULT_CONCURRENCY:-1}
    SELECTED_PLAN="keep the shortest boot route; one explicit boot artifact; fastest validation loop"
    ;;
  supervisor-lite)
    MODE_FAMILY=supervisor
    MODE_INTENT="one active staged kernel plus Base1 control plane"
    SELECTED_CONCURRENCY=${BASE1_PROFILE_DEFAULT_CONCURRENCY:-1}
    SELECTED_PLAN="one active staged kernel plus Base1 control plane; low overhead; X200-friendly"
    ;;
  supervisor-concurrent)
    MODE_FAMILY=supervisor
    MODE_INTENT="multiple staged kernels under Base1 orchestration"
    SELECTED_CONCURRENCY=${BASE1_PROFILE_MAX_CONCURRENCY:-1}
    SELECTED_PLAN="concurrent staged-kernel evidence; higher memory; VM evidence only until reviewed"
    ;;
  workstation-supervisor)
    MODE_FAMILY=supervisor
    MODE_INTENT="larger-memory workflow and parallel validation"
    SELECTED_CONCURRENCY=${BASE1_PROFILE_MAX_CONCURRENCY:-1}
    SELECTED_PLAN="broad parallel workflow; highest resource use; still evidence-bound"
    ;;
  *)
    fail "unknown delivery mode: $DELIVERY_MODE"
    ;;
esac

case "${BASE1_PROFILE_CLASS:-}" in
  low-resource)
    PROFILE_INTENT="4GB-class low-resource target; prefer direct-first or supervisor-lite"
    ;;
  vm-validation)
    PROFILE_INTENT="deterministic VM evidence profile"
    ;;
  workstation)
    PROFILE_INTENT="larger-memory development and validation target"
    ;;
  *)
    PROFILE_INTENT="profile class ${BASE1_PROFILE_CLASS:-unknown}"
    ;;
esac

PROFILE_MEMORY_POLICY=${BASE1_PROFILE_STORAGE_TIER_POLICY:-unknown}
REPORT="$OUT_DIR/delivery-mode-plan.env"

write_report() {
  mkdir -p "$OUT_DIR"
  cat > "$REPORT" <<EOF
BASE1_DELIVERY_MODE_PLANNER_MODE=$MODE
BASE1_DELIVERY_MODE=$DELIVERY_MODE
BASE1_DELIVERY_MODE_FAMILY=$MODE_FAMILY
BASE1_DELIVERY_MODE_INTENT=$MODE_INTENT
BASE1_DELIVERY_PROFILE=$PROFILE
BASE1_DELIVERY_PROFILE_FILE=$PROFILE_FILE
BASE1_DELIVERY_PROFILE_CLASS=${BASE1_PROFILE_CLASS:-}
BASE1_DELIVERY_PROFILE_TARGET_RAM_MB=${BASE1_PROFILE_TARGET_RAM_MB:-}
BASE1_DELIVERY_PROFILE_INTENT=$PROFILE_INTENT
BASE1_DELIVERY_PROFILE_MEMORY_POLICY=$PROFILE_MEMORY_POLICY
BASE1_DELIVERY_PROFILE_DEFAULT_MODE=${BASE1_PROFILE_DEFAULT_DELIVERY_MODE:-}
BASE1_DELIVERY_PROFILE_ALLOWED_MODES=${BASE1_PROFILE_ALLOWED_DELIVERY_MODES:-}
BASE1_DELIVERY_DEFAULT_CONCURRENCY=$SELECTED_CONCURRENCY
BASE1_DELIVERY_PROFILE_MAX_CONCURRENCY=${BASE1_PROFILE_MAX_CONCURRENCY:-}
BASE1_DELIVERY_PROFILE_TMPFS_MB=${BASE1_PROFILE_TMPFS_MB:-}
BASE1_DELIVERY_PROFILE_ZRAM_MB=${BASE1_PROFILE_ZRAM_MB:-}
BASE1_DELIVERY_PROFILE_SWAP_MB=${BASE1_PROFILE_SWAP_MB:-}
BASE1_DELIVERY_PROFILE_SSD_SCRATCH_MB=${BASE1_PROFILE_SSD_SCRATCH_MB:-}
BASE1_DELIVERY_DIRECT_PATH=enabled
BASE1_DELIVERY_SUPERVISOR_PATH=enabled
BASE1_DELIVERY_SHARED_CONTRACT=profiles,policy,artifacts,logs,evidence,storage,non_claims
BASE1_DELIVERY_CLAIM=not_claimed
BASE1_DELIVERY_NON_CLAIM_BOOTABLE=1
BASE1_DELIVERY_NON_CLAIM_INSTALLER=1
BASE1_DELIVERY_NON_CLAIM_RECOVERY=1
BASE1_DELIVERY_NON_CLAIM_HARDENED=1
BASE1_DELIVERY_NON_CLAIM_HYPERVISOR=1
BASE1_DELIVERY_NON_CLAIM_HARDWARE=1
BASE1_DELIVERY_NON_CLAIM_RELEASE_CANDIDATE=1
BASE1_DELIVERY_NON_CLAIM_DAILY_DRIVER=1
EOF
}

printf 'BASE1 DUAL-PATH DELIVERY MODE PLAN\n'
printf 'mode             : %s\n' "$MODE"
printf 'delivery_mode    : %s\n' "$DELIVERY_MODE"
printf 'mode_family      : %s\n' "$MODE_FAMILY"
printf 'mode_intent      : %s\n' "$MODE_INTENT"
printf 'profile          : %s\n' "$PROFILE"
printf 'profile_file     : %s\n' "$PROFILE_FILE"
printf 'profile_class    : %s\n' "${BASE1_PROFILE_CLASS:-}"
printf 'profile_intent   : %s\n' "$PROFILE_INTENT"
printf 'memory_policy    : %s\n' "$PROFILE_MEMORY_POLICY"
printf 'default_concur   : %s\n' "$SELECTED_CONCURRENCY"
printf 'out              : %s\n' "$OUT_DIR"
printf 'report           : %s\n' "$REPORT"
printf '\n'

printf 'path_direct: enabled; first-kernel/single-kernel delivery remains first-class\n'
printf 'path_supervisor: enabled; orchestration/control-plane delivery remains first-class\n'
printf 'bridge: shared profiles, policy vocabulary, artifact IDs, logs, evidence states, storage tiers, and non-claims\n'
printf '\n'
printf 'selected_plan: %s\n' "$SELECTED_PLAN"
printf 'non_claims: no boot-ready claim; no installer claim; no recovery claim; no hardening proof; no hypervisor claim; no hardware validation; no daily-driver claim\n'

if [ "$MODE" = dry-run ]; then
  if [ "$WRITE_REPORT" = yes ]; then
    write_report
    printf 'written_report: %s\n' "$REPORT"
  fi
  printf 'result: dry-run\n'
  exit 0
fi

[ "$MODE" = prepare ] || fail "internal unsupported mode: $MODE"
write_report
printf 'written_report: %s\n' "$REPORT"
printf 'result: prepared\n'

#!/usr/bin/env sh
# Base1 dual-path delivery mode planner.
#
# This chooses between direct first-kernel delivery and supervisor orchestration
# delivery without fragmenting Base1. It creates only build-directory planning
# artifacts. It does not boot kernels, launch QEMU, install Base1, mutate disks,
# modify host boot settings, validate hardware, prove hardening, or claim daily-
# driver readiness.

set -eu

MODE=dry-run
DELIVERY_MODE=${BASE1_DELIVERY_MODE:-direct-first}
PROFILE=${BASE1_DELIVERY_PROFILE:-x200-supervisor-lite}
OUT_DIR=${BASE1_DELIVERY_OUT:-build/base1-delivery-mode}
WRITE_REPORT=no

usage() {
  cat <<'USAGE'
base1 dual-path delivery mode planner

usage:
  sh scripts/base1-delivery-mode-plan.sh [--dry-run|--prepare] [--mode <name>] [--profile <name>] [--write-report]

options:
  --dry-run          print the dual-path delivery plan, default
  --prepare          create a build-only delivery-mode report
  --mode <name>      direct-first, supervisor-lite, supervisor-concurrent, or workstation-supervisor
  --profile <name>   x200-supervisor-lite, x86_64-vm-validation, or workstation-supervisor
  --out <build/dir>  output directory, default: build/base1-delivery-mode
  --write-report     write <out>/delivery-mode-plan.env
  -h, --help         show this help

delivery modes:
  direct-first            fastest first-kernel/single-kernel route
  supervisor-lite         one active staged kernel plus Base1 control plane
  supervisor-concurrent   multiple staged kernels under orchestration
  workstation-supervisor  larger-memory workflow and parallel validation profile

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

case "$DELIVERY_MODE" in
  direct-first)
    MODE_FAMILY=direct
    MODE_INTENT="minimal first-kernel delivery route"
    DEFAULT_CONCURRENCY=1
    ;;
  supervisor-lite)
    MODE_FAMILY=supervisor
    MODE_INTENT="one active staged kernel plus Base1 control plane"
    DEFAULT_CONCURRENCY=1
    ;;
  supervisor-concurrent)
    MODE_FAMILY=supervisor
    MODE_INTENT="multiple staged kernels under Base1 orchestration"
    DEFAULT_CONCURRENCY=3
    ;;
  workstation-supervisor)
    MODE_FAMILY=supervisor
    MODE_INTENT="larger-memory workflow and parallel validation"
    DEFAULT_CONCURRENCY=3
    ;;
  *)
    fail "unknown delivery mode: $DELIVERY_MODE"
    ;;
esac

case "$PROFILE" in
  x200-supervisor-lite)
    PROFILE_INTENT="4GB-class low-resource target; prefer direct-first or supervisor-lite"
    PROFILE_MEMORY_POLICY="serial/headless; one active staged kernel by default; zram plus SSD scratch"
    ;;
  x86_64-vm-validation)
    PROFILE_INTENT="deterministic VM evidence profile"
    PROFILE_MEMORY_POLICY="serial capture, explicit artifacts, no generalized hardware claim"
    ;;
  workstation-supervisor)
    PROFILE_INTENT="larger-memory development and validation target"
    PROFILE_MEMORY_POLICY="concurrency allowed only when logs and artifacts remain explicit"
    ;;
  *)
    fail "unknown profile: $PROFILE"
    ;;
esac

REPORT="$OUT_DIR/delivery-mode-plan.env"

write_report() {
  mkdir -p "$OUT_DIR"
  cat > "$REPORT" <<EOF
BASE1_DELIVERY_MODE_PLANNER_MODE=$MODE
BASE1_DELIVERY_MODE=$DELIVERY_MODE
BASE1_DELIVERY_MODE_FAMILY=$MODE_FAMILY
BASE1_DELIVERY_MODE_INTENT=$MODE_INTENT
BASE1_DELIVERY_PROFILE=$PROFILE
BASE1_DELIVERY_PROFILE_INTENT=$PROFILE_INTENT
BASE1_DELIVERY_PROFILE_MEMORY_POLICY=$PROFILE_MEMORY_POLICY
BASE1_DELIVERY_DEFAULT_CONCURRENCY=$DEFAULT_CONCURRENCY
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
printf 'profile_intent   : %s\n' "$PROFILE_INTENT"
printf 'memory_policy    : %s\n' "$PROFILE_MEMORY_POLICY"
printf 'default_concur   : %s\n' "$DEFAULT_CONCURRENCY"
printf 'out              : %s\n' "$OUT_DIR"
printf 'report           : %s\n' "$REPORT"
printf '\n'

printf 'path_direct: enabled; first-kernel/single-kernel delivery remains first-class\n'
printf 'path_supervisor: enabled; orchestration/control-plane delivery remains first-class\n'
printf 'bridge: shared profiles, policy vocabulary, artifact IDs, logs, evidence states, storage tiers, and non-claims\n'
printf '\n'

case "$DELIVERY_MODE" in
  direct-first)
    printf 'selected_plan: keep the shortest boot route; one explicit boot artifact; fastest validation loop\n'
    ;;
  supervisor-lite)
    printf 'selected_plan: one active staged kernel plus Base1 control plane; low overhead; X200-friendly\n'
    ;;
  supervisor-concurrent)
    printf 'selected_plan: concurrent staged-kernel evidence; higher memory; VM evidence only until reviewed\n'
    ;;
  workstation-supervisor)
    printf 'selected_plan: broad parallel workflow; highest resource use; still evidence-bound\n'
    ;;
esac

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

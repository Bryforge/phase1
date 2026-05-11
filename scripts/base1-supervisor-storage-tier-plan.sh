#!/usr/bin/env sh
# Base1 supervisor storage-tier planner.
#
# Plans RAM, tmpfs, zram, SSD scratch, swap backstop, and evidence paths from
# the selected Base1 profile. This does not mount filesystems, create swap,
# run swapon, format disks, mutate disks, modify host boot settings, prove hardening, claim hypervisor readiness, validate hardware, or claim daily-driver readiness.

set -eu

MODE=dry-run
PROFILE=${BASE1_SUPERVISOR_PROFILE:-x200-supervisor-lite}
PROFILE_DIR=${BASE1_PROFILE_DIR:-profiles/base1}
OUT_DIR=${BASE1_SUPERVISOR_STORAGE_OUT:-build/base1-supervisor-storage-tier}
WRITE_REPORT=no

fail() {
  printf "base1-supervisor-storage-tier-plan: %s\n" "$1" >&2
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
    "base1 supervisor storage-tier planner" \
    "" \
    "usage:" \
    "  sh scripts/base1-supervisor-storage-tier-plan.sh [--dry-run|--prepare] [--profile <name>] [--write-report]" \
    "" \
    "options:" \
    "  --dry-run            print storage-tier plan, default" \
    "  --prepare            write local storage-tier report" \
    "  --profile <name>     profile name, default: x200-supervisor-lite" \
    "  --profile-dir <dir>  profile directory, default: profiles/base1" \
    "  --out <build/dir>    output directory, default: build/base1-supervisor-storage-tier" \
    "  --write-report       write <out>/supervisor-storage-tier.env" \
    "  -h, --help           show this help" \
    "" \
    "tier order:" \
    "  real RAM, small tmpfs, zram, SSD scratch, swap backstop, persistent evidence logs" \
    "" \
    "non-claims:" \
    "  This is a local storage-tier scaffold only. It does not make Base1 bootable," \
    "  installer-ready, recovery-complete, hardened, hypervisor-ready," \
    "  hardware-validated, release-candidate ready, or daily-driver ready."
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

case "${BASE1_PROFILE_STORAGE_TIER_POLICY:-}" in
  zram-plus-ssd-scratch-swap-backstop|build-directory-scratch|workstation-scratch) : ;;
  *) fail "unsupported storage-tier policy: ${BASE1_PROFILE_STORAGE_TIER_POLICY:-unknown}" ;;
esac

REPORT="$OUT_DIR/supervisor-storage-tier.env"

cat_report() {
  cat <<EOF_REPORT
BASE1_SUPERVISOR_STORAGE_MODE=$MODE
BASE1_SUPERVISOR_STORAGE_PROFILE=$PROFILE
BASE1_SUPERVISOR_STORAGE_PROFILE_FILE=$PROFILE_FILE
BASE1_SUPERVISOR_STORAGE_PROFILE_CLASS=${BASE1_PROFILE_CLASS:-}
BASE1_SUPERVISOR_STORAGE_TARGET_RAM_MB=${BASE1_PROFILE_TARGET_RAM_MB:-}
BASE1_SUPERVISOR_STORAGE_TMPFS_MB=${BASE1_PROFILE_TMPFS_MB:-}
BASE1_SUPERVISOR_STORAGE_ZRAM_MB=${BASE1_PROFILE_ZRAM_MB:-}
BASE1_SUPERVISOR_STORAGE_SWAP_MB=${BASE1_PROFILE_SWAP_MB:-}
BASE1_SUPERVISOR_STORAGE_SSD_SCRATCH_MB=${BASE1_PROFILE_SSD_SCRATCH_MB:-}
BASE1_SUPERVISOR_STORAGE_MAX_CONCURRENCY=${BASE1_PROFILE_MAX_CONCURRENCY:-}
BASE1_SUPERVISOR_STORAGE_POLICY=${BASE1_PROFILE_STORAGE_TIER_POLICY:-}
BASE1_SUPERVISOR_STORAGE_TIER_ORDER=ram,tmpfs,zram,ssd_scratch,swap_backstop,evidence_logs
BASE1_SUPERVISOR_STORAGE_DISK_IS_RAM_EQUIVALENT=no
BASE1_SUPERVISOR_STORAGE_CLAIM=not_claimed
BASE1_SUPERVISOR_STORAGE_NON_CLAIM_BOOTABLE=1
BASE1_SUPERVISOR_STORAGE_NON_CLAIM_INSTALLER=1
BASE1_SUPERVISOR_STORAGE_NON_CLAIM_RECOVERY=1
BASE1_SUPERVISOR_STORAGE_NON_CLAIM_HARDENED=1
BASE1_SUPERVISOR_STORAGE_NON_CLAIM_HYPERVISOR=1
BASE1_SUPERVISOR_STORAGE_NON_CLAIM_HARDWARE=1
BASE1_SUPERVISOR_STORAGE_NON_CLAIM_DAILY_DRIVER=1
EOF_REPORT
}

printf "BASE1 SUPERVISOR STORAGE TIER PLAN\n"
printf "mode          : %s\n" "$MODE"
printf "profile       : %s\n" "$PROFILE"
printf "profile_file  : %s\n" "$PROFILE_FILE"
printf "profile_class : %s\n" "${BASE1_PROFILE_CLASS:-}"
printf "target_ram_mb : %s\n" "${BASE1_PROFILE_TARGET_RAM_MB:-}"
printf "tmpfs_mb      : %s\n" "${BASE1_PROFILE_TMPFS_MB:-}"
printf "zram_mb       : %s\n" "${BASE1_PROFILE_ZRAM_MB:-}"
printf "ssd_scratch_mb: %s\n" "${BASE1_PROFILE_SSD_SCRATCH_MB:-}"
printf "swap_mb       : %s\n" "${BASE1_PROFILE_SWAP_MB:-}"
printf "max_concur    : %s\n" "${BASE1_PROFILE_MAX_CONCURRENCY:-}"
printf "storage_policy: %s\n" "${BASE1_PROFILE_STORAGE_TIER_POLICY:-}"
printf "disk_is_ram   : no\n"
printf "\n"

cat_report

if [ "$WRITE_REPORT" = yes ]; then
  mkdir -p "$OUT_DIR"
  cat_report > "$REPORT"
  printf "\nwritten_report: %s\n" "$REPORT"
fi

case "$MODE" in
  dry-run) printf "\nresult: dry-run\n" ;;
  prepare) printf "\nresult: prepared\n" ;;
esac

printf "non_claims: no boot-ready claim; no installer claim; no recovery claim; no hardening proof; no hypervisor claim; no hardware validation; no daily-driver claim\n"

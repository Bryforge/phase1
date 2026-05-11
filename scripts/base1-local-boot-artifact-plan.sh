#!/usr/bin/env sh
# Base1 local boot artifact planner.
#
# This selects one explicit local boot artifact candidate without writing disks,
# modifying host boot settings, formatting filesystems, installing bootloaders,
# launching hardware boot, proving hardening, validating hardware, or claiming
# daily-driver readiness.

set -eu

MODE=dry-run
PROFILE=${BASE1_LOCAL_BOOT_PROFILE:-x200-supervisor-lite}
PROFILE_DIR=${BASE1_PROFILE_DIR:-profiles/base1}
ARTIFACT=${BASE1_LOCAL_BOOT_ARTIFACT:-build/phase1-uefi.img}
OUT_DIR=${BASE1_LOCAL_BOOT_OUT:-build/base1-local-boot-artifact}
WRITE_REPORT=no

fail() {
  printf "base1-local-boot-artifact-plan: %s\n" "$1" >&2
  exit 1
}

require_build_path() {
  case "$1" in
    build/*) return 0 ;;
    *) return 1 ;;
  esac
}

usage() {
  printf "%s\n" \
    "base1 local boot artifact planner" \
    "" \
    "usage:" \
    "  sh scripts/base1-local-boot-artifact-plan.sh [--dry-run|--prepare] [--profile <name>] [--artifact <build/path>] [--write-report]" \
    "" \
    "options:" \
    "  --dry-run            print local boot artifact plan, default" \
    "  --prepare            write local boot artifact report" \
    "  --profile <name>     profile name, default: x200-supervisor-lite" \
    "  --profile-dir <dir>  profile directory, default: profiles/base1" \
    "  --artifact <path>    local boot artifact path, default: build/phase1-uefi.img" \
    "  --out <build/dir>    output directory, default: build/base1-local-boot-artifact" \
    "  --write-report       write <out>/local-boot-artifact-plan.env" \
    "  -h, --help           show this help" \
    "" \
    "non-claims:" \
    "  This does not make Base1 bootable on hardware, installer-ready," \
    "  recovery-complete, hardened, hypervisor-ready, hardware-validated," \
    "  release-candidate ready, or daily-driver ready."
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
    --artifact)
      [ "$#" -ge 2 ] || fail "--artifact requires a value"
      ARTIFACT=$2
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

require_build_path "$OUT_DIR" || fail "output directory must be under build/: $OUT_DIR"
require_build_path "$ARTIFACT" || fail "artifact path must be under build/: $ARTIFACT"

PROFILE_FILE="$PROFILE_DIR/$PROFILE.env"
[ -f "$PROFILE_FILE" ] || fail "profile file not found: $PROFILE_FILE"

# shellcheck disable=SC1090
. "$PROFILE_FILE"

[ "${BASE1_PROFILE_NAME:-}" = "$PROFILE" ] || fail "profile name mismatch in $PROFILE_FILE"
[ "${BASE1_PROFILE_CLAIM:-}" = not_claimed ] || fail "profile must keep BASE1_PROFILE_CLAIM=not_claimed"

ARTIFACT_PRESENT=no
[ -f "$ARTIFACT" ] && ARTIFACT_PRESENT=yes

REPORT="$OUT_DIR/local-boot-artifact-plan.env"

cat_report() {
  cat <<EOF_REPORT
BASE1_LOCAL_BOOT_ARTIFACT_MODE=$MODE
BASE1_LOCAL_BOOT_ARTIFACT_PROFILE=$PROFILE
BASE1_LOCAL_BOOT_ARTIFACT_PROFILE_FILE=$PROFILE_FILE
BASE1_LOCAL_BOOT_ARTIFACT_PROFILE_CLASS=${BASE1_PROFILE_CLASS:-}
BASE1_LOCAL_BOOT_ARTIFACT_PATH=$ARTIFACT
BASE1_LOCAL_BOOT_ARTIFACT_PRESENT=$ARTIFACT_PRESENT
BASE1_LOCAL_BOOT_ARTIFACT_LOCAL_ONLY=1
BASE1_LOCAL_BOOT_ARTIFACT_RECOVERY_EVIDENCE_REQUIRED=1
BASE1_LOCAL_BOOT_ARTIFACT_CLAIM=not_claimed
BASE1_LOCAL_BOOT_NON_CLAIM_HARDWARE_BOOTED=1
BASE1_LOCAL_BOOT_NON_CLAIM_INSTALLER=1
BASE1_LOCAL_BOOT_NON_CLAIM_RECOVERY_COMPLETE=1
BASE1_LOCAL_BOOT_NON_CLAIM_HARDENED=1
BASE1_LOCAL_BOOT_NON_CLAIM_HARDWARE_VALIDATED=1
BASE1_LOCAL_BOOT_NON_CLAIM_DAILY_DRIVER=1
EOF_REPORT
}

printf "BASE1 LOCAL BOOT ARTIFACT PLAN\n"
printf "mode            : %s\n" "$MODE"
printf "profile         : %s\n" "$PROFILE"
printf "profile_file    : %s\n" "$PROFILE_FILE"
printf "artifact        : %s\n" "$ARTIFACT"
printf "artifact_present: %s\n" "$ARTIFACT_PRESENT"
printf "local_only      : yes\n"
printf "claim           : not_claimed\n"
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

printf "non_claims: no hardware boot claim; no installer claim; no recovery-complete claim; no hardening proof; no hardware validation; no daily-driver claim\n"

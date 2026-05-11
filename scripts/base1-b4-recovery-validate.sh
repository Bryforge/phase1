#!/usr/bin/env sh
# Base1 B4 recovery validation scaffold.
# This plans recovery evidence without launching kernels, installing Base1, writing bootloaders, formatting disks, mounting filesystems, modifying host boot settings, fetching network resources, proving hardening, validating hardware, or claiming daily-driver readiness.

set -eu

MODE=dry-run
PROFILE=${BASE1_B4_PROFILE:-x200-supervisor-lite}
PROFILE_DIR=${BASE1_PROFILE_DIR:-profiles/base1}
OUT_DIR=${BASE1_B4_RECOVERY_OUT:-build/base1-b4-recovery-validation}
WRITE_REPORT=no

fail() {
  printf "base1-b4-recovery-validate: %s\n" "$1" >&2
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
    "base1 B4 recovery validation scaffold" \
    "" \
    "usage:" \
    "  sh scripts/base1-b4-recovery-validate.sh [--dry-run|--prepare] [--profile <name>] [--write-report]" \
    "" \
    "options:" \
    "  --dry-run            print recovery validation plan, default" \
    "  --prepare            write local recovery validation report" \
    "  --profile <name>     profile name, default: x200-supervisor-lite" \
    "  --profile-dir <dir>  profile directory, default: profiles/base1" \
    "  --out <build/dir>    output directory, default: build/base1-b4-recovery-validation" \
    "  --write-report       write <out>/b4-recovery-validation.env" \
    "  -h, --help           show this help" \
    "" \
    "non-claims:" \
    "  This does not make Base1 bootable, installer-ready, recovery-complete," \
    "  hardened, hypervisor-ready, hardware-validated, release-candidate ready," \
    "  or daily-driver ready."
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

REPORT="$OUT_DIR/b4-recovery-validation.env"

cat_report() {
  cat <<EOF_REPORT
BASE1_B4_RECOVERY_MODE=$MODE
BASE1_B4_RECOVERY_PROFILE=$PROFILE
BASE1_B4_RECOVERY_PROFILE_FILE=$PROFILE_FILE
BASE1_B4_RECOVERY_PROFILE_CLASS=${BASE1_PROFILE_CLASS:-}
BASE1_B4_RECOVERY_TARGET_RAM_MB=${BASE1_PROFILE_TARGET_RAM_MB:-}
BASE1_B4_RECOVERY_BOOT_ARTIFACT=planned
BASE1_B4_RECOVERY_ARTIFACT=planned
BASE1_B4_RECOVERY_ROLLBACK_PATH=planned
BASE1_B4_RECOVERY_EMERGENCY_STOP=planned
BASE1_B4_RECOVERY_EVIDENCE_PATH=$OUT_DIR
BASE1_B4_RECOVERY_FAILURE_REASON=operator-visible
BASE1_B4_RECOVERY_DRY_RUN_ONLY=1
BASE1_B4_RECOVERY_CLAIM=not_claimed
BASE1_B4_NON_CLAIM_BOOTABLE=1
BASE1_B4_NON_CLAIM_INSTALLER=1
BASE1_B4_NON_CLAIM_RECOVERY_COMPLETE=1
BASE1_B4_NON_CLAIM_HARDENED=1
BASE1_B4_NON_CLAIM_HYPERVISOR=1
BASE1_B4_NON_CLAIM_HARDWARE=1
BASE1_B4_NON_CLAIM_DAILY_DRIVER=1
EOF_REPORT
}

printf "BASE1 B4 RECOVERY VALIDATION\n"
printf "mode          : %s\n" "$MODE"
printf "profile       : %s\n" "$PROFILE"
printf "profile_file  : %s\n" "$PROFILE_FILE"
printf "boot_artifact : planned\n"
printf "recovery_path : planned\n"
printf "rollback_path : planned\n"
printf "emergency_stop: planned\n"
printf "claim         : not_claimed\n"
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

printf "non_claims: no boot-ready claim; no installer claim; no recovery-complete claim; no hardening proof; no hypervisor claim; no hardware validation; no daily-driver claim\n"

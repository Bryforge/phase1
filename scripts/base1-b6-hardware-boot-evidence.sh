#!/usr/bin/env sh
# Base1 B6 hardware boot evidence capture scaffold.
#
# This records operator-observed hardware boot evidence only. It does not write
# disks, format filesystems, install bootloaders, modify host boot settings,
# launch kernels, fetch network resources, prove hardening, or claim daily-driver readiness.

set -eu

MODE=record
PROFILE=${BASE1_B6_PROFILE:-x200-supervisor-lite}
PROFILE_DIR=${BASE1_PROFILE_DIR:-profiles/base1}
ARTIFACT=${BASE1_B6_ARTIFACT:-build/phase1-uefi.img}
MACHINE=${BASE1_B6_MACHINE:-unknown}
RESULT=${BASE1_B6_RESULT:-not_attempted}
OUT_DIR=${BASE1_B6_OUT:-build/base1-b6-hardware-boot-evidence}
WRITE_REPORT=no

fail() {
  printf "base1-b6-hardware-boot-evidence: %s\n" "$1" >&2
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
    "base1 B6 hardware boot evidence capture" \
    "" \
    "usage:" \
    "  sh scripts/base1-b6-hardware-boot-evidence.sh --record --machine <name> --result <state> [--artifact <path>] [--write-report]" \
    "" \
    "result states:" \
    "  not_attempted" \
    "  boot_menu_seen" \
    "  boot_started" \
    "  phase1_marker_seen" \
    "  blocked" \
    "  failed" \
    "" \
    "non-claims:" \
    "  This records hardware boot evidence only. It does not make Base1 installer-ready," \
    "  recovery-complete, hardened, hypervisor-ready, release-candidate ready, or daily-driver ready."
}

while [ "$#" -gt 0 ]; do
  case "$1" in
    --record)
      MODE=record
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
    --machine)
      [ "$#" -ge 2 ] || fail "--machine requires a value"
      MACHINE=$2
      shift 2
      ;;
    --result)
      [ "$#" -ge 2 ] || fail "--result requires a value"
      RESULT=$2
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
  record) : ;;
  *) fail "unsupported mode: $MODE" ;;
esac

case "$RESULT" in
  not_attempted|boot_menu_seen|boot_started|phase1_marker_seen|blocked|failed) : ;;
  *) fail "unsupported result state: $RESULT" ;;
esac

require_build_out_dir "$OUT_DIR" || fail "output directory must be under build/: $OUT_DIR"

case "$ARTIFACT" in
  build/*) : ;;
  *) fail "artifact path must be under build/: $ARTIFACT" ;;
esac

PROFILE_FILE="$PROFILE_DIR/$PROFILE.env"
[ -f "$PROFILE_FILE" ] || fail "profile file not found: $PROFILE_FILE"

# shellcheck disable=SC1090
. "$PROFILE_FILE"

[ "${BASE1_PROFILE_NAME:-}" = "$PROFILE" ] || fail "profile name mismatch in $PROFILE_FILE"
[ "${BASE1_PROFILE_CLAIM:-}" = not_claimed ] || fail "profile must keep BASE1_PROFILE_CLAIM=not_claimed"

ARTIFACT_PRESENT=no
[ -f "$ARTIFACT" ] && ARTIFACT_PRESENT=yes

REPORT="$OUT_DIR/b6-hardware-boot-evidence.env"

cat_report() {
  cat <<EOF_REPORT
BASE1_B6_HARDWARE_BOOT_MODE=$MODE
BASE1_B6_HARDWARE_BOOT_PROFILE=$PROFILE
BASE1_B6_HARDWARE_BOOT_PROFILE_FILE=$PROFILE_FILE
BASE1_B6_HARDWARE_BOOT_MACHINE=$MACHINE
BASE1_B6_HARDWARE_BOOT_ARTIFACT=$ARTIFACT
BASE1_B6_HARDWARE_BOOT_ARTIFACT_PRESENT=$ARTIFACT_PRESENT
BASE1_B6_HARDWARE_BOOT_RESULT=$RESULT
BASE1_B6_HARDWARE_BOOT_EXPECT_MARKER=phase1 6.0.0 ready
BASE1_B6_HARDWARE_BOOT_RECOVERY_EVIDENCE_REQUIRED=1
BASE1_B6_HARDWARE_BOOT_CLAIM=not_claimed
BASE1_B6_NON_CLAIM_INSTALLER=1
BASE1_B6_NON_CLAIM_RECOVERY_COMPLETE=1
BASE1_B6_NON_CLAIM_HARDENED=1
BASE1_B6_NON_CLAIM_HYPERVISOR=1
BASE1_B6_NON_CLAIM_RELEASE_CANDIDATE=1
BASE1_B6_NON_CLAIM_DAILY_DRIVER=1
EOF_REPORT
}

printf "BASE1 B6 HARDWARE BOOT EVIDENCE\n"
printf "mode            : %s\n" "$MODE"
printf "profile         : %s\n" "$PROFILE"
printf "machine         : %s\n" "$MACHINE"
printf "artifact        : %s\n" "$ARTIFACT"
printf "artifact_present: %s\n" "$ARTIFACT_PRESENT"
printf "result          : %s\n" "$RESULT"
printf "claim           : not_claimed\n"
printf "\n"

cat_report

if [ "$WRITE_REPORT" = yes ]; then
  mkdir -p "$OUT_DIR"
  cat_report > "$REPORT"
  printf "\nwritten_report: %s\n" "$REPORT"
fi

printf "\nresult: recorded\n"
printf "non_claims: no installer claim; no recovery-complete claim; no hardening proof; no hypervisor claim; no release-candidate claim; no daily-driver claim\n"

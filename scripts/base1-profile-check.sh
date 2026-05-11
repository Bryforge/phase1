#!/usr/bin/env sh
# Base1 profile checker.
#
# Validates Base1 profile .env files used by direct-first and supervisor
# delivery paths. This is a local profile-contract checker only. It does not
# boot kernels, launch emulators, configure storage tiers, enable swap, mutate
# disks, modify host boot settings, prove hardening, validate hardware, or claim
# daily-driver readiness.

set -eu

PROFILE=${BASE1_PROFILE:-x200-supervisor-lite}
PROFILE_DIR=${BASE1_PROFILE_DIR:-profiles/base1}
MODE=check
OUT_DIR=${BASE1_PROFILE_CHECK_OUT:-build/base1-profile-check}
WRITE_REPORT=no

usage() {
  cat <<'USAGE'
base1 profile checker

usage:
  sh scripts/base1-profile-check.sh [--profile <name>|--all] [--write-report]

options:
  --profile <name>     profile name, default: x200-supervisor-lite
  --all                validate all known Base1 profiles
  --profile-dir <dir>  profile directory, default: profiles/base1
  --out <build/dir>    output directory, default: build/base1-profile-check
  --write-report       write <out>/profile-check.env
  -h, --help           show this help

known profiles:
  x200-supervisor-lite
  x86_64-vm-validation
  workstation-supervisor

required fields:
  BASE1_PROFILE_NAME
  BASE1_PROFILE_CLASS
  BASE1_PROFILE_TARGET_RAM_MB
  BASE1_PROFILE_DEFAULT_DELIVERY_MODE
  BASE1_PROFILE_ALLOWED_DELIVERY_MODES
  BASE1_PROFILE_DEFAULT_CONCURRENCY
  BASE1_PROFILE_MAX_CONCURRENCY
  BASE1_PROFILE_DISPLAY_POLICY
  BASE1_PROFILE_VM_MEMORY_MB
  BASE1_PROFILE_OPENBSD_MEMORY_MB
  BASE1_PROFILE_UEFI_MEMORY_MB
  BASE1_PROFILE_STORAGE_TIER_POLICY
  BASE1_PROFILE_TMPFS_MB
  BASE1_PROFILE_ZRAM_MB
  BASE1_PROFILE_SWAP_MB
  BASE1_PROFILE_SSD_SCRATCH_MB
  BASE1_PROFILE_SECURITY_POSTURE
  BASE1_PROFILE_CLAIM
  BASE1_PROFILE_NON_CLAIM_BOOTABLE
  BASE1_PROFILE_NON_CLAIM_INSTALLER
  BASE1_PROFILE_NON_CLAIM_HARDENED
  BASE1_PROFILE_NON_CLAIM_HYPERVISOR
  BASE1_PROFILE_NON_CLAIM_HARDWARE
  BASE1_PROFILE_NON_CLAIM_DAILY_DRIVER

non-claims:
  This checker validates profile shape only. It does not make Base1 bootable,
  installer-ready, recovery-complete, hardened, hypervisor-ready, hardware-
  validated, release-candidate ready, or daily-driver ready.
USAGE
}

fail() {
  printf 'base1-profile-check: %s\n' "$1" >&2
  exit 1
}

require_build_out_dir() {
  case "$1" in
    build/*) return 0 ;;
    *) return 1 ;;
  esac
}

KNOWN_PROFILES="x200-supervisor-lite x86_64-vm-validation workstation-supervisor"

while [ "$#" -gt 0 ]; do
  case "$1" in
    --profile)
      [ "$#" -ge 2 ] || fail '--profile requires a value'
      PROFILE=$2
      MODE=check
      shift 2
      ;;
    --all)
      MODE=all
      shift
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

REQUIRED_FIELDS='BASE1_PROFILE_NAME
BASE1_PROFILE_CLASS
BASE1_PROFILE_TARGET_RAM_MB
BASE1_PROFILE_DEFAULT_DELIVERY_MODE
BASE1_PROFILE_ALLOWED_DELIVERY_MODES
BASE1_PROFILE_DEFAULT_CONCURRENCY
BASE1_PROFILE_MAX_CONCURRENCY
BASE1_PROFILE_DISPLAY_POLICY
BASE1_PROFILE_VM_MEMORY_MB
BASE1_PROFILE_OPENBSD_MEMORY_MB
BASE1_PROFILE_UEFI_MEMORY_MB
BASE1_PROFILE_STORAGE_TIER_POLICY
BASE1_PROFILE_TMPFS_MB
BASE1_PROFILE_ZRAM_MB
BASE1_PROFILE_SWAP_MB
BASE1_PROFILE_SSD_SCRATCH_MB
BASE1_PROFILE_SECURITY_POSTURE
BASE1_PROFILE_CLAIM
BASE1_PROFILE_NON_CLAIM_BOOTABLE
BASE1_PROFILE_NON_CLAIM_INSTALLER
BASE1_PROFILE_NON_CLAIM_HARDENED
BASE1_PROFILE_NON_CLAIM_HYPERVISOR
BASE1_PROFILE_NON_CLAIM_HARDWARE
BASE1_PROFILE_NON_CLAIM_DAILY_DRIVER'

positive_field() {
  file=$1
  key=$2
  value=$(grep "^$key=" "$file" | head -n 1 | cut -d= -f2-)
  case "$value" in
    ''|*[!0-9]*) return 1 ;;
    *) [ "$value" -gt 0 ] ;;
  esac
}

validate_profile() {
  name=$1
  file="$PROFILE_DIR/$name.env"
  [ -f "$file" ] || fail "profile not found: $file"

  missing=0
  printf 'profile: %s\n' "$name"
  printf 'file   : %s\n' "$file"

  printf '%s\n' "$REQUIRED_FIELDS" | while IFS= read -r key; do
    if grep "^$key=" "$file" >/dev/null 2>&1; then
      :
    else
      printf 'missing_field: %s\n' "$key"
      printf '%s\n' 1 > "$OUT_DIR/.profile-missing-field"
    fi
  done

  if [ -f "$OUT_DIR/.profile-missing-field" ]; then
    missing=1
  fi

  declared=$(grep '^BASE1_PROFILE_NAME=' "$file" | head -n 1 | cut -d= -f2- || true)
  if [ "$declared" != "$name" ]; then
    printf 'name_mismatch: expected=%s declared=%s\n' "$name" "$declared"
    missing=1
  fi

  for numeric in \
    BASE1_PROFILE_TARGET_RAM_MB \
    BASE1_PROFILE_DEFAULT_CONCURRENCY \
    BASE1_PROFILE_MAX_CONCURRENCY \
    BASE1_PROFILE_VM_MEMORY_MB \
    BASE1_PROFILE_OPENBSD_MEMORY_MB \
    BASE1_PROFILE_UEFI_MEMORY_MB \
    BASE1_PROFILE_TMPFS_MB \
    BASE1_PROFILE_ZRAM_MB \
    BASE1_PROFILE_SWAP_MB \
    BASE1_PROFILE_SSD_SCRATCH_MB
  do
    if ! positive_field "$file" "$numeric"; then
      printf 'invalid_numeric: %s\n' "$numeric"
      missing=1
    fi
  done

  claim=$(grep '^BASE1_PROFILE_CLAIM=' "$file" | head -n 1 | cut -d= -f2- || true)
  if [ "$claim" != not_claimed ]; then
    printf 'invalid_claim: %s\n' "$claim"
    missing=1
  fi

  for nonclaim in \
    BASE1_PROFILE_NON_CLAIM_BOOTABLE \
    BASE1_PROFILE_NON_CLAIM_INSTALLER \
    BASE1_PROFILE_NON_CLAIM_HARDENED \
    BASE1_PROFILE_NON_CLAIM_HYPERVISOR \
    BASE1_PROFILE_NON_CLAIM_HARDWARE \
    BASE1_PROFILE_NON_CLAIM_DAILY_DRIVER
  do
    value=$(grep "^$nonclaim=" "$file" | head -n 1 | cut -d= -f2- || true)
    if [ "$value" != 1 ]; then
      printf 'invalid_nonclaim: %s=%s\n' "$nonclaim" "$value"
      missing=1
    fi
  done

  if [ "$missing" -eq 0 ]; then
    printf 'profile_result: pass\n'
    return 0
  fi

  printf 'profile_result: failed\n'
  return 1
}

mkdir -p "$OUT_DIR"
rm -f "$OUT_DIR/.profile-missing-field"
REPORT="$OUT_DIR/profile-check.env"

printf 'BASE1 PROFILE CHECK\n'
printf 'mode       : %s\n' "$MODE"
printf 'profile_dir: %s\n' "$PROFILE_DIR"
printf 'out        : %s\n' "$OUT_DIR"
printf 'report     : %s\n' "$REPORT"
printf '\n'

result=pass
checked=0
failed=0

case "$MODE" in
  check)
    case " $KNOWN_PROFILES " in
      *" $PROFILE "*) ;;
      *) fail "unknown profile: $PROFILE" ;;
    esac
    if validate_profile "$PROFILE"; then
      checked=1
    else
      checked=1
      failed=1
      result=failed
    fi
    ;;
  all)
    for profile in $KNOWN_PROFILES; do
      if validate_profile "$profile"; then
        :
      else
        failed=$((failed + 1))
        result=failed
      fi
      checked=$((checked + 1))
      printf '\n'
    done
    ;;
  *)
    fail "internal unsupported mode: $MODE"
    ;;
esac

if [ "$WRITE_REPORT" = yes ]; then
  cat > "$REPORT" <<EOF
BASE1_PROFILE_CHECK_MODE=$MODE
BASE1_PROFILE_CHECK_RESULT=$result
BASE1_PROFILE_CHECK_COUNT=$checked
BASE1_PROFILE_CHECK_FAILED=$failed
BASE1_PROFILE_CHECK_DIR=$PROFILE_DIR
BASE1_PROFILE_CHECK_CLAIM=not_claimed
BASE1_PROFILE_CHECK_NON_CLAIM_BOOTABLE=1
BASE1_PROFILE_CHECK_NON_CLAIM_INSTALLER=1
BASE1_PROFILE_CHECK_NON_CLAIM_HARDENED=1
BASE1_PROFILE_CHECK_NON_CLAIM_HYPERVISOR=1
BASE1_PROFILE_CHECK_NON_CLAIM_HARDWARE=1
BASE1_PROFILE_CHECK_NON_CLAIM_DAILY_DRIVER=1
EOF
  printf 'written_report: %s\n' "$REPORT"
fi

printf 'checked_profiles: %s\n' "$checked"
printf 'failed_profiles: %s\n' "$failed"
printf 'result: %s\n' "$result"
printf 'non_claims: profile shape only; no boot-ready claim; no installer claim; no hardening proof; no hypervisor claim; no hardware validation; no daily-driver claim\n'

[ "$result" = pass ] || exit 1

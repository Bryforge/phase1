#!/usr/bin/env sh
# Base1 B3 local GNU/Linux fast path.
#
# One-command helper for Linux hosts such as the X200. It stages the local
# /boot kernel/initrd pair into the B3 emulator-prep handoff pipeline, runs the
# bundle doctor, prints the guarded QEMU handoff plan by default, and refreshes
# the B3 validation scaffold.
#
# Safe default:
#   sh scripts/base1-b3-local-linux-fastpath.sh
#
# Optional QEMU execution:
#   sh scripts/base1-b3-local-linux-fastpath.sh --check
#
# Boundaries:
#   writes only under build/;
#   does not install Base1;
#   does not write disks;
#   does not change host boot settings;
#   does not validate physical hardware;
#   does not prove hardening or daily-driver readiness.

set -eu

BOOT_DIR=${BASE1_B3_LOCAL_BOOT:-/boot}
OUT_DIR=${BASE1_B3_LOCAL_OUT:-build/base1-b3-gnulinux-stage}
MODE=dry-run
TIMEOUT_SECONDS=${BASE1_B3_TIMEOUT:-30}
EXPECT=${BASE1_B3_GNULINUX_MARKER:-Linux version}
BOOT_PROFILE=${BASE1_QEMU_BOOT_PROFILE:-hardened}
EXTRA_APPEND=${BASE1_QEMU_EXTRA_APPEND:-}

usage() {
  cat <<'USAGE'
base1 B3 local Linux fast path

usage:
  sh scripts/base1-b3-local-linux-fastpath.sh [options]

options:
  --boot <dir>         boot directory containing vmlinuz/initrd, default: /boot
  --out <build/dir>    output bundle, default: build/base1-b3-gnulinux-stage
  --prepare            stage the bundle only
  --dry-run            stage bundle and print guarded QEMU handoff plan, default
  --check              stage bundle and run bounded guarded QEMU serial check
  --timeout <seconds>  QEMU check timeout, default: 30
  --expect <text>      expected serial marker, default: Linux version
  --boot-profile <p>   QEMU profile: standard or hardened, default: hardened
  --append <text>      extra kernel command-line text for QEMU checker
  -h, --help           show this help

examples:
  git pull origin edge/stable
  sh scripts/base1-b3-local-linux-fastpath.sh

  sh scripts/base1-b3-local-linux-fastpath.sh --check --timeout 60

non-claims:
  This helper creates emulator-prep evidence only. It does not install Base1,
  write devices, validate physical hardware, validate recovery, prove hardening,
  or make Phase1/Base1 daily-driver ready.
USAGE
}

fail() {
  printf 'base1-b3-local-linux-fastpath: %s\n' "$1" >&2
  exit 1
}

need_file() {
  [ -f "$1" ] || fail "missing required file: $1"
}

while [ "$#" -gt 0 ]; do
  case "$1" in
    --boot)
      [ "$#" -ge 2 ] || fail '--boot requires a value'
      BOOT_DIR=$2
      shift 2
      ;;
    --out)
      [ "$#" -ge 2 ] || fail '--out requires a value'
      OUT_DIR=$2
      shift 2
      ;;
    --prepare)
      MODE=prepare
      shift
      ;;
    --dry-run)
      MODE=dry-run
      shift
      ;;
    --check)
      MODE=check
      shift
      ;;
    --timeout)
      [ "$#" -ge 2 ] || fail '--timeout requires a value'
      TIMEOUT_SECONDS=$2
      shift 2
      ;;
    --expect)
      [ "$#" -ge 2 ] || fail '--expect requires a value'
      EXPECT=$2
      shift 2
      ;;
    --boot-profile)
      [ "$#" -ge 2 ] || fail '--boot-profile requires a value'
      BOOT_PROFILE=$2
      shift 2
      ;;
    --append)
      [ "$#" -ge 2 ] || fail '--append requires a value'
      EXTRA_APPEND=$2
      shift 2
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

case "$OUT_DIR" in
  build/*) ;;
  *) fail "output directory must be under build/: $OUT_DIR" ;;
esac

case "$TIMEOUT_SECONDS" in
  ''|*[!0-9]*) fail '--timeout must be a positive integer' ;;
esac
[ "$TIMEOUT_SECONDS" -gt 0 ] || fail '--timeout must be greater than zero'

case "$BOOT_PROFILE" in
  standard|hardened) ;;
  *) fail "unsupported boot profile: $BOOT_PROFILE" ;;
esac

[ -d .git ] || fail 'run from the phase1 repository root'
[ -d "$BOOT_DIR" ] || fail "boot directory not found: $BOOT_DIR"
need_file scripts/base1-b3-gnulinux-stage.sh
need_file scripts/base1-emulator-doctor.sh
need_file scripts/base1-b3-vm-validate.sh

if ! ls "$BOOT_DIR"/vmlinuz "$BOOT_DIR"/vmlinuz-* "$BOOT_DIR"/bzImage "$BOOT_DIR"/kernel "$BOOT_DIR"/Image >/dev/null 2>&1; then
  fail "no kernel found in $BOOT_DIR; provide --boot <dir> or copy a kernel/initrd pair there"
fi

if ! ls "$BOOT_DIR"/initrd.img "$BOOT_DIR"/initrd.img-* "$BOOT_DIR"/initramfs.img "$BOOT_DIR"/initramfs-* "$BOOT_DIR"/initrd "$BOOT_DIR"/initramfs >/dev/null 2>&1; then
  fail "no initrd/initramfs found in $BOOT_DIR; provide --boot <dir> with matching initrd"
fi

case "$MODE" in
  prepare) MODE_FLAG=--prepare ;;
  dry-run) MODE_FLAG=--dry-run ;;
  check) MODE_FLAG=--check ;;
  *) fail "internal unsupported mode: $MODE" ;;
esac

printf 'BASE1 B3 LOCAL LINUX FAST PATH\n'
printf 'mode        : %s\n' "$MODE"
printf 'boot        : %s\n' "$BOOT_DIR"
printf 'out         : %s\n' "$OUT_DIR"
printf 'expect      : %s\n' "$EXPECT"
printf 'boot_profile: %s\n' "$BOOT_PROFILE"
printf '\n'

if [ -n "$EXTRA_APPEND" ]; then
  sh scripts/base1-b3-gnulinux-stage.sh \
    --boot "$BOOT_DIR" \
    --out "$OUT_DIR" \
    "$MODE_FLAG" \
    --timeout "$TIMEOUT_SECONDS" \
    --expect "$EXPECT" \
    --boot-profile "$BOOT_PROFILE" \
    --append "$EXTRA_APPEND"
else
  sh scripts/base1-b3-gnulinux-stage.sh \
    --boot "$BOOT_DIR" \
    --out "$OUT_DIR" \
    "$MODE_FLAG" \
    --timeout "$TIMEOUT_SECONDS" \
    --expect "$EXPECT" \
    --boot-profile "$BOOT_PROFILE"
fi

printf '\n--- bundle doctor ---\n'
sh scripts/base1-emulator-doctor.sh --bundle "$OUT_DIR"

printf '\n--- B3 validation scaffold ---\n'
sh scripts/base1-b3-vm-validate.sh --dry-run

printf '\nbase1-b3-local-linux-fastpath: complete\n'
printf 'bundle: %s\n' "$OUT_DIR"
printf 'default result: emulator-prep only, no hardware/install/recovery/hardening claim\n'
if [ "$MODE" != check ]; then
  printf 'next optional check: sh scripts/base1-b3-local-linux-fastpath.sh --check --timeout 60\n'
fi

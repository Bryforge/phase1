#!/usr/bin/env sh
# Base1 preview gate.
#
# Guarded handoff for emulator-only Base1 preview bundles.
# Default mode is dry-run. The script checks the bundle, requires kernel/initrd,
# and requires an explicit confirmation phrase before handing off to the bundle's
# QEMU scaffold. It never writes real devices or changes host boot settings.

set -eu

BUNDLE_DIR=${BASE1_EMULATOR_BUNDLE:-build/base1-emulator-preview}
DOCTOR=${BASE1_EMULATOR_DOCTOR:-scripts/base1-emulator-doctor.sh}
MODE=dry-run
CONFIRM=${BASE1_PREVIEW_CONFIRM:-}
CONFIRM_PHRASE='RUN BASE1 EMULATOR PREVIEW'

usage() {
  cat <<'USAGE'
base1 preview gate

usage:
  scripts/base1-preview-gate.sh [options]

options:
  --bundle <dir>      Base1 emulator preview bundle directory
  --dry-run           inspect and print the handoff command without starting QEMU
  --execute           allow handoff to the bundle QEMU scaffold after checks pass
  --confirm <phrase>  required with --execute; phrase: RUN BASE1 EMULATOR PREVIEW
  -h, --help          show this help

requirements:
  bundle directory exists
  bundle doctor passes
  staging/boot/vmlinuz exists
  staging/boot/initrd.img exists
  run-qemu-bundle.sh exists
  explicit confirmation phrase is supplied for --execute

non-claims:
  This gate is emulator-only. It is not an installer, hardware validation,
  recovery completion, released Base1 image, secure OS replacement, or
  daily-driver readiness proof.
USAGE
}

fail() {
  printf 'base1 preview gate: %s\n' "$1" >&2
  exit 1
}

note() {
  printf 'base1 preview gate: %s\n' "$1"
}

check_file() {
  path=$1
  label=$2
  [ -f "$BUNDLE_DIR/$path" ] || fail "$label missing: $BUNDLE_DIR/$path"
}

while [ "$#" -gt 0 ]; do
  case "$1" in
    --bundle)
      [ "$#" -ge 2 ] || fail '--bundle requires a value'
      BUNDLE_DIR=$2
      shift 2
      ;;
    --dry-run)
      MODE=dry-run
      shift
      ;;
    --execute)
      MODE=execute
      shift
      ;;
    --confirm)
      [ "$#" -ge 2 ] || fail '--confirm requires a value'
      CONFIRM=$2
      shift 2
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      fail "unknown option: $1"
      ;;
  esac
done

[ -d "$BUNDLE_DIR" ] || fail "bundle directory does not exist: $BUNDLE_DIR"
[ -f "$DOCTOR" ] || fail "doctor script missing: $DOCTOR"

note "mode: $MODE"
note "bundle: $BUNDLE_DIR"
note "running read-only doctor"

sh "$DOCTOR" --bundle "$BUNDLE_DIR" >/dev/null || fail 'bundle doctor failed'

check_file "staging/boot/vmlinuz" "kernel"
check_file "staging/boot/initrd.img" "initrd"
check_file "run-qemu-bundle.sh" "bundle qemu scaffold"

note "doctor passed and boot inputs are present"
note "handoff: sh $BUNDLE_DIR/run-qemu-bundle.sh"

if [ "$MODE" = "dry-run" ]; then
  note "dry-run complete; QEMU was not started"
  note "non-claim: emulator preview was not executed and no boot validation was performed"
  exit 0
fi

if [ "$CONFIRM" != "$CONFIRM_PHRASE" ]; then
  fail "--execute requires --confirm '$CONFIRM_PHRASE'"
fi

note "explicit confirmation accepted"
note "emulator-only handoff starting"
exec sh "$BUNDLE_DIR/run-qemu-bundle.sh"

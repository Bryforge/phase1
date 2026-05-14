#!/usr/bin/env sh
set -eu

PATCH_PATH="patches/fyr-staged-runtime-stub.patch"
SOURCE_PATH="src/main.rs"
MODE="apply"

case "${1:-}" in
  ""|"apply")
    MODE="apply"
    ;;
  "check"|"--check")
    MODE="check"
    ;;
  "help"|"-h"|"--help")
    echo "usage: sh scripts/apply-fyr-staged-runtime-stub.sh [apply|check]"
    echo
    echo "apply  : verify and apply patches/fyr-staged-runtime-stub.patch"
    echo "check  : verify the patch can apply without changing files"
    exit 0
    ;;
  *)
    echo "error: unknown mode '${1:-}'" >&2
    echo "usage: sh scripts/apply-fyr-staged-runtime-stub.sh [apply|check]" >&2
    exit 2
    ;;
esac

echo "fyr staged runtime stub apply helper"
echo "patch     : ${PATCH_PATH}"
echo "source    : ${SOURCE_PATH}"
echo "mode      : ${MODE}"
echo "boundary  : source patch helper only; no Fyr runtime host/network behavior"
echo

if [ ! -f "${PATCH_PATH}" ]; then
  echo "error: missing ${PATCH_PATH}" >&2
  exit 1
fi

if [ ! -f "${SOURCE_PATH}" ]; then
  echo "error: missing ${SOURCE_PATH}" >&2
  exit 1
fi

if grep -Fq 'Some("staged") => fyr_staged(&args[1..]),' "${SOURCE_PATH}"; then
  echo "status    : already-applied"
  echo "next      : run validation commands"
else
  echo "status    : checking patch"
  git apply --check "${PATCH_PATH}"
  if [ "${MODE}" = "check" ]; then
    echo "status    : check-only-pass"
    echo "next      : run 'sh scripts/apply-fyr-staged-runtime-stub.sh apply' when ready"
  else
    echo "status    : applying patch"
    git apply "${PATCH_PATH}"
    echo "status    : applied"
  fi
fi

echo
echo "required validation:"
echo "  cargo fmt --all -- --check"
echo "  cargo test -p phase1 --test fyr_staged_runtime_patch_contract"
echo "  cargo test -p phase1 --test fyr_staged_runtime_apply_helper"
echo "  cargo test -p phase1 --test fyr_black_arts_runtime_stub"
echo "  cargo test -p phase1 --test fyr_black_arts_unknown_action"
echo "  cargo test --workspace --all-targets"
echo
echo "manual runtime smoke after build:"
echo "  fyr staged"
echo "  fyr staged status"
echo "  fyr staged help"
echo "  fyr staged nonsense"
echo
echo "blocked behavior remains: candidate creation, apply/change behavior, validation execution, promotion, discard, host command execution, network access, live-system writes"

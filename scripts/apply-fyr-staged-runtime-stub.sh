#!/usr/bin/env sh
set -eu

PATCH_PATH="patches/fyr-staged-runtime-stub.patch"
SOURCE_PATH="src/main.rs"

echo "fyr staged runtime stub apply helper"
echo "patch     : ${PATCH_PATH}"
echo "source    : ${SOURCE_PATH}"
echo "mode      : local developer checkout"
echo "boundary  : applies source patch only; no Fyr runtime host/network behavior"
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
  echo "status    : applying patch"
  git apply "${PATCH_PATH}"
fi

echo
echo "required validation:"
echo "  cargo fmt --all -- --check"
echo "  cargo test -p phase1 --test fyr_staged_runtime_patch_contract"
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

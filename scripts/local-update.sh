#!/usr/bin/env sh
set -eu

cd "${PHASE1_REPO:-$HOME/phase1_library/phase1}"

echo "== Phase1 update =="
git switch master
git pull --ff-only origin master
git status --short

case "${1:-quick}" in
  quick)
    cargo test learn --all-targets
    cargo test registry --all-targets
    ;;
  full)
    cargo fmt --all -- --check
    cargo test --all-targets
    sh scripts/phase1-learn-shell-smoke.sh
    ;;
  run)
    rm -f target/debug/phase1 target/release/phase1
    cargo run --bin phase1
    ;;
  clean-run)
    rm -f phase1.learn target/debug/phase1 target/release/phase1
    cargo run --bin phase1
    ;;
  *)
    echo "usage: sh scripts/local-update.sh [quick|full|run|clean-run]"
    exit 2
    ;;
esac

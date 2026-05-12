#!/usr/bin/env sh
# Phase1/Base1 active system router.
# One front door for the focused Phase1/Base1 boot/runtime workflow.

set -eu

CMD="${1:-help}"
shift || true

fail() { printf 'phase1-base1: %s\n' "$1" >&2; exit 1; }
run() { script="$1"; shift; [ -f "$script" ] || fail "missing helper: $script"; sh "$script" "$@"; }
run_bash() { script="$1"; shift; [ -f "$script" ] || fail "missing helper: $script"; bash "$script" "$@"; }

case "$CMD" in
  help|-h|--help)
    cat <<'EOF'
Phase1/Base1 active system router

Commands:
  status
  doctor [/dev/sdb]
  stage-kernel
  qemu-preflight
  qemu-build
  qemu-run
  qemu-vesa
  x200-cycle /dev/sdb YES_WRITE_USB checkpoint-name
  x200-framebuffer-augment /dev/sdb YES_WRITE_USB
  set-framebuffer-default /dev/sdb YES_WRITE_USB
  sync
  checkpoint [name]
  promote <commit-or-range> "message"

Examples:
  sh scripts/phase1-base1.sh status
  sh scripts/phase1-base1.sh stage-kernel
  sh scripts/phase1-base1.sh qemu-preflight
  sh scripts/phase1-base1.sh qemu-build
  sh scripts/phase1-base1.sh qemu-run
  sh scripts/phase1-base1.sh x200-cycle /dev/sdb YES_WRITE_USB b47-x200-framebuffer-card
EOF
    ;;
  status) run scripts/black-phase1-status-report.sh "$@" ;;
  doctor) run scripts/black-phase1-doctor.sh "$@" ;;
  stage-kernel|kernel-stage) run_bash scripts/stage-x200-kernel.sh "$@" ;;
  qemu-preflight) run scripts/b47-qemu-framebuffer-preflight.sh "$@" ;;
  qemu-build) run scripts/b47-qemu-framebuffer-lab.sh build "$@" ;;
  qemu-run) run scripts/b47-qemu-framebuffer-lab.sh run "$@" ;;
  qemu-vesa) run scripts/b47-qemu-framebuffer-vesa.sh "$@" ;;
  x200-cycle|cycle) run scripts/black-phase1-cycle.sh "$@" ;;
  x200-framebuffer-augment|framebuffer-augment) run scripts/x200-b47-framebuffer-boot-augment.sh "$@" ;;
  set-framebuffer-default|framebuffer-default) run scripts/x200-b47-force-framebuffer-default.sh "$@" ;;
  sync) run scripts/black-phase1-sync-from-edge.sh "$@" ;;
  checkpoint) run scripts/black-phase1-checkpoint.sh "$@" ;;
  promote) run scripts/black-phase1-promote-to-edge.sh "$@" ;;
  *) fail "unknown command: $CMD. Run: sh scripts/phase1-base1.sh help" ;;
esac

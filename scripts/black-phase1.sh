#!/usr/bin/env sh
# black-phase1 command router.
#
# One front door for the rapid test branch helpers.
#
# Usage:
#   sh scripts/black-phase1.sh status
#   sh scripts/black-phase1.sh bootstrap [/dev/sdb]
#   sh scripts/black-phase1.sh doctor [/dev/sdb]
#   sh scripts/black-phase1.sh push "message"
#   sh scripts/black-phase1.sh x200-test /dev/sdb YES_WRITE_USB
#   sh scripts/black-phase1.sh cycle /dev/sdb YES_WRITE_USB checkpoint-name
#   sh scripts/black-phase1.sh qemu-preflight
#   sh scripts/black-phase1.sh qemu-framebuffer build
#   sh scripts/black-phase1.sh qemu-framebuffer run
#   sh scripts/black-phase1.sh sync
#   sh scripts/black-phase1.sh promote <commit-or-range> "message"
#   sh scripts/black-phase1.sh help

set -eu

CMD="${1:-help}"
shift || true

fail() { printf 'black-phase1: %s\n' "$1" >&2; exit 1; }
run() { script="$1"; shift; [ -f "$script" ] || fail "missing helper: $script"; sh "$script" "$@"; }

case "$CMD" in
  help|-h|--help)
    cat <<'EOF'
black-phase1 rapid branch router

Commands:
  status
      Print branch status, ahead/behind edge, and recommended next command.

  bootstrap [/dev/sdb]
      Check out/update black-phase1 and run doctor.

  doctor [/dev/sdb]
      Validate tools, branch, binary, and optional USB target safety.

  push "message"
      Mac/local rapid commit + rebase + push to origin/black-phase1.

  x200-test /dev/sdb YES_WRITE_USB
      X200 rapid pull/build/preflight/prepare verified test USB.

  cycle /dev/sdb YES_WRITE_USB checkpoint-name
      Full cycle: status, doctor, checkpoint, then verified X200 test media prep.

  qemu-preflight
      Check whether this host can build/run the B47 QEMU framebuffer renderer lab.

  qemu-framebuffer build|run
      Build or run the B47 QEMU framebuffer renderer lab.

  sync
      Rebase black-phase1 on latest origin/edge/stable and force-with-lease only black-phase1.

  promote <commit-or-range> "message"
      Cherry-pick known-good black-phase1 work into edge/stable.

Examples:
  sh scripts/black-phase1.sh status
  sh scripts/black-phase1.sh bootstrap /dev/sdb
  sh scripts/black-phase1.sh push "Try minimal CJK renderer path"
  sh scripts/black-phase1.sh x200-test /dev/sdb YES_WRITE_USB
  sh scripts/black-phase1.sh cycle /dev/sdb YES_WRITE_USB b45-minimal-cjk-test
  sh scripts/black-phase1.sh qemu-preflight
  sh scripts/black-phase1.sh qemu-framebuffer build
  sh scripts/black-phase1.sh qemu-framebuffer run
EOF
    ;;
  status)
    run scripts/black-phase1-status-report.sh "$@"
    ;;
  bootstrap)
    run scripts/black-phase1-bootstrap.sh "$@"
    ;;
  doctor)
    run scripts/black-phase1-doctor.sh "$@"
    ;;
  push)
    run scripts/black-phase1-mac-push.sh "$@"
    ;;
  x200-test|test)
    run scripts/black-phase1-x200-test.sh "$@"
    ;;
  cycle|full-cycle)
    run scripts/black-phase1-cycle.sh "$@"
    ;;
  qemu-preflight|fb-preflight|framebuffer-preflight)
    run scripts/b47-qemu-framebuffer-preflight.sh "$@"
    ;;
  qemu-framebuffer|fb-lab|framebuffer-lab)
    run scripts/b47-qemu-framebuffer-lab.sh "$@"
    ;;
  sync)
    run scripts/black-phase1-sync-from-edge.sh "$@"
    ;;
  promote)
    run scripts/black-phase1-promote-to-edge.sh "$@"
    ;;
  *)
    fail "unknown command: $CMD. Run: sh scripts/black-phase1.sh help"
    ;;
esac

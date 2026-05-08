#!/usr/bin/env sh
set -eu

ROOT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
TERMINAL="$ROOT_DIR/terminal/bin/phase1-terminal"
START="$ROOT_DIR/start_phase1"

fail() {
    echo "test-phase1-terminal: $*" >&2
    exit 1
}

[ -f "$TERMINAL" ] || fail "missing terminal wrapper"
[ -f "$START" ] || fail "missing start_phase1 launcher"

sh -n "$TERMINAL"
sh -n "$START"

help_out=$(sh "$TERMINAL" help)
echo "$help_out" | grep -q "Phase1 Terminal" || fail "help missing title"
echo "$help_out" | grep -q "thin, safe wrapper" || fail "help missing wrapper safety note"
echo "$help_out" | grep -q "phase1-terminal gina" || fail "help missing gina command"

version_out=$(sh "$TERMINAL" version)
echo "$version_out" | grep -q "phase1-terminal for Phase1" || fail "version output missing Phase1 marker"

doctor_out=$(sh "$TERMINAL" doctor)
echo "$doctor_out" | grep -q "Phase1 launch doctor" || fail "doctor did not delegate to start_phase1"
echo "$doctor_out" | grep -q "gina" || fail "doctor missing gina status"
echo "$doctor_out" | grep -q "base1" || fail "doctor missing base1 status"

selftest_out=$(sh "$TERMINAL" selftest)
echo "$selftest_out" | grep -q "selftest: ok" || fail "selftest failed"

echo "test-phase1-terminal: ok"

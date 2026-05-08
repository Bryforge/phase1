#!/usr/bin/env sh
set -eu

ROOT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
PHASE1="$ROOT_DIR/phase1"
TERMINAL="$ROOT_DIR/terminal/bin/phase1-terminal"
START="$ROOT_DIR/start_phase1"
INSTALLER="$ROOT_DIR/scripts/install-phase1-command.sh"

fail() {
    echo "test-phase1-terminal: $*" >&2
    exit 1
}

[ -f "$PHASE1" ] || fail "missing simple phase1 launcher"
[ -f "$TERMINAL" ] || fail "missing terminal wrapper"
[ -f "$START" ] || fail "missing start_phase1 launcher"
[ -f "$INSTALLER" ] || fail "missing command installer"

sh -n "$PHASE1"
sh -n "$TERMINAL"
sh -n "$START"
sh -n "$INSTALLER"

phase1_help=$(sh "$PHASE1" help)
echo "$phase1_help" | grep -q "Phase1" || fail "phase1 help missing title"
echo "$phase1_help" | grep -q "sh phase1" || fail "phase1 help missing fresh clone shortcut"
echo "$phase1_help" | grep -q "install-phase1-command" || fail "phase1 help missing installer note"

phase1_version=$(sh "$PHASE1" version)
echo "$phase1_version" | grep -q "Phase1" || fail "phase1 version output missing Phase1 marker"

phase1_doctor=$(sh "$PHASE1" doctor)
echo "$phase1_doctor" | grep -q "Phase1 launch doctor" || fail "phase1 doctor did not delegate to start_phase1"
echo "$phase1_doctor" | grep -q "launcher" || fail "phase1 doctor missing launcher status"

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

tmp_bin=$(mktemp -d)
PHASE1_BIN_DIR="$tmp_bin" sh "$INSTALLER" >/dev/null
[ -x "$tmp_bin/phase1" ] || fail "installer did not create executable phase1 command"
installed_version=$("$tmp_bin/phase1" version)
echo "$installed_version" | grep -q "Phase1" || fail "installed phase1 command did not run"
rm -rf "$tmp_bin"

echo "test-phase1-terminal: ok"

#!/usr/bin/env sh
# Repair a local Phase1 checkout after commands were accidentally run with sudo.
#
# Usage:
#   sh scripts/black-phase1-repair-local-permissions.sh
#
# This fixes root-owned .git/build/target files so normal git pull/fetch works again.

set -eu

fail() { printf 'black-phase1-repair-local-permissions: %s\n' "$1" >&2; exit 1; }
[ -d .git ] || fail "run from phase1 repository root"
command -v sudo >/dev/null 2>&1 || fail "missing sudo"
command -v id >/dev/null 2>&1 || fail "missing id"

uid="$(id -u)"
gid="$(id -g)"

if [ "$uid" = "0" ]; then
  fail "do not run this repair script with sudo; run it as your normal user"
fi

printf 'Repairing local Phase1 ownership for uid=%s gid=%s\n' "$uid" "$gid"

for path in .git build target scripts src docs tests Cargo.lock Cargo.toml; do
  if [ -e "$path" ]; then
    sudo chown -R "$uid:$gid" "$path"
  fi
done

# Clear stale lock files that may remain after interrupted root-owned git operations.
find .git -name '*.lock' -type f -print -delete 2>/dev/null || true

printf 'DONE: local permissions repaired.\n'
printf 'Next:\n'
printf '  git fetch origin\n'
printf '  git pull --ff-only origin black-phase1\n'

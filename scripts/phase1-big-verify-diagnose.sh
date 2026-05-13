#!/usr/bin/env sh
# Print failed sections from a Phase1 big verification log.

set -eu

LOG=${1:-build/phase1-big-verify/phase1-big-verify.log}

if [ ! -f "$LOG" ]; then
  echo "phase1-big-verify-diagnose: missing log: $LOG" >&2
  exit 1
fi

echo "PHASE1 BIG VERIFY DIAGNOSE"
echo "log: $LOG"
echo

echo "Failed step summary:"
grep -n '^FAILED:' "$LOG" || echo "no FAILED lines found"
echo

awk '
  /^### / {
    if (section != "") {
      sections[section] = block
    }
    section = $0
    block = $0 "\n"
    next
  }
  {
    if (section != "") {
      block = block $0 "\n"
    }
  }
  END {
    if (section != "") {
      sections[section] = block
    }
    for (s in sections) {
      if (sections[s] ~ /FAILED:/ || sections[s] ~ /exit_code: [1-9][0-9]*/) {
        print "---- failed section ----"
        printf "%s", sections[s]
        print "---- end failed section ----\n"
      }
    }
  }
' "$LOG"

echo "Useful next commands:"
echo "  grep -n \"failures:\|FAILED\|error:\|panicked\|exit_code\" $LOG | tail -80"
echo "  awk '/### quality full/{p=1} /### base1 docs gate/{p=0} p' $LOG | tail -160"
echo "  awk '/### base1 reorg gate/{p=1} /### base1 link check/{p=0} p' $LOG | tail -160"

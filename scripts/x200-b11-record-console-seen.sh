#!/usr/bin/env sh
# Record successful X200 B11 GRUB-native Phase1 console evidence.
#
# This records physical-screen evidence only. It does not write disks, boot
# anything, install anything, or make a kernel/daily-driver claim.

set -eu

cd "$(dirname "$0")/.."

RESULT=phase1_grub_console_seen
REPORT=build/base1-b7-hardware-boot-evidence/b7-hardware-boot-evidence.env
PATCH=${BASE1_B11_PATCH:-$HOME/x200-b11-grub-console-seen.patch}

printf 'Recording X200 B11 GRUB-native Phase1 console evidence...\n\n'

sh scripts/x200-b7-record-result.sh "$RESULT"

git add -f "$REPORT"

if git diff --cached --quiet; then
  printf '\nNo staged evidence changes. Existing report may already match.\n'
else
  git commit -m "Record X200 B11 Phase1 GRUB console seen"
fi

git format-patch -1 HEAD --stdout > "$PATCH"

printf '\nB11 evidence recorded.\n'
printf 'Result: %s\n' "$RESULT"
printf 'Patch: %s\n' "$PATCH"
printf '\nMove this patch to the Mac and apply/push from the Mac if GitHub auth is not configured on the X200.\n'
printf 'Easy local transfer option from X200:\n'
printf '  cd ~ && python3 -m http.server 8000\n'

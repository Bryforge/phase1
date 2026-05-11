#!/usr/bin/env sh
# Record the X200 Libreboot/GRUB Phase1 boot-menu-seen evidence.

set -eu

MACHINE=${1:-X200}
ARTIFACT=${2:-build/phase1-uefi.img}

if [ ! -d .git ]; then
  printf 'x200-record-boot-menu-seen: run from the Phase1 repo directory, e.g. cd ~/phase1\n' >&2
  exit 1
fi

printf 'PHASE1 X200 BOOT MENU SEEN RECORDER\n\n'
printf 'machine : %s\n' "$MACHINE"
printf 'artifact: %s\n\n' "$ARTIFACT"

sh scripts/base1-b6-hardware-boot-evidence.sh \
  --record \
  --profile x200-supervisor-lite \
  --machine "$MACHINE" \
  --artifact "$ARTIFACT" \
  --result boot_menu_seen \
  --write-report

printf '\nRecorded B6 result: boot_menu_seen\n'
printf 'Next target after this is boot_started, then phase1_marker_seen.\n'

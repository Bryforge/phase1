#!/usr/bin/env sh
# X200 one-command USB marker setup runner.
#
# Pulls edge/stable, installs required Trisquel packages, then runs the
# Libreboot/GRUB marker USB writer. This can erase the selected USB disk.

set -eu

DISK=${1:-/dev/sdb}
CONFIRM=${2:-}
BRANCH=${PHASE1_BRANCH:-edge/stable}

fail() {
  printf 'x200-run-usb-marker-setup: %s\n' "$1" >&2
  exit 1
}

[ "$CONFIRM" = YES_WRITE_USB ] || fail "usage: sh scripts/x200-run-usb-marker-setup.sh ${DISK} YES_WRITE_USB"

case "$DISK" in
  /dev/sd[a-z]|/dev/vd[a-z]|/dev/hd[a-z]|/dev/mmcblk[0-9]|/dev/nvme[0-9]n[0-9]) : ;;
  *) fail "target must be a whole disk, not a partition: $DISK" ;;
esac

if [ ! -d .git ]; then
  fail 'run this from the Phase1 repo directory, e.g. cd ~/phase1'
fi

printf 'PHASE1 X200 USB MARKER SETUP\n\n'
printf 'repo   : %s\n' "$(pwd)"
printf 'branch : %s\n' "$BRANCH"
printf 'disk   : %s\n\n' "$DISK"

printf 'Syncing repo...\n'
git fetch origin
git switch "$BRANCH"
git pull --ff-only origin "$BRANCH"

printf '\nInstalling Trisquel packages...\n'
sudo apt update
sudo apt install -y parted dosfstools grub-pc-bin

printf '\nRunning USB marker writer...\n'
sh scripts/x200-libreboot-grub-marker-usb.sh "$DISK" YES_WRITE_USB

printf '\nDone. Reboot the X200 and choose Libreboot external GRUB search [s].\n'
printf 'If that does not show the USB, choose SeaBIOS payload [b].\n'

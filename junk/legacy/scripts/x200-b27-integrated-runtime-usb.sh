#!/usr/bin/env sh
# Phase1 / Base1 X200 B27 integrated runtime USB writer.
#
# Purpose:
#   Build one external USB that boots the proven path into a GNU/Linux-backed
#   Phase1 runtime with workspace, supervisor planning, and crypto evidence
#   commands available from one phase1> session.
#
# Route:
#   Libreboot -> SeaBIOS payload -> USB GRUB -> GNU/Linux -> Phase1 runtime
#
# Required local artifacts by default:
#   build/linux/alpine-netboot/vmlinuz
#   build/linux/alpine-netboot/initrd.img
#
# Safety:
#   This erases only the selected USB disk, refuses to run without
#   YES_WRITE_USB, and refuses /dev/sda by default.

set -eu

USB="${1:-/dev/sdb}"
CONFIRM="${2:-}"
PROFILE="${BASE1_B27_PROFILE:-x200-supervisor-concurrent-lab}"
OUT_DIR="${BASE1_B27_OUT:-build/base1-b27-integrated-runtime}"
KERNEL="${BASE1_B27_KERNEL:-build/linux/alpine-netboot/vmlinuz}"
INITRD="${BASE1_B27_INITRD:-build/linux/alpine-netboot/initrd.img}"
REPORT="$OUT_DIR/b27-integrated-runtime-usb.env"
OVERLAY_DIR="$OUT_DIR/initramfs-overlay"
OVERLAY="$OUT_DIR/phase1-integrated-overlay.cpio.gz"

fail() { printf 'x200-b27-integrated-runtime-usb: %s\n' "$1" >&2; exit 1; }
need_cmd() { command -v "$1" >/dev/null 2>&1 || fail "missing command: $1"; }

partition_one() {
  case "$1" in
    /dev/nvme[0-9]n[0-9]|/dev/mmcblk[0-9]) printf '%sp1\n' "$1" ;;
    /dev/sd[a-z]|/dev/vd[a-z]|/dev/hd[a-z]) printf '%s1\n' "$1" ;;
    *) fail "use a whole-disk path such as /dev/sdb, not a partition" ;;
  esac
}

[ "$CONFIRM" = "YES_WRITE_USB" ] || fail "usage: sh scripts/x200-b27-integrated-runtime-usb.sh /dev/sdb YES_WRITE_USB"
[ -b "$USB" ] || fail "not a block device: $USB"
[ "$USB" != "/dev/sda" ] || fail "refusing /dev/sda because it is commonly the internal disk"
[ -f "$KERNEL" ] || fail "missing GNU/Linux kernel: $KERNEL"
[ -f "$INITRD" ] || fail "missing GNU/Linux initrd: $INITRD"

ROOT_SRC="$(findmnt -no SOURCE / 2>/dev/null || true)"
case "$ROOT_SRC" in
  "$USB"|"$USB"[0-9]*|"$USB"p[0-9]*) fail "refusing to write the root filesystem device: $ROOT_SRC" ;;
esac

for cmd in sudo parted mkfs.vfat mount umount sync mkdir cp tee grub-install sha256sum find cpio gzip chmod date; do
  need_cmd "$cmd"
done

PART1="$(partition_one "$USB")"
MNT="$(mktemp -d)"

cleanup() { sudo umount "$MNT" 2>/dev/null || true; rmdir "$MNT" 2>/dev/null || true; }
trap cleanup EXIT INT TERM

mkdir -p "$OUT_DIR"
rm -rf "$OVERLAY_DIR"
mkdir -p \
  "$OVERLAY_DIR/bin" \
  "$OVERLAY_DIR/dev" \
  "$OVERLAY_DIR/etc" \
  "$OVERLAY_DIR/proc" \
  "$OVERLAY_DIR/sys" \
  "$OVERLAY_DIR/run" \
  "$OVERLAY_DIR/tmp" \
  "$OVERLAY_DIR/phase1" \
  "$OVERLAY_DIR/phase1/evidence" \
  "$OVERLAY_DIR/phase1/state" \
  "$OVERLAY_DIR/phase1/workspace" \
  "$OVERLAY_DIR/phase1/help" \
  "$OVERLAY_DIR/phase1/bin"

cat > "$OVERLAY_DIR/init" <<'EOF'
#!/bin/sh
# Phase1 B27 integrated runtime entrypoint.
# GNU/Linux-backed runtime with workspace, supervisor plan, and crypto evidence.

PATH=/phase1/bin:/bin:/sbin:/usr/bin:/usr/sbin
export PATH

mount -t proc proc /proc 2>/dev/null || true
mount -t sysfs sysfs /sys 2>/dev/null || mount -t sysfs sys /sys 2>/dev/null || true
mount -t devtmpfs devtmpfs /dev 2>/dev/null || mount -t tmpfs dev /dev 2>/dev/null || true
[ -c /dev/console ] || mknod /dev/console c 5 1 2>/dev/null || true
[ -c /dev/null ] || mknod /dev/null c 1 3 2>/dev/null || true
[ -c /dev/tty0 ] || mknod /dev/tty0 c 4 0 2>/dev/null || true
mount -t tmpfs tmpfs /run 2>/dev/null || true
mount -t tmpfs tmpfs /tmp 2>/dev/null || true
mkdir -p /phase1/evidence /phase1/state /phase1/workspace /phase1/help /phase1/bin

exec </dev/console >/dev/console 2>&1
stty sane 2>/dev/null || true

cat > /phase1/evidence/b27-runtime.env <<'ENV'
BASE1_B27_INTEGRATED_RUNTIME_MODE=initramfs
BASE1_B27_INTEGRATED_RUNTIME_RESULT=phase1_integrated_runtime_seen
BASE1_B27_RUNTIME_HAS_WORKSPACE=1
BASE1_B27_RUNTIME_HAS_SUPERVISOR_PLAN=1
BASE1_B27_RUNTIME_HAS_CRYPTO_EVIDENCE=1
BASE1_B27_INTEGRATED_RUNTIME_CLAIM=not_claimed
ENV

cat > /phase1/help/export-help.txt <<'EOF_HELP'
Evidence export notes

Evidence is in /phase1/evidence.
This runtime does not auto-network and does not write the internal disk.

After rebooting into Trisquel, record the observed result from the repo:
  cd ~/phase1
  git pull --ff-only origin edge/stable
  sh scripts/x200-record-and-share-result-safe.sh phase1_integrated_runtime_seen

Serve only ~/phase1-share if transferring a patch.
Use placeholders like <X200_IP>; do not commit local IP addresses.
EOF_HELP

phase1_banner() {
  clear 2>/dev/null || true
  cat <<'BANNER'
phase1 6.0.0 ready
Base1 B27 integrated runtime
result: phase1_integrated_runtime_seen

Route: Libreboot -> SeaBIOS -> USB GRUB -> GNU/Linux
Mode : runtime + workspace + supervisor + crypto evidence
Disk : internal disk not touched

Type: help, status, integrated-check, supervisor, crypto
BANNER
}

phase1_status() {
  echo ""
  echo "Phase1 status"
  echo "Runtime  : GNU/Linux initramfs"
  echo "Kernel   : $(uname -r 2>/dev/null || echo unknown)"
  echo "Machine  : $(uname -m 2>/dev/null || echo unknown)"
  echo "Workspace: /phase1/workspace"
  echo "Evidence : /phase1/evidence"
  echo "State    : /phase1/state"
  echo "Disk     : internal disk not touched by Phase1"
  echo ""
}

phase1_help() {
  cat <<'HELP'

Commands:
  help              show this help
  status            show runtime status
  workspace         enter /phase1/workspace
  evidence          list evidence and print core env
  supervisor        create/show supervisor lane plan
  crypto            create/show SHA-256 evidence manifest
  integrated-check  run supervisor + crypto + status
  files             list Phase1 paths
  devices           list /dev entries
  dmesg-tail        show recent kernel messages
  uname             show kernel identity
  export-help       show evidence export notes
  clear             redraw banner
  shell             open /bin/sh
  reboot            reboot machine
  poweroff          power off machine

HELP
}

phase1_evidence() {
  echo ""
  echo "/phase1/evidence"
  ls -la /phase1/evidence 2>/dev/null || true
  echo ""
  [ -f /phase1/evidence/b27-runtime.env ] && cat /phase1/evidence/b27-runtime.env
  echo ""
}

phase1_files() {
  echo ""
  echo "/phase1"
  ls -la /phase1 2>/dev/null || true
  echo ""
  echo "/phase1/evidence"
  ls -la /phase1/evidence 2>/dev/null || true
  echo ""
  echo "/phase1/workspace"
  ls -la /phase1/workspace 2>/dev/null || true
  echo ""
}

phase1_devices() {
  echo ""
  echo "Device snapshot"
  ls /dev 2>/dev/null | head -n 80 || true
  echo ""
}

phase1_dmesg_tail() {
  echo ""
  if command -v dmesg >/dev/null 2>&1; then
    dmesg | tail -n 40
  else
    echo "dmesg command not available in this initramfs"
  fi
  echo ""
}

phase1_workspace() {
  cd /phase1/workspace 2>/dev/null || return
  echo "workspace: /phase1/workspace"
  echo "type 'exit' to return to phase1>"
  /bin/sh
  cd / 2>/dev/null || true
}

phase1_supervisor() {
  mkdir -p /phase1/evidence/b25-supervisor
  cat > /phase1/evidence/b25-supervisor/supervisor-plan.env <<'ENV'
BASE1_B25_SUPERVISOR_PLAN_MODE=runtime-integrated
BASE1_B25_SUPERVISOR_PLAN_PROFILE=x200-supervisor-concurrent-lab
BASE1_B25_SUPERVISOR_LANE_1=linux-runtime
BASE1_B25_SUPERVISOR_LANE_2=workspace
BASE1_B25_SUPERVISOR_LANE_3=openbsd-plan
BASE1_B25_SUPERVISOR_LANE_4=crypto-plan
BASE1_B25_SUPERVISOR_LANE_POLICY=plan_only_no_auto_boot
BASE1_B25_SUPERVISOR_EXPECTED_RESULT=phase1_supervisor_plan_seen
BASE1_B25_SUPERVISOR_CLAIM=not_claimed
ENV
  echo ""
  echo "Phase1 supervisor plan"
  cat /phase1/evidence/b25-supervisor/supervisor-plan.env
  echo ""
}

phase1_crypto() {
  mkdir -p /phase1/evidence/b26-crypto
  MANIFEST=/phase1/evidence/b26-crypto/phase1-evidence-manifest.sha256
  REPORT=/phase1/evidence/b26-crypto/crypto-evidence.env
  : > "$MANIFEST"
  if command -v sha256sum >/dev/null 2>&1; then
    find /phase1/evidence -type f 2>/dev/null | sort | while read f; do
      case "$f" in
        /phase1/evidence/b26-crypto/*) continue ;;
      esac
      sha256sum "$f"
    done > "$MANIFEST"
    MANIFEST_SHA=$(sha256sum "$MANIFEST" | awk '{print $1}')
    COUNT=$(wc -l < "$MANIFEST" | tr -d ' ')
  else
    MANIFEST_SHA=sha256sum-unavailable
    COUNT=0
  fi
  cat > "$REPORT" <<EOF_REPORT
BASE1_B26_CRYPTO_EVIDENCE_MODE=runtime-sha256-manifest
BASE1_B26_CRYPTO_EVIDENCE_MANIFEST=$MANIFEST
BASE1_B26_CRYPTO_EVIDENCE_FILE_COUNT=$COUNT
BASE1_B26_CRYPTO_EVIDENCE_MANIFEST_SHA256=$MANIFEST_SHA
BASE1_B26_CRYPTO_EVIDENCE_RESULT=phase1_evidence_hash_manifest_seen
BASE1_B26_CRYPTO_EVIDENCE_SIGNING=not_enabled
BASE1_B26_CRYPTO_EVIDENCE_CLAIM=not_claimed
EOF_REPORT
  echo ""
  echo "Phase1 crypto evidence"
  cat "$REPORT"
  echo ""
}

phase1_integrated_check() {
  phase1_status
  phase1_supervisor
  phase1_crypto
  cat > /phase1/evidence/b27-integrated-check.env <<'ENV'
BASE1_B27_INTEGRATED_CHECK_RESULT=phase1_integrated_runtime_seen
BASE1_B27_INTEGRATED_CHECK_WORKSPACE=present
BASE1_B27_INTEGRATED_CHECK_SUPERVISOR=present
BASE1_B27_INTEGRATED_CHECK_CRYPTO=present
ENV
  echo "Integrated check complete."
  echo "Evidence: /phase1/evidence/b27-integrated-check.env"
}

phase1_banner

while true; do
  printf 'phase1> '
  read cmd || cmd=shell
  case "$cmd" in
    help|h) phase1_help ;;
    status|s) phase1_status ;;
    evidence|e) phase1_evidence ;;
    workspace|work|w) phase1_workspace ;;
    supervisor|sup) phase1_supervisor ;;
    crypto|hash) phase1_crypto ;;
    integrated-check|check) phase1_integrated_check ;;
    files|ls) phase1_files ;;
    devices|dev) phase1_devices ;;
    dmesg-tail|dmesg) phase1_dmesg_tail ;;
    uname) uname -a 2>/dev/null || true ;;
    export-help|export) cat /phase1/help/export-help.txt ;;
    clear|banner) phase1_banner ;;
    shell|sh) /bin/sh ;;
    reboot) echo b > /proc/sysrq-trigger 2>/dev/null || reboot -f ;;
    poweroff|halt) poweroff -f 2>/dev/null || halt -f ;;
    "") : ;;
    *) echo "unknown command: $cmd"; echo "type: help" ;;
  esac
done
EOF
chmod 0755 "$OVERLAY_DIR/init"

cat > "$OVERLAY_DIR/etc/motd" <<'EOF'
phase1 6.0.0 ready - B27 integrated runtime
EOF

printf 'PHASE1 BASE1 B27 INTEGRATED RUNTIME USB WRITER\n\n'
printf 'profile     : %s\n' "$PROFILE"
printf 'target disk : %s\n' "$USB"
printf 'grub part   : %s\n' "$PART1"
printf 'kernel      : %s\n' "$KERNEL"
printf 'initrd      : %s\n' "$INITRD"
printf 'boot path   : Libreboot -> SeaBIOS -> USB GRUB -> GNU/Linux\n\n'
printf 'Building Phase1 integrated initramfs overlay...\n'

( cd "$OVERLAY_DIR" && find . | cpio -H newc -o 2>/dev/null | gzip -9 > "../phase1-integrated-overlay.cpio.gz" )
[ -s "$OVERLAY" ] || fail "failed to build overlay: $OVERLAY"

printf 'Kernel SHA256:\n'
sha256sum "$KERNEL"
printf 'Initrd SHA256:\n'
sha256sum "$INITRD"
printf 'Overlay SHA256:\n'
sha256sum "$OVERLAY"
printf '\nThis will erase the selected USB target.\n\n'

sudo umount "${USB}"* 2>/dev/null || true
sudo parted -s "$USB" mklabel msdos
sudo parted -s "$USB" mkpart primary fat32 1MiB 100%
sudo parted -s "$USB" set 1 boot on
sudo partprobe "$USB" 2>/dev/null || true
sync
sleep 2
[ -b "$PART1" ] || fail "partition did not appear: $PART1"

sudo mkfs.vfat -F 32 -n PHASE1B27 "$PART1"
sudo mount "$PART1" "$MNT"
sudo mkdir -p \
  "$MNT/boot/grub" \
  "$MNT/grub" \
  "$MNT/boot/phase1" \
  "$MNT/phase1" \
  "$MNT/phase1/evidence" \
  "$MNT/phase1/help"

sudo cp "$KERNEL" "$MNT/boot/phase1/vmlinuz"
sudo cp "$INITRD" "$MNT/boot/phase1/initrd.img"
sudo cp "$OVERLAY" "$MNT/boot/phase1/phase1-integrated-overlay.cpio.gz"

sudo tee "$MNT/phase1/README.txt" >/dev/null <<EOF
Phase1 Base1 B27 integrated runtime USB
Profile: $PROFILE
Boot path: Libreboot -> Load SeaBIOS payload -> USB GRUB -> GNU/Linux

This USB boots an integrated Phase1 runtime with workspace,
supervisor planning, and crypto evidence hashing.
It is not an installer and does not write the internal disk.
EOF

sudo tee "$MNT/phase1/evidence/b27-runtime-prep.env" >/dev/null <<EOF
BASE1_B27_INTEGRATED_RUNTIME_PROFILE=$PROFILE
BASE1_B27_INTEGRATED_RUNTIME_BOOT_PATH=Libreboot_Load_SeaBIOS_payload_to_USB_GRUB_to_GNULinux
BASE1_B27_INTEGRATED_RUNTIME_EXPECTED_RESULT=phase1_integrated_runtime_seen
BASE1_B27_INTEGRATED_RUNTIME_KERNEL=/boot/phase1/vmlinuz
BASE1_B27_INTEGRATED_RUNTIME_INITRD=/boot/phase1/initrd.img
BASE1_B27_INTEGRATED_RUNTIME_OVERLAY=/boot/phase1/phase1-integrated-overlay.cpio.gz
BASE1_B27_INTEGRATED_RUNTIME_CLAIM=not_claimed
EOF

sudo tee "$MNT/boot/grub/grub.cfg" >/dev/null <<'EOF'
set timeout=10
set default=0
set pager=1
set color_normal=white/black
set color_highlight=black/light-gray
terminal_input console
terminal_output console

menuentry "Start Phase1 Integrated" {
    clear
    echo "phase1 6.0.0 ready"
    echo "Loading integrated GNU/Linux Phase1 runtime..."
    echo "Result target: phase1_integrated_runtime_seen"
    linux /boot/phase1/vmlinuz console=tty0 rdinit=/init init=/init nomodeset quiet loglevel=3 panic=30
    initrd /boot/phase1/initrd.img /boot/phase1/phase1-integrated-overlay.cpio.gz
    boot
}

menuentry "Start Phase1 Integrated - verbose" {
    clear
    echo "Loading verbose integrated Phase1 runtime..."
    linux /boot/phase1/vmlinuz console=tty0 rdinit=/init init=/init nomodeset loglevel=7 panic=30
    initrd /boot/phase1/initrd.img /boot/phase1/phase1-integrated-overlay.cpio.gz
    boot
}

menuentry "Phase1 GRUB fallback" {
    clear
    echo "phase1 6.0.0 ready"
    echo "Base1 B27 GRUB fallback console"
    echo "Use this if GNU/Linux loading is blocked."
    sleep --interruptible 999
}

menuentry "File check" {
    clear
    echo "/boot/phase1"
    ls /boot/phase1
    echo ""
    echo "/phase1/evidence"
    ls /phase1/evidence
    echo ""
    sleep --interruptible 999
}

menuentry "Reboot" {
    reboot
}

menuentry "Power off" {
    halt
}
EOF

sudo cp "$MNT/boot/grub/grub.cfg" "$MNT/grub/grub.cfg"

printf 'Installing GRUB BIOS bootloader to %s...\n' "$USB"
sudo grub-install --target=i386-pc --boot-directory="$MNT/boot" --recheck "$USB"
sync
sudo umount "$MNT"
rmdir "$MNT"
trap - EXIT INT TERM

cat > "$REPORT" <<EOF
BASE1_B27_INTEGRATED_RUNTIME_PROFILE=$PROFILE
BASE1_B27_INTEGRATED_RUNTIME_TARGET=$USB
BASE1_B27_INTEGRATED_RUNTIME_PARTITION=$PART1
BASE1_B27_INTEGRATED_RUNTIME_KERNEL=$KERNEL
BASE1_B27_INTEGRATED_RUNTIME_INITRD=$INITRD
BASE1_B27_INTEGRATED_RUNTIME_OVERLAY=$OVERLAY
BASE1_B27_INTEGRATED_RUNTIME_RESULT=prepared
BASE1_B27_INTEGRATED_RUNTIME_EXPECTED_NEXT_RESULT=phase1_integrated_runtime_seen
BASE1_B27_INTEGRATED_RUNTIME_BOOT_PATH=Libreboot_Load_SeaBIOS_payload_to_USB_GRUB_to_GNULinux
BASE1_B27_INTEGRATED_RUNTIME_DISPLAY_POLICY=linux_console_owns_runtime_display
BASE1_B27_INTEGRATED_RUNTIME_CLAIM=not_claimed
EOF

printf '\nDONE: B27 integrated Phase1 runtime USB prepared on %s\n' "$USB"
printf 'Boot path: Libreboot main menu -> Load SeaBIOS (payload).\n'
printf 'Choose: Start Phase1 Integrated.\n'
printf 'If the integrated runtime appears, record: phase1_integrated_runtime_seen\n'

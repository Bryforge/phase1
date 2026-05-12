#!/usr/bin/env sh
# Phase1 / Base1 X200 B35 full system load USB writer.
#
# Purpose:
#   Prepare the next full Phase1 system load attempt using what has worked:
#   - Libreboot GRUB external USB path
#   - real Phase1 splash PNG from assets/phase1-splash.png
#   - Linux-libre baseline kernel loaded by linux16
#   - Phase1-owned tiny initramfs as the main runtime
#
# Why tiny initramfs now:
#   B31/B34 proved linux16 and initrd16 can load the huge full host initrd, but
#   actual boot resets back to Libreboot. B35 avoids that unstable host initrd
#   execution path and uses a Phase1-owned BusyBox initramfs first.
#
# Required local artifacts:
#   build/linux/alpine-netboot/vmlinuz
#   assets/phase1-splash.png
#   static busybox binary, default search:
#     /bin/busybox
#     /usr/bin/busybox
#     /bin/busybox.static
#     /usr/bin/busybox.static
#
# If busybox is missing or dynamic on Trisquel/Debian systems, install:
#   sudo apt install -y busybox-static
#
# Safety:
#   This erases only the selected USB disk, refuses to run without
#   YES_WRITE_USB, and refuses /dev/sda by default.

set -eu

USB="${1:-/dev/sdb}"
CONFIRM="${2:-}"
PROFILE="${BASE1_B35_PROFILE:-x200-supervisor-lite}"
OUT_DIR="${BASE1_B35_OUT:-build/base1-b35-full-system-load}"
KERNEL="${BASE1_B35_KERNEL:-build/linux/alpine-netboot/vmlinuz}"
SPLASH_SRC="${BASE1_B35_SPLASH:-assets/phase1-splash.png}"
BUSYBOX="${BASE1_B35_BUSYBOX:-}"
REPORT="$OUT_DIR/b35-full-system-load-usb.env"
ROOTFS="$OUT_DIR/rootfs"
INITRD="$OUT_DIR/phase1-b35-full-system-initramfs.img"

fail() { printf 'x200-b35-full-system-load-usb: %s\n' "$1" >&2; exit 1; }
need_cmd() { command -v "$1" >/dev/null 2>&1 || fail "missing command: $1"; }

partition_one() {
  case "$1" in
    /dev/nvme[0-9]n[0-9]|/dev/mmcblk[0-9]) printf '%sp1\n' "$1" ;;
    /dev/sd[a-z]|/dev/vd[a-z]|/dev/hd[a-z]) printf '%s1\n' "$1" ;;
    *) fail "use a whole-disk path such as /dev/sdb, not a partition" ;;
  esac
}

find_busybox() {
  if [ -n "$BUSYBOX" ]; then
    printf '%s\n' "$BUSYBOX"
    return
  fi
  for candidate in /bin/busybox /usr/bin/busybox /bin/busybox.static /usr/bin/busybox.static; do
    if [ -x "$candidate" ]; then
      printf '%s\n' "$candidate"
      return
    fi
  done
  printf '\n'
}

[ "$CONFIRM" = "YES_WRITE_USB" ] || fail "usage: sh scripts/x200-b35-full-system-load-usb.sh /dev/sdb YES_WRITE_USB"
[ -b "$USB" ] || fail "not a block device: $USB"
[ "$USB" != "/dev/sda" ] || fail "refusing /dev/sda because it is commonly the internal disk"
[ -f "$KERNEL" ] || fail "missing Linux-libre baseline kernel: $KERNEL"
[ -f "$SPLASH_SRC" ] || fail "missing real Phase1 splash asset: $SPLASH_SRC"

ROOT_SRC="$(findmnt -no SOURCE / 2>/dev/null || true)"
case "$ROOT_SRC" in
  "$USB"|"$USB"[0-9]*|"$USB"p[0-9]*) fail "refusing to write the root filesystem device: $ROOT_SRC" ;;
esac

for cmd in sudo parted mkfs.vfat mount umount sync mkdir cp tee grub-install sha256sum find cpio gzip chmod ln date stat file grep awk wc; do
  need_cmd "$cmd"
done

BUSYBOX_PATH="$(find_busybox)"
[ -n "$BUSYBOX_PATH" ] || fail "missing busybox; install busybox-static or set BASE1_B35_BUSYBOX=/path/to/static/busybox"
[ -x "$BUSYBOX_PATH" ] || fail "busybox is not executable: $BUSYBOX_PATH"

if command -v ldd >/dev/null 2>&1; then
  if ldd "$BUSYBOX_PATH" 2>&1 | grep -qi 'not a dynamic executable'; then
    BUSYBOX_STATIC=yes
  elif ldd "$BUSYBOX_PATH" 2>&1 | grep -qi 'statically linked'; then
    BUSYBOX_STATIC=yes
  else
    BUSYBOX_STATIC=no
  fi
else
  if file "$BUSYBOX_PATH" | grep -qi 'statically linked'; then
    BUSYBOX_STATIC=yes
  else
    BUSYBOX_STATIC=unknown
  fi
fi
[ "$BUSYBOX_STATIC" != no ] || fail "busybox appears dynamic; install busybox-static and rerun"

PART1="$(partition_one "$USB")"
MNT="$(mktemp -d)"

cleanup() { sudo umount "$MNT" 2>/dev/null || true; rmdir "$MNT" 2>/dev/null || true; }
trap cleanup EXIT INT TERM

mkdir -p "$OUT_DIR"
rm -rf "$ROOTFS"
mkdir -p \
  "$ROOTFS/bin" \
  "$ROOTFS/sbin" \
  "$ROOTFS/dev" \
  "$ROOTFS/proc" \
  "$ROOTFS/sys" \
  "$ROOTFS/run" \
  "$ROOTFS/tmp" \
  "$ROOTFS/etc" \
  "$ROOTFS/phase1" \
  "$ROOTFS/phase1/bin" \
  "$ROOTFS/phase1/evidence" \
  "$ROOTFS/phase1/state" \
  "$ROOTFS/phase1/workspace" \
  "$ROOTFS/phase1/assets" \
  "$ROOTFS/phase1/help"

cp "$BUSYBOX_PATH" "$ROOTFS/bin/busybox"
chmod 0755 "$ROOTFS/bin/busybox"
for app in sh ash mount umount mkdir cat echo ls dmesg uname sleep reboot poweroff halt mknod stty clear printf grep awk wc find sort sha256sum head tail date ps free df sync; do
  ln -sf busybox "$ROOTFS/bin/$app" 2>/dev/null || true
done
ln -sf ../bin/busybox "$ROOTFS/sbin/reboot" 2>/dev/null || true
ln -sf ../bin/busybox "$ROOTFS/sbin/poweroff" 2>/dev/null || true
ln -sf ../bin/busybox "$ROOTFS/sbin/halt" 2>/dev/null || true
cp "$SPLASH_SRC" "$ROOTFS/phase1/assets/phase1-splash.png"

cat > "$ROOTFS/init" <<'EOF'
#!/bin/sh
# Phase1 B35 full system load init.
# Phase1-owned BusyBox initramfs runtime.

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

cat > /phase1/evidence/b35-full-system.env <<'ENV'
BASE1_B35_FULL_SYSTEM_MODE=phase1-owned-busybox-initramfs
BASE1_B35_FULL_SYSTEM_RESULT=phase1_full_system_load_seen
BASE1_B35_RUNTIME_HAS_WORKSPACE=1
BASE1_B35_RUNTIME_HAS_SUPERVISOR_PLAN=1
BASE1_B35_RUNTIME_HAS_CRYPTO_EVIDENCE=1
BASE1_B35_RUNTIME_HAS_REAL_SPLASH_ASSET=1
BASE1_B35_FULL_SYSTEM_CLAIM=not_claimed
ENV

cat > /phase1/help/export-help.txt <<'EOF_HELP'
Evidence export notes

Evidence lives in /phase1/evidence during this runtime.
This runtime does not auto-network and does not write the internal disk.

After rebooting into Trisquel, record the observed result from the repo:
  cd ~/phase1
  git pull --ff-only origin edge/stable
  sh scripts/x200-record-and-share-result-safe.sh phase1_full_system_load_seen

Serve only ~/phase1-share if transferring a patch.
Use placeholders like <X200_IP>; do not commit local IP addresses.
EOF_HELP

phase1_banner() {
  clear 2>/dev/null || true
  cat <<'BANNER'
phase1 6.0.0 ready
Base1 B35 full system load
result: phase1_full_system_load_seen

Route: Libreboot GRUB -> linux16 -> Phase1 initramfs
Mode : Phase1-owned BusyBox runtime
Disk : internal disk not touched

Type: help, status, integrated-check, supervisor, crypto
BANNER
}

phase1_status() {
  echo ""
  echo "Phase1 status"
  echo "Runtime  : Phase1-owned BusyBox initramfs"
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
  [ -f /phase1/evidence/b35-full-system.env ] && cat /phase1/evidence/b35-full-system.env
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
  dmesg 2>/dev/null | tail -n 40 || echo "dmesg unavailable"
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
BASE1_B25_SUPERVISOR_PLAN_MODE=b35-full-system-runtime
BASE1_B25_SUPERVISOR_PLAN_PROFILE=x200-supervisor-lite
BASE1_B25_SUPERVISOR_LANE_1=phase1-runtime
BASE1_B25_SUPERVISOR_LANE_2=workspace
BASE1_B25_SUPERVISOR_LANE_3=crypto-plan
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
BASE1_B26_CRYPTO_EVIDENCE_MODE=b35-runtime-sha256-manifest
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
  cat > /phase1/evidence/b35-integrated-check.env <<'ENV'
BASE1_B35_INTEGRATED_CHECK_RESULT=phase1_full_system_load_seen
BASE1_B35_INTEGRATED_CHECK_WORKSPACE=present
BASE1_B35_INTEGRATED_CHECK_SUPERVISOR=present
BASE1_B35_INTEGRATED_CHECK_CRYPTO=present
ENV
  echo "Integrated check complete."
  echo "Evidence: /phase1/evidence/b35-integrated-check.env"
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
chmod 0755 "$ROOTFS/init"

cat > "$ROOTFS/etc/motd" <<'EOF'
phase1 6.0.0 ready - B35 full system load
EOF

printf 'PHASE1 BASE1 B35 FULL SYSTEM LOAD USB WRITER\n\n'
printf 'profile     : %s\n' "$PROFILE"
printf 'target disk : %s\n' "$USB"
printf 'kernel      : %s\n' "$KERNEL"
printf 'busybox     : %s\n' "$BUSYBOX_PATH"
printf 'splash      : %s\n' "$SPLASH_SRC"
printf 'scope       : real splash plus Phase1-owned full runtime\n\n'

( cd "$ROOTFS" && find . | cpio -H newc -o 2>/dev/null | gzip -9 > "../phase1-b35-full-system-initramfs.img" )
[ -s "$INITRD" ] || fail "failed to build initramfs: $INITRD"

printf 'Kernel SHA256:\n'
sha256sum "$KERNEL"
printf 'B35 initramfs SHA256:\n'
sha256sum "$INITRD"
printf 'Real splash SHA256:\n'
sha256sum "$SPLASH_SRC"
printf '\nThis will erase the selected USB target.\n\n'

sudo umount "${USB}"* 2>/dev/null || true
sudo parted -s "$USB" mklabel msdos
sudo parted -s "$USB" mkpart primary fat32 1MiB 100%
sudo parted -s "$USB" set 1 boot on
sudo partprobe "$USB" 2>/dev/null || true
sync
sleep 2
[ -b "$PART1" ] || fail "partition did not appear: $PART1"

sudo mkfs.vfat -F 32 -n PHASE1B35 "$PART1"
sudo mount "$PART1" "$MNT"
sudo mkdir -p "$MNT/boot/grub" "$MNT/grub" "$MNT/boot/phase1" "$MNT/phase1/assets" "$MNT/phase1/evidence"
sudo cp "$KERNEL" "$MNT/boot/phase1/vmlinuz"
sudo cp "$INITRD" "$MNT/boot/phase1/phase1-b35-full-system-initramfs.img"
sudo cp "$SPLASH_SRC" "$MNT/phase1/assets/phase1-splash.png"

KERNEL_SIZE="$(stat -c %s "$KERNEL" 2>/dev/null || stat -f %z "$KERNEL")"
INITRD_SIZE="$(stat -c %s "$INITRD" 2>/dev/null || stat -f %z "$INITRD")"
SPLASH_SIZE="$(stat -c %s "$SPLASH_SRC" 2>/dev/null || stat -f %z "$SPLASH_SRC")"

sudo tee "$MNT/phase1/evidence/b35-prep.env" >/dev/null <<EOF
BASE1_B35_FULL_SYSTEM_PROFILE=$PROFILE
BASE1_B35_FULL_SYSTEM_KERNEL=/boot/phase1/vmlinuz
BASE1_B35_FULL_SYSTEM_INITRD=/boot/phase1/phase1-b35-full-system-initramfs.img
BASE1_B35_FULL_SYSTEM_SPLASH=/phase1/assets/phase1-splash.png
BASE1_B35_FULL_SYSTEM_KERNEL_SIZE=$KERNEL_SIZE
BASE1_B35_FULL_SYSTEM_INITRD_SIZE=$INITRD_SIZE
BASE1_B35_FULL_SYSTEM_SPLASH_SIZE=$SPLASH_SIZE
BASE1_B35_FULL_SYSTEM_EXPECTED_RESULT=phase1_full_system_load_seen
BASE1_B35_REAL_SPLASH_EXPECTED_RESULT=phase1_real_splash_seen
BASE1_B35_FULL_SYSTEM_CLAIM=not_claimed
EOF

sudo tee "$MNT/boot/grub/grub.cfg" >/dev/null <<'EOF'
set timeout=5
set default=1
set pager=1
set color_normal=white/black
set color_highlight=black/light-gray
terminal_input console
terminal_output console

menuentry "B35 Real Phase1 Splash" {
    clear
    echo "Loading real Phase1 splash..."
    insmod all_video
    insmod gfxterm
    insmod png
    set gfxmode=auto
    set gfxpayload=keep
    terminal_output gfxterm
    if background_image /phase1/assets/phase1-splash.png; then
        echo "phase1 6.0.0 ready"
        echo "B35 real Phase1 splash active"
        echo "result: phase1_real_splash_seen"
        echo "Returning to text mode in 3 seconds..."
        sleep 3
    else
        terminal_output console
        clear
        echo "Real PNG splash unavailable. Continuing text-safe."
        sleep 2
    fi
    terminal_output console
}

menuentry "Start Phase1 Full System" {
    clear
    echo "phase1 6.0.0 ready"
    echo "B35 full system load"
    echo "Target: phase1_full_system_load_seen"
    echo "Loading Linux-libre kernel plus Phase1 initramfs..."
    linux16 /boot/phase1/vmlinuz root=/dev/ram0 ro console=tty0 nomodeset loglevel=7 panic=0 phase1.full=1
    echo "linux16 returned; loading Phase1 initramfs"
    initrd16 /boot/phase1/phase1-b35-full-system-initramfs.img
    echo "initrd16 returned; booting now"
    boot
}

menuentry "Start Phase1 Full System - quiet" {
    clear
    echo "phase1 6.0.0 ready"
    echo "B35 full system load quiet"
    linux16 /boot/phase1/vmlinuz root=/dev/ram0 ro console=tty0 nomodeset quiet loglevel=3 panic=0 phase1.full=1
    echo "linux16 returned; loading Phase1 initramfs"
    initrd16 /boot/phase1/phase1-b35-full-system-initramfs.img
    echo "initrd16 returned; booting now"
    boot
}

menuentry "B35 File check" {
    clear
    echo "B35 full system file check"
    echo ""
    ls -lh /boot/phase1
    echo ""
    ls -lh /phase1/assets
    echo ""
    cat /phase1/evidence/b35-prep.env
    echo ""
    sleep --interruptible 999
}

menuentry "B35 Test commands only" {
    clear
    echo "B35 test: before linux16 command"
    linux16 /boot/phase1/vmlinuz root=/dev/ram0 ro console=tty0 nomodeset loglevel=7 panic=0 phase1.full=1
    echo "B35 test: linux16 command returned"
    echo "B35 test: before initrd16 command"
    initrd16 /boot/phase1/phase1-b35-full-system-initramfs.img
    echo "B35 test: initrd16 command returned"
    echo "No boot performed in this entry. ESC returns."
    sleep --interruptible 999
}

menuentry "B35 GRUB fallback" {
    clear
    echo "phase1 6.0.0 ready"
    echo "B35 GRUB fallback console"
    echo "Use if full system load resets."
    sleep --interruptible 999
}

menuentry "Reboot" { reboot }
menuentry "Power off" { halt }
EOF
sudo cp "$MNT/boot/grub/grub.cfg" "$MNT/grub/grub.cfg"

printf 'Installing GRUB BIOS bootloader to %s...\n' "$USB"
sudo grub-install --target=i386-pc --boot-directory="$MNT/boot" "$USB"
sync
sudo umount "$MNT"
rmdir "$MNT"
trap - EXIT INT TERM

cat > "$REPORT" <<EOF
BASE1_B35_FULL_SYSTEM_TARGET=$USB
BASE1_B35_FULL_SYSTEM_PARTITION=$PART1
BASE1_B35_FULL_SYSTEM_KERNEL=$KERNEL
BASE1_B35_FULL_SYSTEM_INITRD=$INITRD
BASE1_B35_FULL_SYSTEM_SPLASH=$SPLASH_SRC
BASE1_B35_FULL_SYSTEM_RESULT=prepared
BASE1_B35_FULL_SYSTEM_EXPECTED_NEXT_RESULT=phase1_full_system_load_seen
BASE1_B35_REAL_SPLASH_EXPECTED_NEXT_RESULT=phase1_real_splash_seen
EOF

printf '\nDONE: B35 full system load USB prepared on %s\n' "$USB"
printf 'Boot path: Libreboot -> external USB GRUB.\n'
printf 'Default entry starts Phase1 full system load.\n'

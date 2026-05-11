#!/usr/bin/env sh
# Base1 x86_64 read-only detection preview.
#
# B1 boot-readiness slice: inspect host facts without modifying boot
# configuration, firmware, partitions, filesystems, packages, or kernel
# command lines.

set -eu

usage() {
  cat <<'EOF'
usage: sh scripts/base1-x86_64-detect.sh --dry-run

Read-only Base1 x86_64 detection preview.

Required:
  --dry-run   prove this command is non-mutating for the initial B1 slice

This script reports hints only and writes nothing.
EOF
}

fail() {
  printf 'base1-x86_64-detect error: %s\n' "$1" >&2
  exit 1
}

have() {
  command -v "$1" >/dev/null 2>&1
}

read_first_line() {
  file=$1
  if [ -r "$file" ]; then
    sed -n '1p' "$file" 2>/dev/null || true
  fi
}

redact_line() {
  # Keep this intentionally conservative and portable.
  # Avoid echoing likely secrets from kernel command lines or environment-like data.
  printf '%s\n' "$1" \
    | sed -E 's/([Tt][Oo][Kk][Ee][Nn]|[Ss][Ee][Cc][Rr][Ee][Tt]|[Pp][Aa][Ss][Ss][Ww][Oo][Rr][Dd]|[Kk][Ee][Yy]|[Cc][Rr][Ee][Dd][Ee][Nn][Tt][Ii][Aa][Ll])=[^ ]+/\1=<redacted>/g'
}

DRY_RUN=no

for arg in "$@"; do
  case "$arg" in
    --dry-run)
      DRY_RUN=yes
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      usage >&2
      fail "unknown argument: $arg"
      ;;
  esac
done

[ "$DRY_RUN" = "yes" ] || {
  usage >&2
  fail "--dry-run is required for B1 read-only detection"
}

printf 'base1-x86_64_detect: start\n'
printf 'status: B1 read-only detection preview\n'
printf 'writes: no\n'
printf 'mutation: no\n'
printf 'network: no\n'
printf '\n'

# Architecture hints
arch_value=unknown
if have uname; then
  arch_value=$(uname -m 2>/dev/null || printf 'unknown')
elif have arch; then
  arch_value=$(arch 2>/dev/null || printf 'unknown')
fi
printf 'architecture:\n'
printf '  machine: %s\n' "$arch_value"
case "$arch_value" in
  x86_64|amd64)
    printf '  x86_64_hint: yes\n'
    ;;
  unknown|'')
    printf '  x86_64_hint: unknown\n'
    printf '  warning: architecture could not be determined with read-only commands\n'
    ;;
  *)
    printf '  x86_64_hint: no\n'
    printf '  warning: this B1 detector is scoped to x86_64 planning\n'
    ;;
esac
printf '\n'

# Firmware hints
printf 'firmware:\n'
if [ -d /sys/firmware/efi ]; then
  printf '  mode_hint: uefi\n'
  if [ -d /sys/firmware/efi/efivars ]; then
    printf '  efivars_hint: present\n'
  else
    printf '  efivars_hint: unknown\n'
  fi
else
  printf '  mode_hint: bios_or_unknown\n'
  printf '  efivars_hint: absent_or_unavailable\n'
fi
printf '\n'

# Boot-loader hints. These are hints only; no boot tool is invoked.
printf 'boot_loader:\n'
found_boot_hint=no
if [ -d /boot/grub ] || [ -d /boot/grub2 ]; then
  printf '  grub_hint: present\n'
  found_boot_hint=yes
else
  printf '  grub_hint: absent_or_unknown\n'
fi
if [ -d /boot/efi/EFI/systemd ] || [ -d /efi/EFI/systemd ]; then
  printf '  systemd_boot_hint: present\n'
  found_boot_hint=yes
else
  printf '  systemd_boot_hint: absent_or_unknown\n'
fi
if [ "$found_boot_hint" = "no" ]; then
  printf '  selected_hint: unknown\n'
else
  printf '  selected_hint: inspect_required\n'
fi
printf '\n'

# Kernel command line, redacted.
printf 'kernel_cmdline:\n'
cmdline=$(read_first_line /proc/cmdline)
if [ -n "$cmdline" ]; then
  printf '  available: yes\n'
  printf '  value: %s\n' "$(redact_line "$cmdline")"
else
  printf '  available: no\n'
  printf '  value: unknown\n'
fi
printf '\n'

# Virtualization hints.
printf 'virtualization:\n'
virt_hint=unknown
if have systemd-detect-virt; then
  if systemd-detect-virt --quiet >/dev/null 2>&1; then
    virt_hint=$(systemd-detect-virt 2>/dev/null || printf 'detected')
  else
    virt_hint=none_detected
  fi
elif [ -r /proc/1/cgroup ] && grep -E 'docker|container|kubepods|libpod|lxc' /proc/1/cgroup >/dev/null 2>&1; then
  virt_hint=container_hint
fi
printf '  hint: %s\n' "$virt_hint"
printf '\n'

# Storage layout hints. Prefer lsblk when available; otherwise read /proc/mounts.
printf 'storage_layout:\n'
if have lsblk; then
  printf '  source: lsblk\n'
  lsblk -o NAME,TYPE,FSTYPE,SIZE,MOUNTPOINTS 2>/dev/null | sed 's/^/  /' || printf '  value: unknown\n'
elif [ -r /proc/mounts ]; then
  printf '  source: /proc/mounts\n'
  sed -n '1,12p' /proc/mounts 2>/dev/null | sed 's/^/  /' || printf '  value: unknown\n'
else
  printf '  source: unknown\n'
  printf '  value: unknown\n'
fi
printf '\n'

# Recovery availability hints.
printf 'recovery:\n'
if [ -d /recovery ] || [ -d /boot/recovery ]; then
  printf '  recovery_path_hint: present\n'
else
  printf '  recovery_path_hint: absent_or_unknown\n'
fi
if [ -r /proc/cmdline ] && grep -E 'recovery|single|emergency|rescue' /proc/cmdline >/dev/null 2>&1; then
  printf '  emergency_mode_hint: present_in_cmdline\n'
else
  printf '  emergency_mode_hint: absent_or_unknown\n'
fi
printf '\n'

printf 'unknowns:\n'
printf '  boot_profile: requires future profile report\n'
printf '  boot_parameters: requires future boot-params report\n'
printf '  hardware_validation: not claimed\n'
printf '  hardened_status: not claimed\n'
printf '\n'

printf 'next_read_only_check: sh scripts/base1-x86_64-boot-params.sh --dry-run (planned)\n'
printf 'base1-x86_64_detect: complete\n'

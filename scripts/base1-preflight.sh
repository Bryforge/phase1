#!/usr/bin/env sh
# Base1 non-destructive readiness checker.
#
# This script prints host facts and warnings only. It does not install packages,
# create users, change services, edit firewall policy, or write system files.

set -eu

BASE1_PROFILE=${BASE1_PROFILE:-secure-default}
BASE1_HARDWARE_TARGET=${BASE1_HARDWARE_TARGET:-auto}
PHASE1_STORAGE_ROOT=${PHASE1_STORAGE_ROOT:-/var/lib/phase1/workspace}
PHASE1_RUNTIME_USER=${PHASE1_RUNTIME_USER:-phase1}
MIN_MEM_KB=${BASE1_MIN_MEM_KB:-262144}
MIN_WORKSPACE_MB=${BASE1_MIN_WORKSPACE_MB:-512}

info() {
  printf 'base1: %s\n' "$1"
}

warn() {
  printf 'base1 warning: %s\n' "$1" >&2
}

have() {
  command -v "$1" >/dev/null 2>&1
}

read_mem_kb() {
  if [ -r /proc/meminfo ]; then
    awk '/^MemTotal:/ { print $2; exit }' /proc/meminfo
  else
    printf '0\n'
  fi
}

workspace_available_mb() {
  parent=$PHASE1_STORAGE_ROOT
  while [ ! -d "$parent" ] && [ "$parent" != "/" ]; do
    parent=$(dirname "$parent")
  done
  df -Pm "$parent" 2>/dev/null | awk 'NR == 2 { print $4; exit }'
}

detect_target() {
  arch=$(uname -m 2>/dev/null || printf unknown)
  model=''
  if [ -r /proc/device-tree/model ]; then
    model=$(tr -d '\000' < /proc/device-tree/model 2>/dev/null || true)
  elif [ -r /sys/class/dmi/id/product_version ]; then
    model=$(cat /sys/class/dmi/id/product_version 2>/dev/null || true)
  elif [ -r /sys/class/dmi/id/product_name ]; then
    model=$(cat /sys/class/dmi/id/product_name 2>/dev/null || true)
  fi

  case "$model" in
    *Raspberry*Pi*) printf 'raspberry-pi\n' ;;
    *ThinkPad*X200*|*X200*) printf 'x200\n' ;;
    *)
      case "$arch" in
        arm*|aarch64) printf 'raspberry-pi-candidate\n' ;;
        x86_64|amd64) printf 'x200-or-generic-candidate\n' ;;
        *) printf 'generic\n' ;;
      esac
      ;;
  esac
}

info "profile: $BASE1_PROFILE"
info "requested hardware target: $BASE1_HARDWARE_TARGET"
info "detected target hint: $(detect_target)"
info "kernel: $(uname -s 2>/dev/null || printf unknown) $(uname -r 2>/dev/null || printf unknown)"
info "architecture: $(uname -m 2>/dev/null || printf unknown)"

mem_kb=$(read_mem_kb)
info "memory-kb: $mem_kb"
if [ "$mem_kb" -gt 0 ] && [ "$mem_kb" -lt "$MIN_MEM_KB" ]; then
  warn "memory is below recommended minimum ${MIN_MEM_KB}KB"
fi

available_mb=$(workspace_available_mb || printf '0')
info "workspace: $PHASE1_STORAGE_ROOT"
info "workspace-available-mb: ${available_mb:-0}"
if [ "${available_mb:-0}" -gt 0 ] && [ "${available_mb:-0}" -lt "$MIN_WORKSPACE_MB" ]; then
  warn "workspace parent has less than recommended ${MIN_WORKSPACE_MB}MB available"
fi

if [ "$(id -u)" = "0" ]; then
  warn "preflight is running as root; Base1 prefers Phase1 runtime checks from an unprivileged context"
fi

if id "$PHASE1_RUNTIME_USER" >/dev/null 2>&1; then
  info "runtime-user: $PHASE1_RUNTIME_USER exists"
else
  warn "runtime-user $PHASE1_RUNTIME_USER does not exist yet"
fi

if [ -d "$PHASE1_STORAGE_ROOT" ]; then
  if [ -w "$PHASE1_STORAGE_ROOT" ]; then
    info "workspace-writable: yes"
  else
    warn "workspace exists but is not writable by this user"
  fi
else
  warn "workspace does not exist yet: $PHASE1_STORAGE_ROOT"
fi

for tool in sh awk sed grep df uname id; do
  if have "$tool"; then
    info "tool:$tool ok"
  else
    warn "required basic tool missing: $tool"
  fi
done

for optional_tool in git cargo rustc systemctl; do
  if have "$optional_tool"; then
    info "optional-tool:$optional_tool present"
  else
    info "optional-tool:$optional_tool not present"
  fi
done

if have ss; then
  inbound=$(ss -ltn 2>/dev/null | awk 'NR > 1 { count++ } END { print count + 0 }')
  info "tcp-listeners: $inbound"
  if [ "$inbound" -gt 0 ]; then
    warn "listening TCP sockets detected; Base1 secure-default prefers no unexpected inbound services"
  fi
else
  info "tcp-listeners: not checked; ss not available"
fi

info "safe-mode-default: PHASE1_SAFE_MODE=1"
info "host-tools-default: PHASE1_ALLOW_HOST_TOOLS=0"
info "preflight complete; no host changes were made"

#!/usr/bin/env sh
# Base1 network lockdown dry-run planner.
#
# This script is intentionally read-only. It previews the network posture Base1
# should enforce later, but it does not change firewall rules, services,
# routes, DNS, SSH, or package state.

set -eu

PROFILE=${BASE1_NETWORK_PROFILE:-secure-default}
TARGET=${BASE1_HARDWARE_TARGET:-auto}
DRY_RUN=0
ALLOW_SSH=${BASE1_ALLOW_SSH:-0}

usage() {
  cat <<'USAGE'
usage: sh scripts/base1-network-lockdown-dry-run.sh --dry-run [--profile secure-default|offline|appliance|dev] [--target name]

Preview Base1 network lockdown policy without changing the host.

Options:
  --dry-run                 required; proves this command is read-only
  --profile <name>          secure-default, offline, appliance, or dev
  --target <name>           hardware target hint, such as raspberry-pi, x200, or generic
  -h, --help                show this help
USAGE
}

info() {
  printf 'base1-network: %s\n' "$1"
}

warn() {
  printf 'base1-network warning: %s\n' "$1" >&2
}

fail() {
  printf 'base1-network error: %s\n' "$1" >&2
  exit 2
}

have() {
  command -v "$1" >/dev/null 2>&1
}

while [ "$#" -gt 0 ]; do
  case "$1" in
    --dry-run)
      DRY_RUN=1
      shift
      ;;
    --profile)
      [ "$#" -ge 2 ] || fail '--profile requires a value'
      PROFILE=$2
      shift 2
      ;;
    --target)
      [ "$#" -ge 2 ] || fail '--target requires a value'
      TARGET=$2
      shift 2
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      fail "unknown argument: $1"
      ;;
  esac
done

[ "$DRY_RUN" = "1" ] || fail 'refusing to run without --dry-run'

case "$PROFILE" in
  secure-default|offline|appliance|dev) ;;
  *) fail "unsupported profile: $PROFILE" ;;
esac

info 'mode: dry-run'
info 'writes: no'
info "profile: $PROFILE"
info "target: $TARGET"
info "allow-ssh: $ALLOW_SSH"
info "kernel: $(uname -s 2>/dev/null || printf unknown) $(uname -r 2>/dev/null || printf unknown)"
info "architecture: $(uname -m 2>/dev/null || printf unknown)"

if have nft; then
  info 'firewall-backend:nftables present'
elif have pfctl; then
  info 'firewall-backend:pf present'
elif have ufw; then
  info 'firewall-backend:ufw present'
elif have iptables; then
  info 'firewall-backend:iptables present'
elif have firewall-cmd; then
  info 'firewall-backend:firewalld present'
else
  warn 'no common firewall backend detected'
fi

if have ss; then
  listeners=$(ss -ltn 2>/dev/null | awk 'NR > 1 { count++ } END { print count + 0 }')
  info "tcp-listeners: $listeners"
  if [ "$listeners" -gt 0 ]; then
    warn 'listening TCP sockets detected; review before promoting lockdown rules'
  fi
elif have netstat; then
  listeners=$(netstat -ltn 2>/dev/null | awk 'NR > 2 { count++ } END { print count + 0 }')
  info "tcp-listeners: $listeners"
else
  info 'tcp-listeners: not checked; ss/netstat not available'
fi

if have systemctl; then
  ssh_state=$(systemctl is-enabled ssh 2>/dev/null || systemctl is-enabled sshd 2>/dev/null || printf unknown)
  info "ssh-service-enabled: $ssh_state"
else
  info 'ssh-service-enabled: not checked; systemctl not available'
fi

info 'planned-loopback: allow'

case "$PROFILE" in
  secure-default)
    info 'planned-inbound: deny by default'
    info 'planned-outbound: allow dns, dhcp, ntp, https-update, package-mirror only when explicitly configured'
    info 'planned-ssh: disabled unless BASE1_ALLOW_SSH=1 and appliance policy exists'
    info 'planned-phase1-host-tools: denied'
    ;;
  offline)
    info 'planned-inbound: deny all non-loopback'
    info 'planned-outbound: deny all non-loopback after installation artifacts are present'
    info 'planned-ssh: disabled'
    info 'planned-phase1-host-tools: denied'
    ;;
  appliance)
    info 'planned-inbound: deny by default; optional single management path only after operator approval'
    if [ "$ALLOW_SSH" = "1" ]; then
      warn 'ssh would require key-only auth, rate limits, host logging, and explicit appliance approval'
    else
      info 'planned-ssh: disabled'
    fi
    info 'planned-outbound: allow updates and time sync; deny broad service exposure'
    info 'planned-phase1-host-tools: denied during normal boot'
    ;;
  dev)
    warn 'dev profile is relaxed and must not be used for release images'
    info 'planned-inbound: report only'
    info 'planned-outbound: report only'
    info 'planned-ssh: operator managed'
    info 'planned-phase1-host-tools: governed by explicit Phase1 trust gates'
    ;;
esac

case "$TARGET" in
  raspberry-pi|raspberry-pi-candidate)
    info 'target-note: keep serial console and local recovery visible before network lockout'
    ;;
  x200|x200-or-generic-candidate)
    info 'target-note: prefer offline-first docs and keyboard-only recovery before network lockout'
    ;;
  generic|auto)
    info 'target-note: no hardware-specific override selected'
    ;;
  *)
    warn "unknown target hint: $TARGET"
    ;;
esac

info 'audit-path: /var/log/phase1-host/base1-network-lockdown.log'
info 'promotion-required: design doc, recovery path, rollback path, tests, and final operator confirmation'
info 'dry-run complete; no network or host changes were made'

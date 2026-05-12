#!/usr/bin/env sh
# black-phase1 doctor helper.
#
# Usage:
#   sh scripts/black-phase1-doctor.sh
#   sh scripts/black-phase1-doctor.sh /dev/sdb
#
# Purpose:
#   Show whether the rapid test branch is ready for Mac push, X200 test prep,
#   or promotion back to edge/stable.

set -u

TARGET="${1:-}"
TEST_BRANCH="black-phase1"
EDGE_BRANCH="edge/stable"
OUT_DIR="build/black-phase1-doctor"
REPORT="$OUT_DIR/black-phase1-doctor.env"

mkdir -p "$OUT_DIR"
: > "$REPORT"

kv() { printf '%s=%s\n' "$1" "$2" | tee -a "$REPORT" >/dev/null; }
has() { command -v "$1" >/dev/null 2>&1; }
say() { printf '%s\n' "$*"; }

failures=""
warnings=""
add_failure() { failures="${failures}${1};"; }
add_warning() { warnings="${warnings}${1};"; }

say "black-phase1 doctor"
say ""

if [ ! -d .git ]; then
  say "RESULT: blocked_with_exact_reason"
  say "failure: not in a git repository"
  exit 1
fi

current="$(git rev-parse --abbrev-ref HEAD 2>/dev/null || echo unknown)"
head="$(git log -1 --oneline 2>/dev/null || echo unknown)"
status="$(git status --short 2>/dev/null || true)"
arch="$(uname -m 2>/dev/null || echo unknown)"
root_src="$(findmnt -no SOURCE / 2>/dev/null || echo unknown)"

kv BLACK_PHASE1_DOCTOR_BRANCH "$current"
kv BLACK_PHASE1_DOCTOR_HEAD "$head"
kv BLACK_PHASE1_DOCTOR_HOST_ARCH "$arch"
kv BLACK_PHASE1_DOCTOR_ROOT_SOURCE "$root_src"
kv BLACK_PHASE1_DOCTOR_TARGET "${TARGET:-none}"

say "branch : $current"
say "head   : $head"
say "arch   : $arch"
say "root   : $root_src"

[ "$current" = "$TEST_BRANCH" ] || add_failure "not-on-black-phase1"
[ -z "$status" ] || add_warning "working-tree-has-local-changes"

say ""
say "tool checks:"
for tool in git cargo rustup file sh awk grep sha256sum sudo findmnt lsblk mount umount grub-install parted mkfs.vfat cpio gzip; do
  if has "$tool"; then
    kv "BLACK_PHASE1_TOOL_$(printf '%s' "$tool" | tr 'a-z-' 'A-Z_')" "yes"
    say "ok: $tool"
  else
    kv "BLACK_PHASE1_TOOL_$(printf '%s' "$tool" | tr 'a-z-' 'A-Z_')" "no"
    say "no: $tool"
    case "$tool" in
      git|sh|awk|grep) add_failure "missing-$tool" ;;
      cargo|file) add_warning "missing-$tool" ;;
      grub-install|parted|mkfs.vfat|cpio|gzip) add_warning "missing-media-tool-$tool" ;;
    esac
  fi
done

say ""
say "branch freshness:"
git fetch origin "$TEST_BRANCH" "$EDGE_BRANCH" >/dev/null 2>&1 || add_warning "git-fetch-failed"
behind_edge="unknown"
ahead_edge="unknown"
if git rev-parse "origin/$EDGE_BRANCH" >/dev/null 2>&1; then
  counts="$(git rev-list --left-right --count "origin/$EDGE_BRANCH...HEAD" 2>/dev/null || echo 'unknown unknown')"
  behind_edge="$(printf '%s' "$counts" | awk '{print $1}')"
  ahead_edge="$(printf '%s' "$counts" | awk '{print $2}')"
fi
kv BLACK_PHASE1_BEHIND_EDGE "$behind_edge"
kv BLACK_PHASE1_AHEAD_EDGE "$ahead_edge"
say "behind edge: $behind_edge"
say "ahead edge : $ahead_edge"

if [ "$behind_edge" != "0" ] && [ "$behind_edge" != "unknown" ]; then
  add_warning "black-phase1-behind-edge"
fi

say ""
say "binary check:"
if [ -x target/release/phase1 ]; then
  bin_file="$(file target/release/phase1 2>/dev/null || echo unknown)"
  kv BLACK_PHASE1_BINARY_PRESENT yes
  kv BLACK_PHASE1_BINARY_FILE "$bin_file"
  say "binary: $bin_file"
  case "$bin_file" in *x86-64*) kv BLACK_PHASE1_BINARY_X200_COMPAT yes ;; *) kv BLACK_PHASE1_BINARY_X200_COMPAT no; add_warning "binary-not-x86_64" ;; esac
else
  kv BLACK_PHASE1_BINARY_PRESENT no
  say "binary: missing target/release/phase1"
  add_warning "binary-not-built"
fi

if [ -n "$TARGET" ]; then
  say ""
  say "target check: $TARGET"
  if [ -b "$TARGET" ]; then
    kv BLACK_PHASE1_TARGET_BLOCK_DEVICE yes
    case "$root_src" in
      "$TARGET"|"$TARGET"[0-9]*|"$TARGET"p[0-9]*)
        kv BLACK_PHASE1_TARGET_IS_ROOT yes
        add_failure "target-is-root-device"
        ;;
      *) kv BLACK_PHASE1_TARGET_IS_ROOT no ;;
    esac
    lsblk -o NAME,PATH,SIZE,MODEL,TRAN,RM,MOUNTPOINTS "$TARGET" 2>/dev/null || true
  else
    kv BLACK_PHASE1_TARGET_BLOCK_DEVICE no
    add_failure "target-not-block-device"
  fi
fi

kv BLACK_PHASE1_DOCTOR_WARNINGS "${warnings:-none}"
kv BLACK_PHASE1_DOCTOR_FAILURES "${failures:-none}"

say ""
if [ -n "$failures" ]; then
  kv BLACK_PHASE1_DOCTOR_RESULT blocked_with_exact_reason
  say "RESULT: blocked_with_exact_reason"
  say "failures: $failures"
  say "warnings: ${warnings:-none}"
  say "report: $REPORT"
  exit 1
fi

kv BLACK_PHASE1_DOCTOR_RESULT ready_for_rapid_work
say "RESULT: ready_for_rapid_work"
say "warnings: ${warnings:-none}"
say "report: $REPORT"

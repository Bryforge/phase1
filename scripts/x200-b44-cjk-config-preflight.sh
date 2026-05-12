#!/usr/bin/env sh
# Phase1 B44 CJK/config preflight.
#
# No writes. This checks whether the host can prepare a Japanese/CJK rendering
# path and whether the current boot image should include a manual config entry.

set -eu

OUT_DIR="${BASE1_B44_OUT:-build/base1-b44-cjk-config-preflight}"
REPORT="$OUT_DIR/b44-cjk-config-preflight.env"
mkdir -p "$OUT_DIR"
: > "$REPORT"

kv() { printf '%s=%s\n' "$1" "$2" | tee -a "$REPORT" >/dev/null; }
has() { command -v "$1" >/dev/null 2>&1; }

printf 'PHASE1 B44 CJK + CONFIG PREFLIGHT\n'
printf 'no writes will be performed\n\n'

renderer=none
if has fbterm; then renderer=fbterm; elif has jfbterm; then renderer=jfbterm; elif has kmscon; then renderer=kmscon; fi
kv BASE1_B44_CJK_RENDERER "$renderer"
printf 'renderer candidate: %s\n' "$renderer"

font_candidate=none
for path in \
  /usr/share/fonts/opentype/noto \
  /usr/share/fonts/truetype/noto \
  /usr/share/fonts/truetype/dejavu \
  /usr/share/consolefonts
 do
  if [ -d "$path" ]; then
    count="$(find "$path" -type f 2>/dev/null | wc -l | awk '{print $1}')"
    printf 'font dir: %s (%s files)\n' "$path" "$count"
    case "$path" in
      *noto*) [ "$count" -gt 0 ] && font_candidate=noto ;;
      *consolefonts*) [ "$count" -gt 0 ] && [ "$font_candidate" = none ] && font_candidate=consolefonts ;;
      *dejavu*) [ "$count" -gt 0 ] && [ "$font_candidate" = none ] && font_candidate=dejavu ;;
    esac
  fi
done

if find /usr/share/fonts /usr/share/consolefonts -type f 2>/dev/null | grep -Ei 'noto.*cjk|cjk|unifont|ipag|takao|sazanami' >/dev/null 2>&1; then
  kv BASE1_B44_CJK_FONT "yes"
  printf 'CJK font candidate: yes\n'
else
  kv BASE1_B44_CJK_FONT "no"
  printf 'CJK font candidate: no\n'
fi
kv BASE1_B44_FONT_CANDIDATE "$font_candidate"

if [ "$renderer" = none ]; then
  kv BASE1_B44_RESULT "blocked_missing_cjk_renderer"
  printf '\nRESULT: blocked_missing_cjk_renderer\n'
  printf 'Install one of: fbterm, jfbterm, kmscon. Also install fonts-noto-cjk or unifont.\n'
else
  kv BASE1_B44_RESULT "ready_for_cjk_packaging"
  printf '\nRESULT: ready_for_cjk_packaging\n'
fi

kv BASE1_B44_CONFIG_BOOT_ENTRY_NEEDED "yes"
kv BASE1_B44_MAIN_AUTOBOOT "yes"
kv BASE1_B44_CONFIG_ENTRY "Configure Phase1 Boot Card"
kv BASE1_B44_CJK_ENTRY "Start Phase1 Japanese Framebuffer UTF-8"
printf 'report: %s\n' "$REPORT"

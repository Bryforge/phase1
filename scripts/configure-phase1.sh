#!/usr/bin/env sh
set -eu

ROOT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
LOCAL_DIR="${PHASE1_LOCAL_DIR:-$ROOT_DIR/.phase1}"
CONFIG_DIR="$LOCAL_DIR/config"
BIN_DIR="$LOCAL_DIR/bin"
LOG_DIR="$LOCAL_DIR/logs"
CONFIG_FILE="$CONFIG_DIR/phase1.env"
DRY_RUN=0
RUN_BASE1=0
RUN_QUALITY=0
INSTALL_TERMINAL=1

usage() {
    cat <<'EOF'
Configure Phase1

Usage:
  sh scripts/configure-phase1.sh [options]

Options:
  --dry-run        Show what would be configured.
  --base1          Run Base1 read-only preflight after configuration.
  --quality        Run lightweight quality checks after configuration.
  --no-terminal    Skip local terminal wrapper creation.
  -h, --help       Show help.

After configuration, launch with:
  sh phase1
  ./phase1
EOF
}

while [ "$#" -gt 0 ]; do
    case "$1" in
        --dry-run) DRY_RUN=1 ;;
        --base1) RUN_BASE1=1 ;;
        --quality) RUN_QUALITY=1 ;;
        --no-terminal) INSTALL_TERMINAL=0 ;;
        -h|--help) usage; exit 0 ;;
        *) echo "unknown option: $1" >&2; usage >&2; exit 1 ;;
    esac
    shift
done

say() { printf '%s\n' "$*"; }

write_config() {
    if [ "$DRY_RUN" = "1" ]; then
        say "dry-run: write $CONFIG_FILE"
        return 0
    fi
    mkdir -p "$CONFIG_DIR" "$BIN_DIR" "$LOG_DIR"
    cat > "$CONFIG_FILE" <<EOF
# Phase1 integrated configuration
PHASE1_HOME="$ROOT_DIR"
PHASE1_LOCAL_DIR="$LOCAL_DIR"
PHASE1_THEME="cyber"
PHASE1_COLOR_MODE="auto"
PHASE1_SAFE_MODE="1"
PHASE1_DEVICE_MODE="desktop"
PHASE1_GINA_ENABLED="1"
PHASE1_AI_MODE="offline"
PHASE1_TERMINAL_TITLE="Phase1 Command Center"
PHASE1_LAUNCH_COMMAND="./phase1"
PHASE1_BASE1_PREFLIGHT="scripts/base1-preflight.sh"
PHASE1_QUALITY_SCORE="scripts/quality-score.sh"
EOF
}

install_terminal_wrapper() {
    if [ "$INSTALL_TERMINAL" != "1" ]; then
        return 0
    fi
    if [ "$DRY_RUN" = "1" ]; then
        say "dry-run: write $BIN_DIR/phase1"
        say "dry-run: write $BIN_DIR/phase1-terminal"
        return 0
    fi
    mkdir -p "$BIN_DIR"
    cat > "$BIN_DIR/phase1" <<EOF
#!/usr/bin/env sh
exec sh "$ROOT_DIR/phase1" "\$@"
EOF
    cat > "$BIN_DIR/phase1-terminal" <<EOF
#!/usr/bin/env sh
exec sh "$ROOT_DIR/terminal/bin/phase1-terminal" "\$@"
EOF
    chmod 0755 "$BIN_DIR/phase1" "$BIN_DIR/phase1-terminal"
}

make_launchers_executable() {
    if [ "$DRY_RUN" = "1" ]; then
        say "dry-run: chmod +x phase1 phase1 scripts/configure-phase1.sh scripts/install-phase1-command.sh"
        return 0
    fi
    chmod 0755 "$ROOT_DIR/phase1" "$ROOT_DIR/phase1" "$ROOT_DIR/scripts/configure-phase1.sh" "$ROOT_DIR/scripts/install-phase1-command.sh"
}

validate_files() {
    missing=0
    for file in phase1 phase1 plugins/gina.wasi plugins/ai.wasi scripts/base1-preflight.sh; do
        if [ ! -f "$ROOT_DIR/$file" ]; then
            say "missing: $file"
            missing=$((missing + 1))
        fi
    done
    if [ "$missing" -gt 0 ]; then
        say "configuration warning: $missing expected files are missing"
    fi
}

run_base1() {
    if [ -f "$ROOT_DIR/scripts/base1-preflight.sh" ]; then
        sh "$ROOT_DIR/scripts/base1-preflight.sh"
    else
        say "Base1 preflight unavailable."
    fi
}

run_quality() {
    if [ -f "$ROOT_DIR/scripts/quality-check.sh" ]; then
        sh "$ROOT_DIR/scripts/quality-check.sh" files
        sh "$ROOT_DIR/scripts/quality-check.sh" scripts
        sh "$ROOT_DIR/scripts/quality-check.sh" score
    elif [ -f "$ROOT_DIR/scripts/quality-score.sh" ]; then
        sh "$ROOT_DIR/scripts/quality-score.sh"
    else
        say "Quality scripts unavailable."
    fi
}

say "Phase1 absolute configuration"
say "home       : $ROOT_DIR"
say "local dir  : $LOCAL_DIR"
say "config     : $CONFIG_FILE"
say "launcher   : ./phase1"
say "gina       : offline operations assistant"
say "base1      : read-only preflight available when present"
say "terminal   : local wrappers enabled"

write_config
install_terminal_wrapper
make_launchers_executable
validate_files

if [ "$RUN_BASE1" = "1" ]; then
    run_base1
fi
if [ "$RUN_QUALITY" = "1" ]; then
    run_quality
fi

say "Configuration complete."
say "Launch command: sh phase1"
say "Executable command: ./phase1"

#!/usr/bin/env sh
set -eu

mode="${1:-quick}"

run() {
    echo "$ $*"
    "$@"
}

check_file() {
    if [ ! -f "$1" ]; then
        echo "quality: missing required file: $1" >&2
        exit 1
    fi
}

check_script_syntax() {
    file="$1"
    if [ -f "$file" ]; then
        run sh -n "$file"
    fi
}

check_required_files() {
    for file in \
        README.md \
        SECURITY.md \
        SECURITY_REVIEW.md \
        UPDATE_PROTOCOL.md \
        QUALITY.md \
        QUALITY_SCORECARD.md \
        Cargo.toml \
        src/main.rs \
        .github/workflows/rust-ci.yml
    do
        check_file "$file"
    done
}

check_scripts() {
    for file in scripts/*.sh; do
        check_script_syntax "$file"
    done
}

check_base1_docs() {
    run sh scripts/base1-doc-integrity.sh
    run sh scripts/base1-link-check.sh
    run sh scripts/base1-test-inventory-verify.sh
}

quick() {
    check_required_files
    check_scripts
    check_base1_docs
    run sh scripts/quality-score.sh
    run cargo fmt --all -- --check
    run cargo check --all-targets
    run cargo clippy --all-targets -- -D warnings
    run cargo test --all-targets
}

full() {
    quick
    if command -v cargo-audit >/dev/null 2>&1; then
        run cargo audit
    else
        echo "quality: cargo-audit not installed; skipping RustSec audit"
    fi
    if command -v cargo-deny >/dev/null 2>&1; then
        run cargo deny check
    else
        echo "quality: cargo-deny not installed; skipping dependency policy check"
    fi
}

case "$mode" in
    quick)
        quick
        ;;
    full)
        full
        ;;
    score)
        run sh scripts/quality-score.sh
        ;;
    scripts)
        check_scripts
        ;;
    files)
        check_required_files
        ;;
    base1-docs)
        check_base1_docs
        ;;
    help|-h|--help)
        cat <<'EOF'
Phase1 quality check

Usage:
  sh scripts/quality-check.sh quick
  sh scripts/quality-check.sh full
  sh scripts/quality-check.sh score
  sh scripts/quality-check.sh scripts
  sh scripts/quality-check.sh files
  sh scripts/quality-check.sh base1-docs
EOF
        ;;
    *)
        echo "quality: unknown mode: $mode" >&2
        exit 1
        ;;
esac

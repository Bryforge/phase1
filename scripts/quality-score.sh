#!/usr/bin/env sh
set -eu

score=0
max=100
missing=""

has_file() {
    [ -f "$1" ]
}

has_dir() {
    [ -d "$1" ]
}

add_missing() {
    if [ "$missing" = "" ]; then
        missing="$1"
    else
        missing="$missing, $1"
    fi
}

award_docs() {
    points=0
    found=0
    for file in README.md SECURITY.md SECURITY_REVIEW.md UPDATE_PROTOCOL.md QUALITY.md docs/quality/QUALITY_SCORECARD.md; do
        if has_file "$file"; then
            found=$((found + 1))
        else
            add_missing "$file"
        fi
    done
    if [ "$found" -eq 6 ]; then
        points=20
    else
        points=$((found * 3))
    fi
    echo "$points"
}

award_scripts() {
    points=0
    for file in scripts/quality-check.sh scripts/quality-score.sh scripts/base1-preflight.sh scripts/test-release-metadata.sh scripts/test-website.sh; do
        if has_file "$file"; then
            points=$((points + 4))
        else
            add_missing "$file"
        fi
    done
    echo "$points"
}

award_rust() {
    if has_file Cargo.toml && has_file src/main.rs; then
        echo 10
    else
        add_missing "Cargo.toml or src/main.rs"
        echo 0
    fi
}

award_tests() {
    if has_dir tests || find src -name '*.rs' -type f 2>/dev/null | xargs grep -q '#\[test\]' 2>/dev/null; then
        echo 15
    else
        add_missing "tests"
        echo 0
    fi
}

award_ci() {
    if has_dir .github/workflows && find .github/workflows -type f | grep -q .; then
        echo 15
    else
        add_missing ".github/workflows"
        echo 0
    fi
}

award_safety() {
    points=0
    for file in SECURITY.md SECURITY_REVIEW.md src/policy.rs src/ops_log.rs; do
        if has_file "$file"; then
            points=$((points + 3))
        else
            add_missing "$file"
        fi
    done
    if [ "$points" -gt 10 ]; then points=10; fi
    echo "$points"
}

award_release() {
    points=0
    for file in UPDATE_PROTOCOL.md CHANGELOG.md Cargo.toml; do
        if has_file "$file"; then
            points=$((points + 4))
        else
            add_missing "$file"
        fi
    done
    if [ "$points" -gt 10 ]; then points=10; fi
    echo "$points"
}

docs=$(award_docs)
scripts=$(award_scripts)
rust=$(award_rust)
tests=$(award_tests)
ci=$(award_ci)
safety=$(award_safety)
release=$(award_release)
score=$((docs + scripts + rust + tests + ci + safety + release))

cat <<EOF
Phase1 Quality Scorecard

required_docs    : $docs/20
required_scripts : $scripts/20
rust_source      : $rust/10
tests            : $tests/15
ci_workflows     : $ci/15
safety_docs      : $safety/10
release_docs     : $release/10
score            : $score/$max
EOF

if [ "$missing" != "" ]; then
    echo "missing          : $missing"
fi

if [ "$score" -ge 90 ]; then
    echo "rating           : excellent"
elif [ "$score" -ge 75 ]; then
    echo "rating           : good"
elif [ "$score" -ge 50 ]; then
    echo "rating           : risky"
else
    echo "rating           : not release-ready"
fi

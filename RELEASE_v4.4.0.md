# Phase1 v4.4.0 Stable

Phase1 v4.4.0 is the stable representation of the 4.2 development checkpoint.

## Status

- Stable version: v4.4.0
- Previous stable: v4.3.0
- Compatibility base: v3.6.0
- Quality score: 100/100
- Release profile: stable
- Default posture: safe mode on, host trust off, volatile state unless explicitly enabled

## Validated gates

- cargo fmt --all -- --check
- cargo check --all-targets
- cargo clippy --all-targets -- -D warnings
- cargo test --all-targets
- sh scripts/quality-score.sh
- sh scripts/test-release-metadata.sh
- sh scripts/test-website.sh

## Highlights

- Stable v4.4.0 release metadata
- Quality score coverage raised to 100/100
- Storage helper release-profile validation
- Correct stable, previous-stable, and compatibility-base reporting
- Aligned host-tool status wording across sysinfo and security
- Short mobile-safe boot notices
- Safe default boot posture restored for public/demo use

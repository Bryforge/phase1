# v4.2.0 Stable

- Promoted Phase1 v4.2.0 from development checkpoint to stable representation.
- Preserved v4.1.0 as previous stable and v3.6.0 as compatibility base.
- Raised quality score coverage to 100/100.
- Added release metadata and website validation scripts.
- Confirmed safe default boot posture for public/demo use.

# Changelog

## v3.6.0 - 2026-05-05

Release focus: roadmap foundation, release readiness, and demo-quality operator experience.

### Added

- `dash` / `dashboard` command with compact and full operator dashboard output.
- `capabilities` / `caps` command for command capability and guard visibility.
- Registry-backed alias normalization for built-in commands.
- Roadmap design index in `ROADMAP_DESIGNS.md`.
- Roadmap design documents under `docs/roadmap/`.
- End-to-end smoke coverage for roadmap aliases, capabilities, dashboard output, network fallbacks, and command behavior.
- `RELEASE_NOTES_v3.6.0.md`.

### Changed

- Bumped runtime and package version from `3.5.0` to `3.6.0`.
- Updated README for release-oriented usage and demo commands.
- Refreshed `Cargo.lock` for the current std-only dependency surface.
- Help quick actions now include `dash --compact`.

### Fixed

- Stabilized command aliases through a single registry path.
- Kept built-in commands ahead of plugins during dispatch.
- Improved release tests to check current version strings.

## v3.5.0

### Added

- Advanced mobile-friendly boot UI.
- Modern prompt renderer.
- End-to-end smoke test harness.
- macOS Wi-Fi scan fallbacks.
- Roadmap groundwork for shell, kernel, security, pipelines, plugins, and TUI.

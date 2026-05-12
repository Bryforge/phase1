# Phase1

<p align="center">
  <a href="https://bryforge.github.io/phase1/">
    <img src="assets/phase1_base_fyr_banner1.png" alt="Phase1 Base1 Fyr public banner" width="780">
  </a>
</p>

Phase1 is now organized as three connected project areas:

```text
phase1/  runtime, shell, operator console, UI, local tools
base1/   boot foundation, hardware targets, QEMU/X200 paths, recovery, media prep
fyr/     Phase1-native language, examples, parser/runtime work, package model
```

Active model:

```text
testing -> active repair/rebuild branch
stable  -> known-good promoted branch
```

Start with [`TRACKER.md`](TRACKER.md) and [`FOCUS.md`](FOCUS.md). Historical, duplicate, experimental, or unclear files are preserved under [`junk/`](junk/).

## Quick start

```bash
git clone https://github.com/Bryforge/phase1.git
cd phase1
cargo run
```

Validation:

```bash
cargo check --all-targets
cargo test --all-targets
```

Legacy launcher compatibility:

```bash
sh scripts/launchers/phase1
```

## Repository map

| Area | Purpose |
| --- | --- |
| [`phase1/`](phase1/) | Phase1 runtime/operator docs and active runtime direction. |
| [`base1/`](base1/) | Base1 boot, hardware, recovery, target, and release docs. |
| [`fyr/`](fyr/) | Fyr language docs and future language work. |
| [`src/`](src/) | Current Rust implementation. |
| [`scripts/`](scripts/) | Active scripts and compatibility launchers. |
| [`docs/`](docs/) | Cross-project docs, OS plans, releases, website docs, navigation. |
| [`shared/`](shared/) | Shared policy, quality, contribution, security, and repo docs. |
| [`root-site/`](root-site/) | Public website files moved out of root. |
| [`junk/`](junk/) | Preserved old, duplicate, generated, experimental, or legacy files. |
| [`tests/`](tests/) | Rust tests and documentation guard tests. |

## Phase1

Phase1 is the operator/runtime system.

Key docs:

- [`phase1/README.md`](phase1/README.md)
- [`phase1/docs/FEATURE_STATUS.md`](phase1/docs/FEATURE_STATUS.md) — moved from `FEATURE_STATUS.md`.
- [`phase1/docs/LEARNING.md`](phase1/docs/LEARNING.md) — moved from `LEARNING.md`.
- [`phase1/docs/OPERATOR_SHELL.md`](phase1/docs/OPERATOR_SHELL.md)
- [`phase1/docs/TERMINAL.md`](phase1/docs/TERMINAL.md)
- [`phase1/docs/STORAGE_GIT_RUST.md`](phase1/docs/STORAGE_GIT_RUST.md)

## Base1

Base1 is the boot, hardware, recovery, and runtime foundation for Phase1.

Primary Base1 docs:

- [`base1/README.md`](base1/README.md)
- [`base1/SECURITY_MODEL.md`](base1/SECURITY_MODEL.md)
- [`base1/HARDWARE_TARGETS.md`](base1/HARDWARE_TARGETS.md)
- [`base1/LIBREBOOT_PROFILE.md`](base1/LIBREBOOT_PROFILE.md)
- [`docs/os/ROADMAP.md`](docs/os/ROADMAP.md)
- [`docs/os/BOOT_READINESS_STATUS.md`](docs/os/BOOT_READINESS_STATUS.md)
- [`docs/os/BOOT_READINESS_RACE_PLAN.md`](docs/os/BOOT_READINESS_RACE_PLAN.md)
- [`docs/os/X86_64_BOOT_SUPPORT_ROADMAP.md`](docs/os/X86_64_BOOT_SUPPORT_ROADMAP.md)
- [`docs/os/BASE1_DRY_RUN_COMMANDS.md`](docs/os/BASE1_DRY_RUN_COMMANDS.md)
- [`docs/os/BASE1_IMAGE_BUILDER.md`](docs/os/BASE1_IMAGE_BUILDER.md) — Base1 image-builder design.
- [`docs/os/BASE1_INSTALLER_DRY_RUN.md`](docs/os/BASE1_INSTALLER_DRY_RUN.md) — Base1 installer dry-run design.
- [`docs/os/INSTALLER_RECOVERY.md`](docs/os/INSTALLER_RECOVERY.md)

Safe Base1 checks:

```bash
sh scripts/base1-x86_64-detect.sh --dry-run
sh scripts/base1-b2-assembly-dry-run.sh --dry-run --profile x86_64-vm-validation
sh scripts/base1-b2-test-suite-check.sh --dry-run --write-report
```

Base1 does not currently claim installer readiness, hardened status, physical hardware validation, release-candidate readiness, or daily-driver readiness.

## Fyr

Fyr is the Phase1-native language track.

- [`fyr/README.md`](fyr/README.md)
- [`fyr/docs/PHASE1_NATIVE_LANGUAGE.md`](fyr/docs/PHASE1_NATIVE_LANGUAGE.md) — moved from `PHASE1_NATIVE_LANGUAGE.md`.
- [`docs/fyr/ROADMAP.md`](docs/fyr/ROADMAP.md)

## Public assets and branding

Current verified public assets:

| Asset | Path |
| --- | --- |
| Public Phase1/Base1/Fyr banner | [`assets/phase1_base_fyr_banner1.png`](assets/phase1_base_fyr_banner1.png) |
| Phase1 boot splash PNG | [`assets/phase1-splash.png`](assets/phase1-splash.png) |
| Fyr symbol | [`assets/fyr_symbol.png`](assets/fyr_symbol.png) |
| Fyr word mark | [`assets/fyr_word.png`](assets/fyr_word.png) |

Asset policy lives in [`assets/README.md`](assets/README.md). Website docs live in [`docs/website/README.md`](docs/website/README.md).

## Documentation indexes

- [`docs/README.md`](docs/README.md)
- [`docs/REPOSITORY_NAVIGATION.md`](docs/REPOSITORY_NAVIGATION.md)
- [`docs/REORGANIZATION_PLAN.md`](docs/REORGANIZATION_PLAN.md)
- [`docs/REORGANIZATION_RESET_PLAN.md`](docs/REORGANIZATION_RESET_PLAN.md)
- [`docs/releases/CHANGELOG.md`](docs/releases/CHANGELOG.md)
- [`docs/website/README.md`](docs/website/README.md)
- [`scripts/README.md`](scripts/README.md)
- [`junk/README.md`](junk/README.md)

Shared governance docs:

- [`shared/docs/CONTRIBUTING.md`](shared/docs/CONTRIBUTING.md) — moved from `CONTRIBUTING.md`.
- [`shared/docs/QUALITY.md`](shared/docs/QUALITY.md) — moved from `QUALITY.md`.
- [`shared/docs/SECURITY.md`](shared/docs/SECURITY.md) — moved from `SECURITY.md`.
- [`shared/docs/SECURITY_REVIEW.md`](shared/docs/SECURITY_REVIEW.md)
- [`shared/docs/UPDATE_PROTOCOL.md`](shared/docs/UPDATE_PROTOCOL.md)

## Quality

```bash
cargo fmt --all -- --check
cargo check --all-targets
cargo test --all-targets
```

Optional helpers:

```bash
sh scripts/quality-check.sh quick
sh scripts/quality-check.sh full
sh scripts/quality-check.sh base1-docs
sh scripts/quality-check.sh base1-reorg
sh scripts/quality-check.sh security-crypto-docs
```

## License

Phase1 is released under GPL-3.0-only.

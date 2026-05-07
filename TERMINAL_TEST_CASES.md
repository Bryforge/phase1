# Phase1 Terminal Test Cases

This document defines quality, color, safety, and performance test cases for Phase1 Terminal.

The goal is to keep Phase1 beautiful, reliable, fast, and safe across Linux and macOS terminal environments.

## Core validation command

```bash
sh scripts/test-phase1-terminal.sh
```

This is the CI-backed terminal validation entry point.

## Test matrix

| Area | Command | Expected result |
| --- | --- | --- |
| Syntax | `sh -n terminal/bin/phase1-terminal` | no syntax errors |
| Syntax | `sh -n scripts/install-phase1-terminal.sh` | no syntax errors |
| Syntax | `sh -n scripts/uninstall-phase1-terminal.sh` | no syntax errors |
| Help | `phase1-terminal help` | lists run/config/profile/colors/theme/selftest/benchmark commands |
| Version | `phase1-terminal version` | prints `phase1-terminal <version>` |
| Env | `phase1-terminal env` | includes terminal, theme, color, safety values |
| Doctor | `phase1-terminal doctor` | prints text diagnostics |
| Doctor JSON | `phase1-terminal doctor --json` | includes version, theme, detected color mode |
| Color detect | `phase1-terminal colors detect` | reports configured and detected color support |
| Color swatches | `phase1-terminal colors swatches` | prints all theme swatches or text fallback |
| Theme list | `phase1-terminal theme list` | lists supported themes |
| Theme preview | `phase1-terminal theme preview matrix` | previews matrix palette |
| Profiles | `phase1-terminal profile list` | lists all profiles |
| Self-test | `phase1-terminal selftest` | returns `selftest: passed` |
| Benchmark | `phase1-terminal benchmark 2` | returns `status    : completed` |
| Installer dry-run | `sh scripts/install-phase1-terminal.sh --dry-run` | prints planned writes and exits successfully |
| Uninstaller dry-run | `sh scripts/uninstall-phase1-terminal.sh --dry-run` | prints planned removals and exits successfully |

## Color quality tests

### CQ-1: Auto color detection

Command:

```bash
phase1-terminal colors detect
```

Expected:

- reports configured color mode
- reports detected color mode
- includes `TERM`
- includes `COLORTERM`
- includes `tput colors`
- does not fail when `tput` is missing or unavailable

### CQ-2: No-color compatibility

Command:

```bash
NO_COLOR=1 phase1-terminal colors detect
```

Expected:

- detected color mode is `mono`
- command exits successfully
- output remains readable without ANSI styling

### CQ-3: Theme swatch fallback

Command:

```bash
NO_COLOR=1 phase1-terminal theme preview all
```

Expected:

- all themes are still listed
- output uses text fallback instead of relying on color blocks

### CQ-4: Truecolor environment pass-through

Command:

```bash
PHASE1_COLOR_MODE=truecolor phase1-terminal env
```

Expected:

- includes `PHASE1_COLOR_MODE=truecolor`
- includes detected color mode data
- does not enable unsafe host behavior

### CQ-5: Phase1 beauty themes

Command:

```bash
phase1-terminal theme preview all
```

Expected themes:

- `cyber`
- `matrix`
- `amber`
- `mono`
- `safe`
- `developer`
- `base1`
- `ice`
- `synthwave`
- `crimson`

## Performance tests

### PERF-1: Terminal command self-test

Command:

```bash
phase1-terminal selftest
```

Expected:

- completes quickly
- reports color detection status
- verifies doctor JSON includes color fields
- verifies theme preview works
- returns success

### PERF-2: Lightweight benchmark

Command:

```bash
phase1-terminal benchmark 25
```

Expected:

- runs repeated detection/doctor/theme operations
- prints iterations
- prints elapsed seconds
- prints configured budget
- exits successfully

### PERF-3: CI benchmark smoke

Command:

```bash
phase1-terminal benchmark 2
```

Expected:

- stable under CI
- no dependence on interactive terminal capabilities
- exits successfully

## Safety tests

### SAFE-1: Defaults remain safe

Command:

```bash
phase1-terminal env
```

Expected:

- `PHASE1_SAFE_MODE=1`
- `PHASE1_DEVICE_MODE=desktop`
- no host trust environment variable is set
- no external AI provider is set

### SAFE-2: Profiles do not enable host trust

Command:

```bash
phase1-terminal profile apply developer
phase1-terminal config show
```

Expected:

- developer profile may change theme
- safe mode remains enabled
- no host network mutation is enabled
- no external AI provider is enabled

### SAFE-3: Config keys are allowlisted

Command:

```bash
phase1-terminal config set UNSAFE_KEY=1
```

Expected:

- command fails
- unsupported key is rejected
- config file is not expanded with arbitrary keys

## Installer tests

### INSTALL-1: Dry-run install

Command:

```bash
sh scripts/install-phase1-terminal.sh --dry-run --no-alias
```

Expected:

- does not write files
- prints install target
- prints config target
- prints color/theme defaults

### INSTALL-2: Dry-run uninstall

Command:

```bash
sh scripts/uninstall-phase1-terminal.sh --dry-run
```

Expected:

- does not delete files
- prints planned removals
- keeps config unless `--remove-config` is explicit

## Manual visual tests

These are not required in CI but should be checked before major releases:

```bash
phase1-terminal theme preview all
phase1-terminal profile apply cyber
phase1-terminal profile apply matrix
phase1-terminal profile apply amber
phase1-terminal profile apply ice
phase1-terminal profile apply crimson
phase1-terminal doctor --verbose
phase1-terminal gina
```

Visual pass criteria:

- theme previews are readable
- colors are vivid where terminal support exists
- mono fallback remains legible
- no unreadable low-contrast combination becomes the default
- Phase1 launch receives theme/color environment values

## Regression checklist

Before merging terminal changes, confirm:

- `sh scripts/test-phase1-terminal.sh` passes
- Rust CI passes terminal validation
- security workflow passes
- README/TERMINAL docs list new commands
- roadmap reflects implemented features
- terminal does not enable host trust, network mutation, or external AI providers

# Phase1 Edge Development

`edge/v4.2.6-dev` is the active development line after the stable `v4.0.0` release point.

## Current identity

| Item | Value |
| --- | --- |
| Development branch | `edge/v4.2.6-dev` |
| Development package version | `4.2.6-dev` |
| Stable release point | `v4.0.0` |
| Stable branch | `release/v4.0.0` |
| Previous stable | `v3.10.9` |
| Compatibility base | `v3.6.0` |

## v4.2.6 edge focus

- Tighten the public website for mobile and in-app browser layouts.
- Prevent horizontal scrolling on iPhone widths.
- Keep the hero, buttons, command strip, terminal demo, and logo stage inside the viewport.
- Reduce oversized mobile text and orbital-globe elements without flattening the visual identity.
- Refresh browser demo version output to match the edge track.

## Recent v4.2.5 edge focus

- Replace the launcher wording with a slower PHASE1 boot splash.
- Add visible preparation spinners for the launcher and boot flow.
- Add the red Japanese Easter egg greeting under the PHASE1 launcher splash.
- Change the boot selector node line to Japanese: `東京-01 // 全サブシステム正常`.
- Expand boot-screen help so the controls are readable before entering the shell.
- Clean up boot labels, capitalization, and operator copy.

## Development rules

- Keep the `-dev` suffix until the next stable promotion.
- Keep `v4.0.0` documentation intact for stable users.
- Label experimental work clearly in docs, tests, and command output.
- Keep safe defaults enabled while adding new capabilities.
- Run formatting, compile, Clippy, and tests before merging development work.

## Start development

```bash
git fetch origin
git checkout edge/v4.2.6-dev
cargo metadata --no-deps --format-version 1 | grep '"version"'
cargo run
```

Expected package version:

```text
4.2.6-dev
```

## Validation

```bash
cargo fmt --all -- --check
cargo check --all-targets
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
```

Before a future stable promotion, also run:

```bash
cargo audit
cargo deny check
```

## Future stable promotion path

1. Create a release branch from the development branch when ready.
2. Remove the `-dev` suffix from `Cargo.toml`.
3. Refresh `Cargo.lock`.
4. Update README, website demo output, wiki docs, in-system wiki fixtures, and release metadata tests.
5. Run the full stable validation gate.
6. Create the release tag only after validation passes.

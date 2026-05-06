# Phase1 Arena Development

Phase1 Arena is the focused game workspace for phase1. It replaces the earlier openDoom prototype name with a cleaner project-owned name and keeps the game implementation isolated from the rest of the simulator.

## Game name

Use **Phase1 Arena** in UI, docs, tests, and release notes.

Compatibility aliases such as `doom` can route to the arena launcher, but new work should use `arena` or `game`.

## Main files

```text
src/arena.rs          game logic, renderer, controls, and game workspace commands
src/wasm.rs           built-in WASI-lite launch routing for arena/game
tests/game.rs         focused integration coverage for the game workspace
scripts/test-game.sh  focused game-only check runner
plugins/arena.wasm    marker for the direct `arena` command in repo-root runs
plugins/game.wasm     marker for the direct `game` workspace command in repo-root runs
```

## Developer commands

Run the full focused game check:

```sh
sh scripts/test-game.sh
```

Run only the game integration suite:

```sh
cargo test --test game -- --nocapture
```

Run the built-in unit tests related to the arena and WASI launcher:

```sh
cargo test --bin phase1 arena
cargo test --bin phase1 wasm
```

## In-app commands

Inside phase1:

```text
wasm run arena demo
wasm run arena script d d d fire map
wasm inspect arena
wasm validate arena
wasm run game status
wasm run game files
wasm run game test-plan
wasm run game roadmap
```

From a repo-root run, the marker files also allow:

```text
arena demo
game status
```

## Design rules

- Keep the game clean-room and original. Do not import proprietary game assets, WAD files, names, maps, sprites, sounds, or code.
- Keep gameplay terminal-first, mobile-friendly, and readable on narrow screens.
- Prefer deterministic scripted checks before adding random behavior.
- Keep game work isolated to `src/arena.rs`, `tests/game.rs`, and game docs unless wiring requires a small launcher update.
- Use the third SemVer number for patch-level game fixes, and minor version bumps for meaningful game capability sets.

## Next game roadmap

- Add multiple compact arena maps.
- Add score persistence through the existing phase1 persistent-state system.
- Add color-safe optional sprites that respect the active theme.
- Add mobile-sized layouts and clearer touch-terminal controls.
- Add a small enemy behavior table so combat tuning stays testable.

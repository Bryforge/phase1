# Development Checkpoint: Edge 6.0.0

This checkpoint records the major edge line after the stable `v5.0.0` public representation.

## Version identity

| Item | Value |
| --- | --- |
| Active package version | `6.0.0` |
| Edge label | `v6.0.0` |
| Stable release point | `v5.0.0` |
| Previous stable | `v4.4.0` |
| Compatibility base | `v3.6.0` |
| Version scheme | `MAJOR.MINOR.PATCH[-dev]` |

## Checkpoint scope

This checkpoint includes the post-`v5.0.0` work that improves Phase1 for real local development and mobile operator usage:

- Guarded host runtime execution is separated from privileged host mutation.
- `PHASE1_ALLOW_HOST_TOOLS=1` can enable guarded language/runtime execution while safe mode remains enabled.
- Privileged host mutation still requires safe mode off plus the trust gate.
- `lang run` supports guarded temporary workspaces, bounded stdin, timeout controls, promptless host-tool environment variables, redacted output, and audit metadata.
- Mobile/narrow terminals use a simple line-editor path to avoid prompt redraw scroll, NUL padding, and paste artifacts.
- The default prompt now uses compact dynamic chips across all device modes, such as `phase1://root ~ [edge safe trust] ⇢`.
- The legacy HUD prompt can still be restored with `PHASE1_COMPACT_PROMPT=0`.
- WASM test fixture directories are unique per process and counter to avoid parallel test collisions.

## Validation target

Before merging future edge work, keep this validation set green:

```bash
cargo fmt --all -- --check
cargo check --workspace --all-targets
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace --all-targets
sh scripts/quality-check.sh quick
sh scripts/test-release-metadata.sh
sh scripts/test-website.sh
```

## Roadmap notes

Recommended follow-up work after this checkpoint:

1. Migrate direct `python`, `py`, `gcc`, and `cc` wrappers onto the same guarded runtime helper used by `lang run`.
2. Add `doctor mobile` for terminal width, prompt mode, line-editor mode, color mode, safe mode, trust gate, and launch advice.
3. Add named boot profiles for phone, laptop development, release demo, and trusted-runtime workflows.
4. Continue keeping `v5.0.0` stable docs intact until the next formal stable promotion.


## v6.0.0 boundary

The v6.0.0 edge line opens after the Base1 recovery USB target-selection and image-provenance read-only checkpoints.

This is still an edge line. It does not claim a finished OS replacement, bootable Base1 image readiness, destructive installer readiness, USB media writing readiness, or real-hardware recovery completion.

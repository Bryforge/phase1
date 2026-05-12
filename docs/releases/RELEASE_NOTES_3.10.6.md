# Phase1 3.10.6 Release Notes

Phase1 3.10.6 promotes the recent operator-console work into a release-ready package build.

## Highlights

- Static boot timestamp replaces the dynamic boot clock to avoid redraw glitches on older or narrow terminals.
- Host-aware boot defaults select laptop, desktop, mobile, ASCII, or color behavior based on the terminal environment.
- Local operations logging is available through `phase1.log` and the `opslog` command family.
- Boot configuration includes clear device-mode and host-trust controls.
- Linux color pack support gives Linux hosts truecolor, 256-color, ANSI, and mono fallback modes.
- Tab completion, command history navigation, AVIM mode controls, and the command-aware HUD are part of the release line.

## Validation checklist

Run this before tagging the release:

```bash
git fetch origin
git reset --hard origin/master
rm -f phase1.conf
cargo fmt --all -- --check
cargo check --all-targets
cargo test --all-targets
cargo run
```

Manual release smoke test:

```text
l
9
1
opslog status
opslog tail
bootcfg show
theme linux status
version --compare
exit
```

## Release tag

Recommended tag:

```bash
git tag v3.10.6
git push origin v3.10.6
```

## Notes

The virtual kernel baseline remains `3.6.0` for compatibility reporting, while the package/release version is `3.10.6`.

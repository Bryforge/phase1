# phase1 v3.6.0 release notes

phase1 v3.6.0 is a release-readiness update focused on making the project easier to demo, test, and evolve into the roadmap architecture.

## Release highlights

- New compact operator dashboard:

```text
dash --compact
```

- New capability report:

```text
capabilities
caps
```

- Registry-backed aliases now normalize through one command registry path.
- Release docs now include a roadmap design index and detailed track documents.
- Smoke tests now cover dashboard output, alias behavior, capabilities, network fallbacks, command errors, process commands, and filesystem commands.
- Runtime version is now `3.6.0`.

## Demo command flow

```text
version
dash --compact
capabilities
spawn worker --background
ps
jobs
audit
browser phase1
```

## Upgrade instructions

```bash
git pull origin master
cargo test --test smoke -- --nocapture
cargo run
```

## Build release binary

```bash
cargo build --release
./target/release/phase1
```

## Safety notes

phase1 is an educational userspace simulator. It models OS behavior but is not a host operating-system sandbox. Host-backed commands remain bounded by validation, timeouts, and dry-run behavior where mutation is involved.

`wifi-connect` remains dry-run by default. Set `PHASE1_ALLOW_HOST_NETWORK_CHANGES=1` only when intentionally testing host network mutation.

## Known limitations

- Persistent shell history is designed but not yet implemented.
- Structured pipelines are designed but not yet implemented.
- The dashboard is text/compact first; fullscreen TUI work remains on the roadmap.
- The release was prepared through GitHub repository updates from ChatGPT. Final local verification should be run on a machine with Rust installed.

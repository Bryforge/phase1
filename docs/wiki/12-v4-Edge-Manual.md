# Phase1 v4 Stable Manual

Stable version: v4.0.0
Previous stable reference: v3.10.9
Compatibility base: v3.6.0

## Verify

```text
version
version --compare
roadmap
security
sysinfo
```

## Editors

```text
ned notes.txt
:w
:wq
```

```text
avim app.py
:help
:w
:wq
```

## Linux colors

```text
theme linux status
theme linux x200
theme linux raspberry-pi
theme linux ansi
theme linux 256
theme linux truecolor
```

## Website quality

The v4 stable website keeps the cyberpunk/neon style while improving mobile readability and desktop performance:

- no duplicate creator labels
- balanced mobile headings
- desktop canvas detail caps
- debounced resize handling
- hidden-tab animation pause
- reduced-motion support

## Validate

```bash
cargo fmt --all -- --check
cargo check --all-targets
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
cargo audit
cargo deny check
```

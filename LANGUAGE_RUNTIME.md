# phase1 native language runtime manager

`lang` is Phase1's in-shell language runtime manager for major open-source programming languages.

It is designed for learning, prototyping, and controlled operator workflows inside the Phase1 terminal environment. Host-backed execution remains disabled by default and must be explicitly enabled.

## Usage

```text
lang list
lang support
lang status [language]
lang doctor [language]
lang detect <file>
lang run <language|auto> <vfs-file | inline-code>
lang security
```

## Supported language families

Current registry:

```text
Rust
C
C++
Go
Zig
Python
JavaScript
TypeScript
Java
Kotlin
Scala
C#
F#
Swift
Ruby
PHP
Perl
Lua
R
Julia
Haskell
OCaml
Elixir
Erlang
Dart
Bash
WebAssembly/WASI
```

## Examples

Create a Rust file in the Phase1 VFS:

```text
echo 'fn main() { println!("hello from phase1"); }' > hello.rs
lang detect hello.rs
lang run rust hello.rs
```

Run inline Python:

```text
lang run python 'print("hello from phase1")'
```

Show all registered runtimes:

```text
lang support
```

Check installed host toolchains after intentionally enabling trusted host tools:

```bash
PHASE1_SAFE_MODE=0 PHASE1_ALLOW_HOST_TOOLS=1 cargo run
```

Then inside Phase1:

```text
lang doctor
```

## Security model

Language execution is powerful because it runs host toolchains. For that reason, execution requires both controls:

```bash
PHASE1_SAFE_MODE=0 PHASE1_ALLOW_HOST_TOOLS=1
```

Security controls:

- Safe mode blocks host-backed language execution by default.
- Source is read from Phase1 VFS files or explicit inline input.
- Source is copied into temporary files before execution.
- Source input is capped at 256 KiB.
- Compile and run commands use timeouts.
- Output is capped and redacted for common sensitive markers.
- Package installation is not implemented in `lang run`.
- Network fetches are not implemented in `lang run`.
- Background daemons are not implemented in `lang run`.
- WASM/WASI remains the preferred sandbox target for future plugin workflows.

## Notes

The `lang` command does not install compilers or interpreters. It detects and uses toolchains already available on the host after the trusted host-tools gate is enabled.

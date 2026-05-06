# Major programming language runtime support roadmap

This roadmap describes how phase1 should grow from a Rust-first virtual OS console into a guarded multi-language developer workspace.

The goal is not to bypass the phase1 safety model. Every host-backed runtime must remain explicit, observable, bounded, and disabled by default.

## Core requirements for every language

Each language integration must include:

- command registry metadata and capability classification
- safe-mode denial by default for host-backed execution
- `PHASE1_SAFE_MODE=0` plus `PHASE1_ALLOW_HOST_TOOLS=1` for trusted host execution
- workspace isolation under `phase1.workspace`
- command timeout and output-size limits
- output redaction for token, password, authorization, and secret markers
- smoke tests for help, denied execution, and trusted execution where the toolchain exists
- documentation for install prerequisites and supported subcommands

## Tier 0 — current foundation

Status: available through the `phase1-storage` helper.

### Rust

Supported path:

```text
phase1-storage rust version
phase1-storage rust run <file.rs | inline-code>
phase1-storage rust init <name>
phase1-storage rust cargo <repo> <check|build|test|run> [args...]
```

Next work:

- expose a shell-facing wrapper command after the helper stabilizes
- detect Cargo workspaces and package manifests
- add optional `cargo fmt` and `cargo clippy` wrappers
- surface structured results for diagnostics

### Git

Supported path:

```text
phase1-storage git clone <url> [name]
phase1-storage git status <name>
phase1-storage git pull <name>
phase1-storage git list
```

Next work:

- add branch list/checkout support with clean-working-tree checks
- add shallow clone depth controls
- add remote URL inspection with secret redaction
- add repository metadata into phase1 dashboard output

## Tier 1 — high-impact first-class runtimes

### Python

Current base: guarded `python` command and plugin execution.

Planned support:

- `python run <file.py | -c code>`
- `python venv <repo>` with workspace-local virtual environments
- `pip install` only inside a managed workspace environment
- test runners: `pytest`, `unittest`

### C and C++

Current base: guarded `gcc`/`cc` C compilation.

Planned support:

- separate `c` and `cpp` helpers
- compiler detection for `cc`, `gcc`, `clang`, `g++`, and `clang++`
- CMake configure/build/test wrapper
- compile database detection and diagnostics

### JavaScript and TypeScript

Planned support:

- runtime detection for Node.js, Bun, and Deno
- package-manager detection for npm, pnpm, yarn, and bun
- `js run`, `js test`, `js build`
- TypeScript support through `tsc`, `tsx`, Deno, or Bun
- lockfile-aware dependency operations

### Go

Planned support:

- `go run`, `go test`, `go build`
- module-aware workspace execution
- cache isolation where supported
- diagnostics for missing `go.mod`

## Tier 2 — managed application runtimes

### Java and Kotlin

Planned support:

- `javac` single-file compile/run
- Maven and Gradle wrappers
- Kotlin compiler detection
- test integration through Maven/Gradle tasks

### C# / .NET

Planned support:

- `dotnet new`, `dotnet build`, `dotnet test`, `dotnet run`
- solution/project detection
- workspace-local generated projects

### Swift

Planned support:

- `swift --version`, `swift run`, `swift test`, `swift build`
- platform-aware messaging when the host toolchain is unavailable

### PHP and Ruby

Planned support:

- PHP CLI run and Composer-aware commands
- PHPUnit detection
- Ruby run, Bundler, Rake, and RSpec support

## Tier 3 — extended language ecosystem

Planned support candidates:

- R and Julia for data/science workflows
- Lua and Perl for scripting
- Zig for systems experiments
- Elixir and Erlang for BEAM workflows
- Scala for JVM projects
- Haskell and OCaml for functional language experiments
- Dart and Flutter where host toolchains exist
- WebAssembly/WASI as the preferred long-term sandbox target

## Integration milestones

### M1 — helper stabilization

- keep `phase1-storage` as the supported command surface
- document storage root and safety controls
- add unit tests for name validation, output redaction, and roadmap coverage

### M2 — shell command wrappers

- add `storage`, `git`, `rust`, and `lang` commands to the main shell registry
- call the helper internally or move stable logic into shared modules
- preserve safe-mode gating and audit records

### M3 — structured execution results

- return table/json/text output from language helpers
- add phase1 pipeline compatibility for language diagnostics
- summarize build/test results in `dash --compact`

### M4 — package-aware workspace model

- detect language manifests and lockfiles
- present `workspace status` across cloned projects
- add opt-in dependency install commands with explicit prompts and docs

### M5 — sandbox-first plugin future

- prefer WASM/WASI where practical
- define a package manifest that lists language runtime requirements
- add signed/trusted package metadata before broader plugin distribution

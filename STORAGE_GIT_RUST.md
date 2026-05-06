# phase1 storage, Git, and Rust workflow

`phase1-storage` is a guarded developer helper for working with cloned repositories and Rust code in a local phase1 workspace.

It is intentionally separate from the core terminal simulator dispatch path so storage and host-backed developer actions can evolve without weakening the default safe boot profile.

## Safety model

The helper keeps all working files under `phase1.workspace` by default:

```text
phase1.workspace/
  repos/    cloned repositories and generated Rust projects
  build/    temporary Rust binaries
  tmp/      generated source files and scratch data
```

`PHASE1_STORAGE_ROOT` can override the workspace path.

Read-only status commands can run without host-tool opt-in. Any command that mutates storage or executes host tooling requires both controls:

```bash
PHASE1_SAFE_MODE=0 PHASE1_ALLOW_HOST_TOOLS=1
```

The helper also sets non-interactive Git environment variables where applicable:

```text
GIT_TERMINAL_PROMPT=0
GCM_INTERACTIVE=never
```

This prevents accidental interactive prompts during clone, status, or pull operations.

## Usage

Build or run the helper with Cargo:

```bash
cargo run --bin phase1-storage -- help
cargo run --bin phase1-storage -- storage status
```

Initialize the storage workspace:

```bash
PHASE1_SAFE_MODE=0 PHASE1_ALLOW_HOST_TOOLS=1 \
  cargo run --bin phase1-storage -- storage init
```

Clone a Git repository into the workspace:

```bash
PHASE1_SAFE_MODE=0 PHASE1_ALLOW_HOST_TOOLS=1 \
  cargo run --bin phase1-storage -- git clone https://github.com/Bryforge/phase1.git
```

Inspect and update a clone:

```bash
PHASE1_SAFE_MODE=0 PHASE1_ALLOW_HOST_TOOLS=1 \
  cargo run --bin phase1-storage -- git status phase1

PHASE1_SAFE_MODE=0 PHASE1_ALLOW_HOST_TOOLS=1 \
  cargo run --bin phase1-storage -- git pull phase1
```

Check Rust toolchain availability:

```bash
PHASE1_SAFE_MODE=0 PHASE1_ALLOW_HOST_TOOLS=1 \
  cargo run --bin phase1-storage -- rust version
```

Create a Rust project under `phase1.workspace/repos`:

```bash
PHASE1_SAFE_MODE=0 PHASE1_ALLOW_HOST_TOOLS=1 \
  cargo run --bin phase1-storage -- rust init hello-rust
```

Run Cargo in a workspace repo:

```bash
PHASE1_SAFE_MODE=0 PHASE1_ALLOW_HOST_TOOLS=1 \
  cargo run --bin phase1-storage -- rust cargo hello-rust check
```

Compile and run a single Rust file or inline `fn main` body:

```bash
PHASE1_SAFE_MODE=0 PHASE1_ALLOW_HOST_TOOLS=1 \
  cargo run --bin phase1-storage -- rust run 'fn main() { println!("hello from rust"); }'
```

## Supported commands

```text
phase1-storage storage status
phase1-storage storage path
phase1-storage storage init
phase1-storage storage list
phase1-storage storage doctor
phase1-storage git clone <url> [name]
phase1-storage git status <name>
phase1-storage git pull <name>
phase1-storage git list
phase1-storage rust version
phase1-storage rust run <file.rs | inline-code>
phase1-storage rust init <name>
phase1-storage rust cargo <repo> <check|build|test|run> [args...]
phase1-storage lang roadmap
```

## Design notes

- Repository names are validated to block path traversal and unsafe names.
- Git clone destinations are restricted to the workspace `repos` directory.
- Cargo subcommands are allow-listed to `check`, `build`, `test`, and `run`.
- Host command output is bounded and applies conservative redaction for common sensitive markers.
- Generated workspace artifacts should stay local under `phase1.workspace`.

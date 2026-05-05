# R5 package and plugin runtime design

## Purpose

phase1 should eventually support an ecosystem of small commands without letting plugins become unsafe or unstructured.

The current Python plugin fallback is useful for experimentation. The long-term design should move toward explicit manifests and a sandboxed runtime.

## Package format

Proposed package layout:

```text
.phasepkg/
  manifest.toml
  bin/plugin.wasm
  man/plugin.1
  tests/smoke.phase1
```

## Manifest fields

```toml
name = "netstat"
version = "0.1.0"
description = "Network status plugin"
entry = "bin/netstat.wasm"
commands = ["netstat"]
capabilities = ["net.read"]
```

## Package commands

```text
pkg list
pkg search <query>
pkg install <path-or-name>
pkg remove <name>
pkg info <name>
pkg verify <name>
```

## Runtime model

Short term:

- keep Python plugins for demos
- validate plugin names
- run with timeout
- pass context through stdin
- built-ins always take precedence

Long term:

- use WASM/WASI for sandboxing
- require manifests
- map plugin capabilities into policy
- require package checksums
- support signed packages

## Plugin execution context

Plugins should receive context in a stable format:

```text
COMMAND=hello
ARGS=--verbose
USER=root
CWD=/home
VERSION=3.5.0
```

Future structured context can become JSON.

## Trust model

Package trust should support:

- local unsigned packages for development
- checksum verification
- signature verification later
- explicit install confirmation in interactive mode

## Implementation checklist

1. Add manifest parser for simple key/value manifests.
2. Add `pkg list` for installed plugins/packages.
3. Add `pkg info` reading manifests.
4. Move Python plugin examples into manifest-backed packages.
5. Add policy checks before plugin execution.
6. Design WASM runtime integration after package metadata is stable.

## Demo commands

```text
plugins
hello
pkg list
pkg info hello
```

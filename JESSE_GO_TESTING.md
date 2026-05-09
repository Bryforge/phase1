# Jesse Go Testing Guide

Phase1 Go support is available in two layers:

1. `lang run go` for guarded temporary-workspace execution from the Phase1 VFS.
2. `go <args...>` for guarded host Go tool passthrough at any Phase1 nest level.

## Quick Go smoke test inside Phase1

```text
go version
lang doctor go
echo 'package main; import "fmt"; func main() { fmt.Println("hello Jesse from Go inside Phase1") }' > jesse.go
lang run go jesse.go
```

Expected result:

```text
hello Jesse from Go inside Phase1
```

## Safety model

- Go uses the host Go toolchain.
- Phase1 does not yet contain its own Go compiler.
- Commands are passed directly without a shell.
- Host tool access requires `PHASE1_ALLOW_HOST_TOOLS=1`.
- Safe mode may stay enabled for guarded runtime workflows.

# AVIM Pro Editor

AVIM is Phase1's native advanced modal editor. This document defines the next pro-editor layer so `avim`, `vim`, and `edit` become genuinely useful inside Phase1 without importing a host editor or weakening the Phase1 safety model.

## Design goal

AVIM Pro should support the full operator loop:

```text
create -> edit -> save -> run -> inspect -> fix -> save -> rerun
```

It should remain Phase1-native:

- VFS-first editing.
- No shell escapes.
- No modelines.
- No plugins.
- No automatic execution on open.
- No host filesystem reads by default.
- No credential or token access.

## Pro command set

### Buffer and file workflow

```text
:status          show file, dirty state, byte count, line count, cursor, undo depth
:open <file>     open another safe VFS file from inside AVIM
:w <file>        save the current buffer as another safe VFS file
:files           list nearby VFS files
:ls              alias for :files
```

### Search and edit workflow

```text
n                repeat search forward
N                repeat search backward
u                undo
redo             redo last undo
:%s/old/new/g    substitute across the buffer
:diff            show edited buffer changes against the file as opened
```

### Language starter workflow

```text
:template go      insert Go starter program
:template rust    insert Rust starter program
:template python  insert Python starter program
:template c       insert C starter program
```

### Run workflow

```text
:run             save and run the current file through Phase1 lang runtime when possible
:run go          force Go runner
:run rust        force Rust runner
:run python      force Python runner
:run c           force C/GCC runner
```

The `:run` command should never use a shell. It should reuse the Phase1 language runtime/guard pathway and print the same host-trust requirements as `lang run`.

## AVIM Pro demo target

```text
avim demo.go
:template go
:w
:run
```

Expected output:

```text
avim: inserted Go starter template
avim: wrote demo.go
avim: running go through Phase1 guarded language runtime
hello from avim + Go inside Phase1
```

## Help target

```text
:help pro
```

Expected output:

```text
AVIM Pro
  :status        buffer status
  :open <file>   switch VFS file
  :w <file>      save as VFS file
  :files         list VFS files
  :template go   insert Go starter
  :run           save and run current file
  :diff          show buffer changes
  N              search backward
  redo           redo last undo
```

## Acceptance tests

- `:status` reports file, dirty state, line count, byte count, cursor, and undo depth.
- `:w <file>` saves a copy without unsafe traversal.
- `:open <file>` refuses unsafe paths and large files.
- `:template go` creates a runnable Go starter.
- `:run` refuses unsupported extensions clearly.
- `:run` uses guarded language runtime behavior, not shell execution.
- `N` searches backward after `/text`.
- `redo` reapplies the last undone buffer change.
- `:help pro` documents all pro commands.

## Security stance

AVIM Pro is intentionally not full Vim. It stays smaller, auditable, and Phase1-native. Dangerous Vim behaviors remain out of scope: `:!cmd`, external filters, host path editing, modelines, remote reads, plugins, and automatic execution.

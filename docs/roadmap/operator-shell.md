# R1 operator shell design

## Purpose

The operator shell is the primary user interface for phase1. It should feel like a polished terminal OS console, not a loose collection of string-matched commands.

## Current state

The project already has:

- command registry metadata
- generated help/man/completion surfaces
- end-to-end smoke tests
- mobile-first prompt and boot UI
- guarded host integrations

## Problems to solve

1. Aliases exist in the registry but dispatch still matches some aliases manually.
2. History exists in memory only.
3. Completion is exposed through `complete`, but there is not yet a line editor.
4. Error formatting is not fully consistent.
5. Command metadata is not yet used for policy enforcement.

## Design

### Command normalization

Before dispatch, the shell should normalize an input command through the registry.

Example:

```text
py      -> python
cc      -> gcc
quit    -> exit
commands -> help
```

This keeps aliases in one place and prevents match arms from drifting.

### Persistent history

The shell should load and save history from:

```text
$PHASE1_HISTORY
$HOME/.phase1_history
.phase1_history
```

Resolution order:

1. If `PHASE1_HISTORY=off`, disable disk history.
2. If `PHASE1_HISTORY` is a path, use that path.
3. Otherwise use `$HOME/.phase1_history` when available.
4. Fallback to `.phase1_history` in the current host directory.

History should:

- cap at 512 entries
- skip empty lines
- append on exit when possible
- avoid failing the shell if the file cannot be written

### Completion

The current `complete <prefix>` command is the stable backend. A future line editor can call the same registry function.

Completion should include:

- command names
- aliases
- plugin names
- VFS path hints later

### Error format

Preferred style:

```text
command: clear problem description
usage: command <required> [optional]
```

Examples:

```text
cat: no such file: demo.txt
usage: wifi-connect <ssid> [password]
```

## Implementation checklist

- Add `registry::canonical_name(name)`.
- Normalize command names in `main.rs` or at the top of `dispatch`.
- Remove duplicate alias arms when safe.
- Add history load/save helpers.
- Add smoke tests for alias behavior.
- Add smoke tests for persistent history using a temp file path.

## Demo commands

```text
complete p
py -c "print('phase1')"
cc 'int main(){return 0;}'
history
quit
```

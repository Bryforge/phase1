# avim — advanced VFS modal editor

`avim` is Phase1's native advanced modal editor. It is Vim-inspired, but it is an original Phase1 implementation built for the simulator's virtual filesystem and safety model.

The current edge build is designed to be easier on small machines and mobile terminals: it shows a visible pink cursor, uses color-coded modes, supports line and character-level editing, and exposes simple command completion help from inside the editor.

The next pro-editor layer is tracked in [`AVIM_PRO.md`](AVIM_PRO.md). That document defines the planned `:status`, `:open`, `:w <file>`, `:files`, `N`, `redo`, `:template`, `:run`, `:diff`, and `:help pro` workflow so AVIM can become the full write-save-run-fix editor inside Phase1 while staying VFS-native and safe.

## Goals

- Provide a capable terminal editor from inside Phase1.
- Keep editing inside the Phase1 VFS.
- Make basic fixes easy after a first draft, not only full-line replacement.
- Avoid risky host-editor surfaces by default.
- Make save/discard behavior explicit.
- Support practical editing on older laptops, mobile shells, and desktop terminals.

## Usage

```text
avim <file>
vim <file>
edit <file>
```

The `vim` and `edit` aliases intentionally route to `avim`.

## Fast path

```text
avim hello.py
i print('hello')       insert text at the cursor
a  # comment           append text after the cursor
e print('fixed')       replace the current line
o print('next')        open a line below
:wq                    save and quit
```

`Esc`, `escape`, `normal`, and `cancel` all return to Normal mode. This matters on hardware where the physical Escape key is unreliable or encoded differently.

## Normal mode

```text
i text          insert text at the visible pink cursor
a text          append text after the visible pink cursor
e text          replace the selected line
o text          open a new line below
O text          open a new line above
h / left        move cursor left
l / right       move cursor right
0 / home        move to beginning of line
$ / end         move to end of line
w               move forward by word
b               move backward by word
j / down        move down one line
k / up          move up one line
pgup / pgdn     move by 10 lines
gg              move to first line
G               move to last line
x               delete character under cursor
backspace       delete character before cursor
dd              delete current line and yank it
yy              yank current line
p               paste yanked line below
u               undo
/text           search forward for text
n               repeat search
Tab             show avim command completions
:               enter command mode
```

## Insert mode

Insert mode is intentionally simple. When you enter `i`, `a`, `e`, `o`, or `O` without text, `avim` waits for one submitted line and applies the pending action.

```text
i
print('hello, world')
```

Use `Esc`, `escape`, `normal`, or `cancel` to cancel a pending edit and return to Normal mode.

## Command mode

```text
:w              save
:q              quit if clean
:q!             discard changes and quit
:wq             save and quit
:x              save and quit
:help           show built-in help
:security       show the editor safety model
:set number     show line numbers
:set nonumber   hide line numbers
:12             jump to line 12
:n 12           jump to line 12
:search text    search forward
:%s/old/new/g   substitute text across the buffer
:read <file>    read another VFS file below the cursor
```

## Pro editor target

```text
avim jesse.go
:template go
:w
:run
```

Expected future workflow:

```text
avim: inserted Go starter template
avim: wrote jesse.go
avim: running go through Phase1 guarded language runtime
hello from avim + Go inside Phase1
```

## Completion behavior

- Shell-level Tab completes `avim` itself and common VFS file names before the editor opens.
- Inside `avim`, pressing Tab or typing a command containing a tab prints matching editor commands.
- Completion suggestions include editing, movement, save, search, read, set, and security commands.
- The pro layer should extend completions for `:status`, `:open`, `:w <file>`, `:files`, `:template`, `:run`, `:diff`, `redo`, and `N`.

## Color model

- Cyan: Normal mode and navigation status.
- Pink/magenta: Insert/edit actions and the visible cursor.
- Yellow: Command mode and warnings.
- Red: invalid commands or blocked actions.
- Green: successful writes.

Set `NO_COLOR=1` or `PHASE1_NO_COLOR=1` to disable ANSI color output.

## Security model

`avim` intentionally does **not** implement:

- shell escapes such as `:!cmd`
- external filters such as `:%!cmd`
- host filesystem reads
- modelines
- plugins
- remote fetches
- background jobs
- automatic execution on open

Additional controls:

- Edits are capped to 256 KiB per file.
- Targets are validated against unsafe traversal-style paths.
- Reads and writes use the existing Phase1 VFS paths.
- Persistence follows the normal Phase1 persistent-state setting.
- The editor does not need GitHub credentials, host tokens, shell profiles, SSH keys, or browser cookies.
- The pro `:run` target must reuse guarded Phase1 runtime paths and must not introduce Vim-style shell execution.

## Why not copy Vim?

Phase1 should stay understandable, auditable, and simulator-native. `avim` borrows familiar modal-editing concepts but avoids importing or embedding a large host editor. That keeps the security boundary smaller and easier to review.

# avim — advanced VFS modal editor

`avim` is Phase1's native advanced modal editor. It is Vim-inspired, but it is an original Phase1 implementation built for the simulator's virtual filesystem and safety model.

## Goals

- Provide a capable terminal editor from inside Phase1.
- Keep editing inside the Phase1 VFS.
- Avoid the risky surfaces that full host editors expose by default.
- Make save/discard behavior explicit.
- Support practical modal-editing workflows on mobile and desktop terminals.

## Usage

```text
avim <file>
vim <file>
edit <file>
```

The `vim` and `edit` aliases intentionally route to `avim`.

## Normal mode

```text
i       enter insert mode on the current line
a       enter insert mode on the current line
o       open a line below and enter insert mode
O       open a line above and enter insert mode
j       move down one line
k       move up one line
gg      move to first line
G       move to last line
dd      delete current line and yank it
yy      yank current line
p       paste yanked line below
u       undo
/text   search forward for text
n       repeat search
:       enter command mode
```

## Insert mode

In this terminal-first build, insert mode replaces the current line with each submitted line. A single `.` or `<esc>` returns to normal mode.

```text
hello world
.
```

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
:%s/old/new/g   substitute text across the buffer
:r <file>       read another VFS file below the cursor
```

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

## Why not copy Vim?

Phase1 should stay understandable, auditable, and simulator-native. `avim` borrows familiar modal-editing concepts but avoids importing or embedding a large host editor. That keeps the security boundary smaller and easier to review.

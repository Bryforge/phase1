# Files, Editors, and Pipelines

![VFS](https://img.shields.io/badge/VFS-/home%20/proc%20/dev%20/tmp%20/var-00d8ff) ![Editor](https://img.shields.io/badge/editor-AVIM-39ff88) ![Pipelines](https://img.shields.io/badge/pipelines-structured-ffcc00)

Phase1 includes a virtual filesystem, a modal VFS editor, and lightweight structured shell pipelines.

## Virtual filesystem layout

| Path | Purpose |
| --- | --- |
| `/home` | User workspace and generated `readme.txt` |
| `/proc` | Runtime and kernel-style information |
| `/dev` | Simulated devices |
| `/tmp` | Temporary workspace |
| `/var/log` | Virtual log area |
| `/etc` | Configuration-style files |
| `/bin` | Command-style namespace |

> [!TIP]
> TRY THIS
>
> ```text
> ls /
> ls /home
> cat /home/readme.txt
> cat /proc/version
> ```

## Create a workspace

> [!TIP]
> TRY THIS
>
> ```text
> mkdir lab
> cd lab
> pwd
> echo Phase1 lab file > note.txt
> cat note.txt
> ```

## File redirection

Phase1 supports simple write and append redirection for VFS workflows.

```text
echo first line > log.txt
echo second line >> log.txt
cat log.txt
```

## Copy, move, and remove files

> [!TIP]
> TRY THIS
>
> ```text
> cp log.txt copy.txt
> mv copy.txt renamed.txt
> ls
> rm renamed.txt
> ls
> ```

## AVIM editor basics

AVIM is a modal VFS editor for files inside Phase1.

Open a file:

```text
avim hello.py
```

Common keys:

| Key | Mode | Action |
| --- | --- | --- |
| `i` | NORMAL | Enter INSERT mode |
| `a` | NORMAL | Append / enter INSERT mode |
| `o` | NORMAL | Open a new line |
| `Esc` | INSERT | Return to NORMAL mode |
| `:wq` | NORMAL | Save and quit |
| `:q!` | NORMAL | Quit without saving |
| `:help` | NORMAL | Show editor help |

> [!TIP]
> TRY THIS
>
> ```text
> avim hello.py
> ```
>
> Then type:
>
> ```text
> i
> print("hello from phase1")
> Esc
> :wq
> ```

## Run the file

Host-backed runtime tools must be enabled first.

```bash
./scripts/phase1-runtimes.sh
```

Then inside Phase1:

```text
py hello.py
```

## Structured pipelines

Pipelines are designed for predictable terminal workflows.

> [!TIP]
> TRY THIS
>
> ```text
> cat log.txt | grep first
> cat log.txt | wc -l
> history | tail -5
> ps | grep phase1
> find /home -type f | sort
> ```

## Pipeline producers

Common producers include:

```text
cat
echo
history
ps
ls
find
audit
env
version
sysinfo
```

## Pipeline filters

Common filters include:

```text
grep
wc
head
tail
sort
uniq
cut
```

## Tutorial: Build a small VFS project

> [!TIP]
> TRY THIS
>
> ```text
> mkdir project
> cd project
> echo Phase1 Project > README.txt
> echo alpha > data.txt
> echo beta >> data.txt
> echo alpha >> data.txt
> cat data.txt | sort | uniq
> cat data.txt | grep alpha | wc -l
> tree /home
> ```

Expected result:

```text
alpha
beta
2
```

## Persistence reminder

Press `p` at boot to enable VAULT mode. `/home` files then persist to `phase1.state`.

> [!CAUTION]
> Do not store credentials, tokens, or private keys in persistent VFS files.

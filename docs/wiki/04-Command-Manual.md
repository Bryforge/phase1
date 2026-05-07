# Command Manual

![Commands](https://img.shields.io/badge/manual-command%20reference-00d8ff) ![Safe](https://img.shields.io/badge/default-safe-39ff88)

This page organizes Phase1 commands by task. Commands marked host-backed require SHIELD off and TRUST HOST on.

## Discover commands

> [!TIP]
> TRY THIS
>
> ```text
> help
> man browser
> complete p
> capabilities
> ```

| Command | Purpose |
| --- | --- |
| `help` | Show command map |
| `man <command>` | Show command details |
| `complete <prefix>` | Show completions |
| `capabilities` | Show command capability metadata and guards |
| `roadmap` | Show completed and planned feature tracks |
| `version` | Print current version |
| `version --compare` | Compare current build with stable/base tracks |

## Filesystem commands

| Command | Purpose |
| --- | --- |
| `pwd` | Print current VFS path |
| `ls [path]` | List VFS files or directories |
| `cd <path>` | Change VFS directory |
| `cat <file>` | Read a VFS file |
| `mkdir <dir>` | Create a directory |
| `touch <file>` | Create an empty file |
| `echo text > file` | Write text to a file |
| `rm <path>` | Remove a file |
| `cp <src> <dst>` | Copy a file |
| `mv <src> <dst>` | Move or rename a file |
| `tree [path]` | Show a tree view |

> [!TIP]
> TRY THIS
>
> ```text
> pwd
> ls /
> mkdir lab
> cd lab
> echo hello phase1 > note.txt
> cat note.txt
> tree /home
> ```

## Text and search commands

| Command | Purpose |
| --- | --- |
| `grep <pattern>` | Filter lines |
| `wc` | Count lines, words, bytes |
| `head` | Show first lines |
| `tail` | Show last lines |
| `find` | Search VFS paths |
| `pipeline` | Show pipeline help/examples |

> [!TIP]
> TRY THIS
>
> ```text
> echo alpha > log.txt
> echo beta >> log.txt
> cat log.txt | grep alpha
> cat log.txt | wc -l
> find /home -type f
> ```

## Process commands

| Command | Purpose |
| --- | --- |
| `ps` | List simulated processes |
| `top` | Show process summary |
| `spawn <name>` | Spawn a simulated process |
| `jobs` | List background jobs |
| `fg` | Bring job foreground |
| `bg` | Send job background |
| `kill <pid>` | Kill simulated process |
| `nice` | Adjust simulated process priority |

> [!TIP]
> TRY THIS
>
> ```text
> ps
> spawn worker --background
> jobs
> kill 2
> ps
> ```

## System commands

| Command | Purpose |
| --- | --- |
| `sysinfo` | Show system summary |
| `dash` | Show dashboard |
| `free` | Show memory model |
| `df` | Show VFS disk model |
| `dmesg` | Show boot/kernel messages |
| `vmstat` | Show virtual memory stats |
| `uname` | Show kernel profile |
| `date` | Show time |
| `uptime` | Show uptime |
| `hostname` | Show node name |
| `audit` | Show audit events |
| `opslog status` | Show local ops log status |
| `opslog tail` | Tail local ops log |

> [!TIP]
> TRY THIS
>
> ```text
> sysinfo
> dash
> audit
> opslog status
> dmesg
> ```

## User and environment commands

| Command | Purpose |
| --- | --- |
| `env` | Show environment |
| `export KEY=value` | Set a shell environment value |
| `unset KEY` | Remove an environment value |
| `whoami` | Show current simulated user |
| `id` | Show simulated identity |
| `su <user>` | Switch simulated user |
| `accounts` | Show privacy-safe account model |
| `history` | Show command history |
| `history status` | Show history mode |
| `security` | Show security posture |
| `theme` | Manage themes |
| `banner` | Preview banner modes |
| `tips` | Show built-in tips |

> [!TIP]
> TRY THIS
>
> ```text
> whoami
> id
> env
> history status
> security
> ```

## Network commands

Host-backed network inspection requires SHIELD off and TRUST HOST on. Network mutation also requires `PHASE1_ALLOW_HOST_NETWORK_CHANGES=1`.

| Command | Purpose |
| --- | --- |
| `ifconfig` | Show interfaces |
| `iwconfig` | Show WiFi summary |
| `nmcli` | Show network manager summary |
| `wifi-scan` | Scan WiFi where supported |
| `wifi-connect "SSID"` | Dry-run or connect if mutation gate enabled |
| `ping <host>` | Ping a safe host |

> [!TIP]
> TRY THIS
>
> ```text
> ifconfig
> iwconfig
> nmcli
> ping example.com
> ```

## Browser command

The browser is a guarded terminal reader. It fetches HTTP/HTTPS pages, extracts readable text, lists links, and never runs JavaScript or stores cookies.

| Command | Purpose |
| --- | --- |
| `browser about` | Show browser help |
| `browser phase1` | Show Phase1 browser page |
| `browser example.com` | Fetch a web page using HTTPS normalization |
| `browser https://example.com` | Fetch explicit HTTPS URL |

> [!TIP]
> TRY THIS
>
> ```text
> browser about
> browser phase1
> browser example.com
> ```

## Developer and language commands

| Command | Purpose |
| --- | --- |
| `avim <file>` | Open the modal VFS editor |
| `lang support` | Show supported language families |
| `lang security` | Show language runtime guard details |
| `lang run <language> <file>` | Run a source file through a guarded host runtime |
| `py <file>` | Python shortcut when allowed |
| `gcc` | C compiler path when allowed |
| `plugins` | Plugin command group when allowed |
| `wasm` | WASI-lite plugin/runtime group |
| `update` | Show update plan |
| `update protocol` | Show update safety protocol |

> [!TIP]
> TRY THIS
>
> ```text
> lang support
> lang security
> avim hello.py
> update protocol
> wasm list
> ```

## Hardware simulation commands

| Command | Purpose |
| --- | --- |
| `lspci` | Show PCI-style devices |
| `pcie` | Show PCIe model |
| `cr3` | Show simulated CR3 |
| `loadcr3 0x2000` | Load simulated CR3 |
| `cr4` | Show simulated CR4 |
| `pcide on` | Toggle simulated PCIDE |

> [!TIP]
> TRY THIS
>
> ```text
> cr3
> loadcr3 0x2000
> cr4
> pcide on
> lspci
> pcie
> ```

## Exit

```text
exit
```

The shutdown banner reports the active Phase1 package version.

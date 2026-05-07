# Boot Modes and Security

![Secure Default](https://img.shields.io/badge/default-safe%20mode%20on-39ff88) ![Host Tools](https://img.shields.io/badge/host%20tools-opt--in-00d8ff) ![Network Changes](https://img.shields.io/badge/network%20changes-separate%20gate-ffcc00)

Phase1 is secure by default. The default boot profile blocks host-backed commands until the operator explicitly opts in.

## Boot selector controls

| Key | Action |
| --- | --- |
| `1` or `Enter` | Boot into the shell |
| `2` | Toggle neon/color output |
| `3` | Toggle ASCII-compatible display |
| `4` | Toggle SHIELD / safe mode |
| `5` | Toggle quick boot |
| `6` | Cycle device mode |
| `l` | Laptop UI |
| `w` | Desktop UI |
| `t` | Toggle TRUST HOST |
| `e` | Toggle EDGE / bleeding-edge channel |
| `p` | Toggle VAULT / persistent state |
| `d` | Open storage / Git / Rust dock |
| `7` | Reboot selector |
| `8` | Shutdown / abort boot |
| `9` | Save `phase1.conf` |
| `0` | Reset saved config |
| `h` | Help |

## Security gates

Host-backed commands require both gates:

| Gate | Required state | Meaning |
| --- | --- | --- |
| SHIELD | Off | Safe boot restrictions are disabled |
| TRUST HOST | On | Operator allows local host-backed tools |

> [!IMPORTANT]
> SHIELD off alone is not enough. TRUST HOST must also be enabled before Python, browser fetches, compilers, plugins, and updater execution can run.

## Fast runtime mode

> [!TIP]
> TRY THIS
>
> ```bash
> ./scripts/phase1-runtimes.sh
> ```

This starts Phase1 with host-backed runtime tools enabled:

```bash
PHASE1_SAFE_MODE=0
PHASE1_ALLOW_HOST_TOOLS=1
PHASE1_BLEEDING_EDGE=1
```

## Manual runtime mode

At the boot selector:

```text
4    SHIELD off
t    TRUST HOST on
1    BOOT
```

Then:

```text
security
py hello.py
browser example.com
lang support
```

## Host network mutation gate

Network inspection and network mutation are separate. Host network changes require:

```bash
PHASE1_ALLOW_HOST_NETWORK_CHANGES=1
```

Without that variable, commands such as WiFi connection attempts remain dry-run or blocked.

## Persistence / VAULT mode

Press `p` in the boot selector to enable persistent state.

| File | Purpose |
| --- | --- |
| `phase1.state` | Persists `/home` virtual files |
| `phase1.history` | Persists sanitized command history |
| `phase1.log` | Local operations log |

> [!CAUTION]
> Do not store real secrets in persistent state. Phase1 redacts history, but files you intentionally write may still contain sensitive data.

## Recommended profiles

| Profile | Boot setup | Use it for |
| --- | --- | --- |
| Safe demo | Default, press `1` | Exploring commands safely |
| Runtime dev | `./scripts/phase1-runtimes.sh` | Python, Rust, browser, plugins |
| Persistent lab | Press `p`, then `1` | VFS labs that survive restarts |
| Full host lab | SHIELD off, TRUST HOST on, network changes env set | Advanced local network testing |

## Verify your posture

> [!TIP]
> TRY THIS
>
> ```text
> security
> capabilities
> bootcfg show
> ```

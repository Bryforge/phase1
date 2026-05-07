# Browser and Networking

![Browser](https://img.shields.io/badge/browser-terminal%20reader-00d8ff) ![Network](https://img.shields.io/badge/network-guarded-39ff88) ![Safety](https://img.shields.io/badge/mutation-separate%20gate-ffcc00)

Phase1 includes a guarded terminal browser and an optimized network inspection layer. Both are designed to be useful while staying explicit and safe.

## Security model

Browser fetches and host network inspection are host-backed operations. They require:

```text
SHIELD off
TRUST HOST on
```

Use the runtime launcher:

```bash
./scripts/phase1-runtimes.sh
```

or manually at the boot selector:

```text
4    SHIELD off
t    TRUST HOST on
1    BOOT
```

## Browser overview

The Phase1 browser is a terminal-reader browser. It does not run JavaScript, store cookies, or preserve login sessions.

It provides:

- HTTP/HTTPS fetching through guarded host tools
- URL normalization such as `example.com` to HTTPS
- blocked URL credentials
- connect timeout, total timeout, redirect limit, and max download size
- content type and final URL display
- readable HTML extraction
- title extraction
- heading, paragraph, list, image-alt, and link rendering
- indexed links after the page body
- script/style/noscript/svg/canvas/iframe removal
- HTML entity decoding

## Browser commands

> [!TIP]
> TRY THIS
>
> ```text
> browser about
> browser phase1
> browser example.com
> browser https://example.com
> browser https://github.com/Bryforge/phase1
> ```

## Browser safety boundaries

| Boundary | Behavior |
| --- | --- |
| JavaScript | Not executed |
| Cookies | Not stored |
| Credentials in URL | Blocked |
| Protocols | HTTP/HTTPS only |
| Huge pages | Bounded and truncated |
| Script/style content | Removed |

> [!IMPORTANT]
> The browser is a safe terminal reader, not a replacement for a full graphical browser.

## Tutorial: Inspect a page safely

> [!TIP]
> TRY THIS
>
> ```text
> browser example.com
> ```

Look for:

```text
status
content-type
final-url
title
links
```

Then try:

```text
browser https://github.com/Bryforge/phase1
```

## Network overview

The network layer has safe defaults and optimized host inspection.

It includes:

- cached host interface refreshes to reduce repeated probes
- stable sorted interface output
- Linux `ip -o addr show` parsing
- Linux MAC and operstate lookup
- Linux WiFi metadata through `nmcli` or `iwgetid` when available
- macOS `ifconfig` parsing
- macOS WiFi device and current SSID detection
- bounded command output and truncation markers
- stricter host validation for `ping`
- SSID sanitization
- dry-run WiFi connection by default

## Network commands

> [!TIP]
> TRY THIS
>
> ```text
> ifconfig
> iwconfig
> nmcli
> ping example.com
> wifi-scan
> ```

## WiFi connection behavior

`wifi-connect` is guarded. By default it is a dry-run or safe blocked operation.

To permit host network changes, launch with:

```bash
PHASE1_ALLOW_HOST_NETWORK_CHANGES=1 ./scripts/phase1-runtimes.sh
```

Then inside Phase1:

```text
wifi-connect "Network Name"
```

> [!CAUTION]
> Only enable host network changes on your own machine and network.

## Tutorial: Network inspection lab

Start with runtime mode:

```bash
./scripts/phase1-runtimes.sh
```

Inside Phase1:

```text
security
ifconfig
nmcli
iwconfig
ping example.com
```

Read the output for:

- active interfaces
- IPv4 addresses
- MAC addresses where available
- WiFi SSID where available
- safe-mode or host-tool denial messages

## Troubleshooting browser and network commands

| Symptom | Cause | Fix |
| --- | --- | --- |
| `disabled by safe boot profile` | SHIELD is on | Press `4` at boot or use runtime launcher |
| `enable trusted host tools` | TRUST HOST is off | Press `t` at boot or use runtime launcher |
| Browser fetch fails | Host lacks curl or network | Install curl or check host network |
| WiFi scan unavailable | Host command unsupported | Use OS-native network tools outside Phase1 |
| WiFi connect blocked | Mutation gate off | Set `PHASE1_ALLOW_HOST_NETWORK_CHANGES=1` |

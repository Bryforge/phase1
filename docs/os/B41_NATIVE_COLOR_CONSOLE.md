# B41 native color console plan

Status: planning scaffold

Scope: X200 / B38 working boot protocol / B40 native binary loader fix / full Phase1 color console.

## Purpose

B41 makes the native Phase1 console boot with color enabled by default.

The current working boot chain is:

`Libreboot GRUB -> normal linux -> normal initrd -> rdinit=/init -> Phase1 native console`

B40 launches the native Phase1 binary, but the initramfs was conservative and forced `PHASE1_COLOR_MODE=mono`. B41 keeps the working B40 loader path but enables the color terminal environment.

## Runtime color environment

B41 sets:

```text
TERM=linux
COLORTERM=truecolor
PHASE1_THEME=crimson
PHASE1_COLOR_MODE=auto
PHASE1_FORCE_COLOR=1
NO_COLOR unset
```

## Success state

`phase1_native_color_console_seen`

This means the native Phase1 console appeared with color enabled.

## Fallback state

`phase1_native_console_seen`

The native console appeared without confirmed color.

`phase1_full_system_load_seen`

The Phase1-owned fallback runtime remained usable.

## Notes

The X200 text console is not a modern GPU terminal, so color depth may be limited by the firmware/text mode. B41 enables the best color behavior available from the current boot path without changing the proven boot protocol.

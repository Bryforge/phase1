# B44 Unicode font system plan

Status: implementation plan and boot-media packaging target  
Scope: Phase1 native boot on X200/Libreboot and compatible Linux builders

## Purpose

B44 adds a comprehensive Unicode/font support layer for Phase1 boot media.

The goal is not only `LANG=C.UTF-8`. The goal is to package the pieces that are required to render many scripts, including Japanese, on a minimal booted Phase1 runtime.

## Core distinction

```text
UTF-8 bytes/environment  -> lets programs carry Unicode text
Fonts                    -> provide glyph shapes
Renderer                 -> draws glyphs on the screen
```

A raw Linux VT may still fail to draw Japanese even with UTF-8 enabled and CJK fonts packaged. Japanese/CJK generally needs a framebuffer or graphical text renderer capable of using the packaged font.

## B44 packaging targets

### Environment

```text
LANG=C.UTF-8
LC_ALL=C.UTF-8
LC_CTYPE=C.UTF-8
LANGUAGE=ja:en
PHASE1_UTF8=1
PHASE1_UNICODE=1
PHASE1_LANGUAGE=ja
PHASE1_JAPANESE_SUPPORT=1
PHASE1_CJK_GLYPH_SUPPORT=attempt
PHASE1_FONT_PACK=unicode-full
```

### Fonts

Best-effort packaged directories when available on the builder:

```text
/usr/share/fonts
/usr/share/fontconfig
/usr/share/consolefonts
/usr/share/consoletrans
/etc/fonts
```

Preferred packages on Debian/Ubuntu/Trisquel builders:

```text
fonts-noto-core
fonts-noto-cjk
fonts-noto-color-emoji
fonts-dejavu-core
unifont
fontconfig
console-setup
kbd
fbterm
```

### Renderers/helpers

Best-effort packaged commands when available:

```text
fbterm
jfbterm
kmscon
bterm
setfont
unicode_start
kbd_mode
fc-match
fc-cache
```

## Boot entries

The boot menu should expose separate paths:

```text
Start Phase1 Stable Safe Color UTF-8        default auto-boot
Configure Phase1 Boot Card                  manual selector/card
Start Phase1 Unicode Font Lab               diagnostics, no claim
Start Phase1 Japanese Framebuffer UTF-8     renderer attempt
Start Phase1 ASCII Safe Fallback            compatibility only
```

## Evidence policy

Do not claim full Japanese glyph rendering until it is physically observed.

Allowed evidence states:

```text
phase1_unicode_font_packaged
phase1_cjk_renderer_attempted
phase1_japanese_utf8_ready
phase1_japanese_glyphs_seen      only after visible screen confirmation
```

Evidence fields:

```text
BASE1_B44_UTF8_ENV_READY=1
BASE1_B44_FONT_PACKAGED=0|1
BASE1_B44_FONT_SOURCE_COUNT=<n>
BASE1_B44_CJK_FONT_CANDIDATE=0|1
BASE1_B44_RENDERER_CANDIDATE=none|fbterm|jfbterm|kmscon|bterm
BASE1_B44_JAPANESE_GLYPH_RENDERING=not_claimed|seen
```

## Next implementation target

Add a post-writer Unicode/font augmentation script that:

1. runs the current verified B43/B42 media writer;
2. mounts the prepared USB;
3. extracts the initramfs;
4. copies fonts/renderers/helpers into the initramfs;
5. patches `/init` to expose Unicode diagnostics and renderer attempts;
6. repacks the initramfs;
7. patches GRUB entries;
8. verifies all paths by readback.

This avoids more blind reboot loops and keeps the B38/B41-proven boot protocol intact.

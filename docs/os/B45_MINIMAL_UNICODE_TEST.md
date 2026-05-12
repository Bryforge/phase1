# B45 minimal Unicode test plan

Status: next hardware-test prep  
Scope: X200 Libreboot GRUB / Phase1 native boot / minimal Japanese glyph experiment

## Decision

Do not package every available font for the next run. B44 copied thousands of font files and still did not prove Japanese glyph rendering. The next test should be small, explicit, and easy to reason about.

## Minimal test target

Package only:

1. one CJK-capable font candidate;
2. one renderer candidate;
3. one small Japanese test file;
4. one small Unicode/rounded-corner test file;
5. explicit evidence.

## Preferred font priority

Use the first available file from this priority order:

```text
Unifont .ttf/.otf/.pcf/.psf
Noto Sans Mono CJK JP
Noto Sans CJK JP
DejaVu Sans Mono fallback, not CJK-complete
```

## Preferred renderer priority

Use the first available command from this priority order:

```text
fbterm
jfbterm
kmscon
bterm
linux-vt fallback
```

## Test strings

Japanese test should be short:

```text
こんにちは、ハッカー
```

Rounded corner test should be short:

```text
╭─ Phase1 ─╮
╰──────────╯
```

ASCII fallback should remain available:

```text
+-- Phase1 --+
+------------+
```

## Evidence policy

Do not claim Japanese glyph success unless physically observed.

Use:

```text
BASE1_B45_MINIMAL_FONT=<path or none>
BASE1_B45_MINIMAL_RENDERER=<renderer or linux-vt>
BASE1_B45_TEST_STRING=hello-hacker-ja
BASE1_B45_JAPANESE_GLYPH_RENDERING=not_claimed
```

## Why this is better than B44

B44 proved file packaging but was too broad. A minimal font/render test gives a clearer answer:

- if one known CJK font and renderer work, expand carefully;
- if they do not work, pivot to full userspace or Phase1-owned framebuffer rendering;
- avoid large initramfs growth and long repack times.

# B42 Japanese UTF-8 support plan

Status: planning scaffold

Scope: X200 / B38-B41 working native Phase1 boot path / UTF-8 and Japanese character readiness.

## Purpose

B42 ensures the booted native Phase1 runtime carries explicit UTF-8 and Japanese-language support metadata and environment settings.

The working boot path remains:

`Libreboot GRUB -> normal linux -> normal initrd -> rdinit=/init -> native Phase1 console`

## Runtime environment

B42 sets:

```text
LANG=C.UTF-8
LC_ALL=C.UTF-8
LC_CTYPE=C.UTF-8
LANGUAGE=ja:en
PHASE1_LANGUAGE=ja
PHASE1_I18N=1
PHASE1_UTF8=1
PHASE1_UNICODE=1
PHASE1_JAPANESE_SUPPORT=1
```

Color remains enabled:

```text
TERM=linux
COLORTERM=truecolor
PHASE1_COLOR_MODE=auto
PHASE1_FORCE_COLOR=1
```

## Runtime files

B42 packages a Japanese UTF-8 verification file:

`/phase1/i18n/ja/boot-test.txt`

Expected text:

```text
フェーズ1 起動完了
日本語 UTF-8 テスト
安全・非公開・強力
```

## Limitations

The X200 Linux text console may not have CJK glyph fonts loaded. The bytes and UTF-8 environment can be correct even if firmware/text console glyph rendering is limited. Full Japanese glyph display may require framebuffer font support or a graphical/user-space terminal later.

## Success state

`phase1_japanese_utf8_ready`

This means the booted Phase1 environment includes Japanese UTF-8 environment variables and a packaged Japanese verification file.

## Follow-up

Later work should add a proper framebuffer/terminal font pipeline for guaranteed CJK glyph rendering on bare metal text consoles.

# B46 Japanese renderer-only policy

Status: correction to B46 direction  
Branch: `black-phase1`

## Decision

Do not replace the Japanese greeting with ASCII text.

The colors are working. The issue is specifically that the boot console renderer does not draw Japanese glyphs. Therefore the right fix is not an ASCII downgrade. The right fix is a Japanese-capable renderer path.

## User requirement

The boot greeting should remain Japanese:

```text
こんにちは、ハッカー！
```

Color policy remains:

```text
stable/safe console: ice/blue
stable/safe Japanese greeting: bright red
edge console: crimson/red
edge Japanese greeting: bright blue
```

## Correct implementation direction

The UI should continue to emit Japanese text. The boot/runtime should provide a renderer capable of drawing it.

Viable paths:

1. framebuffer terminal with a known CJK font;
2. full userspace terminal with fontconfig/Noto CJK;
3. Phase1-owned framebuffer text renderer;
4. graphics-mode boot card rendered from a font into an image.

## What not to do

Do not change the Japanese greeting to:

```text
HELLO, HACKER
```

except inside an explicit emergency ASCII fallback entry selected by the operator.

## Evidence policy

Until a physical screen shows the Japanese text correctly:

```text
BASE1_B46_JAPANESE_TEXT_EMITTED=1
BASE1_B46_JAPANESE_GLYPH_RENDERING=not_claimed
```

Once it works physically:

```text
BASE1_B46_JAPANESE_GLYPH_RENDERING=seen
```

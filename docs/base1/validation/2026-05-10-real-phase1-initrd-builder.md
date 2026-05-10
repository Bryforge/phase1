# Base1 real Phase1 initrd preview builder validation — 2026-05-10

## Scope

This report records the repeatable preview-builder path for placing the real `phase1` binary into a Base1 preview initramfs.

## Evidence

Validated flow:

~~~sh
RUSTFLAGS="-C linker=rust-lld" cargo build -p phase1 --bin phase1 --release --target x86_64-unknown-linux-musl

sh scripts/base1-real-phase1-initrd-preview.sh \
  --base-initrd build/base1-real-boot/initramfs-virt \
  --phase1-bin target/x86_64-unknown-linux-musl/release/phase1 \
  --out build/base1-real-boot/initramfs-real-phase1.img

sh scripts/base1-preview-stack.sh \
  --bundle build/base1-real-phase1-demo \
  --kernel build/base1-real-boot/vmlinuz-virt \
  --initrd build/base1-real-boot/initramfs-real-phase1.img \
  --image-mb 64 \
  --no-qemu-check

PATH="/opt/homebrew/bin:$PATH" sh scripts/base1-qemu-boot-check.sh \
  --bundle build/base1-real-phase1-demo \
  --execute \
  --confirm launch-qemu-base1-preview \
  --timeout 60 \
  --expect "phase1.workspace"
~~~

Observed serial evidence included:

- `Linux version`
- `base1 init wrapper reached`
- `base1 launching real Phase1 binary`
- `phase1.log`
- `phase1.workspace`
- `phase1.conf`

## Result

PASS for preview-level evidence: the guarded QEMU serial-capture path observed output from the real Phase1 binary inside the preview initramfs.

Exit code `124` is expected because `gtimeout` terminates the bounded QEMU run after the timeout.

## Non-claims

This report does not claim:

- Installer readiness.
- Hardware validation.
- Recovery completeness.
- Daily-driver readiness.
- A released Base1 image.
- Full Base1 userspace handoff.
- That Phase1 cleanly exited.
- That the preview initramfs is production-ready.

## Promotion rule

Base1 may only move beyond this evidence level after the real Phase1 binary is launched by the normal Base1 boot payload with serial evidence, provenance, repeatable build instructions, installer boundary checks, and a non-preview release boundary.

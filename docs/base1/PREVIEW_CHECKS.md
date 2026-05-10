# Base1 Preview Checks

This page records the current command set for validating the Base1 safe preview stack after syncing `edge/stable`.

It is a command checklist only. It does not launch the emulator, install Base1, validate hardware, complete recovery, create a released image, or prove that Base1 is bootable.

## Sync

~~~sh
cd ~/phase1_library/phase1
git checkout edge/stable
git pull --ff-only origin edge/stable
~~~

## Preview-stack test set

Run the full preview-stack guard set:

~~~sh
cargo test -p phase1 --test base1_preview_inputs_script
cargo test -p phase1 --test base1_emulator_preview_script
cargo test -p phase1 --test base1_emulator_doctor_script
cargo test -p phase1 --test base1_preview_gate_script
cargo test -p phase1 --test base1_preview_provenance_script
cargo test -p phase1 --test base1_preview_verify_script
cargo test -p phase1 --test base1_preview_stack_script
cargo test -p phase1 --test base1_preview_stack_runbook_docs
cargo test -p phase1 --test base1_preview_stack_validation_report_docs
~~~

## Safe manual smoke

~~~sh
mkdir -p build
printf 'kernel placeholder\n' > build/base1-test-vmlinuz
printf 'initrd placeholder\n' > build/base1-test-initrd.img

sh scripts/base1-preview-stack.sh \
  --bundle build/base1-preview-stack-demo \
  --kernel build/base1-test-vmlinuz \
  --initrd build/base1-test-initrd.img \
  --image-mb 1 \
  --no-qemu-check
~~~

The safe stack should end with:

~~~text
result: pass
base1 preview stack: complete: safe preview stack passed with provenance verification
~~~

## Evidence produced

A successful safe stack run should produce:

- `build/base1-preview-stack-demo/reports/provenance.env`
- `build/base1-preview-stack-demo/reports/SHA256SUMS`

The final verification step reads `reports/SHA256SUMS` and checks that each listed file still matches the recorded SHA-256.

## Non-claims

This page does not claim that Base1 is bootable.

It also does not claim:

- Base1 is installer-ready.
- Base1 is hardware-validated.
- Base1 is recovery-complete.
- Base1 is daily-driver ready.
- The preview stack launches QEMU.
- The placeholder kernel or initrd is a validated Base1 kernel/initrd.

Use `docs/base1/validation/2026-05-10-preview-stack.md` as the current dated evidence record for this preview-only path.

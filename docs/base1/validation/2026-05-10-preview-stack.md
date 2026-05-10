# Base1 Validation Report: Safe Preview Stack

Date: 2026-05-10

Status: preview evidence record

Result: PASS for preview-stack mechanics when the listed tests pass locally or in CI.

Promotion recommendation: keep Base1 at preview-only for this path. Do not promote this evidence to bootable, installer-ready, hardware-validated, recovery-complete, secure OS replacement, or daily-driver-ready status.

## Scope

This report records the current safe Base1 emulator-preview stack evidence.

The stack covered by this report is:

1. `scripts/base1-preview-inputs.sh`
2. `scripts/base1-emulator-preview.sh`
3. `scripts/base1-emulator-doctor.sh`
4. `scripts/base1-preview-gate.sh --dry-run`
5. `scripts/base1-preview-provenance.sh`
6. `scripts/base1-preview-verify.sh`
7. `scripts/base1-preview-stack.sh`

This report is about mechanics and guardrails only. It records that the preview stack can check inputs, generate a preview bundle under `build/`, run read-only checks, refuse unsafe handoff by default, write provenance/checksums, and verify the recorded checksums.

## Evidence files

Primary docs:

- `docs/base1/PREVIEW_STACK_RUNBOOK.md`
- `docs/base1/README.md`
- `docs/base1/READINESS_MATRIX.md`
- `docs/base1/VALIDATION_REPORT_TEMPLATE.md`
- `docs/base1/validation/README.md`

Primary scripts:

- `scripts/base1-preview-inputs.sh`
- `scripts/base1-emulator-preview.sh`
- `scripts/base1-emulator-doctor.sh`
- `scripts/base1-preview-gate.sh`
- `scripts/base1-preview-provenance.sh`
- `scripts/base1-preview-verify.sh`
- `scripts/base1-preview-stack.sh`

Primary tests:

- `tests/base1_preview_inputs_script.rs`
- `tests/base1_emulator_preview_script.rs`
- `tests/base1_emulator_doctor_script.rs`
- `tests/base1_preview_gate_script.rs`
- `tests/base1_preview_provenance_script.rs`
- `tests/base1_preview_verify_script.rs`
- `tests/base1_preview_stack_script.rs`
- `tests/base1_preview_stack_runbook_docs.rs`

## Validation commands

Recommended test commands:

```sh
cargo test -p phase1 --test base1_preview_inputs_script
cargo test -p phase1 --test base1_emulator_preview_script
cargo test -p phase1 --test base1_emulator_doctor_script
cargo test -p phase1 --test base1_preview_gate_script
cargo test -p phase1 --test base1_preview_provenance_script
cargo test -p phase1 --test base1_preview_verify_script
cargo test -p phase1 --test base1_preview_stack_script
cargo test -p phase1 --test base1_preview_stack_runbook_docs
```

Recommended safe manual smoke command:

```sh
mkdir -p build
printf 'kernel placeholder\n' > build/base1-test-vmlinuz
printf 'initrd placeholder\n' > build/base1-test-initrd.img

sh scripts/base1-preview-stack.sh \
  --bundle build/base1-preview-stack-demo \
  --kernel build/base1-test-vmlinuz \
  --initrd build/base1-test-initrd.img \
  --image-mb 1 \
  --no-qemu-check

sh scripts/base1-preview-verify.sh \
  --bundle build/base1-preview-stack-demo
```

## Expected outputs

A successful preview-stack run creates a bundle under `build/` with preview-only artifacts including:

- `manifest.env`
- `README.txt`
- `base1-sandbox.raw`
- `base1-rootfs-preview.tar` when present
- `run-qemu-bundle.sh`
- `staging/manifest.env`
- `staging/boot/grub/grub.cfg`
- `staging/boot/vmlinuz`
- `staging/boot/initrd.img`
- `reports/provenance.env`
- `reports/SHA256SUMS`

A successful verification run reports SHA-256 matches for each listed file in `reports/SHA256SUMS`.

## Guardrails observed

The current stack is designed to preserve these guardrails:

- Output bundle path remains under `build/`.
- QEMU is not launched by the stack.
- The gate path defaults to dry-run.
- The execution path requires a separate explicit confirmation phrase in `scripts/base1-preview-gate.sh`.
- Provenance is recorded inside the preview bundle.
- Checksum verification fails on drift.
- Real-device write tools are excluded from the preview scripts and tests.
- The runbook states the preview-only boundary.

## What this validates

This evidence validates the following narrow claims only:

- The safe preview stack exists.
- The preview stack is wired in a predictable order.
- Placeholder kernel/initrd inputs can be packaged into a preview bundle.
- The bundle doctor can inspect the generated preview bundle.
- The preview gate can perform a dry-run without launching QEMU.
- Provenance and SHA-256 checksum files can be generated.
- The verifier can detect checksum drift.
- The documentation names the safe path and non-claims.

## What this does not validate

This report does not claim that Base1 is bootable.

This report also does not validate or claim:

- A released Base1 image.
- A secure OS replacement.
- A daily-driver-ready system.
- A hardware installation path.
- A destructive installer path.
- Recovery USB completion.
- Rollback completion.
- Real hardware behavior.
- Emulator launch success.
- Kernel correctness.
- Initrd correctness.
- GRUB runtime behavior.
- Any chain of trust beyond preview-bundle checksums.

## Promotion boundary

This report supports the preview evidence level (`preview`) for the safe preview stack mechanics only.

Do not use this report to mark Base1 as bootable, validated on hardware, installer-ready, recovery-complete, secure OS replacement, or daily-driver ready.

Before promotion to any stronger status, require a separate report with real target identity, exact kernel/initrd provenance, emulator launch evidence, boot logs, failure logs, rollback/recovery notes, and explicit non-claims for what remains untested.

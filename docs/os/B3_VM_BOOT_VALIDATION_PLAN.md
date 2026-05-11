# Base1 B3 VM boot validation plan

Status: planning plus initial UEFI proof-of-life script
Scope: evidence needed before Base1 can claim VM boot validation for a named profile

## Purpose

B3 is the next boot-readiness target after B2 dry-run assembly.

B3 means a named virtual-machine profile has been run through an expected boot path and has a written validation report. It does not mean physical hardware support, installer readiness, hardening, release-candidate readiness, or daily-driver readiness.

The first concrete bridge toward B3 is now a QEMU/OVMF UEFI proof-of-life script. It is intentionally narrower than full B3 validation: it builds a local UEFI FAT image, boots it in QEMU, displays the fitted Phase1 word-mark splash, emits a serial marker, and can check that marker in a captured serial log.

## Entry gate

B3 validation should not start until the focused B2 test suite has passed locally or in CI.

B2 test bundle:

- [`B2_DRY_RUN_ASSEMBLY_TEST_SUITE.md`](B2_DRY_RUN_ASSEMBLY_TEST_SUITE.md)

The UEFI proof-of-life path may be used as development scaffolding before full B3 validation, but it must not be described as installer readiness, hardware validation, or full VM validation by itself.

## Initial profile

Start with:

```text
x86_64-vm-validation
```

This keeps the next validation step away from physical hardware claims.

## Planned dry-run command shape

```bash
sh scripts/base1-b3-vm-validate.sh --dry-run --profile x86_64-vm-validation
```

The first command surface should be dry-run only. Any real VM run should come later with a separate validation report and captured logs.

## Current proof-of-life command shape

Build the local UEFI proof image:

```bash
sh scripts/base1-b3-uefi-proof.sh --build
```

Run it visibly in QEMU:

```bash
sh scripts/base1-b3-uefi-proof.sh --build --run
```

Run the evidence check with serial capture:

```bash
sh scripts/base1-b3-uefi-proof.sh --build --check
```

The check looks for this serial marker:

```text
phase1 6.0.0 ready
```

Expected local outputs:

```text
build/base1-b3-uefi-proof.img
build/base1-b3-uefi-proof/reports/b3-serial.log
build/base1-b3-uefi-proof/reports/b3-summary.env
```

This proof image is a local build artifact and should not be committed.

## Evidence required for B3

A future B3 validation report should include:

- selected VM profile;
- VM runtime used;
- architecture and firmware mode;
- boot artifact identifier;
- command used for the VM run;
- boot result;
- Phase1 launch result;
- emergency fallback result or known limitation;
- captured logs path;
- known limitations;
- explicit non-claims.

## B3 checklist

- [ ] B2 test suite has passed locally or in CI.
- [ ] VM profile is explicit.
- [ ] VM runtime is explicit.
- [ ] Boot artifact is explicit.
- [ ] Boot command is documented.
- [ ] Logs are captured.
- [ ] Phase1 launch result is recorded.
- [ ] Emergency fallback result or limitation is recorded.
- [ ] Known limitations are documented.
- [ ] VM result is not generalized to physical hardware.
- [ ] Non-claims are preserved.

## Proof-of-life checklist

- [x] B3 UEFI proof script exists.
- [x] B3 UEFI proof script tests exist.
- [x] Proof image path is local under `build/`.
- [x] QEMU launch uses OVMF pflash and an explicit raw image drive.
- [x] Serial marker is explicit.
- [x] Serial log and summary output paths are explicit.
- [x] Phase1 word-mark splash is fitted before GRUB displays it.
- [x] Non-claims are preserved.
- [ ] Proof script has passed locally on a Mac with QEMU, mtools, and x86_64 GRUB installed.
- [ ] Passing proof evidence has been copied into a validation report.

## Related docs

- [`BOOT_READINESS_STATUS.md`](BOOT_READINESS_STATUS.md)
- [`BOOT_READINESS_RACE_PLAN.md`](BOOT_READINESS_RACE_PLAN.md)
- [`ROADMAP.md`](ROADMAP.md)
- [`X86_64_BOOT_SUPPORT_ROADMAP.md`](X86_64_BOOT_SUPPORT_ROADMAP.md)
- [`B2_DRY_RUN_ASSEMBLY_PLAN.md`](B2_DRY_RUN_ASSEMBLY_PLAN.md)
- [`B2_DRY_RUN_ASSEMBLY_TEST_SUITE.md`](B2_DRY_RUN_ASSEMBLY_TEST_SUITE.md)
- [`QEMU_VISUAL_BOOT_PREVIEW.md`](QEMU_VISUAL_BOOT_PREVIEW.md)

## Non-claims

This B3 plan does not make Base1 bootable on physical hardware, installer-ready, recovery-complete, hardened, hardware-validated, release-candidate ready, or daily-driver ready.

It defines the evidence required for a future named VM boot validation claim. The current `base1-b3-uefi-proof.sh` script is only a QEMU/OVMF proof-of-life scaffold until a validation report with captured logs exists.

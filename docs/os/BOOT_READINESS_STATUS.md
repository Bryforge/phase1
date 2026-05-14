# Base1 boot readiness status

Status: active readiness tracker
Scope: Base1 boot readiness levels, open prerequisites, validation evidence, and coding-start gate

## Purpose

This tracker shows what must be finished before Phase1/Base1 moves from planning back into implementation work for boot readiness.

The finish-first planning layer is complete for B1. The first B1 read-only detection script, guard tests, limitations note, limitations tests, validation report, and validation report tests now exist while preserving the B1 boundary.

B2 planning has started with a dry-run assembly plan, plan tests, initial dry-run assembly script, script tests, limitations note, limitations tests, validation report, validation report tests, output review, output review tests, focused test-suite command bundle, test-suite bundle tests, focused test-suite checker script, focused test-suite checker tests, OS roadmap boot-readiness tests, QEMU visual boot preview script, visual boot preview script tests, QEMU visual boot preview docs, QEMU visual boot preview doc tests, and a local focused-suite pass record.

B3 planning has started with a VM boot validation plan, plan tests, limitations note, limitations tests, VM boot log capture notes, VM boot log capture notes tests, an initial B3 UEFI proof-of-life script, B3 UEFI proof script tests, B3 kernel/initrd handoff script, B3 kernel/initrd handoff tests, B3 kernel/initrd handoff docs, B3 kernel/initrd handoff doc tests, B3 GNU/Linux stage script, B3 GNU/Linux stage tests, B3 GNU/Linux stage docs, B3 GNU/Linux stage doc tests, B3 OpenBSD stage script, B3 OpenBSD stage tests, B3 OpenBSD stage docs, B3 OpenBSD stage doc tests, B3 OpenBSD serial limitation docs, B3 OpenBSD serial limitation tests, B3 VM validation script, B3 VM validation script tests, an initial B5 local boot artifact planning evidence is recorded in `build/base1-local-boot-artifact/local-boot-artifact-plan.env` with `BASE1_LOCAL_BOOT_ARTIFACT_PRESENT=yes` and remains a local artifact planning claim only.

B4 reviewed recovery evidence is recorded in [`B4_RECOVERY_EVIDENCE.md`](B4_RECOVERY_EVIDENCE.md) with `reviewed_recovery_evidence`, planned boot artifact, planned rollback path, planned emergency stop path, and `BASE1_B4_RECOVERY_CLAIM=not_claimed`. This remains recovery-validation evidence only and does not claim physical hardware readiness.

B3 reviewed VM evidence is recorded in [`B3_REVIEWED_VM_EVIDENCE.md`](B3_REVIEWED_VM_EVIDENCE.md) with `reviewed_vm_evidence`, `BASE1_B3_EVIDENCE_STATE=evidence-present`, `BASE1_B3_EVIDENCE_SUMMARY_COUNT=4`, and `BASE1_B3_LOG_REVIEW_RESULT=pass`. This remains VM evidence only and does not claim physical hardware readiness.

B3 VM validation report scaffold, and B3 VM validation report tests.

## Current readiness level

Current level: **B2 — Dry-run assembly ready, initial script present**

Target next level: **B3 — VM boot validated**

Do not claim Base1 boot readiness, installer readiness, hardware validation, hardened status, recovery completion, or daily-driver readiness from this tracker alone.

## Boot readiness ladder

| Level | Name | Status | Required before claim strengthens |
| --- | --- | --- | --- |
| B0 | Documentation ready | Complete for B1 start | Roadmaps, status tracker, checklist, links, tests. |
| B1 | Read-only detection ready | Initial script present | Dry-run detection script, no writes, architecture/firmware/boot hints, limitations note, validation report. |
| B2 | Dry-run assembly ready | Local focused-suite pass record present | Dry-run assembly plan, image/install/recovery previews with explicit no-write behavior, limitations note, validation report, output review, test-suite bundle. |
| B3 | VM boot validated | Planning, proof-of-life, handoff, GNU/Linux, OpenBSD, and validation-report scaffolds present | VM boot validation plan, reviewed VM boot report, logs, known limitations. |
| B4 | Recovery validated | Not started | Emergency shell, recovery media, rollback report. |
| B5 | Physical target validated | Not started | Named hardware validation report. |
| B6 | Release candidate | Not started | Repeatable build, validation bundle, docs, rollback evidence. |

## Finish-before-coding checklist

Before coding the B1 implementation slice, the finish-first checklist was completed:

- [x] Boot readiness race plan.
- [x] x86_64 boot support roadmap.
- [x] README boot-readiness and x86_64 references.
- [x] Contribution guidelines for hardening and x86_64 boot work.
- [x] Repository navigation and reorganization indexes.
- [x] Asset index and current Fyr asset references.
- [x] Boot readiness status tracker.
- [x] Boot readiness status tracker tests.
- [x] OS roadmap link to this status tracker.
- [x] README link to this status tracker.
- [x] B1 implementation issue/plan for read-only x86_64 detection.
- [x] B1 plan tests.
- [x] B1 plan link from OS roadmap, x86_64 roadmap, and race plan.

Finish-first status: **complete for B1 implementation start**.

## B1 implementation status

B1 initial implementation is now present:

```bash
sh scripts/base1-x86_64-detect.sh --dry-run
```

Initial B1 tests are present:

```bash
cargo test -p phase1 --test base1_x86_64_detect_script
```

B1 limitations are documented in [`B1_READ_ONLY_DETECTION_LIMITATIONS.md`](B1_READ_ONLY_DETECTION_LIMITATIONS.md).

B1 limitations tests are present:

```bash
cargo test -p phase1 --test b1_read_only_detection_limitations_docs
```

B1 validation expectations are documented in [`B1_READ_ONLY_DETECTION_VALIDATION.md`](B1_READ_ONLY_DETECTION_VALIDATION.md).

B1 completion evidence is recorded locally in `build/base1-b1-completion/b1-completion-summary.env` and remains a read-only detection claim only.

B1 validation report tests are present:

```bash
cargo test -p phase1 --test b1_read_only_detection_validation_docs
```

The B1 detector must stay inside the read-only, dry-run, non-mutating scope defined by [`B1_READ_ONLY_DETECTION_PLAN.md`](B1_READ_ONLY_DETECTION_PLAN.md).

## B1 completion checklist

Before B1 is considered complete, confirm:

- [x] B1 implementation plan exists.
- [x] B1 read-only detector script exists.
- [x] B1 detector script tests exist.
- [x] B1 known limitations are documented.
- [x] B1 limitations tests exist.
- [x] B1 validation report exists.
- [x] B1 validation report tests exist.
- [x] B1 detector test suite passes in CI or local validation.
- [x] B1 status is linked from README, OS roadmap, race plan, and x86_64 roadmap after implementation.
- [x] B1 output is reviewed for secret redaction.
- [x] B1 does not contain mutating boot, disk, package, or network commands.

## B2 implementation status

B2 dry-run assembly initial implementation is now present:

```bash
sh scripts/base1-b2-assembly-dry-run.sh --dry-run --profile x86_64-vm-validation
```

B2 plan tests are present:

```bash
cargo test -p phase1 --test b2_dry_run_assembly_plan_docs
```

B2 script tests are present:

```bash
cargo test -p phase1 --test base1_b2_assembly_dry_run_script
```

B2 limitations are documented in [`B2_DRY_RUN_ASSEMBLY_LIMITATIONS.md`](B2_DRY_RUN_ASSEMBLY_LIMITATIONS.md).

B2 limitations tests are present:

```bash
cargo test -p phase1 --test b2_dry_run_assembly_limitations_docs
```

B2 validation expectations are documented in [`B2_DRY_RUN_ASSEMBLY_VALIDATION.md`](B2_DRY_RUN_ASSEMBLY_VALIDATION.md).

B2 validation report tests are present:

```bash
cargo test -p phase1 --test b2_dry_run_assembly_validation_docs
```

B2 output review is documented in [`B2_DRY_RUN_ASSEMBLY_OUTPUT_REVIEW.md`](B2_DRY_RUN_ASSEMBLY_OUTPUT_REVIEW.md).

B2 output review tests are present:

```bash
cargo test -p phase1 --test b2_dry_run_assembly_output_review_docs
```

B2 focused test-suite evidence is recorded locally in `build/base1-b2-test-suite/b2-test-suite-summary.env` with `BASE1_B2_TEST_SUITE_RESULT=pass` and remains a dry-run assembly claim only.

B2 focused test-suite command bundle is documented in [`B2_DRY_RUN_ASSEMBLY_TEST_SUITE.md`](B2_DRY_RUN_ASSEMBLY_TEST_SUITE.md).

B2 test-suite bundle tests are present:

```bash
cargo test -p phase1 --test b2_dry_run_assembly_test_suite_docs
```

B2 focused test-suite checker script is present:

```bash
sh scripts/base1-b2-test-suite-check.sh --check --write-report
```

B2 focused test-suite checker tests are present:

```bash
cargo test -p phase1 --test base1_b2_test_suite_check_script
```

B2 focused test-suite local pass record is present:

```text
build/base1-b2-test-suite/b2-test-suite-summary.env
```

OS roadmap boot-readiness tests are present:

```bash
cargo test -p phase1 --test os_roadmap_boot_readiness_docs
```

B2 QEMU visual boot preview script is present:

```bash
sh scripts/base1-qemu-visual-boot-preview.sh --build
sh scripts/base1-qemu-visual-boot-preview.sh --build --run
```

B2 QEMU visual boot preview script tests are present:

```bash
cargo test -p phase1 --test base1_qemu_visual_boot_preview_script
```

B2 QEMU visual boot preview documentation is present:

- [`QEMU_VISUAL_BOOT_PREVIEW.md`](QEMU_VISUAL_BOOT_PREVIEW.md)

B2 QEMU visual boot preview documentation tests are present:

```bash
cargo test -p phase1 --test qemu_visual_boot_preview_docs
```

B2 status and boundaries are linked from README, OS roadmap, race plan, and x86_64 roadmap.

B2 local focused-suite validation has passed, but the QEMU visual boot preview remains showcase-only and does not strengthen boot-readiness claims.

## B2 completion checklist

Before B2 is considered complete, confirm:

- [x] B2 dry-run assembly plan exists.
- [x] B2 plan tests exist.
- [x] B2 dry-run assembly script exists.
- [x] B2 script tests exist.
- [x] B2 known limitations are documented.
- [x] B2 limitations tests exist.
- [x] B2 validation report exists.
- [x] B2 validation report tests exist.
- [x] B2 status is linked from README, OS roadmap, race plan, and x86_64 roadmap.
- [x] B2 does not contain mutating boot, disk, package, or network commands.
- [x] B2 output is reviewed for secret redaction.
- [x] B2 output review tests exist.
- [x] B2 focused test-suite command bundle exists.
- [x] B2 test-suite bundle tests exist.
- [x] B2 focused test-suite checker script exists.
- [x] B2 focused test-suite checker tests exist.
- [x] B2 test suite passes in CI or local validation.
- [x] OS roadmap boot-readiness tests exist.
- [x] B2 QEMU visual boot preview script exists.
- [x] B2 QEMU visual boot preview script tests exist.
- [x] B2 QEMU visual boot preview documentation exists.
- [x] B2 QEMU visual boot preview documentation tests exist.

## B3 planning status

B3 remains planning plus proof-of-life, handoff, and GNU/Linux staging scaffolding until B2 validation has passed locally or in CI and B3 validation logs and report exist.


B3 VM boot validation planning is now present:

- [`B3_VM_BOOT_VALIDATION_PLAN.md`](B3_VM_BOOT_VALIDATION_PLAN.md)
- [`B3_VM_BOOT_LOGS.md`](B3_VM_BOOT_LOGS.md) — B3 VM boot logs.

B3 plan tests are present:

```bash
cargo test -p phase1 --test b3_vm_boot_validation_plan_docs
```

B3 limitations are documented in [`B3_VM_BOOT_VALIDATION_LIMITATIONS.md`](B3_VM_BOOT_VALIDATION_LIMITATIONS.md).

B3 limitations tests are present:

```bash
cargo test -p phase1 --test b3_vm_boot_validation_limitations_docs
```

B3 VM boot log capture notes are documented in [`B3_VM_BOOT_LOGS.md`](B3_VM_BOOT_LOGS.md).

B3 log capture notes exist.

B3 VM boot logs are documented in [`B3_VM_BOOT_LOGS.md`](B3_VM_BOOT_LOGS.md).

B3 VM boot log capture notes tests are present:

```bash
cargo test -p phase1 --test b3_vm_boot_logs_docs
```

Planned B3 dry-run command shape:

```bash
sh scripts/base1-b3-vm-validate.sh --dry-run --profile x86_64-vm-validation
```

B3 UEFI proof-of-life script is present:

```bash
sh scripts/base1-b3-uefi-proof.sh --build
sh scripts/base1-b3-uefi-proof.sh --build --run
sh scripts/base1-b3-uefi-proof.sh --build --check
```

B3 UEFI proof-of-life script tests are present:

```bash
cargo test -p phase1 --test base1_b3_uefi_proof_script
```

B3 kernel/initrd handoff script is present:

```bash
sh scripts/base1-b3-kernel-handoff.sh --kernel /path/to/vmlinuz --initrd /path/to/initrd.img --prepare
sh scripts/base1-b3-kernel-handoff.sh --kernel /path/to/vmlinuz --initrd /path/to/initrd.img --dry-run
sh scripts/base1-b3-kernel-handoff.sh --kernel /path/to/vmlinuz --initrd /path/to/initrd.img --check
```

B3 kernel/initrd handoff script tests are present:

```bash
cargo test -p phase1 --test base1_b3_kernel_handoff_script
```

B3 kernel/initrd handoff documentation is present:

- [`B3_KERNEL_INITRD_HANDOFF.md`](B3_KERNEL_INITRD_HANDOFF.md)

B3 kernel/initrd handoff documentation tests are present:

```bash
cargo test -p phase1 --test b3_kernel_initrd_handoff_docs
```

B3 GNU/Linux stage script is present:

```bash
sh scripts/base1-b3-gnulinux-stage.sh --kernel /path/to/vmlinuz --initrd /path/to/initrd.img --prepare
sh scripts/base1-b3-gnulinux-stage.sh --boot /path/to/boot --dry-run
sh scripts/base1-b3-gnulinux-stage.sh --boot /path/to/boot --check
```

B3 GNU/Linux stage script tests are present:

```bash
cargo test -p phase1 --test base1_b3_gnulinux_stage_script
```

B3 GNU/Linux stage documentation is present:

- [`B3_GNULINUX_STAGE.md`](B3_GNULINUX_STAGE.md)

B3 GNU/Linux stage documentation tests are present:

```bash
cargo test -p phase1 --test b3_gnulinux_stage_docs
```

B3 OpenBSD stage script is present:

```bash
sh scripts/base1-b3-openbsd-stage.sh --img /path/to/miniroot.img --dry-run
sh scripts/base1-b3-openbsd-stage.sh --img /path/to/miniroot.img --check --check-mode launch
```

B3 OpenBSD stage script tests are present:

```bash
cargo test -p phase1 --test base1_b3_openbsd_stage_script
```

B3 OpenBSD stage documentation is present:

- [`B3_OPENBSD_STAGE.md`](B3_OPENBSD_STAGE.md)

B3 OpenBSD stage documentation tests are present:

```bash
cargo test -p phase1 --test b3_openbsd_stage_docs
```

B3 OpenBSD serial marker limitation is documented:

- [`B3_OPENBSD_SERIAL_LIMITATION.md`](B3_OPENBSD_SERIAL_LIMITATION.md)

B3 OpenBSD serial marker limitation tests are present:

```bash
cargo test -p phase1 --test b3_openbsd_serial_limitation_docs
```

B3 VM validation script is present:

```bash
sh scripts/base1-b3-vm-validate.sh --dry-run --profile x86_64-vm-validation
```

B3 VM validation script tests are present:

```bash
cargo test -p phase1 --test base1_b3_vm_validate_script
```

B3 VM validation report scaffold is present:

- [`B3_VM_VALIDATION_REPORT.md`](B3_VM_VALIDATION_REPORT.md)

B3 VM validation report tests are present:

```bash
cargo test -p phase1 --test b3_vm_validation_report_docs
```

B3 remains planning plus proof-of-life, handoff, GNU/Linux staging, OpenBSD staging, and validation-report scaffolding until a reviewed B3 log bundle/report exists.

## B3 completion checklist

Before B3 is considered complete, confirm:

- [x] B3 VM boot validation plan exists.
- [x] B3 plan tests exist.
- [x] B3 limitations note exists.
- [x] B3 limitations tests exist.
- [x] B3 log capture notes exists.
- [x] B3 log capture notes tests exist.
- [x] B2 test suite has passed locally or in CI.
- [x] B3 UEFI proof script exists.
- [x] B3 UEFI proof script tests exist.
- [x] B3 kernel/initrd handoff script exists.
- [x] B3 kernel/initrd handoff script tests exist.
- [x] B3 kernel/initrd handoff documentation exists.
- [x] B3 kernel/initrd handoff documentation tests exist.
- [x] B3 GNU/Linux stage script exists.
- [x] B3 GNU/Linux stage script tests exist.
- [x] B3 GNU/Linux stage documentation exists.
- [x] B3 GNU/Linux stage documentation tests exist.
- [x] B3 OpenBSD stage script exists.
- [x] B3 OpenBSD stage script tests exist.
- [x] B3 OpenBSD stage documentation exists.
- [x] B3 OpenBSD stage documentation tests exist.
- [x] B3 OpenBSD serial marker limitation is documented.
- [x] B3 OpenBSD serial marker limitation tests exist.
- [x] B3 VM validation script exists.
- [x] B3 VM validation script tests exist.
- [x] B3 validation report scaffold exists.
- [x] B3 validation report scaffold tests exist.
- [x] known-good local kernel/initrd pair has been staged and checked.
- [x] known-good local GNU/Linux kernel/initrd pair has been staged and checked.
- [x] OpenBSD launch-check evidence has been staged and checked.
- [x] OpenBSD serial marker evidence is captured or documented as a limitation.
- [ ] B3 validation report has been reviewed against captured logs.
- [ ] VM profile is explicit.
- [ ] VM runtime is explicit.
- [ ] boot artifact is explicit.
- [ ] boot logs are captured.
- [ ] Phase1 launch result is recorded.
- [ ] non-claims are preserved.

## Planned first coding slice

The first implementation slice is:

```bash
sh scripts/base1-x86_64-detect.sh --dry-run
```

Implementation plan:

- [`B1_READ_ONLY_DETECTION_PLAN.md`](B1_READ_ONLY_DETECTION_PLAN.md)

Limitations note:

- [`B1_READ_ONLY_DETECTION_LIMITATIONS.md`](B1_READ_ONLY_DETECTION_LIMITATIONS.md)

Validation report:

- [`B1_READ_ONLY_DETECTION_VALIDATION.md`](B1_READ_ONLY_DETECTION_VALIDATION.md)

Expected behavior:

- read-only;
- no host mutation;
- reports `writes: no`;
- reports architecture hints;
- reports firmware hints;
- reports boot-loader hints;
- reports virtualization hints;
- reports storage-layout hints;
- reports recovery availability hints;
- fails closed when required facts are unknown.

## Evidence map

| Evidence | Status | Path |
| --- | --- | --- |
| OS roadmap | Present | [`ROADMAP.md`](ROADMAP.md) |
| OS roadmap boot-readiness tests | Present | `tests/os_roadmap_boot_readiness_docs.rs` |
| Boot readiness race plan | Present | [`BOOT_READINESS_RACE_PLAN.md`](BOOT_READINESS_RACE_PLAN.md) |
| x86_64 boot support roadmap | Present | [`X86_64_BOOT_SUPPORT_ROADMAP.md`](X86_64_BOOT_SUPPORT_ROADMAP.md) |
| Boot readiness status tracker | Present | [`BOOT_READINESS_STATUS.md`](BOOT_READINESS_STATUS.md) |
| B1 read-only detection plan | Present | [`B1_READ_ONLY_DETECTION_PLAN.md`](B1_READ_ONLY_DETECTION_PLAN.md) |
| B1 limitations note | Present | [`B1_READ_ONLY_DETECTION_LIMITATIONS.md`](B1_READ_ONLY_DETECTION_LIMITATIONS.md) |
| B1 validation report | Present | [`B1_READ_ONLY_DETECTION_VALIDATION.md`](B1_READ_ONLY_DETECTION_VALIDATION.md) |
| B1 plan tests | Present | `tests/b1_read_only_detection_plan_docs.rs` |
| B1 limitations tests | Present | `tests/b1_read_only_detection_limitations_docs.rs` |
| B1 validation report tests | Present | `tests/b1_read_only_detection_validation_docs.rs` |
| B1 detection script | Present | `scripts/base1-x86_64-detect.sh` |
| B1 detection tests | Present | `tests/base1_x86_64_detect_script.rs` |
| B2 dry-run assembly plan | Present | [`B2_DRY_RUN_ASSEMBLY_PLAN.md`](B2_DRY_RUN_ASSEMBLY_PLAN.md) |
| B2 dry-run assembly limitations note | Present | [`B2_DRY_RUN_ASSEMBLY_LIMITATIONS.md`](B2_DRY_RUN_ASSEMBLY_LIMITATIONS.md) |
| B2 dry-run assembly validation report | Present | [`B2_DRY_RUN_ASSEMBLY_VALIDATION.md`](B2_DRY_RUN_ASSEMBLY_VALIDATION.md) |
| B2 dry-run assembly output review | Present | [`B2_DRY_RUN_ASSEMBLY_OUTPUT_REVIEW.md`](B2_DRY_RUN_ASSEMBLY_OUTPUT_REVIEW.md) |
| B2 dry-run assembly test suite | Present | [`B2_DRY_RUN_ASSEMBLY_TEST_SUITE.md`](B2_DRY_RUN_ASSEMBLY_TEST_SUITE.md) |
| B2 dry-run assembly plan tests | Present | `tests/b2_dry_run_assembly_plan_docs.rs` |
| B2 dry-run assembly limitations tests | Present | `tests/b2_dry_run_assembly_limitations_docs.rs` |
| B2 dry-run assembly validation tests | Present | `tests/b2_dry_run_assembly_validation_docs.rs` |
| B2 dry-run assembly output review tests | Present | `tests/b2_dry_run_assembly_output_review_docs.rs` |
| B2 dry-run assembly test-suite tests | Present | `tests/b2_dry_run_assembly_test_suite_docs.rs` |
| B2 dry-run assembly script | Present | `scripts/base1-b2-assembly-dry-run.sh` |
| B2 dry-run assembly tests | Present | `tests/base1_b2_assembly_dry_run_script.rs` |
| B2 focused test-suite checker script | Present | `scripts/base1-b2-test-suite-check.sh` |
| B2 focused test-suite checker tests | Present | `tests/base1_b2_test_suite_check_script.rs` |
| B2 focused test-suite local pass record | Present local evidence | `build/base1-b2-test-suite/b2-test-suite-summary.env` |
| B2 QEMU visual boot preview script | Present | `scripts/base1-qemu-visual-boot-preview.sh` |
| B2 QEMU visual boot preview tests | Present | `tests/base1_qemu_visual_boot_preview_script.rs` |
| B2 QEMU visual boot preview docs | Present | [`QEMU_VISUAL_BOOT_PREVIEW.md`](QEMU_VISUAL_BOOT_PREVIEW.md) |
| B2 QEMU visual boot preview docs tests | Present | `tests/qemu_visual_boot_preview_docs.rs` |
| B3 VM boot validation plan | Present | [`B3_VM_BOOT_VALIDATION_PLAN.md`](B3_VM_BOOT_VALIDATION_PLAN.md) |
| B3 VM boot validation limitations note | Present | [`B3_VM_BOOT_VALIDATION_LIMITATIONS.md`](B3_VM_BOOT_VALIDATION_LIMITATIONS.md) |
| B3 VM boot log capture notes | Present | [`B3_VM_BOOT_LOGS.md`](B3_VM_BOOT_LOGS.md) |
| B3 VM boot log capture notes tests | Present | `tests/b3_vm_boot_logs_docs.rs` |
| B3 kernel/initrd handoff doc | Present | [`B3_KERNEL_INITRD_HANDOFF.md`](B3_KERNEL_INITRD_HANDOFF.md) |
| B3 GNU/Linux stage doc | Present | [`B3_GNULINUX_STAGE.md`](B3_GNULINUX_STAGE.md) |
| B3 OpenBSD stage doc | Present | [`B3_OPENBSD_STAGE.md`](B3_OPENBSD_STAGE.md) |
| B3 OpenBSD serial limitation doc | Present | [`B3_OPENBSD_SERIAL_LIMITATION.md`](B3_OPENBSD_SERIAL_LIMITATION.md) |
| B3 VM validation report scaffold | Present | [`B3_VM_VALIDATION_REPORT.md`](B3_VM_VALIDATION_REPORT.md) |
| B3 VM boot validation plan tests | Present | `tests/b3_vm_boot_validation_plan_docs.rs` |
| B3 VM boot validation limitations tests | Present | `tests/b3_vm_boot_validation_limitations_docs.rs` |
| B3 kernel/initrd handoff doc tests | Present | `tests/b3_kernel_initrd_handoff_docs.rs` |
| B3 GNU/Linux stage doc tests | Present | `tests/b3_gnulinux_stage_docs.rs` |
| B3 OpenBSD stage doc tests | Present | `tests/b3_openbsd_stage_docs.rs` |
| B3 OpenBSD serial limitation tests | Present | `tests/b3_openbsd_serial_limitation_docs.rs` |
| B3 VM validation report tests | Present | `tests/b3_vm_validation_report_docs.rs` |
| B3 UEFI proof script | Present | `scripts/base1-b3-uefi-proof.sh` |
| B3 UEFI proof tests | Present | `tests/base1_b3_uefi_proof_script.rs` |
| B3 kernel/initrd handoff script | Present | `scripts/base1-b3-kernel-handoff.sh` |
| B3 kernel/initrd handoff tests | Present | `tests/base1_b3_kernel_handoff_script.rs` |
| B3 GNU/Linux stage script | Present | `scripts/base1-b3-gnulinux-stage.sh` |
| B3 GNU/Linux stage tests | Present | `tests/base1_b3_gnulinux_stage_script.rs` |
| B3 OpenBSD stage script | Present | `scripts/base1-b3-openbsd-stage.sh` |
| B3 OpenBSD stage tests | Present | `tests/base1_b3_openbsd_stage_script.rs` |
| Base1 dual-path delivery doc | Present | [`BASE1_DUAL_PATH_DELIVERY.md`](BASE1_DUAL_PATH_DELIVERY.md) |
| Base1 supervisor profiles doc | Present | [`BASE1_SUPERVISOR_PROFILES.md`](BASE1_SUPERVISOR_PROFILES.md) |
| Base1 supervisor orchestration doc | Present | [`BASE1_SUPERVISOR_ORCHESTRATION.md`](BASE1_SUPERVISOR_ORCHESTRATION.md) |
| Base1 supervisor control-plane contract | Present | [`BASE1_SUPERVISOR_CONTROL_PLANE.md`](BASE1_SUPERVISOR_CONTROL_PLANE.md) |
| Base1 supervisor policy bus contract | Present | [`BASE1_SUPERVISOR_POLICY_BUS.md`](BASE1_SUPERVISOR_POLICY_BUS.md) |
| Base1 supervisor orchestration planner | Present | `scripts/base1-supervisor-orchestration-plan.sh` |
| Base1 supervisor control-plane dispatcher | Present | `scripts/base1-supervisor-control-plane.sh` |
| Base1 supervisor policy bus dispatcher | Present | `scripts/base1-supervisor-policy-bus.sh` |
| Base1 supervisor storage-tier contract | Present | [`BASE1_SUPERVISOR_STORAGE_TIER.md`](BASE1_SUPERVISOR_STORAGE_TIER.md) |
| Base1 supervisor storage-tier contract tests | Present | `tests/base1_supervisor_storage_tier_docs.rs` |
| Base1 supervisor storage-tier planner | Present | `scripts/base1-supervisor-storage-tier-plan.sh` |
| Base1 supervisor storage-tier planner tests | Present | `tests/base1_supervisor_storage_tier_plan_script.rs` |
| B4 recovery validation doc | Present | [`B4_RECOVERY_VALIDATION.md`](B4_RECOVERY_VALIDATION.md) |
| B4 recovery evidence | Present | [`B4_RECOVERY_EVIDENCE.md`](B4_RECOVERY_EVIDENCE.md) |
| B5 local boot artifact plan | Present | [`B5_LOCAL_BOOT_ARTIFACT_PLAN.md`](B5_LOCAL_BOOT_ARTIFACT_PLAN.md) |
| B4 recovery validation script | Present | `scripts/base1-b4-recovery-validate.sh` |
| B3 reviewed VM evidence | Present | [`B3_REVIEWED_VM_EVIDENCE.md`](B3_REVIEWED_VM_EVIDENCE.md) |
| B3 VM validation script | Present | `scripts/base1-b3-vm-validate.sh` |
| B3 VM validation tests | Present | `tests/base1_b3_vm_validate_script.rs` |
| VM validation report | Present scaffold | [`B3_VM_VALIDATION_REPORT.md`](B3_VM_VALIDATION_REPORT.md) |
| Recovery validation report | Not started | planned |
| Hardware validation report | Not started | planned |

## Hardening status

Hardening is a roadmap goal and design direction.

Current status: **planned, evidence-bound**.

Do not describe the current Base1 boot path as hardened until implementation, tests, validation reports, recovery evidence, and review evidence support that claim.

## Non-claims

This status tracker does not make Base1 bootable, installer-ready, recovery-complete, hardened, hardware-validated, release-candidate ready, or daily-driver ready.

It records that the first B1 read-only detection script exists and remains bounded to detection-preview behavior only. B2 has an initial dry-run assembly script and local focused-suite pass record, but remains dry-run preview only. B3 is planning plus proof-of-life, handoff, GNU/Linux staging, OpenBSD staging, and validation-report scaffolding until reviewed VM validation evidence exists.

B6 named X200 hardware boot evidence is present.

Current B6 X200 evidence is recorded in `build/base1-b6-hardware-boot-evidence/b6-hardware-boot-evidence.env` with `BASE1_B6_HARDWARE_BOOT_RESULT=phase1_marker_seen` and `BASE1_B6_HARDWARE_BOOT_CLAIM=not_claimed`.

B6 has named X200 hardware marker evidence, but it remains non-claim evidence only: no installer, recovery-complete, hardening, release-candidate, or daily-driver claim.

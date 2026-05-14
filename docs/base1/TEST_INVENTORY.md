# Base1 test inventory

Status: active test inventory seed
Scope: Base1-related Rust tests and quality-gate tests

## Purpose

This inventory records known Base1-related tests before broader repository reorganization.

It is preservation-first. It does not remove, rename, or move tests. It gives maintainers a reviewable list of the test coverage that protects Base1 docs, scripts, release/checkpoint notes, compatibility paths, and quality gates.

## Inventory rule

Update this file when Base1-related tests are added, moved, renamed, or split.

Before broader organization work, compare this inventory against the repository `tests/` directory and update missing entries.

Use the read-only reporter:

```bash
sh scripts/base1-test-inventory.sh
```

The reporter lists `tests/base1_*.rs`, `tests/quality_base1_*.rs`, and other `tests/*base1*.rs` files without changing the repository.

Use the read-only verifier:

```bash
sh scripts/base1-test-inventory-verify.sh
```

The verifier compares reporter output against this document and fails if a reported Base1 test is missing from the inventory.

## Core and OS-track tests

| Test file | Coverage |
| --- | --- |
| `tests/base1_foundation.rs` | Base1 foundation docs, scripts, and initial contract. |
| `tests/base1_image_builder_docs.rs` | Base1 image-builder design references and Stage 1 boot-surface docs. |
| `tests/os_replacement_track_docs.rs` | Phase1 OS-track documentation guardrails. |
| `tests/base1_network_lockdown_docs.rs` | Base1 network lockdown dry-run docs, script guardrails, and command index. |

## Organization and quality tests

| Test file | Coverage |
| --- | --- |
| `tests/base1_inventory_docs.rs` | Base1 inventory and test inventory docs. |
| `tests/base1_manual_verification_rule_docs.rs` | Base1 manual verification rule docs. |
| `tests/base1_migration_table_docs.rs` | Base1 migration table docs. |
| `tests/base1_root_compatibility_map_docs.rs` | Root checkpoint-note compatibility map, mirror paths, and integrity references. |
| `tests/base1_script_compatibility_plan_docs.rs` | Script compatibility plan docs. |
| `tests/base1_link_check_strategy_docs.rs` | Link-check strategy docs. |
| `tests/base1_link_check_script.rs` | Local Base1 link-check script behavior. |
| `tests/base1_test_inventory_script.rs` | Test inventory reporter behavior. |
| `tests/base1_test_inventory_verify_script.rs` | Test inventory verifier behavior. |
| `tests/base1_reorganization_verify_script.rs` | Reorganization verification bundle behavior. |
| `tests/base1_reorganization_verification_report_template_docs.rs` | Reorganization verification report template docs. |
| `tests/base1_post_reorganization_layout_docs.rs` | Post-reorganization layout docs. |
| `tests/base1_pre_move_checklist_docs.rs` | Pre-move checklist docs. |
| `tests/base1_release_pre_move_checks_docs.rs` | Release/checkpoint pre-move checks docs. |
| `tests/base1_reorganization_readiness_docs.rs` | Reorganization readiness checklist docs. |
| `tests/quality_base1_docs_gate.rs` | Quality gate integration for `base1-docs`, integrity, link checks, and test inventory verification. |
| `tests/base1_b6_x200_marker_checkpoint_release_docs.rs` | B6 X200 marker checkpoint release note, release index, inventory entry, and non-claim boundary. |

## Libreboot tests

| Test file | Coverage |
| --- | --- |
| `tests/base1_libreboot_milestone_script.rs` | Libreboot milestone command. |
| `tests/base1_libreboot_milestone_docs.rs` | Libreboot milestone docs. |
| `tests/base1_libreboot_docs_script.rs` | Libreboot docs command. |
| `tests/base1_libreboot_docs_summary_docs.rs` | Libreboot docs summary. |
| `tests/base1_libreboot_quickstart_docs.rs` | Libreboot quickstart. |
| `tests/base1_libreboot_command_index_docs.rs` | Libreboot command index. |
| `tests/base1_libreboot_profile_docs.rs` | Libreboot profile docs. |
| `tests/base1_libreboot_preflight_docs.rs` | Libreboot preflight docs. |
| `tests/base1_libreboot_preflight_script.rs` | Libreboot preflight script. |
| `tests/base1_libreboot_grub_recovery_docs.rs` | GRUB recovery docs. |
| `tests/base1_grub_recovery_dry_run_script.rs` | GRUB recovery dry-run script output. |
| `tests/base1_libreboot_operator_checklist_docs.rs` | Libreboot operator checklist. |
| `tests/base1_libreboot_checklist_script.rs` | Libreboot checklist script. |
| `tests/base1_libreboot_validation_bundle.rs` | Libreboot validation bundle. |
| `tests/base1_libreboot_validation_report_docs.rs` | Libreboot validation report docs. |
| `tests/base1_libreboot_report_script.rs` | Libreboot report command. |
| `tests/base1_libreboot_release_notes_docs.rs` | Libreboot v1 checkpoint release notes and compatibility links. |
| `tests/base1_libreboot_patch_release_notes_docs.rs` | Libreboot v1.1 checkpoint release notes and compatibility links. |

## Recovery USB hardware tests

| Test file | Coverage |
| --- | --- |
| `tests/base1_recovery_usb_hardware_summary_script.rs` | Recovery USB hardware summary command. |
| `tests/base1_recovery_usb_hardware_summary_docs.rs` | Recovery USB hardware summary docs. |
| `tests/base1_recovery_usb_hardware_report_script.rs` | Recovery USB hardware report command. |
| `tests/base1_recovery_usb_hardware_validation_bundle.rs` | Recovery USB hardware validation bundle. |
| `tests/base1_recovery_usb_hardware_checklist_script.rs` | Recovery USB hardware checklist command. |
| `tests/base1_recovery_usb_hardware_checklist_docs.rs` | Recovery USB hardware checklist docs. |
| `tests/base1_recovery_usb_hardware_release_notes_docs.rs` | Recovery USB hardware checkpoint release notes and compatibility links. |

## Recovery USB target-selection tests

| Test file | Coverage |
| --- | --- |
| `tests/base1_recovery_usb_target_summary_script.rs` | Recovery USB target summary command. |
| `tests/base1_recovery_usb_target_summary_docs.rs` | Recovery USB target summary docs. |
| `tests/base1_recovery_usb_target_validation_bundle.rs` | Recovery USB target validation bundle. |
| `tests/base1_recovery_usb_target_report_script.rs` | Recovery USB target report command. |
| `tests/base1_recovery_usb_target_dry_run_script.rs` | Recovery USB target dry-run command. |
| `tests/base1_recovery_usb_target_command_index_docs.rs` | Recovery USB target command index. |
| `tests/base1_recovery_usb_target_release_notes_docs.rs` | Recovery USB target checkpoint release notes and compatibility links. |

## Recovery USB image-provenance tests

| Test file | Coverage |
| --- | --- |
| `tests/base1_recovery_usb_image_command_index_docs.rs` | Recovery USB image command index. |
| `tests/base1_recovery_usb_image_summary_script.rs` | Recovery USB image summary command. |
| `tests/base1_recovery_usb_image_summary_docs.rs` | Recovery USB image summary docs. |
| `tests/base1_recovery_usb_image_validation_bundle.rs` | Recovery USB image validation bundle. |
| `tests/base1_recovery_usb_image_report_script.rs` | Recovery USB image report command. |
| `tests/base1_recovery_usb_image_provenance_docs.rs` | Recovery USB image provenance docs. |
| `tests/base1_recovery_usb_image_release_notes_docs.rs` | Recovery USB image checkpoint release notes and compatibility links. |

## Recovery USB emergency-shell tests

| Test file | Coverage |
| --- | --- |
| `tests/base1_recovery_usb_emergency_shell_command_index_docs.rs` | Recovery USB emergency shell command index. |
| `tests/base1_recovery_usb_emergency_shell_summary_script.rs` | Recovery USB emergency shell summary command. |
| `tests/base1_recovery_usb_emergency_shell_summary_docs.rs` | Recovery USB emergency shell summary docs. |
| `tests/base1_recovery_usb_emergency_shell_validation_bundle.rs` | Recovery USB emergency shell validation bundle. |
| `tests/base1_recovery_usb_emergency_shell_report_script.rs` | Recovery USB emergency shell report command. |
| `tests/base1_recovery_usb_emergency_shell_docs.rs` | Recovery USB emergency shell docs. |
| `tests/base1_recovery_usb_emergency_shell_release_notes_docs.rs` | Recovery USB emergency shell checkpoint release notes and compatibility links. |

## Recovery USB shared tests

| Test file | Coverage |
| --- | --- |
| `tests/base1_recovery_usb_command_index_docs.rs` | Shared recovery USB command index and guardrails. |

## Real-device read-only tests

Known real-device read-only test coverage should be inventoried here before a full reorganization. Current real-device docs and scripts are protected by `scripts/base1-doc-integrity.sh`, but this section should be expanded with exact `tests/` filenames after a repository-wide test listing.

## Next inventory work

This file is still a seed inventory. The next step is to compare the reporter output from `sh scripts/base1-test-inventory.sh` against this inventory and add any missing Base1 preview or real-device test files.

## Non-claims

This inventory does not make Base1 installer-ready, hardware-validated, or daily-driver ready. It only improves repository organization safety.

## Reporter parity test path index

The following test paths are mirrored from `scripts/base1-test-inventory.sh` so the read-only inventory verifier can detect stale documentation coverage.

- `tests/base1_b2_assembly_dry_run_script.rs`
- `tests/base1_b2_test_suite_check_script.rs`
- `tests/base1_b3_gnulinux_stage_script.rs`
- `tests/base1_b3_kernel_handoff_script.rs`
- `tests/base1_b3_log_bundle_review_script.rs`
- `tests/base1_b3_openbsd_stage_script.rs`
- `tests/base1_b3_uefi_proof_script.rs`
- `tests/base1_b3_vm_validate_script.rs`
- `tests/base1_b4_recovery_validate_script.rs`
- `tests/base1_b6_hardware_boot_evidence_script.rs`
- `tests/base1_b6_x200_marker_checkpoint_release_docs.rs`
- `tests/base1_boot_preview_script.rs`
- `tests/base1_delivery_mode_plan_script.rs`
- `tests/base1_docs_evidence_chain_report_docs.rs`
- `tests/base1_documentation_map_docs.rs`
- `tests/base1_documentation_map_verification_docs.rs`
- `tests/base1_documentation_organization_plan_docs.rs`
- `tests/base1_dry_run_command_index_docs.rs`
- `tests/base1_dual_path_delivery_docs.rs`
- `tests/base1_emulator_doctor_script.rs`
- `tests/base1_emulator_preview_script.rs`
- `tests/base1_installer_dry_run_docs.rs`
- `tests/base1_installer_dry_run_script.rs`
- `tests/base1_installer_recovery_docs.rs`
- `tests/base1_libreboot_index_script.rs`
- `tests/base1_local_boot_artifact_plan_script.rs`
- `tests/base1_preview_checks_docs.rs`
- `tests/base1_preview_gate_script.rs`
- `tests/base1_preview_inputs_script.rs`
- `tests/base1_preview_provenance_script.rs`
- `tests/base1_preview_stack_runbook_docs.rs`
- `tests/base1_preview_stack_script.rs`
- `tests/base1_preview_stack_validation_report_docs.rs`
- `tests/base1_preview_verify_script.rs`
- `tests/base1_profile_check_script.rs`
- `tests/base1_public_surface_link_coverage.rs`
- `tests/base1_qemu_boot_check_script.rs`
- `tests/base1_qemu_phase1_marker_report_docs.rs`
- `tests/base1_qemu_real_phase1_binary_report_docs.rs`
- `tests/base1_qemu_visual_boot_preview_script.rs`
- `tests/base1_readiness_matrix_docs.rs`
- `tests/base1_real_device_readonly_bundle_report_docs.rs`
- `tests/base1_real_device_readonly_checklist_docs.rs`
- `tests/base1_real_device_readonly_doctor_index_docs.rs`
- `tests/base1_real_device_readonly_doctor_script.rs`
- `tests/base1_real_device_readonly_evidence_capture_report_docs.rs`
- `tests/base1_real_device_readonly_index_docs.rs`
- `tests/base1_real_device_readonly_preview_script.rs`
- `tests/base1_real_device_readonly_report_script.rs`
- `tests/base1_real_device_readonly_report_template_docs.rs`
- `tests/base1_real_device_readonly_runbook_docs.rs`
- `tests/base1_real_device_readonly_status_summary_docs.rs`
- `tests/base1_real_device_readonly_validation_bundle.rs`
- `tests/base1_real_device_readonly_validation_docs.rs`
- `tests/base1_real_phase1_initrd_builder_report_docs.rs`
- `tests/base1_real_phase1_initrd_preview_script.rs`
- `tests/base1_recovery_command_docs.rs`
- `tests/base1_recovery_dry_run_script.rs`
- `tests/base1_recovery_usb_checkpoint_docs.rs`
- `tests/base1_recovery_usb_design_docs.rs`
- `tests/base1_recovery_usb_dry_run_script.rs`
- `tests/base1_recovery_usb_index_script.rs`
- `tests/base1_recovery_usb_target_selection_docs.rs`
- `tests/base1_recovery_usb_validation_bundle.rs`
- `tests/base1_recovery_usb_validation_report_docs.rs`
- `tests/base1_rollback_metadata_docs.rs`
- `tests/base1_rollback_metadata_dry_run_script.rs`
- `tests/base1_storage_layout_checker_docs.rs`
- `tests/base1_storage_layout_dry_run_script.rs`
- `tests/base1_supervisor_artifact_flow_docs.rs`
- `tests/base1_supervisor_control_plane_docs.rs`
- `tests/base1_supervisor_control_plane_script.rs`
- `tests/base1_supervisor_orchestration_docs.rs`
- `tests/base1_supervisor_orchestration_plan_script.rs`
- `tests/base1_supervisor_policy_bus_docs.rs`
- `tests/base1_supervisor_policy_bus_script.rs`
- `tests/base1_supervisor_profiles_docs.rs`
- `tests/base1_supervisor_storage_tier_docs.rs`
- `tests/base1_supervisor_storage_tier_plan_script.rs`
- `tests/base1_validation_report_template_docs.rs`
- `tests/base1_validation_reports_index_docs.rs`
- `tests/base1_validation_runbook_docs.rs`
- `tests/base1_x86_64_detect_script.rs`

This section is documentation-only. It does not make Base1 bootable, installer-ready, recovery-complete, hardware-validated, hardened, VM-validated, release-candidate ready, or daily-driver ready.

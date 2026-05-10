# Base1 test inventory

Status: active test inventory seed
Scope: Base1-related Rust tests and quality-gate tests

## Purpose

This inventory records known Base1-related tests before broader repository reorganization.

It is preservation-first. It does not remove, rename, or move tests. It gives maintainers a reviewable list of the test coverage that protects Base1 docs, scripts, release/checkpoint notes, compatibility paths, and quality gates.

## Inventory rule

Update this file when Base1-related tests are added, moved, renamed, or split.

Before broader organization work, compare this inventory against the repository `tests/` directory and update missing entries.

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
| `tests/base1_root_compatibility_map_docs.rs` | Root checkpoint-note compatibility map, mirror paths, and integrity references. |
| `tests/quality_base1_docs_gate.rs` | Quality gate integration for `base1-docs` and `scripts/base1-doc-integrity.sh`. |

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

This file is still a seed inventory. The next step is to add a generated or manually verified full test listing for every `tests/base1_*`, `tests/quality_base1_*`, and related Base1 preview/real-device test.

## Non-claims

This inventory does not make Base1 installer-ready, hardware-validated, or daily-driver ready. It only improves repository organization safety.

# Cargo all-targets failure diagnostics

Generated UTC: 2026-05-13T23:14:08Z
Commit: 134ecd0774dac3c5bf44ce62e7f645db8776825f

## Failure index
```text
214:test asset_index_marks_old_fyr_flame_reference_outdated ... ok
215:test asset_index_marks_old_phase1_splash_and_logo_references_outdated ... ok
681:test base1_emulator_doctor_reports_missing_bundle ... ok
935:test base1_link_check_script_reports_missing_targets ... ok
1000:test post_reorganization_layout_preserves_public_compatibility_paths ... FAILED
1002:failures:
1006:thread 'post_reorganization_layout_preserves_public_compatibility_paths' (187290) panicked at tests/base1_post_reorganization_layout_docs.rs:41:9:
1007:missing compatibility text Root-level checkpoint notes remain compatibility paths.: # Base1 post-reorganization layout
1159:failures:
1162:test result: FAILED. 4 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
1164:error: test failed, to rerun pass `--test base1_post_reorganization_layout_docs`
```

## Final output
```text
     Running tests/base1_libreboot_grub_recovery_docs.rs (target/debug/deps/base1_libreboot_grub_recovery_docs-c5071b6e388d6d1e)

running 3 tests
test libreboot_docs_link_grub_recovery_notes ... ok
test libreboot_grub_recovery_notes_define_read_only_recovery_path ... ok
test readme_links_grub_recovery_notes ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/base1_libreboot_index_script.rs (target/debug/deps/base1_libreboot_index_script-052405e29f689c25)

running 3 tests
test libreboot_index_script_avoids_mutating_tools_and_secret_terms ... ok
test libreboot_index_script_help_is_read_only ... ok
test libreboot_index_script_prints_docs_and_scripts ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/base1_libreboot_milestone_docs.rs (target/debug/deps/base1_libreboot_milestone_docs-374a6972334e5bee)

running 3 tests
test libreboot_milestone_lists_current_docs_and_scripts ... ok
test libreboot_indexes_link_milestone_checkpoint ... ok
test libreboot_milestone_records_read_only_checkpoint ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/base1_libreboot_milestone_script.rs (target/debug/deps/base1_libreboot_milestone_script-30148d460fcd0226)

running 3 tests
test libreboot_milestone_script_avoids_mutating_tools_and_sensitive_terms ... ok
test libreboot_milestone_script_lists_surfaces_and_non_claims ... ok
test libreboot_milestone_script_prints_checkpoint_summary ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/base1_libreboot_operator_checklist_docs.rs (target/debug/deps/base1_libreboot_operator_checklist_docs-6a90dc72909034a5)

running 3 tests
test libreboot_docs_link_operator_checklist ... ok
test libreboot_operator_checklist_defines_grub_first_readiness_path ... ok
test readme_links_operator_checklist ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/base1_libreboot_patch_release_notes_docs.rs (target/debug/deps/base1_libreboot_patch_release_notes_docs-ec41064569384f71)

running 2 tests
test libreboot_patch_release_notes_record_v1_1_checkpoint ... ok
test readme_and_release_index_link_v1_1_patch_release_notes ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/base1_libreboot_preflight_docs.rs (target/debug/deps/base1_libreboot_preflight_docs-5687564a93179b8d)

running 3 tests
test libreboot_profile_links_preflight_notes ... ok
test libreboot_preflight_doc_exists_and_defines_read_only_checks ... ok
test readme_links_libreboot_preflight_notes ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/base1_libreboot_preflight_script.rs (target/debug/deps/base1_libreboot_preflight_script-71d56437d0a8f415)

running 3 tests
test libreboot_preflight_script_avoids_mutating_tools ... ok
test libreboot_preflight_script_help_is_read_only ... ok
test libreboot_preflight_script_reports_read_only_grub_first_notes ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/base1_libreboot_profile_docs.rs (target/debug/deps/base1_libreboot_profile_docs-6a15ba2f54895475)

running 3 tests
test hardware_targets_link_libreboot_profile ... ok
test libreboot_profile_doc_exists_and_defines_boundary ... ok
test readme_links_libreboot_profile ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/base1_libreboot_quickstart_docs.rs (target/debug/deps/base1_libreboot_quickstart_docs-3abe87d2c6692617)

running 3 tests
test libreboot_index_links_quickstart ... ok
test libreboot_quickstart_defines_safe_grub_first_path ... ok
test readme_links_libreboot_quickstart ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/base1_libreboot_release_notes_docs.rs (target/debug/deps/base1_libreboot_release_notes_docs-83e4a95055e8a50b)

running 3 tests
test libreboot_release_notes_list_surfaces_and_non_claims ... ok
test libreboot_indexes_link_release_notes ... ok
test libreboot_release_notes_record_checkpoint_status ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/base1_libreboot_report_script.rs (target/debug/deps/base1_libreboot_report_script-57c8253a784ce8ab)

running 3 tests
test libreboot_report_script_avoids_mutating_tools_and_secret_terms ... ok
test libreboot_report_script_lists_validation_commands ... ok
test libreboot_report_script_prints_validation_template ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/base1_libreboot_validation_bundle.rs (target/debug/deps/base1_libreboot_validation_bundle-02b1d63bf2997b5f)

running 3 tests
test libreboot_validation_bundle_avoids_mutating_tools_and_secret_terms ... ok
test libreboot_validation_bundle_help_is_read_only ... ok
test libreboot_validation_bundle_runs_all_read_only_previews ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s

     Running tests/base1_libreboot_validation_report_docs.rs (target/debug/deps/base1_libreboot_validation_report_docs-465fe9fdcc4a3ecf)

running 3 tests
test libreboot_command_index_links_validation_report ... ok
test libreboot_validation_report_template_defines_safe_target_summary ... ok
test readme_links_validation_report ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/base1_link_check_script.rs (target/debug/deps/base1_link_check_script-c49d819ee6ffa5eb)

running 5 tests
test base1_link_check_script_is_read_only_and_local_only ... ok
test base1_link_check_script_checks_required_surfaces ... ok
test base1_link_check_script_reports_missing_targets ... ok
test base1_link_check_script_skips_external_and_anchor_links ... ok
test quality_gate_runs_base1_link_checker ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/base1_link_check_strategy_docs.rs (target/debug/deps/base1_link_check_strategy_docs-3ff4b1877c84754b)

running 5 tests
test link_check_strategy_defines_future_checker_behavior ... ok
test link_check_strategy_defines_required_surfaces ... ok
test link_check_strategy_is_linked_from_indexes_and_integrity_gate ... ok
test link_check_strategy_preserves_compatibility_paths ... ok
test link_check_strategy_preserves_non_claims ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/base1_local_boot_artifact_plan_script.rs (target/debug/deps/base1_local_boot_artifact_plan_script-4583c280aea238ff)

running 4 tests
test base1_local_boot_artifact_plan_help_documents_scope ... ok
test base1_local_boot_artifact_plan_prepare_writes_report ... ok
test base1_local_boot_artifact_plan_rejects_unknown_profile_and_non_build_paths ... ok
test base1_local_boot_artifact_plan_script_exists_and_has_valid_shell_syntax ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests/base1_manual_verification_rule_docs.rs (target/debug/deps/base1_manual_verification_rule_docs-fc4a8ae4646b7494)

running 4 tests
test manual_documents_base1_verification_rule ... ok
test manual_explains_reorganization_verifier_scope ... ok
test manual_preserves_base1_non_claims ... ok
test manual_keeps_organization_guardrails_visible ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/base1_migration_table_docs.rs (target/debug/deps/base1_migration_table_docs-f6e09574b10936b2)

running 5 tests
test migration_table_blocks_broad_moves_until_safety_work_exists ... ok
test migration_table_lists_key_current_and_target_paths ... ok
test migration_table_lists_required_groups ... ok
test migration_table_preserves_compatibility_decisions ... ok
test migration_table_preserves_non_claims ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/base1_network_lockdown_docs.rs (target/debug/deps/base1_network_lockdown_docs-0ec84eeec4879630)

running 4 tests
test dry_run_command_index_includes_network_lockdown ... ok
test base1_readme_links_network_lockdown_preview ... ok
test network_lockdown_doc_defines_read_only_contract ... ok
test network_lockdown_script_requires_dry_run_and_reports_no_writes ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/base1_post_reorganization_layout_docs.rs (target/debug/deps/base1_post_reorganization_layout_docs-3138d9422083651c)

running 5 tests
test post_reorganization_layout_defines_stable_public_paths ... ok
test post_reorganization_layout_is_linked_and_integrity_checked ... ok
test post_reorganization_layout_preserves_non_claims ... ok
test post_reorganization_layout_lists_major_layout_sections ... ok
test post_reorganization_layout_preserves_public_compatibility_paths ... FAILED

failures:

---- post_reorganization_layout_preserves_public_compatibility_paths stdout ----

thread 'post_reorganization_layout_preserves_public_compatibility_paths' (187290) panicked at tests/base1_post_reorganization_layout_docs.rs:41:9:
missing compatibility text Root-level checkpoint notes remain compatibility paths.: # Base1 post-reorganization layout

Status: proposed stable layout
Scope: Base1 documentation, scripts, tests, release/checkpoint notes, and compatibility paths

## Purpose

This document names the intended stable public paths after Base1 organization work matures.

It is a layout target, not a move instruction. Current paths remain recoverable unless explicit future approval says otherwise.

## Stable public paths

These paths should remain stable for users, contributors, and existing links:

| Path | Role |
| --- | --- |
| `base1/README.md` | Base1 public overview. |
| `docs/base1/README.md` | Base1 manual and source-of-truth index. |
| `docs/base1/DOCUMENTATION_MAP.md` | Base1 navigation map. |
| `docs/base1/INVENTORY.md` | Base1 inventory. |
| `docs/base1/TEST_INVENTORY.md` | Base1 test inventory. |
| `docs/base1/MIGRATION_TABLE.md` | Migration planning table. |
| `docs/base1/SCRIPT_COMPATIBILITY_PLAN.md` | Script compatibility plan. |
| `docs/base1/LINK_CHECK_STRATEGY.md` | Link-check strategy. |
| `docs/base1/REORGANIZATION_READINESS.md` | Reorganization readiness checklist. |
| `docs/base1/RELEASE_ARCHIVE_MAP.md` | Release archive map. |

## Core docs layout

Core Base1 architecture and public-facing design documents remain under `base1/` unless a future compatibility plan says otherwise.

```text
base1/README.md
base1/SECURITY_MODEL.md
base1/HARDWARE_TARGETS.md
base1/PHASE1_COMPATIBILITY.md
base1/ROADMAP.md
base1/NETWORK_LOCKDOWN_DRY_RUN.md
base1/config/base1-secure-profile.toml
base1/systemd/phase1-base1.service
```

## Organization docs layout

Organization, inventory, readiness, and validation-control docs live under `docs/base1/`.

```text
docs/base1/README.md
docs/base1/DOCUMENTATION_MAP.md
docs/base1/DOCUMENTATION_ORGANIZATION_PLAN.md
docs/base1/INVENTORY.md
docs/base1/TEST_INVENTORY.md
docs/base1/MIGRATION_TABLE.md
docs/base1/SCRIPT_COMPATIBILITY_PLAN.md
docs/base1/LINK_CHECK_STRATEGY.md
docs/base1/REORGANIZATION_READINESS.md
docs/base1/RELEASE_ARCHIVE_MAP.md
```

## Release/checkpoint notes layout

Organized release/checkpoint browsing lives under `docs/base1/releases/`.

Former root-level checkpoint names remain documented in `docs/base1/RELEASE_ARCHIVE_MAP.md`, but the current repository does not require root-level release files to exist.

```text
docs/base1/releases/README.md
docs/base1/releases/RELEASE_BASE1_LIBREBOOT_READONLY_V1.md
docs/base1/releases/RELEASE_BASE1_LIBREBOOT_READONLY_V1_1.md
docs/base1/releases/RELEASE_BASE1_RECOVERY_USB_HARDWARE_READONLY_V1.md
docs/base1/releases/RELEASE_BASE1_RECOVERY_USB_TARGET_READONLY_V1.md
docs/base1/releases/RELEASE_BASE1_RECOVERY_USB_IMAGE_READONLY_V1.md
docs/base1/releases/RELEASE_BASE1_RECOVERY_USB_EMERGENCY_SHELL_READONLY_V1.md
```

## Real-device read-only layout

Real-device read-only evidence and runbooks remain under `docs/base1/real-device/`.

```text
docs/base1/real-device/README.md
docs/base1/real-device/READONLY_VALIDATION_PLAN.md
docs/base1/real-device/READONLY_REPORT_TEMPLATE.md
docs/base1/real-device/READONLY_VALIDATION_BUNDLE_REPORT.md
docs/base1/real-device/RUNBOOK.md
docs/base1/real-device/CHECKLIST.md
docs/base1/real-device/STATUS_SUMMARY.md
docs/base1/real-device/reports/
```

## OS-track design slices

Base1 OS-track design slices remain in `docs/os/` while Phase1 OS-track docs live there.

```text
docs/os/BASE1_IMAGE_BUILDER.md
docs/os/BASE1_INSTALLER_DRY_RUN.md
docs/os/BASE1_RECOVERY_COMMAND.md
docs/os/BASE1_STORAGE_LAYOUT_CHECKER.md
docs/os/BASE1_ROLLBACK_METADATA.md
docs/os/BASE1_DRY_RUN_COMMANDS.md
```

## Script layout

Current script paths remain the stable operator interface.

```text
scripts/base1-*.sh
```

A future internal layout such as `scripts/base1/<group>/` may be added only with wrappers that preserve existing `scripts/base1-*.sh` command paths.

## Test layout

Base1 integration tests remain discoverable by Cargo under `tests/`.

```text
tests/base1_*.rs
tests/quality_base1_*.rs
tests/*base1*.rs
```

Do not move or rename tests unless `docs/base1/TEST_INVENTORY.md` and related tests are updated in the same change.

## Compatibility policy

Compatibility paths remain valid even after organized archives are added. In particular:

- Organized release/checkpoint archives stay available.
- Former root-level release/checkpoint filenames remain documented in the archive map.
- Existing script paths stay available.
- Existing Cargo integration test discovery stays available.
- Existing public documentation links stay recoverable.

## Validation before any movement

Run:

```bash
sh scripts/quality-check.sh base1-docs
cargo test --all-targets
```

## Non-claims

This layout document does not make Base1 installer-ready, hardware-validated, or daily-driver ready. It only defines proposed stable repository paths for safer organization.

note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    post_reorganization_layout_preserves_public_compatibility_paths

test result: FAILED. 4 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass `--test base1_post_reorganization_layout_docs`
```

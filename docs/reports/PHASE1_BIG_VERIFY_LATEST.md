# Phase1 big verification latest report

Generated UTC: 2026-05-13T22:47:03Z
Source branch: edge/stable
Source commit: cae1fb623f4c4fc1b7caeac72988ed62cf1e4b5f
Host: X200
Host kernel: Linux X200 6.8.0-110-generic #110trisquel35 SMP PREEMPT_DYNAMIC Wed Apr 15 21:32:36 UTC 2026 x86_64 x86_64 x86_64 GNU/Linux

## Result

| Field | Value |
| --- | --- |
| Result | failed |
| Steps | 15 |
| Failed steps | 2 |
| Verification start UTC | 2026-05-13T22:41:20Z |
| Verification end UTC | 2026-05-13T22:41:44Z |
| Source summary | build/phase1-big-verify/summary.env |
| Source report | build/phase1-big-verify/report.md |
| Source log | build/phase1-big-verify/phase1-big-verify.log |

## Failed step summary

```text
1824:FAILED: quality full
3480:FAILED: base1 reorg gate
```

## Verification report snapshot

# Phase1 big verification report

Start UTC: 2026-05-13T22:41:20Z
End UTC: 2026-05-13T22:41:44Z
Head: cae1fb623f4c4fc1b7caeac72988ed62cf1e4b5f
Result: failed
Steps run: 15
Failed steps: 2

Log: build/phase1-big-verify/phase1-big-verify.log
Summary: build/phase1-big-verify/summary.env

## Git status after run

```text
 M README.md
 M docs/os/B3_X200_EMULATOR_EVIDENCE_REPORT.md
 M tests/readme_navigation_reorganization_links.rs
?? EDGE.md
?? EDGE_STABLE_CHECKPOINT.md
?? FEATURE_STATUS.md
?? REPO_DOCTRINE.md
?? WIKI_ROADMAP.md
```

## Non-claims

This report records repository and local emulator-evidence validation only. It does not claim installer readiness, physical hardware validation, recovery validation, hardening proof, release-candidate readiness, or daily-driver readiness.

## Log tail

Last 800 lines from the local verifier log:

```text

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s

     Running tests/base1_delivery_mode_plan_script.rs (target/debug/deps/base1_delivery_mode_plan_script-089f2a8b532027b5)

running 8 tests
test base1_delivery_mode_plan_help_documents_modes_profiles_contract_and_non_claims ... ok
test base1_delivery_mode_plan_dry_run_defaults_to_profile_default_x200_direct_first ... ok
test base1_delivery_mode_plan_preserves_boundaries_and_best_of_both_worlds ... ok
test base1_delivery_mode_plan_prepare_writes_report_from_profile_contract ... ok
test base1_delivery_mode_plan_rejects_mode_disallowed_by_profile ... ok
test base1_delivery_mode_plan_script_exists_and_has_valid_shell_syntax ... ok
test base1_delivery_mode_plan_supports_concurrent_when_profile_allows_it ... ok
test base1_delivery_mode_plan_rejects_unknown_args_profiles_and_non_build_out ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s

     Running tests/base1_docs_evidence_chain_report_docs.rs (target/debug/deps/base1_docs_evidence_chain_report_docs-301ed60697f029ad)

running 5 tests
test base1_docs_evidence_chain_report_exists ... ok
test base1_docs_evidence_chain_report_links_expected_surfaces ... ok
test base1_docs_evidence_chain_report_preserves_documentation_only_scope ... ok
test base1_docs_evidence_chain_report_preserves_non_claims ... ok
test validation_index_links_docs_evidence_chain_report ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/base1_documentation_map_docs.rs (target/debug/deps/base1_documentation_map_docs-2a2c1e4a5a69ae3d)

running 6 tests
test base1_documentation_map_exists ... ok
test base1_documentation_map_links_core_entry_points ... ok
test base1_documentation_map_lists_readonly_scripts ... ok
test base1_documentation_map_links_real_device_track ... ok
test base1_documentation_map_preserves_non_claims ... ok
test base1_index_links_documentation_map ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/base1_documentation_map_verification_docs.rs (target/debug/deps/base1_documentation_map_verification_docs-4336159ad5ed3eca)

running 4 tests
test documentation_map_links_release_pre_move_checks ... ok
test documentation_map_links_organization_guardrails ... ok
test documentation_map_lists_verification_commands ... ok
test documentation_map_preserves_non_claims ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/base1_documentation_organization_plan_docs.rs (target/debug/deps/base1_documentation_organization_plan_docs-a816b5566a680653)

running 5 tests
test base1_documentation_organization_plan_exists ... ok
test base1_documentation_organization_plan_lists_groups ... ok
test base1_documentation_organization_plan_preserves_move_rules ... ok
test base1_documentation_organization_plan_preserves_non_claims ... ok
test base1_index_links_documentation_organization_plan ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/base1_dry_run_command_index_docs.rs (target/debug/deps/base1_dry_run_command_index_docs-354a29b371abc820)

running 3 tests
test dry_run_command_index_documents_all_current_scripts ... ok
test os_roadmap_links_dry_run_command_index ... ok
test readme_links_dry_run_command_index ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/base1_dual_path_delivery_docs.rs (target/debug/deps/base1_dual_path_delivery_docs-3ae5ba078689264b)

running 6 tests
test base1_dual_path_delivery_defines_purpose_and_paths ... ok
test base1_dual_path_delivery_defines_shared_contract_and_profiles ... ok
test base1_dual_path_delivery_preserves_direct_first_kernel_path ... ok
test base1_dual_path_delivery_defines_x200_and_delivery_modes ... ok
test base1_dual_path_delivery_preserves_non_claims ... ok
test base1_dual_path_delivery_preserves_supervisor_orchestration_path ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/base1_emulator_doctor_script.rs (target/debug/deps/base1_emulator_doctor_script-7624b9565869e44e)

running 4 tests
test base1_emulator_doctor_avoids_mutating_tools ... ok
test base1_emulator_doctor_exists_and_documents_boundary ... ok
test base1_emulator_doctor_reports_missing_bundle ... ok
test base1_emulator_doctor_checks_generated_bundle_without_launching ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s

     Running tests/base1_emulator_preview_script.rs (target/debug/deps/base1_emulator_preview_script-a8a01a19466cc02f)

running 4 tests
test base1_emulator_preview_refuses_output_outside_build ... ok
test base1_emulator_preview_script_avoids_real_device_tools ... ok
test base1_emulator_preview_script_exists_and_documents_boundary ... ok
test base1_emulator_preview_generates_bundle_under_build ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s

     Running tests/base1_foundation.rs (target/debug/deps/base1_foundation-ab7b93e27323eff9)

running 8 tests
test base1_foundation_docs_exist ... ok
test base1_compatibility_contract_defaults_to_safe_mode ... ok
test base1_hardware_targets_include_raspberry_pi_and_x200 ... ok
test base1_launcher_refuses_root_by_default ... ok
test base1_profile_is_secure_by_default ... ok
test base1_security_model_preserves_phase1_host_boundary ... ok
test base1_systemd_template_has_hardening_directives ... ok
test base1_preflight_is_non_destructive ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/base1_grub_recovery_dry_run_script.rs (target/debug/deps/base1_grub_recovery_dry_run_script-6201fba838621819)

running 4 tests
test grub_recovery_dry_run_help_is_read_only ... ok
test grub_recovery_dry_run_reports_read_only_recovery_plan ... ok
test grub_recovery_dry_run_script_avoids_mutating_tools ... ok
test grub_recovery_dry_run_requires_dry_run_flag ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/base1_image_builder_docs.rs (target/debug/deps/base1_image_builder_docs-64e13e237aa67f82)

running 3 tests
test image_builder_doc_defines_stage1_boot_surface ... ok
test os_roadmap_links_image_builder_design ... ok
test readme_links_base1_image_builder_design ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/base1_installer_dry_run_docs.rs (target/debug/deps/base1_installer_dry_run_docs-df5ca140154dfff3)

running 3 tests
test installer_dry_run_doc_exists_and_defines_safe_command ... ok
test os_roadmap_links_installer_dry_run_design ... ok
test readme_links_installer_dry_run_design ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/base1_installer_dry_run_script.rs (target/debug/deps/base1_installer_dry_run_script-e716205299f1768e)

running 4 tests
test installer_dry_run_refuses_without_dry_run_flag ... ok
test installer_dry_run_requires_target ... ok
test installer_dry_run_script_avoids_destructive_tools ... ok
test installer_dry_run_reports_preview_without_writes ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/base1_installer_recovery_docs.rs (target/debug/deps/base1_installer_recovery_docs-cc2e56dd8afbb80f)

running 3 tests
test installer_recovery_doc_exists_and_defines_contracts ... ok
test os_roadmap_mentions_installer_recovery_design ... ok
test readme_links_installer_recovery_design ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/base1_inventory_docs.rs (target/debug/deps/base1_inventory_docs-397ab4c5863407dc)

running 5 tests
test base1_inventory_docs_keep_non_claims ... ok
test base1_inventory_lists_required_groups ... ok
test base1_inventory_preserves_key_paths ... ok
test base1_test_inventory_lists_required_test_groups ... ok
test base1_test_inventory_preserves_key_test_paths ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/base1_libreboot_checklist_script.rs (target/debug/deps/base1_libreboot_checklist_script-1fdfdd5ac75d89cb)

running 3 tests
test libreboot_checklist_script_avoids_mutating_tools_and_secret_terms ... ok
test libreboot_checklist_script_lists_required_dry_runs ... ok
test libreboot_checklist_script_reports_operator_summary ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/base1_libreboot_command_index_docs.rs (target/debug/deps/base1_libreboot_command_index_docs-de8574444eaae3b3)

running 4 tests
test libreboot_command_index_lists_docs_and_scripts ... ok
test libreboot_command_index_preserves_guardrails ... ok
test libreboot_docs_link_command_index ... ok
test readme_links_libreboot_command_index ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/base1_libreboot_docs_script.rs (target/debug/deps/base1_libreboot_docs_script-7ee988654f8cf8f7)

running 3 tests
test libreboot_docs_script_avoids_mutating_tools_and_secret_terms ... ok
test libreboot_docs_script_lists_first_commands ... ok
test libreboot_docs_script_prints_docs_path ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/base1_libreboot_docs_summary_docs.rs (target/debug/deps/base1_libreboot_docs_summary_docs-1c01cd106922b5e8)

running 3 tests
test libreboot_docs_summary_lists_core_docs_and_commands ... ok
test libreboot_docs_summary_preserves_read_only_guardrails ... ok
test libreboot_index_and_readme_link_docs_summary ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

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
test libreboot_milestone_records_read_only_checkpoint ... ok
test libreboot_indexes_link_milestone_checkpoint ... ok

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
test libreboot_preflight_doc_exists_and_defines_read_only_checks ... ok
test libreboot_profile_links_preflight_notes ... ok
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
test link_check_strategy_defines_required_surfaces ... ok
test link_check_strategy_defines_future_checker_behavior ... ok
test link_check_strategy_is_linked_from_indexes_and_integrity_gate ... ok
test link_check_strategy_preserves_compatibility_paths ... ok
test link_check_strategy_preserves_non_claims ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/base1_local_boot_artifact_plan_script.rs (target/debug/deps/base1_local_boot_artifact_plan_script-4583c280aea238ff)

running 4 tests
test base1_local_boot_artifact_plan_help_documents_scope ... ok
test base1_local_boot_artifact_plan_rejects_unknown_profile_and_non_build_paths ... ok
test base1_local_boot_artifact_plan_prepare_writes_report ... ok
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
test migration_table_preserves_compatibility_decisions ... ok
test migration_table_lists_required_groups ... ok
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
test post_reorganization_layout_preserves_public_compatibility_paths ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/base1_pre_move_checklist_docs.rs (target/debug/deps/base1_pre_move_checklist_docs-bf2c934f4f1fe7ab)

running 6 tests
test pre_move_checklist_defines_required_before_move_items ... ok
test pre_move_checklist_blocks_unsafe_moves ... ok
test pre_move_checklist_preserves_first_safe_candidate_limits ... ok
test pre_move_checklist_is_linked_and_integrity_checked ... ok
test pre_move_checklist_requires_tests_before_moves ... ok
test pre_move_checklist_preserves_non_claims ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/base1_preview_checks_docs.rs (target/debug/deps/base1_preview_checks_docs-17e17a087635daf4)

running 5 tests
test base1_preview_checks_doc_exists ... ok
test base1_index_links_preview_checks_doc ... ok
test base1_preview_checks_lists_current_test_set ... ok
test base1_preview_checks_preserves_non_claims ... ok
test base1_preview_checks_records_safe_manual_smoke ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/base1_preview_gate_script.rs (target/debug/deps/base1_preview_gate_script-3517531736b99702)

running 6 tests
test base1_preview_gate_avoids_real_device_tools ... ok
test base1_preview_gate_dry_run_passes_when_boot_inputs_exist ... ok
test base1_preview_gate_dry_run_refuses_missing_boot_inputs ... ok
test base1_preview_gate_exists_and_documents_boundary ... ok
test base1_preview_gate_reports_missing_bundle ... ok
test base1_preview_gate_execute_requires_confirmation ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.09s

     Running tests/base1_preview_inputs_script.rs (target/debug/deps/base1_preview_inputs_script-ddd98a6c618c2034)

running 5 tests
test base1_preview_inputs_avoids_mutating_tools ... ok
test base1_preview_inputs_passes_with_placeholder_files ... ok
test base1_preview_inputs_script_exists_and_documents_boundary ... ok
test base1_preview_inputs_warns_when_bundle_outside_build ... ok
test base1_preview_inputs_reports_missing_required_paths ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/base1_preview_provenance_script.rs (target/debug/deps/base1_preview_provenance_script-79e0955e334e5fe4)

running 5 tests
test base1_preview_provenance_exists_and_documents_boundary ... ok
test base1_preview_provenance_avoids_real_device_tools ... ok
test base1_preview_provenance_refuses_bundle_outside_build ... ok
test base1_preview_provenance_reports_missing_bundle ... ok
test base1_preview_provenance_records_hashes_for_generated_bundle ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.42s

     Running tests/base1_preview_stack_runbook_docs.rs (target/debug/deps/base1_preview_stack_runbook_docs-5ff2c87ae2482a5b)

running 5 tests
test base1_preview_stack_runbook_exists ... ok
test base1_index_links_preview_stack_runbook ... ok
test base1_preview_stack_runbook_lists_safe_stack_order ... ok
test base1_preview_stack_runbook_preserves_non_claims ... ok
test base1_preview_stack_runbook_preserves_promotion_rule ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/base1_preview_stack_script.rs (target/debug/deps/base1_preview_stack_script-5a4ffcfde982664c)

running 4 tests
test base1_preview_stack_exists_and_documents_boundary ... ok
test base1_preview_stack_avoids_real_device_tools ... ok
test base1_preview_stack_requires_kernel_and_initrd ... ok
test base1_preview_stack_runs_full_safe_flow_with_placeholder_inputs ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.78s

     Running tests/base1_preview_stack_validation_report_docs.rs (target/debug/deps/base1_preview_stack_validation_report_docs-9d8c053d23b9c39f)

running 5 tests
test base1_preview_stack_validation_report_exists ... ok
test preview_stack_report_preserves_non_claims ... ok
test preview_stack_report_records_scope_and_evidence ... ok
test validation_index_links_preview_stack_report ... ok
test preview_stack_report_preserves_promotion_boundary ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/base1_preview_verify_script.rs (target/debug/deps/base1_preview_verify_script-09644db98194f29b)

running 6 tests
test base1_preview_verify_exists_and_documents_boundary ... ok
test base1_preview_verify_avoids_real_device_tools ... ok
test base1_preview_verify_passes_generated_stack_bundle ... ok
test base1_preview_verify_fails_after_bundle_drift ... ok
test base1_preview_verify_refuses_bundle_outside_build ... ok
test base1_preview_verify_reports_missing_bundle ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.27s

     Running tests/base1_profile_check_script.rs (target/debug/deps/base1_profile_check_script-c4b34bcfec4c0be9)

running 6 tests
test base1_profile_check_help_documents_contract_and_non_claims ... ok
test base1_profile_check_rejects_unknown_profile_and_non_build_out ... ok
test base1_profile_check_script_exists_and_has_valid_shell_syntax ... ok
test base1_profile_check_single_x200_profile_passes ... ok
test base1_profile_files_preserve_expected_claim_boundaries_and_resource_profiles ... ok
test base1_profile_check_all_profiles_pass_and_write_report ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.33s

     Running tests/base1_public_surface_link_coverage.rs (target/debug/deps/base1_public_surface_link_coverage-afb5a6e543bc5e4c)

running 3 tests
test release_indexes_link_all_base1_release_notes ... ok
test base1_readme_links_all_base1_public_surfaces ... ok
test root_readme_links_all_base1_public_surfaces ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/base1_qemu_boot_check_script.rs (target/debug/deps/base1_qemu_boot_check_script-1e6acc095f0b4d67)

running 5 tests
test base1_qemu_boot_check_execute_requires_confirmation ... ok
test base1_qemu_boot_check_dry_run_does_not_launch ... ok
test base1_qemu_boot_check_exists_and_documents_boundary ... ok
test base1_qemu_boot_check_refuses_bundle_outside_build ... ok
test base1_qemu_boot_check_reports_missing_bundle ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests/base1_qemu_phase1_marker_report_docs.rs (target/debug/deps/base1_qemu_phase1_marker_report_docs-6ea51a990bd68f12)

running 5 tests
test qemu_phase1_marker_report_exists ... ok
test qemu_phase1_marker_report_preserves_non_claims ... ok
test qemu_phase1_marker_report_preserves_promotion_rule ... ok
test qemu_phase1_marker_report_records_evidence ... ok
test validation_index_links_qemu_phase1_marker_report ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/base1_qemu_real_phase1_binary_report_docs.rs (target/debug/deps/base1_qemu_real_phase1_binary_report_docs-c827aa8c622082fa)

running 5 tests
test qemu_real_phase1_binary_report_exists ... ok
test qemu_real_phase1_binary_report_preserves_non_claims ... ok
test qemu_real_phase1_binary_report_preserves_promotion_rule ... ok
test qemu_real_phase1_binary_report_records_evidence ... ok
test validation_index_links_qemu_real_phase1_binary_report ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/base1_qemu_visual_boot_preview_script.rs (target/debug/deps/base1_qemu_visual_boot_preview_script-1acd6757148ed267)

running 8 tests
test qemu_visual_boot_preview_script_help_documents_usage_and_boundaries ... FAILED
test qemu_visual_boot_preview_script_exists_and_has_valid_shell_syntax ... ok
test qemu_visual_boot_preview_script_preserves_non_install_boundary ... ok
test qemu_visual_boot_preview_script_rejects_unknown_argument ... FAILED
test qemu_visual_boot_preview_script_uses_expected_local_artifacts ... ok
test qemu_visual_boot_preview_script_uses_phase1_wordmark_and_fits_it ... ok
test qemu_visual_boot_preview_script_requires_action ... FAILED
test qemu_visual_boot_preview_script_uses_safe_qemu_launch_shape ... ok

failures:

---- qemu_visual_boot_preview_script_help_documents_usage_and_boundaries stdout ----

thread 'qemu_visual_boot_preview_script_help_documents_usage_and_boundaries' (101967) panicked at tests/base1_qemu_visual_boot_preview_script.rs:35:5:
--help should succeed
stdout:

stderr:
scripts/base1-qemu-visual-boot-preview.sh: 5: set: Illegal option -o pipefail

note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

---- qemu_visual_boot_preview_script_rejects_unknown_argument stdout ----

thread 'qemu_visual_boot_preview_script_rejects_unknown_argument' (101971) panicked at tests/base1_qemu_visual_boot_preview_script.rs:95:5:
stderr was: scripts/base1-qemu-visual-boot-preview.sh: 5: set: Illegal option -o pipefail


---- qemu_visual_boot_preview_script_requires_action stdout ----

thread 'qemu_visual_boot_preview_script_requires_action' (101973) panicked at tests/base1_qemu_visual_boot_preview_script.rs:75:5:
stderr was: scripts/base1-qemu-visual-boot-preview.sh: 5: set: Illegal option -o pipefail



failures:
    qemu_visual_boot_preview_script_help_documents_usage_and_boundaries
    qemu_visual_boot_preview_script_rejects_unknown_argument
    qemu_visual_boot_preview_script_requires_action

test result: FAILED. 5 passed; 3 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass `--test base1_qemu_visual_boot_preview_script`
exit_code: 101
FAILED: base1 reorg gate

### base1 link check
$ sh scripts/base1-link-check.sh
base1-link-check: mode: read-only
base1-link-check: external-links: skipped
base1-link-check: anchors: file-only check
base1-link-check: files-checked: 79
base1-link-check: missing-targets: 0
base1-link-check: link check complete; no host changes were made
exit_code: 0
PASS: base1 link check

### B2 focused test-suite
$ sh scripts/base1-b2-test-suite-check.sh --check
BASE1 B2 FOCUSED TEST SUITE
mode  : check
out   : build/base1-b2-test-suite
report: build/base1-b2-test-suite/b2-test-suite-summary.env


>>> cargo test -p phase1 --test b2_dry_run_assembly_plan_docs
exit_code: 0

>>> cargo test -p phase1 --test base1_b2_assembly_dry_run_script
exit_code: 0

>>> cargo test -p phase1 --test b2_dry_run_assembly_limitations_docs
exit_code: 0

>>> cargo test -p phase1 --test b2_dry_run_assembly_validation_docs
exit_code: 0

>>> cargo test -p phase1 --test b2_dry_run_assembly_output_review_docs
exit_code: 0

>>> cargo test -p phase1 --test boot_readiness_status_docs
exit_code: 0

>>> cargo test -p phase1 --test boot_readiness_race_plan_docs
exit_code: 0

>>> cargo test -p phase1 --test x86_64_boot_support_roadmap_docs
exit_code: 0

>>> cargo test -p phase1 --test readme_navigation_reorganization_links
exit_code: 0

result: pass
summary: build/base1-b2-test-suite/b2-test-suite-summary.env
log: build/base1-b2-test-suite/b2-test-suite.log
non_claims: focused B2 test evidence only; no bootability claim; no installer claim; no VM/hardware validation claim
exit_code: 0
PASS: B2 focused test-suite

### B3 VM validation scaffold
$ sh scripts/base1-b3-vm-validate.sh --dry-run --write-report
BASE1 B3 VM VALIDATION SCAFFOLD
mode       : dry-run
profile    : x86_64-vm-validation
profile_file: profiles/base1/x86_64-vm-validation.env
profile_cls : vm-validation
report     : build/base1-b3-vm-validation/b3-validation-scaffold.env
evidence   : evidence-present
claim      : not_claimed

BASE1_B3_VM_VALIDATION_MODE=scaffold-only
BASE1_B3_VM_VALIDATION_PROFILE=x86_64-vm-validation
BASE1_B3_VM_VALIDATION_PROFILE_FILE=profiles/base1/x86_64-vm-validation.env
BASE1_B3_VM_VALIDATION_PROFILE_CLASS=vm-validation
BASE1_B3_VM_VALIDATION_PROFILE_TARGET_RAM_MB=4096
BASE1_B3_VM_VALIDATION_PROFILE_DEFAULT_MODE=supervisor-lite
BASE1_B3_VM_VALIDATION_PROFILE_ALLOWED_MODES=direct-first,supervisor-lite,supervisor-concurrent
BASE1_B3_VM_VALIDATION_PROFILE_MAX_CONCURRENCY=3
BASE1_B3_VM_VALIDATION_PROFILE_VM_MEMORY_MB=1024
BASE1_B3_VM_VALIDATION_PROFILE_OPENBSD_MEMORY_MB=1024
BASE1_B3_VM_VALIDATION_PROFILE_STORAGE_TIER_POLICY=build-directory-scratch
BASE1_B3_EXPECT_MARKER=phase1 6.0.0 ready
BASE1_B3_EVIDENCE_STATE=evidence-present
BASE1_B3_EVIDENCE_SUMMARY_COUNT=4
BASE1_B3_UEFI_SUMMARY=build/base1-b3-uefi-proof/reports/b3-summary.env
BASE1_B3_UEFI_LOG=build/base1-b3-uefi-proof/reports/b3-serial.log
BASE1_B3_UEFI_SUMMARY_PRESENT=yes
BASE1_B3_HANDOFF_SUMMARY=build/base1-b3-kernel-handoff/reports/qemu-boot-summary.env
BASE1_B3_HANDOFF_LOG=build/base1-b3-kernel-handoff/reports/qemu-boot.log
BASE1_B3_HANDOFF_SUMMARY_PRESENT=yes
BASE1_B3_GNULINUX_SUMMARY=build/base1-b3-gnulinux-stage/reports/qemu-boot-summary.env
BASE1_B3_GNULINUX_LOG=build/base1-b3-gnulinux-stage/reports/qemu-boot.log
BASE1_B3_GNULINUX_SUMMARY_PRESENT=yes
BASE1_B3_OPENBSD_SUMMARY=build/base1-b3-openbsd-stage/reports/openbsd-qemu-summary.env
BASE1_B3_OPENBSD_LOG=build/base1-b3-openbsd-stage/reports/openbsd-qemu-boot.log
BASE1_B3_OPENBSD_SUMMARY_PRESENT=yes
BASE1_B3_VALIDATION_CLAIM=not_claimed
BASE1_B3_NON_CLAIM_BOOTABLE_PHYSICAL=1
BASE1_B3_NON_CLAIM_INSTALLER=1
BASE1_B3_NON_CLAIM_RECOVERY=1
BASE1_B3_NON_CLAIM_HARDENED=1
BASE1_B3_NON_CLAIM_HYPERVISOR=1
BASE1_B3_NON_CLAIM_HARDWARE=1
BASE1_B3_NON_CLAIM_RELEASE_CANDIDATE=1
BASE1_B3_NON_CLAIM_DAILY_DRIVER=1

written_report: build/base1-b3-vm-validation/b3-validation-scaffold.env

next_required_evidence:
  - B2 test suite pass record
  - B3 UEFI proof summary/log
  - B3 kernel/initrd handoff summary/log
  - B3 GNU/Linux stage summary/log when used
  - B3 OpenBSD stage summary/log when used
  - validation report promoted from scaffold to reviewed evidence
non_claims: no installer; no recovery validation; no hardening; no hypervisor claim; no hardware validation; no daily-driver claim
exit_code: 0
PASS: B3 VM validation scaffold

### B3 log bundle review
$ sh scripts/base1-b3-log-bundle-review.sh --review
BASE1 B3 LOG BUNDLE REVIEW
mode  : review
out   : build/base1-b3-vm-validation
report: build/base1-b3-vm-validation/b3-log-bundle-review.env

b2_summary: build/base1-b2-test-suite/b2-test-suite-summary.env
uefi_summary: build/base1-b3-uefi-proof/reports/b3-summary.env
uefi_log: build/base1-b3-uefi-proof/reports/b3-serial.log
handoff_summary: build/base1-b3-kernel-handoff/reports/qemu-boot-summary.env
handoff_log: build/base1-b3-kernel-handoff/reports/qemu-boot.log
gnulinux_summary: build/base1-b3-gnulinux-stage/reports/qemu-boot-summary.env
gnulinux_log: build/base1-b3-gnulinux-stage/reports/qemu-boot.log
openbsd_summary: build/base1-b3-openbsd-stage/reports/openbsd-qemu-summary.env
openbsd_log: build/base1-b3-openbsd-stage/reports/openbsd-qemu-boot.log
openbsd_limitation: docs/os/B3_OPENBSD_SERIAL_LIMITATION.md

present: b2_summary -> build/base1-b2-test-suite/b2-test-suite-summary.env
pass_marker: b2_summary
present: uefi_summary -> build/base1-b3-uefi-proof/reports/b3-summary.env
pass_marker: uefi_summary
present: uefi_log -> build/base1-b3-uefi-proof/reports/b3-serial.log
present: handoff_summary -> build/base1-b3-kernel-handoff/reports/qemu-boot-summary.env
pass_marker: handoff_summary
present: handoff_log -> build/base1-b3-kernel-handoff/reports/qemu-boot.log
present: gnulinux_summary -> build/base1-b3-gnulinux-stage/reports/qemu-boot-summary.env
pass_marker: gnulinux_summary
present: gnulinux_log -> build/base1-b3-gnulinux-stage/reports/qemu-boot.log
present: openbsd_summary -> build/base1-b3-openbsd-stage/reports/openbsd-qemu-summary.env
pass_marker: openbsd_summary
present: openbsd_log -> build/base1-b3-openbsd-stage/reports/openbsd-qemu-boot.log
present: openbsd_limitation -> docs/os/B3_OPENBSD_SERIAL_LIMITATION.md

result: pass
failed_checks: 0
summary: build/base1-b3-vm-validation/b3-log-bundle-review.env
non_claims: local B3 log review only; no boot-ready claim; no installer claim; no hardware validation; no hardening proof; no daily-driver claim
exit_code: 0
PASS: B3 log bundle review

### B3 X200 report refresh
$ sh scripts/base1-b3-x200-upload-report.sh
base1-b3-x200-upload-report: wrote docs/os/B3_X200_EMULATOR_EVIDENCE_REPORT.md
base1-b3-x200-upload-report: complete
report: docs/os/B3_X200_EMULATOR_EVIDENCE_REPORT.md
exit_code: 0
PASS: B3 X200 report refresh

### status json parse
$ python3 -m json.tool site/status.json >/dev/null
exit_code: 0
PASS: status json parse

### status badge parse
$ python3 -m json.tool site/status-badge.json >/dev/null
exit_code: 0
PASS: status badge parse

### status docs mention roadmap
$ grep -E 'Overall estimated roadmap completion|Non-claims' docs/status/PROJECT_STATUS.md >/dev/null
exit_code: 0
PASS: status docs mention roadmap

### wiki source present
$ test -d docs/wiki && test -f docs/wiki/Home.md
exit_code: 0
PASS: wiki source present

### publish wiki script syntax
$ sh -n scripts/publish-wiki.sh
exit_code: 0
PASS: publish wiki script syntax
```

## Non-claims

This uploaded report records repository and local emulator-evidence validation only. It does not claim installer readiness, physical hardware validation, recovery validation, hardening proof, release-candidate readiness, or daily-driver readiness.

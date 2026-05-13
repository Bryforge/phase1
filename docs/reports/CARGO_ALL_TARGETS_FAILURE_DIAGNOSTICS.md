# Cargo all-targets failure diagnostics

Generated UTC: 2026-05-13T23:16:56Z
Commit: e70cfe01b3cfb894bddd34ceb9afbe2f7ff52ca1
Result code: 0

## Failure index
```text
102:test registry::tests::man_pages_are_generated ... ok
214:test asset_index_marks_old_fyr_flame_reference_outdated ... ok
215:test asset_index_marks_old_phase1_splash_and_logo_references_outdated ... ok
550:test base1_b3_uefi_proof_searches_root_before_loading_font_and_keeps_menu_overlay ... ok
592:test base1_boot_preview_script_refuses_unsafe_output_roots ... ok
681:test base1_emulator_doctor_reports_missing_bundle ... ok
682:test base1_emulator_doctor_checks_generated_bundle_without_launching ... ok
699:test base1_compatibility_contract_defaults_to_safe_mode ... ok
702:test base1_launcher_refuses_root_by_default ... ok
936:test base1_link_check_script_reports_missing_targets ... ok
947:test link_check_strategy_preserves_compatibility_paths ... ok
978:test migration_table_preserves_compatibility_decisions ... ok
1000:test post_reorganization_layout_preserves_public_compatibility_paths ... ok
1031:test base1_preview_gate_dry_run_refuses_missing_boot_inputs ... ok
1034:test base1_preview_gate_reports_missing_bundle ... ok
1044:test base1_preview_inputs_reports_missing_required_paths ... ok
1056:test base1_preview_provenance_reports_missing_bundle ... ok
1057:test base1_preview_provenance_records_hashes_for_generated_bundle ... ok
1099:test base1_preview_verify_passes_generated_stack_bundle ... ok
1100:test base1_preview_verify_reports_missing_bundle ... ok
1122:test root_readme_links_all_base1_public_surfaces ... ok
1133:test base1_qemu_boot_check_reports_missing_bundle ... ok
1705:test release_pre_move_checks_preserve_root_and_mirror_paths ... ok
1717:test reorganization_readiness_requires_compatibility_and_validation ... ok
1725:test reorganization_report_template_includes_compatibility_review_and_decision ... ok
1735:test reorganization_verifier_handles_missing_cargo_explicitly ... ok
1761:     Running tests/base1_root_compatibility_map_docs.rs (target/debug/deps/base1_root_compatibility_map_docs-28d7aaea7857c153)
1766:test release_archive_map_lists_former_root_and_archived_paths ... ok
1770:     Running tests/base1_script_compatibility_plan_docs.rs (target/debug/deps/base1_script_compatibility_plan_docs-f12306681b9797b1)
1773:test script_compatibility_plan_defines_stable_operator_paths ... ok
1774:test script_compatibility_plan_is_linked_from_base1_indexes ... ok
1775:test script_compatibility_plan_lists_future_candidate_groups ... ok
1776:test script_compatibility_plan_requires_wrappers_before_moves ... ok
1777:test script_compatibility_plan_preserves_non_claims ... ok
1879:test base1_supervisor_profiles_doc_preserves_compatibility_rules_and_non_claims ... ok
2099:test docs_sync_script_and_outputs_are_present ... FAILED
2100:test generated_docs_have_managed_blocks ... ok
2101:test generated_docs_preserve_stable_base_and_edge_path ... FAILED
2103:failures:
2107:thread 'docs_sync_script_and_outputs_are_present' (211534) panicked at tests/docs_sync_guard.rs:10:5:
2108:assertion failed: script.contains("base/v4.2.0")
2111:---- generated_docs_preserve_stable_base_and_edge_path stdout ----
2113:thread 'generated_docs_preserve_stable_base_and_edge_path' (211536) panicked at tests/docs_sync_guard.rs:32:9:
2117:failures:
2119:    generated_docs_preserve_stable_base_and_edge_path
2121:test result: FAILED. 1 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
2123:error: test failed, to rerun pass `--test docs_sync_guard`
```

## Final output
```text
test release_pre_move_checks_preserve_non_claims ... ok
test release_pre_move_checks_preserve_root_and_mirror_paths ... ok
test release_pre_move_checks_require_release_note_tests ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/base1_reorganization_readiness_docs.rs (target/debug/deps/base1_reorganization_readiness_docs-cdbd56af4059f38a)

running 5 tests
test reorganization_readiness_lists_remaining_blockers ... ok
test reorganization_readiness_lists_current_safeguards ... ok
test reorganization_readiness_records_current_state ... ok
test reorganization_readiness_preserves_non_claims ... ok
test reorganization_readiness_requires_compatibility_and_validation ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/base1_reorganization_verification_report_template_docs.rs (target/debug/deps/base1_reorganization_verification_report_template_docs-58daa9f2eaf80399)

running 5 tests
test reorganization_report_template_defines_metadata_and_commands ... ok
test reorganization_report_template_includes_compatibility_review_and_decision ... ok
test reorganization_report_template_lists_expected_passing_conditions ... ok
test reorganization_report_template_is_linked_and_integrity_checked ... ok
test reorganization_report_template_preserves_non_claims ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/base1_reorganization_verify_script.rs (target/debug/deps/base1_reorganization_verify_script-6360819e9954996e)

running 4 tests
test reorganization_verifier_handles_missing_cargo_explicitly ... ok
test reorganization_verifier_is_integrity_checked ... ok
test reorganization_verifier_is_read_only ... ok
test reorganization_verifier_runs_required_gates ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/base1_rollback_metadata_docs.rs (target/debug/deps/base1_rollback_metadata_docs-3f6d5b7da0b69ed9)

running 3 tests
test os_roadmap_links_rollback_metadata_design ... ok
test readme_links_rollback_metadata_design ... ok
test rollback_metadata_doc_exists_and_defines_safe_contract ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/base1_rollback_metadata_dry_run_script.rs (target/debug/deps/base1_rollback_metadata_dry_run_script-dd0a53bc69b62dd4)

running 4 tests
test rollback_metadata_dry_run_refuses_without_dry_run_flag ... ok
test rollback_metadata_dry_run_accepts_target_preview ... ok
test rollback_metadata_dry_run_script_avoids_destructive_tools_and_secrets ... ok
test rollback_metadata_dry_run_reports_preview_without_writes ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests/base1_root_compatibility_map_docs.rs (target/debug/deps/base1_root_compatibility_map_docs-28d7aaea7857c153)

running 3 tests
test documentation_map_and_manual_link_release_archive_map ... ok
test integrity_gate_checks_release_archive_map_and_release_archives ... ok
test release_archive_map_lists_former_root_and_archived_paths ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/base1_script_compatibility_plan_docs.rs (target/debug/deps/base1_script_compatibility_plan_docs-f12306681b9797b1)

running 5 tests
test script_compatibility_plan_defines_stable_operator_paths ... ok
test script_compatibility_plan_is_linked_from_base1_indexes ... ok
test script_compatibility_plan_lists_future_candidate_groups ... ok
test script_compatibility_plan_requires_wrappers_before_moves ... ok
test script_compatibility_plan_preserves_non_claims ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/base1_storage_layout_checker_docs.rs (target/debug/deps/base1_storage_layout_checker_docs-a687d0b1113b31e7)

running 3 tests
test os_roadmap_links_storage_layout_checker_design ... ok
test readme_links_storage_layout_checker_design ... ok
test storage_layout_checker_doc_exists_and_defines_read_only_surface ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/base1_storage_layout_dry_run_script.rs (target/debug/deps/base1_storage_layout_dry_run_script-f338a58d2c93cd60)

running 4 tests
test storage_layout_dry_run_refuses_without_dry_run_flag ... ok
test storage_layout_dry_run_requires_target ... ok
test storage_layout_dry_run_reports_preview_without_writes ... ok
test storage_layout_dry_run_script_avoids_destructive_tools ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/base1_supervisor_artifact_flow_docs.rs (target/debug/deps/base1_supervisor_artifact_flow_docs-c577de62036a4aca)

running 4 tests
test base1_supervisor_artifact_flow_defines_scope_and_purpose ... ok
test base1_supervisor_artifact_flow_lists_artifact_classes ... ok
test base1_supervisor_artifact_flow_preserves_required_fields ... ok
test base1_supervisor_artifact_flow_preserves_x200_vm_and_non_claims ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/base1_supervisor_control_plane_docs.rs (target/debug/deps/base1_supervisor_control_plane_docs-568e98211defc0ee)

running 4 tests
test base1_supervisor_control_plane_defines_scope_and_purpose ... ok
test base1_supervisor_control_plane_lists_command_surface ... ok
test base1_supervisor_control_plane_preserves_non_claims ... ok
test base1_supervisor_control_plane_preserves_policy_gates ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/base1_supervisor_control_plane_script.rs (target/debug/deps/base1_supervisor_control_plane_script-35f1a9ed9a0051fd)

running 7 tests
test base1_supervisor_control_plane_help_documents_commands_profiles_and_non_claims ... ok
test base1_supervisor_control_plane_blocks_x200_concurrent_launch_preview_by_policy ... ok
test base1_supervisor_control_plane_rejects_unknown_command_profile_and_non_build_out ... ok
test base1_supervisor_control_plane_records_policy_decision_for_status_report ... ok
test base1_supervisor_control_plane_script_exists_and_has_valid_shell_syntax ... ok
test base1_supervisor_control_plane_supports_vm_capture_evidence_profile ... ok
test base1_supervisor_control_plane_status_writes_report_for_x200_profile ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.05s

     Running tests/base1_supervisor_orchestration_docs.rs (target/debug/deps/base1_supervisor_orchestration_docs-9765ef563e776764)

running 1 test
test base1_supervisor_orchestration_doc_defines_model_profiles_and_limits ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/base1_supervisor_orchestration_plan_script.rs (target/debug/deps/base1_supervisor_orchestration_plan_script-6b109f7007af4e75)

running 5 tests
test base1_supervisor_orchestration_plan_defaults_to_x200_safe_shape ... ok
test base1_supervisor_orchestration_plan_prepare_writes_report ... ok
test base1_supervisor_orchestration_plan_rejects_unknown_profile_and_non_build_out ... ok
test base1_supervisor_orchestration_plan_script_exists_and_has_valid_shell_syntax ... ok
test base1_supervisor_orchestration_plan_supports_vm_concurrent_profile_shape ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests/base1_supervisor_policy_bus_docs.rs (target/debug/deps/base1_supervisor_policy_bus_docs-172896b6aa062957)

running 4 tests
test base1_supervisor_policy_bus_defines_scope_and_purpose ... ok
test base1_supervisor_policy_bus_lists_inputs_and_decisions ... ok
test base1_supervisor_policy_bus_preserves_non_claims ... ok
test base1_supervisor_policy_bus_preserves_x200_and_vm_rules ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/base1_supervisor_policy_bus_script.rs (target/debug/deps/base1_supervisor_policy_bus_script-432d6d97405b6464)

running 6 tests
test base1_supervisor_policy_bus_allows_vm_concurrent_as_evidence_required ... ok
test base1_supervisor_policy_bus_denies_x200_concurrent_mode ... ok
test base1_supervisor_policy_bus_help_documents_policy_surface ... ok
test base1_supervisor_policy_bus_script_exists_and_has_valid_shell_syntax ... ok
test base1_supervisor_policy_bus_rejects_unknown_command_profile_and_non_build_out ... ok
test base1_supervisor_policy_bus_status_writes_x200_report ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests/base1_supervisor_profiles_docs.rs (target/debug/deps/base1_supervisor_profiles_docs-076b963afe6c3d99)

running 4 tests
test base1_supervisor_profiles_doc_defines_purpose_and_profile_set ... ok
test base1_supervisor_profiles_doc_defines_x200_vm_and_workstation_intents ... ok
test base1_supervisor_profiles_doc_lists_required_fields ... ok
test base1_supervisor_profiles_doc_preserves_compatibility_rules_and_non_claims ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/base1_supervisor_storage_tier_docs.rs (target/debug/deps/base1_supervisor_storage_tier_docs-73d7d528609e74f5)

running 5 tests
test base1_supervisor_storage_tier_defines_scope_and_purpose ... ok
test base1_supervisor_storage_tier_preserves_non_claims ... ok
test base1_supervisor_storage_tier_preserves_policy_fields ... ok
test base1_supervisor_storage_tier_preserves_tier_order ... ok
test base1_supervisor_storage_tier_preserves_x200_and_vm_rules ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/base1_supervisor_storage_tier_plan_script.rs (target/debug/deps/base1_supervisor_storage_tier_plan_script-bc87b7d79f0f6948)

running 4 tests
test base1_supervisor_storage_tier_plan_help_documents_scope ... ok
test base1_supervisor_storage_tier_plan_rejects_unknown_profile_and_non_build_out ... ok
test base1_supervisor_storage_tier_plan_script_exists_and_has_valid_shell_syntax ... ok
test base1_supervisor_storage_tier_plan_writes_x200_report ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests/base1_test_inventory_script.rs (target/debug/deps/base1_test_inventory_script-236d5d7aac4ab1b6)

running 4 tests
test base1_test_inventory_is_documented_and_integrity_checked ... ok
test base1_test_inventory_script_is_read_only ... ok
test base1_test_inventory_script_reports_counts ... ok
test base1_test_inventory_script_lists_expected_patterns ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/base1_test_inventory_verify_script.rs (target/debug/deps/base1_test_inventory_verify_script-6148d4a331fbd610)

running 4 tests
test base1_test_inventory_verifier_compares_reporter_to_docs ... ok
test base1_test_inventory_verifier_is_read_only ... ok
test base1_test_inventory_verifier_is_wired_into_quality_gate ... ok
test base1_test_inventory_verifier_preserves_failure_behavior ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/base1_validation_report_template_docs.rs (target/debug/deps/base1_validation_report_template_docs-4f1c89f0131c84ba)

running 5 tests
test base1_index_links_validation_report_template ... ok
test base1_validation_report_template_exists ... ok
test base1_validation_report_template_preserves_result_labels ... ok
test base1_validation_report_template_preserves_promotion_ladder ... ok
test base1_validation_report_template_records_required_fields ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/base1_validation_reports_index_docs.rs (target/debug/deps/base1_validation_reports_index_docs-af5a2847b6793a45)

running 5 tests
test base1_index_links_validation_reports_archive ... ok
test base1_validation_reports_index_exists ... ok
test validation_reports_index_links_template_and_matrix ... ok
test validation_reports_index_preserves_non_claims_and_ladder ... ok
test validation_reports_index_preserves_required_fields ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/base1_validation_runbook_docs.rs (target/debug/deps/base1_validation_runbook_docs-7e88446dd0bd5e68)

running 4 tests
test base1_validation_runbook_exists ... ok
test base1_index_links_validation_runbook ... ok
test base1_validation_runbook_lists_docs_only_checks ... ok
test base1_validation_runbook_preserves_documentation_only_scope ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/base1_x86_64_detect_script.rs (target/debug/deps/base1_x86_64_detect_script-428ba1eddabbbf53)

running 5 tests
test base1_x86_64_detect_is_documented_from_b1_plan_and_status ... ok
test base1_x86_64_detect_preserves_read_only_scope_in_source ... ok
test base1_x86_64_detect_requires_dry_run ... ok
test base1_x86_64_detect_script_exists_and_has_valid_shell_syntax ... ok
test base1_x86_64_detect_dry_run_reports_no_writes ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s

     Running tests/bleeding.rs (target/debug/deps/bleeding-48644a45bbf50207)

running 6 tests
test bleeding_structured_pipelines_filter_text ... ok
test bleeding_edge_boot_switch_updates_ui_channel_and_version ... ok
test bleeding_tab_completion_expands_commands_and_arguments ... ok
test bleeding_theme_palettes_are_selectable ... ok
test bleeding_wasi_lite_plugins_are_sandboxed ... ok
test bleeding_version_and_roadmap_are_visible ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s

     Running tests/boot_greeting.rs (target/debug/deps/boot_greeting-fd15cb29385cdd1c)

running 1 test
test boot_screen_uses_japanese_hacker_greeting ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/boot_readiness_race_plan_docs.rs (target/debug/deps/boot_readiness_race_plan_docs-6afe6066f4c0edad)

running 10 tests
test boot_readiness_race_plan_defines_goal_and_boundary ... ok
test boot_readiness_race_plan_defines_fast_sprints ... ok
test boot_readiness_race_plan_defines_readiness_ladder ... ok
test boot_readiness_race_plan_documents_b1_script_and_tests ... ok
test boot_readiness_race_plan_documents_b2_script_and_tests ... ok
test boot_readiness_race_plan_documents_b3_planning ... ok
test boot_readiness_race_plan_is_linked_from_os_roadmap_and_readme ... ok
test boot_readiness_race_plan_lists_required_artifacts ... ok
test boot_readiness_race_plan_preserves_checklist_and_safety_rules ... ok
test boot_readiness_race_plan_preserves_non_claims ... ok

test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/boot_readiness_status_docs.rs (target/debug/deps/boot_readiness_status_docs-a6cf39f6d3c4a98e)

running 13 tests
test boot_readiness_status_defines_b1_implementation_status ... ok
test boot_readiness_status_defines_b2_implementation_status ... ok
test boot_readiness_status_defines_current_level_and_target ... ok
test boot_readiness_status_defines_b3_planning_proof_handoff_and_stage_status ... ok
test boot_readiness_status_defines_first_coding_slice ... ok
test boot_readiness_status_is_linked_from_required_docs ... ok
test boot_readiness_status_preserves_b1_completion_checklist ... ok
test boot_readiness_status_preserves_b2_completion_checklist ... ok
test boot_readiness_status_preserves_b3_completion_checklist ... ok
test boot_readiness_status_preserves_finish_before_coding_checklist ... ok
test boot_readiness_status_preserves_evidence_map ... ok
test boot_readiness_status_preserves_hardening_and_non_claims ... ok
test boot_readiness_status_preserves_ladder ... ok

test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests/codex_pr_checklist_docs.rs (target/debug/deps/codex_pr_checklist_docs-70a65dc249fe29a5)

running 3 tests
test codex_pr_checklist_contains_required_sections ... ok
test codex_pr_checklist_exists ... ok
test developer_index_links_pr_checklist ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/codex_templates_docs.rs (target/debug/deps/codex_templates_docs-d10fefb24cbcd429)

running 4 tests
test codex_template_docs_exist ... ok
test docs_index_links_codex_templates ... ok
test status_blocks_include_required_statuses ... ok
test template_index_links_core_templates ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/community_automated_support_ai_docs.rs (target/debug/deps/community_automated_support_ai_docs-ef02133790fa0a92)

running 8 tests
test automated_support_ai_roadmap_defines_goal_and_scope ... ok
test automated_support_ai_roadmap_defines_escalation_and_issue_routing ... ok
test automated_support_ai_roadmap_defines_phased_capabilities ... ok
test automated_support_ai_roadmap_is_linked_from_community_index ... ok
test automated_support_ai_roadmap_prefers_read_only_diagnostics ... ok
test automated_support_ai_roadmap_preserves_non_claims ... ok
test automated_support_ai_roadmap_preserves_non_goals ... ok
test automated_support_ai_roadmap_preserves_security_and_privacy_rules ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/contributing_guidelines_docs.rs (target/debug/deps/contributing_guidelines_docs-2dc5c818df98dc0b)

running 7 tests
test contributing_guidelines_define_branch_pr_and_validation_workflow ... ok
test contributing_guidelines_define_pr_checklist ... ok
test contributing_guidelines_define_project_scope_and_ground_rules ... ok
test contributing_guidelines_define_security_crypto_base1_and_fyr_rules ... ok
test contributing_guidelines_preserve_docs_claim_rules ... ok
test contributing_guidelines_define_testing_expectations_and_rejection_criteria ... ok
test contributing_guidelines_preserve_non_claims ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/dev_dock_self_hosted.rs (target/debug/deps/dev_dock_self_hosted-32fa33f0afe750a9)

running 3 tests
test dev_dock_docs_explain_inside_phase1_workflow ... ok
test dev_dock_exposes_self_hosted_workflow_commands ... ok
test dev_plugin_reads_phase1_context_args_from_stdin ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.06s

     Running tests/developer_docs_contribution_links.rs (target/debug/deps/developer_docs_contribution_links-5f713f020ac10b23)

running 5 tests
test developer_docs_define_development_reading_order ... ok
test developer_docs_include_contribution_gates ... ok
test developer_docs_link_repository_contribution_guidelines ... ok
test developer_docs_mark_crypto_as_safety_sensitive ... ok
test developer_docs_preserve_developer_rule ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/docs_index_contribution_links.rs (target/debug/deps/docs_index_contribution_links-50433a7dde7e287d)

running 3 tests
test docs_index_lists_contributor_and_community_paths ... ok
test docs_index_links_contribution_entry_points ... ok
test docs_index_preserves_status_and_safety_language ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/docs_sync_guard.rs (target/debug/deps/docs_sync_guard-305a19ef5a671fd8)

running 3 tests
test docs_sync_script_and_outputs_are_present ... FAILED
test generated_docs_have_managed_blocks ... ok
test generated_docs_preserve_stable_base_and_edge_path ... FAILED

failures:

---- docs_sync_script_and_outputs_are_present stdout ----

thread 'docs_sync_script_and_outputs_are_present' (211534) panicked at tests/docs_sync_guard.rs:10:5:
assertion failed: script.contains("base/v4.2.0")
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

---- generated_docs_preserve_stable_base_and_edge_path stdout ----

thread 'generated_docs_preserve_stable_base_and_edge_path' (211536) panicked at tests/docs_sync_guard.rs:32:9:
README.md lost the frozen stable base


failures:
    docs_sync_script_and_outputs_are_present
    generated_docs_preserve_stable_base_and_edge_path

test result: FAILED. 1 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass `--test docs_sync_guard`
```

## Non-claims

This diagnostics report records repository test output only. It does not claim installer readiness, physical hardware validation, recovery validation, hardening proof, release-candidate readiness, or daily-driver readiness.

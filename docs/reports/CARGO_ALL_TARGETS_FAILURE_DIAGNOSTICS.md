# Cargo all-targets failure diagnostics

Generated UTC: 2026-05-13T23:19:43Z
Commit: 4b27ba8d83612e1ba59651caa465ed112f780339
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
935:test base1_link_check_script_reports_missing_targets ... ok
948:test link_check_strategy_preserves_compatibility_paths ... ok
978:test migration_table_preserves_compatibility_decisions ... ok
1000:test post_reorganization_layout_preserves_public_compatibility_paths ... ok
1031:test base1_preview_gate_dry_run_refuses_missing_boot_inputs ... ok
1034:test base1_preview_gate_reports_missing_bundle ... ok
1044:test base1_preview_inputs_reports_missing_required_paths ... ok
1056:test base1_preview_provenance_reports_missing_bundle ... ok
1057:test base1_preview_provenance_records_hashes_for_generated_bundle ... ok
1098:test base1_preview_verify_passes_generated_stack_bundle ... ok
1100:test base1_preview_verify_reports_missing_bundle ... ok
1122:test root_readme_links_all_base1_public_surfaces ... ok
1133:test base1_qemu_boot_check_reports_missing_bundle ... ok
1704:test release_pre_move_checks_preserve_root_and_mirror_paths ... ok
1717:test reorganization_readiness_requires_compatibility_and_validation ... ok
1725:test reorganization_report_template_includes_compatibility_review_and_decision ... ok
1735:test reorganization_verifier_handles_missing_cargo_explicitly ... ok
1761:     Running tests/base1_root_compatibility_map_docs.rs (target/debug/deps/base1_root_compatibility_map_docs-28d7aaea7857c153)
1766:test release_archive_map_lists_former_root_and_archived_paths ... ok
1770:     Running tests/base1_script_compatibility_plan_docs.rs (target/debug/deps/base1_script_compatibility_plan_docs-f12306681b9797b1)
1773:test script_compatibility_plan_defines_stable_operator_paths ... ok
1774:test script_compatibility_plan_is_linked_from_base1_indexes ... ok
1775:test script_compatibility_plan_lists_future_candidate_groups ... ok
1776:test script_compatibility_plan_preserves_non_claims ... ok
1777:test script_compatibility_plan_requires_wrappers_before_moves ... ok
1879:test base1_supervisor_profiles_doc_preserves_compatibility_rules_and_non_claims ... ok
2100:test generated_docs_have_managed_blocks ... ok
2101:test old_root_generated_docs_are_not_required_or_regenerated ... ok
2102:test generated_docs_preserve_current_stable_base_edge_version_and_edge_path ... ok
2167:test fyr_color_reports_missing_file ... ok
2245:test fyr_parser_reports_missing_main ... ok
2246:test fyr_parser_reports_missing_semicolon ... ok
2270:test fyr_test_reports_missing_package_manifest ... ok
2382:test nest_destroy_active_context_returns_to_root ... ok
2384:test nest_destroy_reports_missing_child_context ... ok
2391:test nest_enter_reports_missing_child_context ... ok
2393:test nest_exit_at_root_is_safe ... ok
2404:test nest_inspect_reports_missing_child ... ok
2423:test nest_status_reports_root_context ... ok
2432:test nest_tree_reports_empty_root ... ok
2525:test pull_request_template_preserves_compatibility_checklist ... ok
2585:test compatibility_base_remains_documented ... ok
2608:test reorganization_plan_defines_destination_map_and_root_policy ... ok
2642:     Running tests/security_crypto_compatibility_profile_docs.rs (target/debug/deps/security_crypto_compatibility_profile_docs-24511fa5c9fabb27)
2645:test compatibility_profile_defines_purpose_and_operator ... ok
2646:test compatibility_profile_defines_warning_audit_and_migration_behavior ... ok
2647:test compatibility_profile_is_linked_from_profiles_index ... ok
2648:test compatibility_profile_lists_allowed_control_points ... ok
2649:test compatibility_profile_preserves_non_claims ... ok
2650:test compatibility_profile_rejects_unsafe_entries_and_new_weak_data ... ok
2651:test compatibility_profile_requires_explicit_warning_and_consent ... ok
2652:test compatibility_profile_requires_registry_template_and_statuses ... ok
2802:test crypto_provider_template_lists_profile_and_control_point_compatibility ... ok
2900:test v6_edge_defaults_to_crimson_theme_when_theme_is_missing_or_empty ... ok
2916:test website_docs_mark_outdated_visual_assets_without_using_them_as_current ... ok
2935:test homepage_has_inline_founder_profile_guard ... FAILED
2937:test homepage_keeps_static_offline_friendly_dependency_posture_and_cache_busts_assets ... FAILED
2938:test homepage_preserves_project_identity_and_metadata ... FAILED
2940:test website_mobile_fix_prevents_fragmented_headings_and_duplicate_creator_labels ... FAILED
2941:test site_js_implements_canvas_terminal_progressive_enhancement_and_performance_guards ... FAILED
2943:failures:
2947:thread 'homepage_has_inline_founder_profile_guard' (240058) panicked at tests/website_phase_b.rs:161:9:
2948:missing "#founder .founder-copy > .eyebrow"
2953:thread 'homepage_keeps_static_offline_friendly_dependency_posture_and_cache_busts_assets' (240060) panicked at tests/website_phase_b.rs:161:9:
2954:missing "./site/styles.css?v=4.0.0-stable-2"
2958:thread 'homepage_preserves_project_identity_and_metadata' (240061) panicked at tests/website_phase_b.rs:161:9:
2959:missing "name\": \"phase1\""
2963:thread 'website_mobile_fix_prevents_fragmented_headings_and_duplicate_creator_labels' (240064) panicked at tests/website_phase_b.rs:161:9:
2964:missing "Founder-section cleanup"
2968:thread 'site_js_implements_canvas_terminal_progressive_enhancement_and_performance_guards' (240062) panicked at tests/website_phase_b.rs:161:9:
2969:missing "desktop ? 180 : 210"
2972:failures:
2979:test result: FAILED. 2 passed; 5 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
2981:error: test failed, to rerun pass `--test website_phase_b`
```

## Final output
```text
test quality_scripts_exist_and_are_valid_shell ... ok
test quality_workflow_exists ... ok
test quality_score_reports_score_and_rating ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests/readme_navigation_reorganization_links.rs (target/debug/deps/readme_navigation_reorganization_links-085e987ed14d3c5a)

running 7 tests
test readme_links_boot_readiness_b1_docs_and_detector ... ok
test readme_links_boot_readiness_b2_docs_and_assembly_preview ... ok
test readme_links_core_navigation_and_reorganization_docs ... ok
test readme_links_current_public_asset_components ... ok
test readme_links_organized_destinations ... ok
test readme_links_current_support_templates ... ok
test readme_links_x86_64_and_hardening_roadmap ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/release_metadata.rs (target/debug/deps/release_metadata-8502d2f4ab5d2e4a)

running 7 tests
test cargo_metadata_matches_current_track ... ok
test compatibility_base_remains_documented ... ok
test edge_checkpoint_records_current_dev_boundary ... ok
test edge_track_is_documented_when_package_is_dev ... ok
test release_metadata_is_consistent_across_public_docs ... ok
test website_demo_reports_current_stable_track ... ok
test stale_dev_release_lines_are_removed_from_release_facing_files ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/reorganization_destination_indexes.rs (target/debug/deps/reorganization_destination_indexes-1add267adca92407)

running 5 tests
test release_docs_index_defines_preservation_first_release_home ... ok
test examples_index_defines_safe_example_home ... ok
test reorganization_plan_mentions_destination_folders ... ok
test tools_index_defines_internal_tooling_boundary ... ok
test website_docs_index_defines_public_site_home_and_claim_safety ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/reorganization_plan_docs.rs (target/debug/deps/reorganization_plan_docs-03db4699fac2919c)

running 8 tests
test reorganization_plan_defines_destination_map_and_root_policy ... ok
test reorganization_plan_defines_minimalist_target_structure ... ok
test reorganization_plan_defines_special_handling_for_base1_and_crypto ... ok
test reorganization_plan_defines_phases_move_map_and_rollback ... ok
test reorganization_plan_is_linked_from_docs_index_and_navigation ... ok
test reorganization_plan_preserves_create_a_place_policy ... ok
test reorganization_plan_preserves_first_rules ... ok
test reorganization_plan_preserves_non_claims ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/repo_channel_guard.rs (target/debug/deps/repo_channel_guard-f77fa53d34a0eddb)

running 2 tests
test release_docs_keep_4_2_0_as_stability_base ... ok
test repository_model_documents_stable_base_and_edge_default ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/repository_navigation_docs.rs (target/debug/deps/repository_navigation_docs-10f515dec967df39)

running 9 tests
test docs_index_links_repository_navigation_guide ... ok
test repository_navigation_guide_defines_destination_organizer_rules ... ok
test repository_navigation_guide_defines_issue_template_chooser ... ok
test repository_navigation_guide_defines_purpose_and_scope ... ok
test repository_navigation_guide_defines_quality_gate_chooser ... ok
test repository_navigation_guide_lists_fast_paths ... ok
test repository_navigation_guide_defines_reader_paths ... ok
test repository_navigation_guide_maps_repository_paths ... ok
test repository_navigation_guide_preserves_reorganization_rules_and_non_claims ... ok

test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/security_crypto_compatibility_profile_docs.rs (target/debug/deps/security_crypto_compatibility_profile_docs-24511fa5c9fabb27)

running 8 tests
test compatibility_profile_defines_purpose_and_operator ... ok
test compatibility_profile_defines_warning_audit_and_migration_behavior ... ok
test compatibility_profile_is_linked_from_profiles_index ... ok
test compatibility_profile_lists_allowed_control_points ... ok
test compatibility_profile_preserves_non_claims ... ok
test compatibility_profile_rejects_unsafe_entries_and_new_weak_data ... ok
test compatibility_profile_requires_explicit_warning_and_consent ... ok
test compatibility_profile_requires_registry_template_and_statuses ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/security_crypto_config_schema_docs.rs (target/debug/deps/security_crypto_config_schema_docs-63af2869bc1ce800)

running 7 tests
test crypto_config_schema_defines_default_safe_posture ... ok
test crypto_config_schema_defines_fail_closed_validation ... ok
test crypto_config_schema_defines_warning_and_audit_rules ... ok
test crypto_config_schema_includes_safe_and_rejected_examples ... ok
test crypto_config_schema_is_linked_from_security_index_roadmap_and_integrity_gate ... ok
test crypto_config_schema_lists_allowed_profiles_and_scopes ... ok
test crypto_config_schema_preserves_non_claims ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/security_crypto_doc_integrity_script.rs (target/debug/deps/security_crypto_doc_integrity_script-09e6324f54c9169b)

running 8 tests
test quality_check_runs_security_crypto_docs_gate ... ok
test security_crypto_doc_integrity_gate_checks_config_and_implementation_plans ... ok
test security_crypto_doc_integrity_gate_checks_links_and_guardrails ... ok
test security_crypto_doc_integrity_gate_checks_operator_command_plan ... ok
test security_crypto_doc_integrity_gate_checks_profile_non_claims ... ok
test security_crypto_doc_integrity_gate_checks_required_docs ... ok
test security_crypto_doc_integrity_gate_is_read_only ... ok
test security_crypto_doc_integrity_gate_checks_provider_registry_and_template ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/security_crypto_high_security_profile_docs.rs (target/debug/deps/security_crypto_high_security_profile_docs-5ab21acbd8d8c5d7)

running 8 tests
test high_security_profile_defines_purpose_and_operator ... ok
test high_security_profile_defines_base1_requirements ... ok
test high_security_profile_guards_downgrades_and_audit ... ok
test high_security_profile_is_linked_from_profiles_index ... ok
test high_security_profile_lists_stricter_control_points ... ok
test high_security_profile_rejects_unsafe_or_ambiguous_entries ... ok
test high_security_profile_preserves_non_claims ... ok
test high_security_profile_requires_registry_template_and_eligible_statuses ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/security_crypto_implementation_plan_docs.rs (target/debug/deps/security_crypto_implementation_plan_docs-66ee3ddd63c3f456)

running 8 tests
test crypto_implementation_plan_defines_provider_abstraction_requirements ... ok
test crypto_implementation_plan_blocks_runtime_use_until_prerequisites ... ok
test crypto_implementation_plan_defines_safe_sequence ... ok
test crypto_implementation_plan_is_linked_from_index_roadmap_and_integrity_gate ... ok
test crypto_implementation_plan_lists_required_docs_including_provider_registry ... ok
test crypto_implementation_plan_defines_scoped_integration_order ... ok
test crypto_implementation_plan_preserves_non_claims ... ok
test crypto_implementation_plan_preserves_security_rules ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/security_crypto_lab_only_profile_docs.rs (target/debug/deps/security_crypto_lab_only_profile_docs-359d4fb4d7605cc7)

running 8 tests
test lab_only_profile_defines_purpose_and_operator ... ok
test lab_only_profile_blocks_production_control_points ... ok
test lab_only_profile_defines_warnings_audit_and_cleanup ... ok
test lab_only_profile_is_linked_from_profiles_index ... ok
test lab_only_profile_limits_allowed_control_points ... ok
test lab_only_profile_preserves_non_claims ... ok
test lab_only_profile_rejects_real_protection_and_unsafe_claims ... ok
test lab_only_profile_requires_registry_and_template ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/security_crypto_operator_commands_docs.rs (target/debug/deps/security_crypto_operator_commands_docs-1da832b5b9e4ffe9)

running 7 tests
test crypto_operator_commands_are_linked_from_security_index_and_roadmap ... ok
test crypto_operator_commands_define_allowed_scopes_and_profiles ... ok
test crypto_operator_commands_define_audit_expectations ... ok
test crypto_operator_commands_define_planned_command_surface ... ok
test crypto_operator_commands_define_selection_safety_gates ... ok
test crypto_operator_commands_define_verification_rules ... ok
test crypto_operator_commands_preserve_non_claims ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/security_crypto_policy_roadmap_docs.rs (target/debug/deps/security_crypto_policy_roadmap_docs-d4d62d99086e4a87)

running 11 tests
test crypto_algorithm_template_preserves_safety_requirements ... ok
test crypto_algorithm_template_defines_required_sections ... ok
test crypto_policy_roadmap_defines_operator_selectable_profiles ... ok
test crypto_policy_roadmap_defines_points_of_control ... ok
test crypto_policy_roadmap_documents_provider_registry ... ok
test crypto_policy_roadmap_links_all_profile_planning_docs ... ok
test crypto_policy_roadmap_documents_registry_and_algorithm_requirements ... ok
test crypto_policy_roadmap_links_operator_commands_and_config_schema ... ok
test crypto_policy_roadmap_rejects_custom_security_primitives ... ok
test crypto_policy_roadmap_preserves_non_claims ... ok
test security_index_links_crypto_policy_surface ... ok

test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/security_crypto_post_quantum_preview_profile_docs.rs (target/debug/deps/security_crypto_post_quantum_preview_profile_docs-9434926f315340d8)

running 8 tests
test post_quantum_preview_profile_defines_purpose_and_operator ... ok
test post_quantum_preview_profile_defines_base1_preview_requirements ... ok
test post_quantum_preview_profile_is_linked_from_profiles_index ... ok
test post_quantum_preview_profile_lists_preview_control_points ... ok
test post_quantum_preview_profile_preserves_non_claims ... ok
test post_quantum_preview_profile_rejects_unsafe_claims_and_silent_migration ... ok
test post_quantum_preview_profile_requires_registry_template_and_statuses ... ok
test post_quantum_preview_profile_requires_preview_warnings ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/security_crypto_profiles_docs.rs (target/debug/deps/security_crypto_profiles_docs-d995a7268ae2c4e9)

running 7 tests
test crypto_profiles_index_defines_profile_classes ... ok
test crypto_profiles_index_is_linked_from_security_index_and_roadmap ... ok
test crypto_profiles_index_links_current_profile_drafts ... ok
test crypto_profiles_index_links_related_crypto_docs ... ok
test crypto_profiles_index_preserves_non_claims ... ok
test crypto_profiles_index_preserves_profile_safety_rules ... ok
test crypto_profiles_index_requires_profile_documentation_fields ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/security_crypto_provider_registry_docs.rs (target/debug/deps/security_crypto_provider_registry_docs-a4210ce5ea8a5342)

running 9 tests
test crypto_provider_registry_defines_purpose_and_security_goal ... ok
test crypto_provider_registry_defines_status_labels ... ok
test crypto_provider_registry_is_linked_from_security_docs ... ok
test crypto_provider_registry_lists_required_provider_metadata ... ok
test crypto_provider_registry_preserves_non_claims ... ok
test crypto_provider_registry_requires_fail_closed_selection ... ok
test crypto_provider_registry_requires_provider_template ... ok
test crypto_provider_template_is_linked_from_registry_index_and_integrity_gate ... ok
test crypto_provider_registry_requires_review_before_profile_use ... ok

test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/security_crypto_provider_template_docs.rs (target/debug/deps/security_crypto_provider_template_docs-711b96fad7d72eec)

running 7 tests
test crypto_provider_template_is_linked_from_provider_registry_index_and_integrity_gate ... ok
test crypto_provider_template_defines_required_sections ... ok
test crypto_provider_template_lists_profile_and_control_point_compatibility ... ok
test crypto_provider_template_preserves_non_claims ... ok
test crypto_provider_template_preserves_review_checklist ... ok
test crypto_provider_template_requires_fail_closed_and_explicit_fallback ... ok
test crypto_provider_template_requires_source_license_and_pinning ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/security_crypto_quality_docs.rs (target/debug/deps/security_crypto_quality_docs-bf61f9470cd27e13)

running 4 tests
test quality_docs_describe_security_crypto_docs_gate ... ok
test quality_docs_list_security_crypto_required_script ... ok
test quality_docs_define_security_crypto_gate_scope ... ok
test quality_docs_preserve_crypto_safety_baseline ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/security_crypto_registry_docs.rs (target/debug/deps/security_crypto_registry_docs-b4c3c999bd36ec9d)

running 6 tests
test crypto_registry_defines_profile_classes_and_control_points ... ok
test crypto_registry_defines_entry_status_labels ... ok
test crypto_registry_is_linked_from_security_index_and_roadmap ... ok
test crypto_registry_lists_algorithm_families ... ok
test crypto_registry_requires_algorithm_template_and_review_fields ... ok
test crypto_registry_preserves_initial_non_claims ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/security_crypto_safe_default_profile_docs.rs (target/debug/deps/security_crypto_safe_default_profile_docs-647ef0ca3bbcb192)

running 7 tests
test safe_default_profile_defines_purpose_and_operator ... ok
test safe_default_profile_is_linked_from_profiles_index ... ok
test safe_default_profile_guards_downgrades_and_audit ... ok
test safe_default_profile_lists_control_points ... ok
test safe_default_profile_preserves_non_claims ... ok
test safe_default_profile_rejects_unsafe_entries ... ok
test safe_default_profile_requires_registry_and_template ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/security_usability_goal_docs.rs (target/debug/deps/security_usability_goal_docs-b6807e135d1d9518)

running 5 tests
test security_policy_defines_security_usability_goal ... ok
test security_policy_links_crypto_policy_goal ... ok
test security_review_checklist_includes_usability_review ... ok
test trust_model_defines_security_and_usability_principle ... ok
test main_readme_links_security_crypto_policy ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/smoke.rs (target/debug/deps/smoke-f924f8cb75339bff)

running 13 tests
test boot_help_man_and_completion_work ... ok
test expected_errors_are_clear ... ok
test network_commands_have_stable_safe_output ... ok
test filesystem_commands_round_trip ... ok
test persistent_history_restores_and_sanitizes_commands ... ok
test preboot_persistent_state_mode_is_toggleable_and_restores_home_files ... ok
test proc_sys_audit_and_arch_commands_work ... ok
test safe_off_without_host_tools_still_blocks_host_commands ... ok
test secure_default_blocks_host_backed_commands ... ok
test security_and_accounts_reports_are_privacy_safe ... ok
test roadmap_aliases_capabilities_and_dashboard_work ... ok
test text_search_and_chain_commands_work ... ok
test user_env_browser_and_sandbox_commands_work ... ok

test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.21s

     Running tests/storage_smoke.rs (target/debug/deps/storage_smoke-83803149c30f96b0)

running 5 tests
test language_roadmap_names_major_runtime_families ... ok
test repository_and_cargo_inputs_are_validated ... ok
test mutating_storage_git_and_rust_actions_are_blocked_by_default ... ok
test storage_init_and_rust_init_work_only_after_explicit_trust_gate ... ok
test storage_status_is_read_only_and_guarded_by_default ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests/updater_edge_stable_evidence_docs.rs (target/debug/deps/updater_edge_stable_evidence_docs-736b2d078eb0c5fe)

running 4 tests
test update_protocol_links_updater_edge_stable_evidence ... ok
test updater_edge_stable_evidence_preserves_non_claims ... ok
test updater_edge_stable_evidence_records_target_and_success_path ... ok
test updater_edge_stable_evidence_report_exists ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/v6_crimson_default_theme.rs (target/debug/deps/v6_crimson_default_theme-ab382ccefff5a60c)

running 3 tests
test bleeding_edge_theme_remains_available_as_legacy_manual_palette ... ok
test v6_edge_defaults_to_crimson_theme_when_theme_is_missing_or_empty ... ok
test v6_edge_selector_should_not_default_display_to_bleeding_edge ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/v6_crimson_theme_command.rs (target/debug/deps/v6_crimson_theme_command-fefa2034516807e8)

running 1 test
test theme_status_alias_and_edge_defaults_use_crimson ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/website_asset_components_docs.rs (target/debug/deps/website_asset_components_docs-c778b55a3e0ec740)

running 4 tests
test current_public_asset_files_exist ... ok
test website_docs_mark_outdated_visual_assets_without_using_them_as_current ... ok
test website_docs_reference_current_public_asset_components ... ok
test asset_index_and_readme_agree_on_current_public_assets ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/website_direction_plan.rs (target/debug/deps/website_direction_plan-74bfea93714f2f9d)

running 4 tests
test direction_plan_preserves_static_security_posture ... ok
test next_roadmap_implementation_has_clear_pr_sequence ... ok
test project_and_company_pages_stay_distinct ... ok
test roadmap_points_to_next_design_implementation_docs ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/website_phase_b.rs (target/debug/deps/website_phase_b-2e66bb22dc75e966)

running 7 tests
test homepage_has_inline_founder_profile_guard ... FAILED
test homepage_includes_phase_b_landing_sections ... ok
test homepage_keeps_static_offline_friendly_dependency_posture_and_cache_busts_assets ... FAILED
test homepage_preserves_project_identity_and_metadata ... FAILED
test styles_cover_terminal_roadmap_mobile_and_reveal_states ... ok
test website_mobile_fix_prevents_fragmented_headings_and_duplicate_creator_labels ... FAILED
test site_js_implements_canvas_terminal_progressive_enhancement_and_performance_guards ... FAILED

failures:

---- homepage_has_inline_founder_profile_guard stdout ----

thread 'homepage_has_inline_founder_profile_guard' (240058) panicked at tests/website_phase_b.rs:161:9:
missing "#founder .founder-copy > .eyebrow"
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

---- homepage_keeps_static_offline_friendly_dependency_posture_and_cache_busts_assets stdout ----

thread 'homepage_keeps_static_offline_friendly_dependency_posture_and_cache_busts_assets' (240060) panicked at tests/website_phase_b.rs:161:9:
missing "./site/styles.css?v=4.0.0-stable-2"

---- homepage_preserves_project_identity_and_metadata stdout ----

thread 'homepage_preserves_project_identity_and_metadata' (240061) panicked at tests/website_phase_b.rs:161:9:
missing "name\": \"phase1\""

---- website_mobile_fix_prevents_fragmented_headings_and_duplicate_creator_labels stdout ----

thread 'website_mobile_fix_prevents_fragmented_headings_and_duplicate_creator_labels' (240064) panicked at tests/website_phase_b.rs:161:9:
missing "Founder-section cleanup"

---- site_js_implements_canvas_terminal_progressive_enhancement_and_performance_guards stdout ----

thread 'site_js_implements_canvas_terminal_progressive_enhancement_and_performance_guards' (240062) panicked at tests/website_phase_b.rs:161:9:
missing "desktop ? 180 : 210"


failures:
    homepage_has_inline_founder_profile_guard
    homepage_keeps_static_offline_friendly_dependency_posture_and_cache_busts_assets
    homepage_preserves_project_identity_and_metadata
    site_js_implements_canvas_terminal_progressive_enhancement_and_performance_guards
    website_mobile_fix_prevents_fragmented_headings_and_duplicate_creator_labels

test result: FAILED. 2 passed; 5 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass `--test website_phase_b`
```

## Non-claims

This diagnostics report records repository test output only. It does not claim installer readiness, physical hardware validation, recovery validation, hardening proof, release-candidate readiness, or daily-driver readiness.

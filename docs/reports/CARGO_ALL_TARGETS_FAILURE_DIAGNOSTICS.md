# Cargo all-targets failure diagnostics

Generated UTC: 2026-05-13T23:03:03Z
Commit: afd7110ababbb99c310908fe4f2f2d266d3ec1b6

## Failure index
```text
214:test asset_index_marks_old_fyr_flame_reference_outdated ... ok
215:test asset_index_marks_old_phase1_splash_and_logo_references_outdated ... ok
681:test base1_emulator_doctor_reports_missing_bundle ... ok
935:test base1_link_check_script_reports_missing_targets ... ok
1031:test base1_preview_gate_dry_run_refuses_missing_boot_inputs ... ok
1034:test base1_preview_gate_reports_missing_bundle ... ok
1044:test base1_preview_inputs_reports_missing_required_paths ... ok
1056:test base1_preview_provenance_reports_missing_bundle ... ok
1101:test base1_preview_verify_reports_missing_bundle ... ok
1133:test base1_qemu_boot_check_reports_missing_bundle ... ok
1162:test qemu_visual_boot_preview_script_help_documents_usage_and_boundaries ... FAILED
1165:test qemu_visual_boot_preview_script_rejects_unknown_argument ... FAILED
1168:test qemu_visual_boot_preview_script_requires_action ... FAILED
1171:failures:
1175:thread 'qemu_visual_boot_preview_script_help_documents_usage_and_boundaries' (108510) panicked at tests/base1_qemu_visual_boot_preview_script.rs:35:5:
1186:thread 'qemu_visual_boot_preview_script_rejects_unknown_argument' (108514) panicked at tests/base1_qemu_visual_boot_preview_script.rs:95:5:
1192:thread 'qemu_visual_boot_preview_script_requires_action' (108516) panicked at tests/base1_qemu_visual_boot_preview_script.rs:75:5:
1197:failures:
1202:test result: FAILED. 5 passed; 3 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
1204:error: test failed, to rerun pass `--test base1_qemu_visual_boot_preview_script`
```

## Final output
```text
test link_check_strategy_defines_required_surfaces ... ok
test link_check_strategy_is_linked_from_indexes_and_integrity_gate ... ok
test link_check_strategy_preserves_compatibility_paths ... ok
test link_check_strategy_preserves_non_claims ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/base1_local_boot_artifact_plan_script.rs (target/debug/deps/base1_local_boot_artifact_plan_script-4583c280aea238ff)

running 4 tests
test base1_local_boot_artifact_plan_help_documents_scope ... ok
test base1_local_boot_artifact_plan_rejects_unknown_profile_and_non_build_paths ... ok
test base1_local_boot_artifact_plan_script_exists_and_has_valid_shell_syntax ... ok
test base1_local_boot_artifact_plan_prepare_writes_report ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests/base1_manual_verification_rule_docs.rs (target/debug/deps/base1_manual_verification_rule_docs-fc4a8ae4646b7494)

running 4 tests
test manual_documents_base1_verification_rule ... ok
test manual_explains_reorganization_verifier_scope ... ok
test manual_keeps_organization_guardrails_visible ... ok
test manual_preserves_base1_non_claims ... ok

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
test base1_readme_links_network_lockdown_preview ... ok
test dry_run_command_index_includes_network_lockdown ... ok
test network_lockdown_doc_defines_read_only_contract ... ok
test network_lockdown_script_requires_dry_run_and_reports_no_writes ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/base1_post_reorganization_layout_docs.rs (target/debug/deps/base1_post_reorganization_layout_docs-3138d9422083651c)

running 5 tests
test post_reorganization_layout_is_linked_and_integrity_checked ... ok
test post_reorganization_layout_defines_stable_public_paths ... ok
test post_reorganization_layout_lists_major_layout_sections ... ok
test post_reorganization_layout_preserves_non_claims ... ok
test post_reorganization_layout_preserves_public_compatibility_paths ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/base1_pre_move_checklist_docs.rs (target/debug/deps/base1_pre_move_checklist_docs-bf2c934f4f1fe7ab)

running 6 tests
test pre_move_checklist_blocks_unsafe_moves ... ok
test pre_move_checklist_defines_required_before_move_items ... ok
test pre_move_checklist_is_linked_and_integrity_checked ... ok
test pre_move_checklist_preserves_first_safe_candidate_limits ... ok
test pre_move_checklist_preserves_non_claims ... ok
test pre_move_checklist_requires_tests_before_moves ... ok

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
test base1_preview_gate_dry_run_refuses_missing_boot_inputs ... ok
test base1_preview_gate_dry_run_passes_when_boot_inputs_exist ... ok
test base1_preview_gate_exists_and_documents_boundary ... ok
test base1_preview_gate_reports_missing_bundle ... ok
test base1_preview_gate_execute_requires_confirmation ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.09s

     Running tests/base1_preview_inputs_script.rs (target/debug/deps/base1_preview_inputs_script-ddd98a6c618c2034)

running 5 tests
test base1_preview_inputs_avoids_mutating_tools ... ok
test base1_preview_inputs_passes_with_placeholder_files ... ok
test base1_preview_inputs_reports_missing_required_paths ... ok
test base1_preview_inputs_script_exists_and_documents_boundary ... ok
test base1_preview_inputs_warns_when_bundle_outside_build ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests/base1_preview_provenance_script.rs (target/debug/deps/base1_preview_provenance_script-79e0955e334e5fe4)

running 5 tests
test base1_preview_provenance_avoids_real_device_tools ... ok
test base1_preview_provenance_exists_and_documents_boundary ... ok
test base1_preview_provenance_refuses_bundle_outside_build ... ok
test base1_preview_provenance_reports_missing_bundle ... ok
test base1_preview_provenance_records_hashes_for_generated_bundle ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.42s

     Running tests/base1_preview_stack_runbook_docs.rs (target/debug/deps/base1_preview_stack_runbook_docs-5ff2c87ae2482a5b)

running 5 tests
test base1_index_links_preview_stack_runbook ... ok
test base1_preview_stack_runbook_exists ... ok
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

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.77s

     Running tests/base1_preview_stack_validation_report_docs.rs (target/debug/deps/base1_preview_stack_validation_report_docs-9d8c053d23b9c39f)

running 5 tests
test base1_preview_stack_validation_report_exists ... ok
test preview_stack_report_preserves_non_claims ... ok
test preview_stack_report_preserves_promotion_boundary ... ok
test preview_stack_report_records_scope_and_evidence ... ok
test validation_index_links_preview_stack_report ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/base1_preview_verify_script.rs (target/debug/deps/base1_preview_verify_script-09644db98194f29b)

running 6 tests
test base1_preview_verify_avoids_real_device_tools ... ok
test base1_preview_verify_exists_and_documents_boundary ... ok
test base1_preview_verify_passes_generated_stack_bundle ... ok
test base1_preview_verify_fails_after_bundle_drift ... ok
test base1_preview_verify_refuses_bundle_outside_build ... ok
test base1_preview_verify_reports_missing_bundle ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.25s

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
test base1_qemu_boot_check_dry_run_does_not_launch ... ok
test base1_qemu_boot_check_exists_and_documents_boundary ... ok
test base1_qemu_boot_check_execute_requires_confirmation ... ok
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

thread 'qemu_visual_boot_preview_script_help_documents_usage_and_boundaries' (108510) panicked at tests/base1_qemu_visual_boot_preview_script.rs:35:5:
--help should succeed
stdout:

stderr:
scripts/base1-qemu-visual-boot-preview.sh: 5: set: Illegal option -o pipefail

note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

---- qemu_visual_boot_preview_script_rejects_unknown_argument stdout ----

thread 'qemu_visual_boot_preview_script_rejects_unknown_argument' (108514) panicked at tests/base1_qemu_visual_boot_preview_script.rs:95:5:
stderr was: scripts/base1-qemu-visual-boot-preview.sh: 5: set: Illegal option -o pipefail


---- qemu_visual_boot_preview_script_requires_action stdout ----

thread 'qemu_visual_boot_preview_script_requires_action' (108516) panicked at tests/base1_qemu_visual_boot_preview_script.rs:75:5:
stderr was: scripts/base1-qemu-visual-boot-preview.sh: 5: set: Illegal option -o pipefail



failures:
    qemu_visual_boot_preview_script_help_documents_usage_and_boundaries
    qemu_visual_boot_preview_script_rejects_unknown_argument
    qemu_visual_boot_preview_script_requires_action

test result: FAILED. 5 passed; 3 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

error: test failed, to rerun pass `--test base1_qemu_visual_boot_preview_script`
```

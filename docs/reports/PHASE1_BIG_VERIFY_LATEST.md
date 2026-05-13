# Phase1 big verification latest report

Generated UTC: 2026-05-13T22:36:11Z
Source branch: edge/stable
Source commit: c1e8c3339da4f369bc8b8178816292116037b67b
Host: X200
Host kernel: Linux X200 6.8.0-110-generic #110trisquel35 SMP PREEMPT_DYNAMIC Wed Apr 15 21:32:36 UTC 2026 x86_64 x86_64 x86_64 GNU/Linux

## Result

| Field | Value |
| --- | --- |
| Result | failed |
| Steps | 15 |
| Failed steps | 2 |
| Verification start UTC | 2026-05-13T22:34:30Z |
| Verification end UTC | 2026-05-13T22:34:48Z |
| Source summary | build/phase1-big-verify/summary.env |
| Source report | build/phase1-big-verify/report.md |
| Source log | build/phase1-big-verify/phase1-big-verify.log |

## Failed step summary

```text
1634:FAILED: quality full
3100:FAILED: base1 reorg gate
```

## Verification report snapshot

# Phase1 big verification report

Start UTC: 2026-05-13T22:34:30Z
End UTC: 2026-05-13T22:34:48Z
Head: c1e8c3339da4f369bc8b8178816292116037b67b
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

Last 220 lines from the local verifier log:

```text
BASE1_B3_NON_CLAIM_INSTALLER=1
BASE1_B3_NON_CLAIM_HARDWARE=1
BASE1_B3_NON_CLAIM_DAILY_DRIVER=1
EOF

  printf 'result: %s\n' "$result"
  printf 'serial_log: %s\n' "$SERIAL_LOG"
  printf 'summary: %s\n' "$SUMMARY"
  printf 'non_claims: emulator-only; no installer; no hardware validation; no daily-driver claim\n'
  [ "$result" = pass ] || exit 1
}

if [ "$BUILD" = yes ]; then
  build_image
fi
if [ "$RUN" = yes ]; then
  run_visible
fi
if [ "$CHECK" = yes ]; then
  run_check
fi



failures:
    base1_b3_uefi_proof_help_documents_usage_marker_display_and_boundaries
    base1_b3_uefi_proof_searches_root_before_loading_font_and_keeps_menu_overlay

test result: FAILED. 4 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

error: test failed, to rerun pass `--test base1_b3_uefi_proof_script`
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

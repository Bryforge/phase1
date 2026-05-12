#[test]
fn b2_output_review_defines_scope_and_command() {
    let review = std::fs::read_to_string("docs/os/B2_DRY_RUN_ASSEMBLY_OUTPUT_REVIEW.md")
        .expect("B2 dry-run assembly output review");

    assert!(
        review.contains("Base1 B2 dry-run assembly output review"),
        "{review}"
    );
    assert!(
        review.contains("secret-redaction and safe-output review for `scripts/base1-b2-assembly-dry-run.sh --dry-run --profile <profile>`"),
        "{review}"
    );
    assert!(
        review.contains(
            "sh scripts/base1-b2-assembly-dry-run.sh --dry-run --profile x86_64-vm-validation"
        ),
        "{review}"
    );
}

#[test]
fn b2_output_review_lists_safe_output_fields() {
    let review = std::fs::read_to_string("docs/os/B2_DRY_RUN_ASSEMBLY_OUTPUT_REVIEW.md")
        .expect("B2 dry-run assembly output review");

    for text in [
        "command start/completion markers",
        "B2 readiness level",
        "`writes: no`",
        "`mutation: no`",
        "`network: no`",
        "selected profile",
        "machine architecture hint",
        "firmware hint",
        "profile assumptions",
        "image-builder preview status",
        "boot handoff preview status",
        "installer preview status",
        "recovery preview status",
        "rollback preview status",
        "validation bundle planned path",
        "explicit known limitations",
        "next validation step",
    ] {
        assert!(
            review.contains(text),
            "missing safe output field {text}: {review}"
        );
    }
}

#[test]
fn b2_output_review_lists_forbidden_output() {
    let review = std::fs::read_to_string("docs/os/B2_DRY_RUN_ASSEMBLY_OUTPUT_REVIEW.md")
        .expect("B2 dry-run assembly output review");

    for text in [
        "tokens",
        "private keys",
        "passwords",
        "credentials",
        "recovery codes",
        "GitHub, SSH, Apple ID, email, or cloud credentials",
        "environment variables",
        "full private logs",
        "private user files",
        "secret-bearing kernel command-line values",
        "mutable boot-loader configuration contents",
        "EFI variable contents",
        "partition table write commands",
        "package-manager install commands",
        "network request contents",
    ] {
        assert!(
            review.contains(text),
            "missing forbidden output text {text}: {review}"
        );
    }
}

#[test]
fn b2_output_review_preserves_source_output_observations() {
    let review = std::fs::read_to_string("docs/os/B2_DRY_RUN_ASSEMBLY_OUTPUT_REVIEW.md")
        .expect("B2 dry-run assembly output review");

    for text in [
        "It reads only the machine architecture through `uname -m` or `arch`.",
        "It checks only `/sys/firmware/efi` directory presence for a firmware hint.",
        "It does not print `/proc/cmdline`.",
        "It does not print environment variables.",
        "It does not print file contents from `/boot`, `/etc`, EFI variables, initramfs files, or partitions.",
        "It does not call network tools.",
        "It does not call package managers.",
        "It does not call mutating disk or boot-loader commands.",
        "release-candidate readiness explicitly unclaimed",
    ] {
        assert!(review.contains(text), "missing source-output observation {text}: {review}");
    }
}

#[test]
fn b2_output_review_preserves_review_checklist() {
    let review = std::fs::read_to_string("docs/os/B2_DRY_RUN_ASSEMBLY_OUTPUT_REVIEW.md")
        .expect("B2 dry-run assembly output review");

    for text in [
        "output avoids environment variables",
        "output avoids credentials and tokens",
        "output avoids private keys and recovery codes",
        "output avoids private file contents",
        "output avoids mutable boot configuration contents",
        "output avoids EFI variable contents",
        "output avoids secret-bearing kernel command-line values",
        "output prints `writes: no`",
        "output prints `mutation: no`",
        "output prints `network: no`",
        "output keeps B2 claims bounded to dry-run preview only",
        "script/test suite passes in CI or local validation",
    ] {
        assert!(
            review.contains(text),
            "missing output review checklist text {text}: {review}"
        );
    }
}

#[test]
fn b2_output_review_links_related_docs() {
    let review = std::fs::read_to_string("docs/os/B2_DRY_RUN_ASSEMBLY_OUTPUT_REVIEW.md")
        .expect("B2 dry-run assembly output review");

    for link in [
        "BOOT_READINESS_STATUS.md",
        "B2_DRY_RUN_ASSEMBLY_PLAN.md",
        "B2_DRY_RUN_ASSEMBLY_LIMITATIONS.md",
        "B2_DRY_RUN_ASSEMBLY_VALIDATION.md",
    ] {
        assert!(
            review.contains(link),
            "missing related doc link {link}: {review}"
        );
    }
}

#[test]
fn b2_output_review_is_linked_from_status_tracker() {
    let status = std::fs::read_to_string("docs/os/BOOT_READINESS_STATUS.md")
        .expect("boot readiness status tracker");

    assert!(
        status.contains("B2_DRY_RUN_ASSEMBLY_OUTPUT_REVIEW.md"),
        "{status}"
    );
    assert!(
        status.contains("B2 output is reviewed for secret redaction."),
        "{status}"
    );
}

#[test]
fn b2_output_review_preserves_non_claims() {
    let review = std::fs::read_to_string("docs/os/B2_DRY_RUN_ASSEMBLY_OUTPUT_REVIEW.md")
        .expect("B2 dry-run assembly output review");

    for text in [
        "does not make Base1 bootable",
        "installer-ready",
        "recovery-complete",
        "hardened",
        "VM-validated",
        "hardware-validated",
        "release-candidate ready",
        "daily-driver ready",
    ] {
        assert!(review.contains(text), "missing non-claim {text}: {review}");
    }
}

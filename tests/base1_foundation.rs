use std::fs;
use std::path::Path;

#[test]
fn base1_foundation_docs_exist() {
    for path in [
        "base1/README.md",
        "base1/SECURITY_MODEL.md",
        "base1/HARDWARE_TARGETS.md",
        "base1/PHASE1_COMPATIBILITY.md",
        "base1/ROADMAP.md",
        "base1/config/base1-secure-profile.toml",
        "base1/systemd/phase1-base1.service",
        "scripts/base1-preflight.sh",
        "scripts/base1-phase1-run.sh",
    ] {
        assert!(Path::new(path).is_file(), "missing Base1 file: {path}");
    }
}

#[test]
fn base1_security_model_preserves_phase1_host_boundary() {
    let model = read("base1/SECURITY_MODEL.md");
    assert_contains_all(
        &model,
        &[
            "Base1 should preserve host integrity when Phase1 fails",
            "Base1 treats Phase1 as a contained workload",
            "No host mutation from Phase1 except through explicit maintenance tools",
            "The project should not claim OpenBSD parity",
            "Phase1 must not be able to directly change",
        ],
    );
}

#[test]
fn base1_compatibility_contract_defaults_to_safe_mode() {
    let contract = read("base1/PHASE1_COMPATIBILITY.md");
    assert_contains_all(
        &contract,
        &[
            "BASE1_PHASE1_CONTRACT=0.1",
            "PHASE1_SAFE_MODE=1",
            "PHASE1_ALLOW_HOST_TOOLS=0",
            "Phase1 safe mode stays enabled",
            "Host-backed tools stay disabled",
        ],
    );
}

#[test]
fn base1_hardware_targets_include_raspberry_pi_and_x200() {
    let targets = read("base1/HARDWARE_TARGETS.md");
    assert_contains_all(
        &targets,
        &[
            "Raspberry Pi",
            "ThinkPad X200",
            "BASE1_HARDWARE_TARGET=raspberry-pi",
            "BASE1_HARDWARE_TARGET=x200",
            "Firewall default deny inbound",
        ],
    );
}

#[test]
fn base1_profile_is_secure_by_default() {
    let profile = read("base1/config/base1-secure-profile.toml");
    assert_contains_all(
        &profile,
        &[
            "profile = \"secure-default\"",
            "default_safe_mode = true",
            "default_allow_host_tools = false",
            "allow_passwordless_host_mutation = false",
            "default_inbound_network = \"deny\"",
            "ssh_default = \"disabled\"",
        ],
    );
}

#[test]
fn base1_preflight_is_non_destructive() {
    let script = read("scripts/base1-preflight.sh");
    assert_contains_all(
        &script,
        &[
            "non-destructive readiness checker",
            "no host changes were made",
            "PHASE1_SAFE_MODE=1",
            "PHASE1_ALLOW_HOST_TOOLS=0",
        ],
    );
    assert_not_contains_any(
        &script,
        &[
            " useradd ",
            " adduser ",
            " systemctl enable ",
            " mkfs.",
            " parted ",
            " sgdisk ",
            " iptables -A ",
            " nft add ",
        ],
    );
}

#[test]
fn base1_launcher_refuses_root_by_default() {
    let launcher = read("scripts/base1-phase1-run.sh");
    assert_contains_all(
        &launcher,
        &[
            "refusing to launch Phase1 as root",
            "BASE1_ALLOW_ROOT_PHASE1",
            "PHASE1_SAFE_MODE",
            "PHASE1_ALLOW_HOST_TOOLS",
            "exec \"$PHASE1_BIN\"",
        ],
    );
}

#[test]
fn base1_systemd_template_has_hardening_directives() {
    let unit = read("base1/systemd/phase1-base1.service");
    assert_contains_all(
        &unit,
        &[
            "User=phase1",
            "NoNewPrivileges=true",
            "ProtectSystem=strict",
            "PrivateTmp=true",
            "PrivateDevices=true",
            "PHASE1_SAFE_MODE=1",
            "PHASE1_ALLOW_HOST_TOOLS=0",
            "ReadWritePaths=/var/lib/phase1 /run/phase1",
        ],
    );
}

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"))
}

fn assert_contains_all(text: &str, needles: &[&str]) {
    for needle in needles {
        assert!(text.contains(needle), "missing {needle:?}");
    }
}

fn assert_not_contains_any(text: &str, needles: &[&str]) {
    for needle in needles {
        assert!(
            !text.contains(needle),
            "unexpected unsafe pattern {needle:?}"
        );
    }
}

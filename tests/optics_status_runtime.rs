use std::io::Write;
use std::process::{Command, Stdio};

fn run_phase1(input: &str) -> String {
    let mut child = Command::new(env!("CARGO_BIN_EXE_phase1"))
        .env("PHASE1_TEST_MODE", "1")
        .env("PHASE1_PERSISTENT_STATE", "0")
        .env("PHASE1_COOKED_INPUT", "1")
        .env("PHASE1_NO_COLOR", "1")
        .env("PHASE1_ASCII", "1")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn phase1");

    child
        .stdin
        .as_mut()
        .expect("stdin")
        .write_all(format!("\n{input}").as_bytes())
        .expect("write stdin");

    let output = child.wait_with_output().expect("phase1 output");
    format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    )
}

#[test]
fn optics_status_reports_preview_readiness_and_activation_gate() {
    let output = run_phase1("optics status\nexit\n");

    for required in [
        "phase1 wasi run",
        "plugin : optics",
        "runtime: phase1-wasi-lite",
        "sandbox: fs=virtual net=disabled host=blocked",
        "cap    : none",
        "args   : status",
        "OPTICS STATUS",
        "mode        : preview-only",
        "renderer    : rust-static-renderer",
        "top-rail    : ready-preview",
        "bottom-rail : ready-preview",
        "live-hud    : disabled",
        "activation  : explicit-gate-required",
        "input       : raw-command-preserved",
        "history     : unchanged",
        "parser      : unchanged",
        "not-compositor",
        "not-security-boundary",
        "not-crypto-enforcement",
        "not-system-integrity-guarantee",
        "not-base1-boot-environment",
        "status : ok",
        "exit   : 0",
    ] {
        assert!(output.contains(required), "missing {required:?}:\n{output}");
    }

    for forbidden in [
        "live-hud    : enabled",
        "runtime=wired",
        "security-boundary claimed",
        "crypto-enforcement claimed",
        "system-integrity-guarantee claimed",
        "base1 boot environment claimed",
        "host=enabled",
    ] {
        assert!(
            !output.contains(forbidden),
            "forbidden {forbidden:?}:\n{output}"
        );
    }
}

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
fn optics_preview_runs_as_read_only_wasi_lite_surface() {
    let output = run_phase1("optics preview\nexit\n");

    for required in [
        "phase1 wasi run",
        "plugin : optics",
        "runtime: phase1-wasi-lite",
        "sandbox: fs=virtual net=disabled host=blocked",
        "cap    : none",
        "args   : preview",
        "OPTICS PRO COMMAND SURFACE",
        "status      complete-read-only-fixture",
        "mode        preview-only",
        "runtime     phase1-wasi-lite",
        "renderer    rust-static-renderer",
        "profile     PRO",
        "codename    Optics",
        "channel     phase1 edge enabled",
        "PHASE UNIVERSE VISIBILITY",
        "origin=0/0",
        "route=ROOT",
        "axis=ROOT",
        "path=ROOT>0/0",
        "breadcrumb=ROOT",
        "trace=trace-preview",
        "safe-portal=planned",
        "rollback=available",
        "host-effect=none",
        "external-effect=none",
        "OPTICS HUD RAIL RENDER",
        "not-security-boundary",
        "not-crypto-enforcement",
        "not-base1-boot-environment",
        "not-live-phase-movement",
        "not-origin-mutation",
        "not-safe-portal-recovery-execution",
        "not-runtime-domain-mutation",
        "not-host-mutation",
        "not-external-effects",
        "status : ok",
        "exit   : 0",
    ] {
        assert!(output.contains(required), "missing {required:?}:\n{output}");
    }

    for forbidden in [
        "OPTICS PRO PREVIEW",
        "runtime     wired",
        "security-boundary claimed",
        "crypto-enforcement claimed",
        "base1 boot environment claimed",
        "host=enabled",
    ] {
        assert!(
            !output.contains(forbidden),
            "forbidden {forbidden:?}:\n{output}"
        );
    }
}

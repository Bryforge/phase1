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
        .write_all(format!("\n{input}\nexit\n").as_bytes())
        .expect("write stdin");

    let output = child.wait_with_output().expect("phase1 output");
    format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    )
}

fn assert_complete_preview_card(output: &str, expected_args: &str) {
    for required in [
        "phase1 wasi run",
        "plugin : optics",
        "runtime: phase1-wasi-lite",
        "sandbox: fs=virtual net=disabled host=blocked",
        "cap    : none",
        expected_args,
        "OPTICS PRO COMMAND SURFACE",
        "status      complete-read-only-fixture",
        "mode        preview-only",
        "COMMANDS",
        "optics help",
        "optics preview",
        "optics status",
        "optics rails",
        "optics device mobile",
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
}

#[test]
fn optics_preview_and_help_show_complete_read_only_card() {
    for (command, args) in [
        ("optics preview", "args   : preview"),
        ("optics help", "args   : help"),
    ] {
        let output = run_phase1(command);
        assert_complete_preview_card(&output, args);
        assert!(!output.contains("command not found"), "{output}");
    }
}

#[test]
fn optics_status_still_reports_renderer_contract() {
    let output = run_phase1("optics status");

    for required in [
        "OPTICS STATUS",
        "mode        : preview-only",
        "renderer    : rust-static-renderer",
        "top-rail    : ready-preview",
        "bottom-rail : ready-preview",
        "live-hud    : disabled",
        "activation  : explicit-gate-required",
        "OPTICS PRO SHELL RAIL CONTRACT",
        "A TOP RAIL",
        "B COMMAND RAIL",
        "C STATUS HUD",
        "D BOTTOM HUD",
        "origin=0/0",
        "route=ROOT",
        "trace=trace-preview",
        "host-effect=none",
        "external-effect=none",
        "status : ok",
    ] {
        assert!(output.contains(required), "missing {required:?}:\n{output}");
    }
}

#[test]
fn optics_device_profiles_are_all_routable() {
    for device in ["mobile", "laptop", "desktop", "terminal"] {
        let output = run_phase1(&format!("optics device {device}"));
        assert!(output.contains("OPTICS DEVICE PREVIEW"), "{output}");
        assert!(
            output.contains(&format!("device      : {device}")),
            "{output}"
        );
        assert!(output.contains(&format!("device={device}")), "{output}");
        assert!(output.contains("OPTICS HUD RAIL RENDER"), "{output}");
        assert!(output.contains("status : ok"), "{output}");
    }
}

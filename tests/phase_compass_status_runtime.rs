#[path = "../src/wasm.rs"]
mod wasm;

use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

fn run_phase1_script(script: &str) -> String {
    let mut child = Command::new(env!("CARGO_BIN_EXE_phase1"))
        .env("PHASE1_TEST_MODE", "1")
        .env("PHASE1_PERSISTENT_STATE", "0")
        .env("PHASE1_COOKED_INPUT", "1")
        .env("PHASE1_NO_COLOR", "1")
        .env("PHASE1_ASCII", "1")
        .env("PHASE1_BOOT_SELECTOR", "1")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn phase1");

    child
        .stdin
        .as_mut()
        .expect("stdin")
        .write_all(format!("\n{script}\nexit\n").as_bytes())
        .expect("write phase1 input");

    let output = child.wait_with_output().expect("phase1 output");
    let combined = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    assert!(output.status.success(), "phase1 failed:\n{combined}");
    combined
}

fn assert_compass_core(output: &str) {
    for required in [
        "PHASE COMPASS",
        "mode=status-only",
        "mutation=disabled",
        "origin=0/0",
        "root-anchor=ROOT",
        "current-route=ROOT",
        "current-axis=ROOT",
        "path=ROOT>0/0",
        "breadcrumb=ROOT",
        "trace-id=trace-preview",
        "safe-portal=planned",
        "rollback-target=available",
        "operator-intent=explicit",
        "claim-boundary=phase-compass-status-only",
        "ROOT DIRECTION MAP",
        "L/NUM  <----  ROOT  ---->  R/NUM",
    ] {
        assert!(output.contains(required), "missing {required:?}:\n{output}");
    }
}

#[test]
fn phase_compass_wasi_plugin_runs_from_default_plugins_directory() {
    let output = wasm::execute_plugin(Path::new("plugins"), "phase", &["whereami".to_string()]);

    for required in [
        "phase1 wasi run",
        "plugin : phase",
        "runtime: phase1-wasi-lite",
        "sandbox: fs=virtual net=disabled host=blocked",
        "cap    : none",
        "args   : whereami",
        "status : ok",
        "exit   : 0",
    ] {
        assert!(output.contains(required), "missing {required:?}:\n{output}");
    }
    assert_compass_core(&output);
}

#[test]
fn phase_whereami_runs_inside_phase1_by_default() {
    let output = run_phase1_script("phase whereami");
    assert_compass_core(&output);
}

#[test]
fn phase_compass_path_and_map_aliases_are_available() {
    for command in ["phase compass", "phase path", "phase map"] {
        let output = run_phase1_script(command);
        assert_compass_core(&output);
    }
}

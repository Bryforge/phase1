#[path = "../src/wasm.rs"]
mod wasm;

use std::fs;
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

#[test]
fn phase_proof_wasi_plugin_runs_from_default_plugins_directory() {
    let output = wasm::execute_plugin(Path::new("plugins"), "phase-proof", &[]);

    for required in [
        "phase1 wasi run",
        "plugin : phase-proof",
        "runtime: phase1-wasi-lite",
        "sandbox: fs=virtual net=disabled host=blocked",
        "PHASE UNIVERSE PROOF PROGRAM",
        "status=default-runtime-surface",
        "mode=proof-display",
        "execution-state=not-executed",
        "mutation=disabled",
        "host-effect=none",
        "external-effect=none",
        "ROUTE MODEL",
        "LaTeX: D = {u,d,L,R}",
        "LaTeX: O = 0/0",
        "example=ROOT>R/3>u/1",
        "STATE MODEL",
        "TRANSITION MODEL",
        "HEALTH MODEL",
        "LOCK MODEL",
        "SAFETY INVARIANTS",
        "I1 ROOT must remain reachable from every valid Phase state",
        "I8 simulated domain networks must not mutate host networking",
        "PROOF LADDER",
        "claim -> contract -> fixture -> test -> demo -> report -> review -> promotion",
        "PHASE1 UNIVERSE",
        "rooted traceable recoverable testable provable",
        "claim-boundary=proof-display-only",
        "status : ok",
        "exit   : 0",
    ] {
        assert!(output.contains(required), "missing {required:?}:\n{output}");
    }
}

#[test]
fn phase_proof_plugin_is_runnable_as_phase1_command_by_default() {
    let combined = run_phase1_script("phase-proof");

    assert!(
        combined.contains("PHASE UNIVERSE PROOF PROGRAM"),
        "{combined}"
    );
    assert!(
        combined.contains("LaTeX: P = ROOT > r_1 > r_2 > ... > r_n"),
        "{combined}"
    );
    assert!(
        combined.contains("claim-boundary=proof-display-only"),
        "{combined}"
    );
    assert!(
        !combined.contains("command not found: phase-proof"),
        "{combined}"
    );
}

#[test]
fn phase_proof_fyr_fixture_exists_and_is_executable_source() {
    let fixture = fs::read_to_string("docs/fyr/examples/phase-proof.fyr")
        .expect("phase proof Fyr fixture should exist");

    for required in [
        "fn main() -> i32",
        "PHASE UNIVERSE FYR PROOF PROGRAM",
        "D = {u,d,L,R}",
        "O = 0/0",
        "P = ROOT > r_1 > r_2 > ... > r_n",
        "T(S,r) -> S_prime",
        "breadcrumb(S_prime) = breadcrumb(S) > r",
        "claim -> contract -> fixture -> test -> demo -> report -> review -> promotion",
        "rooted traceable recoverable testable provable",
        "assert_eq(42, 42)",
        "return 0",
    ] {
        assert!(
            fixture.contains(required),
            "missing Fyr proof marker: {required}"
        );
    }
}

#[test]
fn phase_proof_fyr_program_runs_inside_phase1() {
    let source = concat!(
        "fn main() -> i32 { ",
        "print(\"PHASE UNIVERSE FYR PROOF PROGRAM\"); ",
        "print(\"D = {u,d,L,R}\"); ",
        "print(\"O = 0/0\"); ",
        "print(\"P = ROOT > r_1 > r_2 > ... > r_n\"); ",
        "print(\"example = ROOT>R/3>u/1\"); ",
        "print(\"T(S,r) -> S_prime\"); ",
        "print(\"breadcrumb(S_prime) = breadcrumb(S) > r\"); ",
        "print(\"claim -> contract -> fixture -> test -> demo -> report -> review -> promotion\"); ",
        "print(\"rooted traceable recoverable testable provable\"); ",
        "assert_eq(42, 42); ",
        "assert(true); ",
        "return 0; }"
    );
    let script = format!(
        "echo '{source}' > phase-proof.fyr\nfyr check phase-proof.fyr\nfyr run phase-proof.fyr"
    );
    let combined = run_phase1_script(&script);

    for required in [
        "fyr check: ok phase-proof.fyr",
        "PHASE UNIVERSE FYR PROOF PROGRAM",
        "D = {u,d,L,R}",
        "O = 0/0",
        "P = ROOT > r_1 > r_2 > ... > r_n",
        "example = ROOT>R/3>u/1",
        "T(S,r) -> S_prime",
        "breadcrumb(S_prime) = breadcrumb(S) > r",
        "claim -> contract -> fixture -> test -> demo -> report -> review -> promotion",
        "rooted traceable recoverable testable provable",
    ] {
        assert!(
            combined.contains(required),
            "missing {required:?}:\n{combined}"
        );
    }

    assert!(
        !combined.contains("command not found"),
        "unexpected missing command:\n{combined}"
    );
}

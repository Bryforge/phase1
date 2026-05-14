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
fn optics_rails_execution_shows_hud_rail_render_not_completion_only() {
    let output = run_phase1("optics rails\nexit\n");

    for required in [
        "phase1 wasi run",
        "plugin : optics",
        "args   : rails",
        "OPTICS HUD RAIL RENDER",
        "TOP product=Phase1 channel=edge profile=PRO",
        "CENTER role=command-output chrome=none-permanent",
        "BOT color=bright-blue input=active mutation=none",
        "not-security-boundary",
        "not-crypto-enforcement",
        "not-system-integrity-guarantee",
    ] {
        assert!(output.contains(required), "missing {required:?}:\n{output}");
    }
}

#[test]
fn optics_registry_aliases_execute_preview_surfaces() {
    let output = run_phase1("pro\nhudrails\nexit\n");

    for required in [
        "phase1 wasi run",
        "plugin : optics",
        "OPTICS PRO PREVIEW",
        "OPTICS HUD RAIL RENDER",
        "TOP product=Phase1 channel=edge profile=PRO",
        "BOT color=bright-blue input=active mutation=none",
        "status : ok",
        "exit   : 0",
    ] {
        assert!(output.contains(required), "missing {required:?}:\n{output}");
    }

    for forbidden in [
        "command not found: pro",
        "command not found: hudrails",
        "security-boundary claimed",
        "crypto-enforcement claimed",
        "system-integrity-guarantee claimed",
    ] {
        assert!(
            !output.contains(forbidden),
            "forbidden {forbidden:?}:\n{output}"
        );
    }
}

#[test]
fn completion_routes_remain_discovery_only() {
    let output = run_phase1("complete opt\ncomplete pro\ncomplete hud\nexit\n");

    assert!(output.contains("optics"), "{output}");
    assert!(output.contains("pro"), "{output}");
    assert!(output.contains("hudrails"), "{output}");
    assert!(
        !output.contains("OPTICS HUD RAIL RENDER"),
        "completion should not execute rails: {output}"
    );
}

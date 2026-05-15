use std::io::Write;
use std::process::{Command, Stdio};

fn run_phase1(input: &str) -> String {
    let mut child = Command::new(env!("CARGO_BIN_EXE_phase1"))
        .env("PHASE1_TEST_MODE", "1")
        .env("PHASE1_PERSISTENT_STATE", "0")
        .env("PHASE1_COOKED_INPUT", "1")
        .env("PHASE1_NO_COLOR", "1")
        .env("PHASE1_ASCII", "1")
        .env("PHASE1_OPTICS_PRO", "1")
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

fn normalize_terminal_output(raw: &str) -> String {
    let mut out = String::with_capacity(raw.len());
    let mut chars = raw.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '\u{1b}' && chars.peek() == Some(&'[') {
            chars.next();
            for code in chars.by_ref() {
                if code.is_ascii_alphabetic() {
                    break;
                }
            }
        } else if ch != '\r' {
            out.push(ch);
        }
    }
    out
}

fn has_blank_line_between_command_and_status(output: &str) -> bool {
    let normalized = normalize_terminal_output(output);
    let lines = normalized.lines().collect::<Vec<_>>();
    lines.windows(3).any(|window| {
        window[0].contains("phase1://root ~ >")
            && window[1].trim().is_empty()
            && window[2].contains("C STATUS HUD")
    })
}

#[test]
fn optics_pro_shell_frame_is_default_active_input_surface() {
    let output = run_phase1("exit\n");

    for required in [
        "A TOP RAIL",
        "B COMMAND RAIL",
        "phase1://root ~ >",
        "C STATUS HUD",
        "D BOTTOM HUD",
        "result=ready mutation=none integrity=not-checked crypto=chain-planned",
        "input=active command=none task=idle warning=none copy-safe=raw-command-preserved",
        "shutdown: phase1",
    ] {
        assert!(output.contains(required), "missing {required:?}:\n{output}");
    }

    assert!(
        has_blank_line_between_command_and_status(&output),
        "B command rail must have a blank readability line before C: {output}"
    );

    let live_ops = output.find("LIVE OPS");
    let top_rail = output.find("A TOP RAIL").expect("top rail should exist");
    if let Some(live_ops) = live_ops {
        assert!(
            live_ops < top_rail,
            "old live ops card may appear only before active Optics frame: {output}"
        );
    }
}

#[test]
fn legacy_shell_ui_escape_hatch_preserves_old_prompt() {
    let mut child = Command::new(env!("CARGO_BIN_EXE_phase1"))
        .env("PHASE1_TEST_MODE", "1")
        .env("PHASE1_PERSISTENT_STATE", "0")
        .env("PHASE1_COOKED_INPUT", "1")
        .env("PHASE1_NO_COLOR", "1")
        .env("PHASE1_ASCII", "1")
        .env("PHASE1_LEGACY_SHELL_UI", "1")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn phase1");

    child
        .stdin
        .as_mut()
        .expect("stdin")
        .write_all(b"\nexit\n")
        .expect("write stdin");

    let output = child.wait_with_output().expect("phase1 output");
    let output = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let normalized = normalize_terminal_output(&output);

    assert!(
        normalized.contains("phase1://root ~ edge safe trust")
            || normalized.contains("phase1://root ~ ❯"),
        "{output}"
    );
    assert!(!normalized.contains("A TOP RAIL"), "{output}");
}

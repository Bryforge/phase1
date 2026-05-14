use std::fs;
use std::io::Write;
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

static RUN_COUNTER: AtomicU64 = AtomicU64::new(0);

fn run_phase1(script: &str) -> String {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_nanos())
        .unwrap_or(0);
    let seq = RUN_COUNTER.fetch_add(1, Ordering::Relaxed);
    let run_dir = std::env::temp_dir().join(format!(
        "phase1-fyr-f3-expr-{}-{nonce}-{seq}",
        std::process::id()
    ));

    let _ = fs::remove_dir_all(&run_dir);
    fs::create_dir_all(&run_dir).expect("create run dir");

    let mut child = Command::new(env!("CARGO_BIN_EXE_phase1"))
        .current_dir(&run_dir)
        .env("PHASE1_NO_COLOR", "1")
        .env("PHASE1_ASCII", "1")
        .env("PHASE1_COOKED_INPUT", "1")
        .env("PHASE1_BLEEDING_EDGE", "1")
        .env("PHASE1_MOBILE_MODE", "0")
        .env("PHASE1_DEVICE_MODE", "desktop")
        .env_remove("PHASE1_THEME")
        .env_remove("PHASE1_ALLOW_HOST_TOOLS")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn phase1");

    child
        .stdin
        .as_mut()
        .expect("stdin")
        .write_all(format!("\n{script}").as_bytes())
        .expect("write script");

    let output = child.wait_with_output().expect("wait phase1");
    let _ = fs::remove_dir_all(&run_dir);

    let mut combined = String::new();
    combined.push_str(&String::from_utf8_lossy(&output.stdout));
    combined.push_str(&String::from_utf8_lossy(&output.stderr));
    assert!(output.status.success(), "phase1 failed:\n{combined}");
    combined
}

#[test]
fn fyr_f3_supports_nested_integer_expression_groups() {
    let output = run_phase1(
        "echo 'fn main() -> i32 { let answer = ((6 + 8) * (9 - 3)) / 2; assert_eq(answer, 42); return answer; }' > expr.fyr\nfyr check expr.fyr\nfyr build expr.fyr\nexit\n",
    );

    assert!(output.contains("fyr check: ok expr.fyr"), "{output}");
    assert!(
        output.contains("status  : dry-run artifact ready"),
        "{output}"
    );
}

#[test]
fn fyr_f3_reports_division_by_zero_in_let_expression() {
    let output = run_phase1(
        "echo 'fn main() -> i32 { let bad = 10 / (5 - 5); assert_eq(bad, 0); return bad; }' > bad.fyr\nfyr check bad.fyr\nexit\n",
    );

    assert!(output.contains("division by zero"), "{output}");
}

#[test]
fn fyr_f3_reports_missing_right_hand_integer_operand() {
    let output = run_phase1(
        "echo 'fn main() -> i32 { let bad = 40 + ; assert_eq(bad, 40); return bad; }' > bad.fyr\nfyr check bad.fyr\nexit\n",
    );

    assert!(
        output.contains("expected integer expression") || output.contains("expected integer let binding value"),
        "{output}"
    );
}

#[test]
fn fyr_f3_boolean_operator_precedence_remains_deterministic() {
    let output = run_phase1(
        "echo 'fn main() -> i32 { let answer = 42; if (answer > 40 && answer < 50) { assert_eq(answer, 42); return answer; } return 0; }' > bool.fyr\nfyr check bool.fyr\nfyr build bool.fyr\nexit\n",
    );

    assert!(output.contains("fyr check: ok bool.fyr"), "{output}");
    assert!(
        output.contains("status  : dry-run artifact ready"),
        "{output}"
    );
}

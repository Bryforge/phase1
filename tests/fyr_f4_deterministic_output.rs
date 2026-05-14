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
        "phase1-fyr-f4-deterministic-{}-{nonce}-{seq}",
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
        .env("PHASE1_ALLOW_HOST_TOOLS", "1")
        .env_remove("PHASE1_THEME")
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

fn fyr_relevant_lines(output: &str) -> Vec<String> {
    output
        .lines()
        .map(str::trim)
        .filter(|line| {
            line.starts_with("fyr ")
                || line.starts_with("package :")
                || line.starts_with("source  :")
                || line.starts_with("ast     :")
                || line.starts_with("backend :")
                || line.starts_with("host    :")
                || line.starts_with("status  :")
                || line.starts_with("test    :")
                || line.starts_with("passed  :")
                || line.starts_with("failed  :")
                || *line == "deterministic run ok"
        })
        .map(ToString::to_string)
        .collect()
}

fn assert_same_fyr_output(script: &str) {
    let first = fyr_relevant_lines(&run_phase1(script));
    let second = fyr_relevant_lines(&run_phase1(script));
    assert_eq!(first, second, "Fyr output should be deterministic");
}

#[test]
fn fyr_build_output_is_deterministic() {
    assert_same_fyr_output("fyr init app\nfyr build app\nexit\n");
}

#[test]
fn fyr_run_output_is_deterministic() {
    assert_same_fyr_output(
        "echo 'fn main() -> i32 { print(\"deterministic run ok\"); return 0; }' > run.fyr\nfyr check run.fyr\nfyr run run.fyr\nexit\n",
    );
}

#[test]
fn fyr_test_output_is_deterministic() {
    assert_same_fyr_output(
        "fyr init app\necho 'fn main() -> i32 { assert_eq(21 + 21, 42); return 0; }' > app/tests/math.fyr\nfyr test app\nexit\n",
    );
}

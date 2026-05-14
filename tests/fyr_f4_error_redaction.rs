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
        "phase1-fyr-f4-errors-{}-{nonce}-{seq}",
        std::process::id()
    ));

    let _ = fs::remove_dir_all(&run_dir);
    fs::create_dir_all(&run_dir).expect("create run dir");
    let host_path_marker = run_dir.display().to_string();

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
    assert!(
        !combined.contains(&host_path_marker),
        "Fyr diagnostics leaked host run dir {host_path_marker}:\n{combined}"
    );
    combined
}

fn assert_redacted(output: &str) {
    for forbidden in [
        "/tmp/",
        "phase1-fyr-f4-errors",
        "CARGO_BIN_EXE_phase1",
        "PHASE1_ALLOW_HOST_TOOLS",
        "cargo ",
        "rustc ",
        "bash:",
        "sh:",
        "http://",
        "https://",
    ] {
        assert!(
            !output.contains(forbidden),
            "diagnostic should not contain {forbidden:?}:\n{output}"
        );
    }
}

#[test]
fn fyr_check_error_is_target_scoped_and_redacted() {
    let output = run_phase1(
        "echo 'fn main() -> i32 { print(\"unterminated); return 0; }' > bad.fyr\nfyr check bad.fyr\nexit\n",
    );

    assert!(output.contains("fyr check: bad.fyr:"), "{output}");
    assert!(output.contains("unterminated string literal"), "{output}");
    assert_redacted(&output);
}

#[test]
fn fyr_package_error_is_package_scoped_and_redacted() {
    let output = run_phase1("fyr check ghost_package\nexit\n");

    assert!(output.contains("fyr check: ghost_package:"), "{output}");
    assert!(output.contains("missing package manifest"), "{output}");
    assert_redacted(&output);
}

#[test]
fn fyr_test_error_stays_inside_vfs_package_scope() {
    let output = run_phase1(
        "fyr init app\necho 'fn main() -> i32 { let bad = 10 / (5 - 5); return bad; }' > app/tests/bad.fyr\nfyr test app\nexit\n",
    );

    assert!(output.contains("app/tests/bad.fyr"), "{output}");
    assert!(output.contains("division by zero"), "{output}");
    assert_redacted(&output);
}

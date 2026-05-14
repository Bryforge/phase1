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
        "phase1-fyr-f4-runtime-{}-{nonce}-{seq}",
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

fn assert_no_host_tool_output(output: &str) {
    for forbidden in [
        "cargo run",
        "cargo build",
        "rustc ",
        "sh:",
        "bash:",
        "network",
        "download",
    ] {
        assert!(
            !output.to_lowercase().contains(&forbidden.to_lowercase()),
            "unexpected host-tool marker {forbidden:?} in output:\n{output}"
        );
    }
}

#[test]
fn fyr_build_reports_host_none_even_when_host_tools_are_allowed() {
    let output = run_phase1("fyr init app\nfyr build app\nexit\n");

    assert!(output.contains("fyr build"), "{output}");
    assert!(output.contains("package : app"), "{output}");
    assert!(output.contains("source  : app/src/main.fyr"), "{output}");
    assert!(output.contains("backend : seed/interpreted"), "{output}");
    assert!(output.contains("host    : none"), "{output}");
    assert!(
        output.contains("status  : dry-run artifact ready"),
        "{output}"
    );
    assert_no_host_tool_output(&output);
}

#[test]
fn fyr_run_uses_vfs_source_without_host_tool_markers() {
    let output = run_phase1(
        "echo 'fn main() -> i32 { print(\"f4 runtime stays vfs only\"); return 0; }' > safe.fyr\nfyr check safe.fyr\nfyr run safe.fyr\nexit\n",
    );

    assert!(output.contains("fyr check: ok safe.fyr"), "{output}");
    assert!(output.contains("f4 runtime stays vfs only"), "{output}");
    assert_no_host_tool_output(&output);
}

#[test]
fn fyr_test_runs_vfs_package_tests_without_host_tool_markers() {
    let output = run_phase1(
        "fyr init app\necho 'fn main() -> i32 { assert_eq(21 + 21, 42); return 0; }' > app/tests/math.fyr\nfyr test app\nexit\n",
    );

    assert!(output.contains("test    : app/tests/math.fyr ok"), "{output}");
    assert!(output.contains("failed  : 0"), "{output}");
    assert!(output.contains("status  : ok"), "{output}");
    assert_no_host_tool_output(&output);
}

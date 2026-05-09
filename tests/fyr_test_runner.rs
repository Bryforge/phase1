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
        "phase1-fyr-test-{}-{nonce}-{seq}",
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
fn fyr_test_package_runs_seed_tests() {
    let output = run_phase1("fyr init app\nfyr test app\nexit\n");

    assert!(output.contains("fyr test"), "{output}");
    assert!(output.contains("package : app"), "{output}");
    assert!(output.contains("tests   : 1"), "{output}");
    assert!(output.contains("passed  : 1"), "{output}");
    assert!(output.contains("failed  : 0"), "{output}");
    assert!(output.contains("status  : ok"), "{output}");
}

#[test]
fn fyr_test_reports_missing_package_manifest() {
    let output = run_phase1("mkdir app\nmkdir app/tests\nfyr test app\nexit\n");

    assert!(
        output.contains("fyr test: app: missing package manifest app/fyr.toml"),
        "{output}"
    );
}

#[test]
fn fyr_test_reports_each_test_file() {
    let output = run_phase1("fyr init app\nfyr test app\nexit\n");

    assert!(
        output.contains("test    : app/tests/smoke.fyr ok"),
        "{output}"
    );
    assert!(output.contains("tests   : 1"), "{output}");
    assert!(output.contains("passed  : 1"), "{output}");
    assert!(output.contains("failed  : 0"), "{output}");
    assert!(output.contains("status  : ok"), "{output}");
}

#[test]
fn fyr_test_reports_failed_test_file_diagnostics() {
    let output = run_phase1(
        "fyr init app\necho 'print(\"missing entry\");' > app/tests/bad.fyr\nfyr test app\nexit\n",
    );

    assert!(
        output.contains("test    : app/tests/bad.fyr failed: missing fn main entry point"),
        "{output}"
    );
    assert!(
        output.contains("test    : app/tests/smoke.fyr ok"),
        "{output}"
    );
    assert!(output.contains("tests   : 2"), "{output}");
    assert!(output.contains("passed  : 1"), "{output}");
    assert!(output.contains("failed  : 1"), "{output}");
    assert!(output.contains("status  : failed"), "{output}");
}

#[test]
fn fyr_test_assert_eq_passes() {
    let output = run_phase1(
        "fyr init app\necho 'fn main() -> i32 { assert_eq(1, 1); return 0; }' > app/tests/assert_ok.fyr\nfyr test app\nexit\n",
    );

    assert!(
        output.contains("test    : app/tests/assert_ok.fyr ok"),
        "{output}"
    );
    assert!(output.contains("passed  : 2"), "{output}");
    assert!(output.contains("failed  : 0"), "{output}");
    assert!(output.contains("status  : ok"), "{output}");
}

#[test]
fn fyr_test_assert_eq_failure_is_reported() {
    let output = run_phase1(
        "fyr init app\necho 'fn main() -> i32 { assert_eq(1, 2); return 0; }' > app/tests/assert_fail.fyr\nfyr test app\nexit\n",
    );

    assert!(
        output.contains("test    : app/tests/assert_fail.fyr failed: assertion failed: 1 != 2"),
        "{output}"
    );
    assert!(output.contains("passed  : 1"), "{output}");
    assert!(output.contains("failed  : 1"), "{output}");
    assert!(output.contains("status  : failed"), "{output}");
}

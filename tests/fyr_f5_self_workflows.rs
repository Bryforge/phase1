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
        "phase1-fyr-f5-self-{}-{nonce}-{seq}",
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

fn assert_no_host_markers(output: &str) {
    for forbidden in ["cargo ", "rustc ", "bash:", "sh:", "http://", "https://", "network"] {
        assert!(
            !output.to_lowercase().contains(&forbidden.to_lowercase()),
            "unexpected host marker {forbidden:?}:\n{output}"
        );
    }
}

fn assert_contains(path: &str, needle: &str) {
    let text = fs::read_to_string(path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"));
    assert!(text.contains(needle), "{path} should contain: {needle}");
}

#[test]
fn fyr_self_reports_current_surface_without_host_markers() {
    let output = run_phase1("fyr self\nexit\n");

    assert!(output.to_lowercase().contains("fyr"), "{output}");
    assert!(
        output.to_lowercase().contains("phase1")
            || output.to_lowercase().contains("self")
            || output.to_lowercase().contains("vfs"),
        "{output}"
    );
    assert_no_host_markers(&output);
}

#[test]
fn fyr_inspection_workflows_doc_preserves_f5_non_claims() {
    let path = "docs/fyr/SELF_WORKFLOWS.md";

    assert_contains(path, "Status: active F5 planning and evidence document");
    assert_contains(path, "Repository inspection workflows are still planned");
    assert_contains(path, "operate on VFS fixtures first");
    assert_contains(path, "avoid host shell access");
    assert_contains(path, "avoid network access");
    assert_contains(path, "F5 can be marked complete only when those workflows are implemented, tested, documented");
}

#[test]
fn fyr_inspection_workflows_doc_lists_target_workflows() {
    let path = "docs/fyr/SELF_WORKFLOWS.md";

    for item in [
        "Repository manifest reader.",
        "Documentation consistency helper.",
        "Checkpoint metadata helper.",
        "Public status reader or documented deferral.",
        "Fixture-based validation helper.",
    ] {
        assert_contains(path, item);
    }
}

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
        "phase1-fyr-f4-vfs-{}-{nonce}-{seq}",
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
    for forbidden in [
        "cargo ", "rustc ", "bash:", "sh:", "http://", "https://", "download", "network",
    ] {
        assert!(
            !output.to_lowercase().contains(&forbidden.to_lowercase()),
            "unexpected host marker {forbidden:?}:\n{output}"
        );
    }
}

#[test]
fn fyr_new_cat_check_run_stays_inside_vfs() {
    let output = run_phase1(
        "fyr new hello\nfyr cat hello.fyr\nfyr check hello.fyr\nfyr run hello.fyr\nexit\n",
    );

    assert!(output.contains("fyr new: created hello.fyr"), "{output}");
    assert!(output.contains("fn main() -> i32"), "{output}");
    assert!(output.contains("fyr check: ok hello.fyr"), "{output}");
    assert!(output.contains("Hello, hacker!"), "{output}");
    assert_no_host_markers(&output);
}

#[test]
fn fyr_init_creates_vfs_package_that_checks_builds_tests_and_runs() {
    let output = run_phase1("fyr init app\nfyr check app\nfyr build app\nfyr test app\nfyr run app/src/main.fyr\nexit\n");

    assert!(output.contains("fyr init: created package app"), "{output}");
    assert!(output.contains("manifest: app/fyr.toml"), "{output}");
    assert!(output.contains("main    : app/src/main.fyr"), "{output}");
    assert!(output.contains("tests   : app/tests/smoke.fyr"), "{output}");
    assert!(
        output.contains("fyr check: ok app/src/main.fyr"),
        "{output}"
    );
    assert!(output.contains("package : app"), "{output}");
    assert!(output.contains("backend : seed/interpreted"), "{output}");
    assert!(output.contains("host    : none"), "{output}");
    assert!(
        output.contains("test    : app/tests/smoke.fyr ok"),
        "{output}"
    );
    assert!(output.contains("Hello from Fyr package"), "{output}");
    assert_no_host_markers(&output);
}

#[test]
fn fyr_new_refuses_to_overwrite_existing_vfs_file() {
    let output = run_phase1("fyr new hello\nfyr new hello\nfyr cat hello.fyr\nexit\n");

    assert!(output.contains("fyr new: created hello.fyr"), "{output}");
    assert!(
        output.contains("fyr new: hello.fyr already exists"),
        "{output}"
    );
    assert!(output.contains("Hello, hacker!"), "{output}");
    assert_no_host_markers(&output);
}

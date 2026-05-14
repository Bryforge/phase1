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
        "phase1-fyr-f3-package-{}-{nonce}-{seq}",
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
fn fyr_f3_reports_missing_package_manifest() {
    let output = run_phase1("fyr check ghost_package\nexit\n");

    assert!(
        output.contains("fyr check: ghost_package: missing package manifest ghost_package/fyr.toml"),
        "{output}"
    );
}

#[test]
fn fyr_f3_reports_missing_package_main() {
    let output = run_phase1("mkdir app\necho 'name = \"app\"' > app/fyr.toml\nfyr check app\nexit\n");

    assert!(
        output.contains("fyr check: app: missing package main app/src/main.fyr"),
        "{output}"
    );
}

#[test]
fn fyr_f3_reports_duplicate_package_main() {
    let output = run_phase1(
        "fyr init app\necho 'fn main() -> i32 { return 0; }' > app/src/extra.fyr\nfyr check app\nexit\n",
    );

    assert!(output.contains("fyr check: app: duplicate fn main"), "{output}");
}

#[test]
fn fyr_f3_build_surfaces_package_source_path() {
    let output = run_phase1("fyr init app\nfyr build app\nexit\n");

    assert!(output.contains("package : app"), "{output}");
    assert!(output.contains("source  : app/src/main.fyr"), "{output}");
    assert!(output.contains("backend : seed/interpreted"), "{output}");
    assert!(output.contains("host    : none"), "{output}");
}

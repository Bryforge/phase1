use std::fs;
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

static RUN_COUNTER: AtomicU64 = AtomicU64::new(0);

fn temp_dir(name: &str) -> std::path::PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_nanos())
        .unwrap_or(0);
    let seq = RUN_COUNTER.fetch_add(1, Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!("phase1-{name}-{nonce}-{seq}"));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("create temp dir");
    dir
}

fn run_script(args: &[&str]) -> (bool, String) {
    let output = Command::new("sh")
        .arg("scripts/phase1-integrity-verify.sh")
        .args(args)
        .output()
        .expect("run integrity script");
    let text = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    (output.status.success(), text)
}

#[test]
fn integrity_verify_script_exists_and_documents_read_only_scope() {
    let script = fs::read_to_string("scripts/phase1-integrity-verify.sh")
        .expect("integrity verifier script should exist");

    for required in [
        "Read-only SHA-256 integrity report helper.",
        "--manifest <file>",
        "--file <path>",
        "read-only",
        "no repair",
        "no deletion",
        "no manifest rewrite",
        "local files only",
    ] {
        assert!(script.contains(required), "missing {required:?}: {script}");
    }
}

#[test]
fn integrity_verify_file_reports_sha256_without_mutating_file() {
    let dir = temp_dir("integrity-file");
    let file = dir.join("sample.txt");
    fs::write(&file, "phase1\n").expect("write sample");

    let before = fs::read(&file).expect("read before");
    let path = file.to_string_lossy().to_string();
    let (ok, output) = run_script(&["--file", &path]);
    let after = fs::read(&file).expect("read after");
    let _ = fs::remove_dir_all(&dir);

    assert!(ok, "script should pass: {output}");
    assert_eq!(before, after, "script must not mutate target file");
    assert!(output.contains("path:"), "{output}");
    assert!(
        output.contains("sha256: 583c40a164e83290fdbf230004b813d4782ee3a128aba11bfeddb2c793ffbc3c"),
        "{output}"
    );
    assert!(output.contains("result: ok"), "{output}");
}

#[test]
fn integrity_verify_manifest_reports_changed_without_repair() {
    let dir = temp_dir("integrity-manifest");
    let file = dir.join("sample.txt");
    let manifest = dir.join("manifest.sha256");
    fs::write(&file, "changed\n").expect("write changed sample");
    fs::write(
        &manifest,
        format!(
            "583c40a164e83290fdbf230004b813d4782ee3a128aba11bfeddb2c793ffbc3c  {}\n",
            file.display()
        ),
    )
    .expect("write manifest");

    let before = fs::read(&file).expect("read before");
    let manifest_path = manifest.to_string_lossy().to_string();
    let (ok, output) = run_script(&["--manifest", &manifest_path]);
    let after = fs::read(&file).expect("read after");
    let _ = fs::remove_dir_all(&dir);

    assert!(!ok, "changed manifest should fail closed: {output}");
    assert_eq!(before, after, "script must not repair changed target file");
    assert!(output.contains("changed:"), "{output}");
    assert!(output.contains("failures: 1"), "{output}");
    assert!(output.contains("result: changed"), "{output}");
}

#[test]
fn integrity_verify_manifest_rejects_malformed_expected_hash() {
    let dir = temp_dir("integrity-invalid-manifest");
    let file = dir.join("sample.txt");
    let manifest = dir.join("manifest.sha256");
    fs::write(&file, "phase1\n").expect("write sample");
    fs::write(
        &manifest,
        format!("not-a-sha256  {}\n", file.display()),
    )
    .expect("write manifest");

    let manifest_path = manifest.to_string_lossy().to_string();
    let (ok, output) = run_script(&["--manifest", &manifest_path]);
    let _ = fs::remove_dir_all(&dir);

    assert!(!ok, "malformed manifest should fail: {output}");
    assert!(output.contains("result: manifest-invalid"), "{output}");
}

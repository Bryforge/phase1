use std::fs;
use std::io::Write;
use std::process::{self, Command, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

static RUN_COUNTER: AtomicU64 = AtomicU64::new(0);

fn run_phase1(input: &str) -> String {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_nanos())
        .unwrap_or(0);
    let seq = RUN_COUNTER.fetch_add(1, Ordering::Relaxed);
    let run_dir = std::env::temp_dir().join(format!(
        "phase1-analysis-load-metadata-{}-{nonce}-{seq}",
        process::id()
    ));
    let _ = fs::remove_dir_all(&run_dir);
    fs::create_dir_all(&run_dir).expect("create analysis load metadata test dir");

    let mut child = Command::new(env!("CARGO_BIN_EXE_phase1"))
        .current_dir(&run_dir)
        .env("PHASE1_TEST_MODE", "1")
        .env("PHASE1_PERSISTENT_STATE", "0")
        .env("PHASE1_COOKED_INPUT", "1")
        .env("PHASE1_BLEEDING_EDGE", "1")
        .env_remove("PHASE1_THEME")
        .env_remove("PHASE1_ALLOW_HOST_TOOLS")
        .env_remove("PHASE1_SAFE_MODE")
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
    let _ = fs::remove_dir_all(&run_dir);
    format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    )
}

#[test]
fn analyze_load_records_vfs_file_metadata_without_execution() {
    let output = run_phase1(
        "echo 'phase1-sample' > sample.bin\nanalyze load sample.bin\nanalyze load sample.bin\nexit\n",
    );

    for required in [
        "phase1 analysis load",
        "status           : loaded",
        "status           : duplicate",
        "id               : sha256-016644b74537",
        "path             : /home/sample.bin",
        "size-bytes       : 14",
        "sha256           : 016644b74537df25d6d98eaf0e62f5a71340a7ad7222d7d652c4d2d238109445",
        "source           : vfs",
        "loaded-at        : session",
        "trust-state      : untrusted",
        "execution-state  : not-executed",
        "host-execution   : disabled",
        "sandbox-claim    : not-claimed",
        "dynamic-analysis : future-restricted",
        "claim-boundary   : metadata-only-loading",
    ] {
        assert!(output.contains(required), "missing {required:?}:\n{output}");
    }

    for forbidden in [
        "host=enabled",
        "host-execution   : enabled",
        "execution-state  : executed",
        "sandbox-claim    : hardened",
        "malware-safe",
        "production forensic",
    ] {
        assert!(
            !output.contains(forbidden),
            "forbidden {forbidden:?}:\n{output}"
        );
    }
}

#[test]
fn analyze_load_rejects_missing_and_directory_paths_without_execution() {
    let output = run_phase1("analyze load missing.bin\nanalyze load /home\nexit\n");

    for required in [
        "status           : missing",
        "error            : no-such-vfs-file",
        "path             : /home/missing.bin",
        "status           : unsupported",
        "error            : directory-not-sample",
        "path             : /home",
        "execution-state  : not-executed",
        "host-execution   : disabled",
        "sandbox-claim    : not-claimed",
    ] {
        assert!(output.contains(required), "missing {required:?}:\n{output}");
    }
}

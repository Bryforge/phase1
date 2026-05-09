use std::fs;
use std::io::Write;
use std::process::{Command, Stdio};
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
        "phase1-issue67-{}-{nonce}-{seq}",
        std::process::id()
    ));

    let _ = fs::remove_dir_all(&run_dir);
    fs::create_dir_all(&run_dir).expect("create test run dir");

    let mut child = Command::new(env!("CARGO_BIN_EXE_phase1"))
        .current_dir(&run_dir)
        .env("PHASE1_NO_COLOR", "1")
        .env("PHASE1_ASCII", "1")
        .env("PHASE1_COOKED_INPUT", "1")
        .env("PHASE1_BLEEDING_EDGE", "0")
        .env("PHASE1_MOBILE_MODE", "0")
        .env("PHASE1_DEVICE_MODE", "desktop")
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
        .write_all(input.as_bytes())
        .expect("write input");

    let output = child.wait_with_output().expect("wait phase1");
    let _ = fs::remove_dir_all(&run_dir);

    let mut combined = String::new();
    combined.push_str(&String::from_utf8_lossy(&output.stdout));
    combined.push_str(&String::from_utf8_lossy(&output.stderr));

    assert!(output.status.success(), "phase1 failed:\n{combined}");
    combined
}

#[test]
fn issue67_pasted_command_block_runs_each_line_independently() {
    let output = run_phase1(
        "\n\
echo alpha > issue67.txt\n\
echo beta >> issue67.txt\n\
cat issue67.txt\n\
wc -l issue67.txt\n\
exit\n",
    );

    assert!(
        output.contains("alpha"),
        "missing first pasted command output:\n{output}"
    );
    assert!(
        output.contains("beta"),
        "missing second pasted command output:\n{output}"
    );
    assert!(
        output.contains("2 issue67.txt") || output.contains("    2 issue67.txt"),
        "wc did not see both lines:\n{output}"
    );
}

#[test]
fn issue67_bracketed_paste_markers_are_ignored() {
    let output = run_phase1(
        "\n\x1b[200~echo pasted-one\n\
echo pasted-two\n\x1b[201~exit\n",
    );

    assert!(
        output.contains("pasted-one"),
        "first bracketed paste line missing:\n{output}"
    );
    assert!(
        output.contains("pasted-two"),
        "second bracketed paste line missing:\n{output}"
    );
    assert!(
        !output.contains("command not found: \u{1b}"),
        "bracketed paste marker leaked into command parser:\n{output}"
    );
}

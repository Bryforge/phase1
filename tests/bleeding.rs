use std::fs;
use std::io::Write;
use std::process::{Command, Stdio};
use std::time::{SystemTime, UNIX_EPOCH};

fn run_phase1(script: &str) -> String {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_nanos())
        .unwrap_or(0);
    let run_dir = std::env::temp_dir().join(format!("phase1-bleeding-{nonce}"));
    let _ = fs::remove_dir_all(&run_dir);
    fs::create_dir_all(&run_dir).expect("create bleeding test dir");

    let binary = env!("CARGO_BIN_EXE_phase1");
    let mut child = Command::new(binary)
        .current_dir(&run_dir)
        .env("PHASE1_NO_COLOR", "1")
        .env("PHASE1_ASCII", "1")
        .env("COLUMNS", "100")
        .env("LINES", "30")
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
fn bleeding_version_and_roadmap_are_visible() {
    let output = run_phase1("version --compare\nroadmap\npipeline\nupdate protocol\nexit\n");
    assert!(output.contains("phase1 version report"));
    assert!(output.contains("release version : 3.6.0"));
    assert!(output.contains("bleeding edge   : 3.7.1-dev"));
    assert!(output.contains("version scheme  : MAJOR.MINOR.PATCH[-dev]"));
    assert!(output.contains("protocol file   : UPDATE_PROTOCOL.md"));
    assert!(output.contains("Update protocol and semantic patch versioning"));
    assert!(output.contains("phase1 pipelines"));
    assert!(output.contains("phase1 update protocol"));
    assert!(output.contains("third number"));
}

#[test]
fn bleeding_structured_pipelines_filter_text() {
    let output = run_phase1(
        "echo alpha > log.txt\necho beta >> log.txt\necho alpha beta >> log.txt\ncat log.txt | grep alpha | wc -l\necho c b a | cut -d ' ' -f 2\nexit\n",
    );
    assert!(output.contains("    2"), "pipeline count missing:\n{output}");
    assert!(output.contains("b"), "cut pipeline output missing:\n{output}");
}

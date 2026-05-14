use std::fs;
use std::io::Write;
use std::process::{self, Command, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

static RUN_COUNTER: AtomicU64 = AtomicU64::new(0);

fn run_phase1_with_analyze_plugin(input: &str) -> String {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_nanos())
        .unwrap_or(0);
    let seq = RUN_COUNTER.fetch_add(1, Ordering::Relaxed);
    let run_dir = std::env::temp_dir().join(format!(
        "phase1-analysis-command-stub-{}-{nonce}-{seq}",
        process::id()
    ));
    let plugin_dir = run_dir.join("plugins");
    let _ = fs::remove_dir_all(&run_dir);
    fs::create_dir_all(&plugin_dir).expect("create analysis plugin dir");
    fs::write(plugin_dir.join("analyze.wasm"), b"\0asm\x01\0\0\0")
        .expect("write analyze wasm marker");
    fs::write(
        plugin_dir.join("analyze.wasi"),
        concat!(
            "name=analyze\n",
            "capability=none\n",
            "stdout=phase1 analysis\n",
            "stdout=status           : planned\n",
            "stdout=mode             : no-execute\n",
            "stdout=execution-state  : not-executed\n",
            "stdout=host-execution   : disabled\n",
            "stdout=sandbox-claim    : not-claimed\n",
            "stdout=static-analysis  : planned\n",
            "stdout=dynamic-analysis : future-restricted\n",
            "stdout=sample-registry  : planned\n",
            "stdout=reports          : planned\n",
            "stdout=fyr-integration  : planned\n",
            "stdout=base1-evidence   : planned\n",
            "stdout=claim-boundary   : metadata-only-planning\n",
            "stdout=load             : planned-no-op\n",
            "stdout=inspect          : planned-no-op\n",
            "stdout=report           : planned-no-op\n",
            "stdout=forget           : planned-no-op\n",
        ),
    )
    .expect("write analyze wasi manifest");

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
fn analyze_command_stub_reports_no_execute_boundary() {
    let output = run_phase1_with_analyze_plugin(
        "wasm list\nanalyze\nanalyze status\nanalyze help\nanalyze load sample.bin\nexit\n",
    );

    for required in [
        "analyze",
        "phase1 wasi run",
        "plugin : analyze",
        "sandbox: fs=virtual net=disabled host=blocked",
        "phase1 analysis",
        "status           : planned",
        "mode             : no-execute",
        "execution-state  : not-executed",
        "host-execution   : disabled",
        "sandbox-claim    : not-claimed",
        "static-analysis  : planned",
        "dynamic-analysis : future-restricted",
        "sample-registry  : planned",
        "reports          : planned",
        "fyr-integration  : planned",
        "base1-evidence   : planned",
        "claim-boundary   : metadata-only-planning",
        "load             : planned-no-op",
        "inspect          : planned-no-op",
        "report           : planned-no-op",
        "forget           : planned-no-op",
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

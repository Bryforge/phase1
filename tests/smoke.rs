use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::{self, Command, Stdio};
use std::time::{SystemTime, UNIX_EPOCH};

fn run_phase1(script: &str) -> String {
    let run_dir = unique_run_dir();
    fs::create_dir_all(&run_dir).expect("create phase1 smoke temp directory");
    let output = run_phase1_in_dir(&run_dir, &format!("\n{script}"));
    let _ = fs::remove_dir_all(&run_dir);
    output
}

fn run_phase1_host_enabled(script: &str) -> String {
    let run_dir = unique_run_dir();
    fs::create_dir_all(&run_dir).expect("create phase1 smoke temp directory");
    let output = run_phase1_in_dir(&run_dir, &format!("4\n\n{script}"));
    let _ = fs::remove_dir_all(&run_dir);
    output
}

fn run_phase1_in_dir(run_dir: &Path, input: &str) -> String {
    let binary = env!("CARGO_BIN_EXE_phase1");
    let mut child = Command::new(binary)
        .current_dir(run_dir)
        .env("PHASE1_NO_COLOR", "1")
        .env("PHASE1_ASCII", "1")
        .env("COLUMNS", "100")
        .env("LINES", "30")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn phase1 binary");

    {
        let stdin = child.stdin.as_mut().expect("phase1 stdin");
        stdin.write_all(input.as_bytes()).expect("write phase1 script");
    }

    let output = child.wait_with_output().expect("wait for phase1");

    let mut combined = String::new();
    combined.push_str(&String::from_utf8_lossy(&output.stdout));
    combined.push_str(&String::from_utf8_lossy(&output.stderr));
    assert!(output.status.success(), "phase1 exited unsuccessfully:\n{combined}");
    combined
}

fn unique_run_dir() -> std::path::PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_nanos())
        .unwrap_or(0);
    std::env::temp_dir().join(format!("phase1-smoke-{}-{nonce}", process::id()))
}

fn assert_contains_all(output: &str, needles: &[&str]) {
    for needle in needles {
        assert!(output.contains(needle), "missing expected output: {needle}\n--- output ---\n{output}");
    }
}

#[test]
fn boot_help_man_and_completion_work() {
    let output = run_phase1("help\ncomplete p\nman browser\nversion\nexit\n");
    assert_contains_all(
        &output,
        &[
            "security safe",
            "PHASE1 // ADVANCED OPERATOR CONSOLE",
            "phase1 // command map",
            "proc",
            "python",
            "browser",
            "usage      : browser <url|phase1|about>",
            "phase1 3.6.0",
        ],
    );
}

#[test]
fn secure_default_blocks_host_backed_commands() {
    let output = run_phase1("ifconfig\niwconfig\nwifi-scan\nwifi-connect demo\nping example.com\nbrowser phase1\npy -c \"print('blocked')\"\nexit\n");
    assert_contains_all(
        &output,
        &[
            "security safe",
            "safe-mode: host network inspection disabled",
            "safe-mode: host WiFi inspection disabled",
            "wifi-scan: disabled by safe boot profile",
            "wifi-connect: disabled by safe boot profile",
            "ping: disabled by safe boot profile",
            "browser: disabled by safe boot profile",
            "python: disabled by safe boot profile",
        ],
    );
    assert!(!output.contains("blocked"), "safe mode executed Python unexpectedly:\n{output}");
}

#[test]
fn preboot_persistent_state_mode_is_toggleable_and_restores_home_files() {
    let run_dir = unique_run_dir();
    fs::create_dir_all(&run_dir).expect("create persistent state smoke directory");

    let first = run_phase1_in_dir(&run_dir, "p\n\necho persisted value > keep.txt\nexit\n");
    assert_contains_all(
        &first,
        &[
            "persistent state  on",
            "persistent state: enabled; no saved state found at phase1.state",
        ],
    );
    assert!(run_dir.join("phase1.conf").exists(), "boot config was not saved");
    assert!(run_dir.join("phase1.state").exists(), "persistent state file was not saved");

    let second = run_phase1_in_dir(&run_dir, "\ncat keep.txt\nbootcfg show\nexit\n");
    assert_contains_all(
        &second,
        &[
            "persistent state: restored",
            "persisted value",
            "persistent state  : on",
            "state file        : phase1.state",
        ],
    );

    let _ = fs::remove_dir_all(&run_dir);
}

#[test]
fn roadmap_aliases_capabilities_and_dashboard_work() {
    let output = run_phase1_host_enabled("commands\ncaps\ndash --compact\npy -c \"print('alias-ok')\"\nquit\n");
    assert_contains_all(
        &output,
        &[
            "safe mode         off",
            "phase1 // command map",
            "command        category capability",
            "wifi-connect",
            "dry-run by default",
            "PHASE1 DASHBOARD v3.6.0",
            "CORE  user=root",
            "PROC  tasks=",
            "HW    cr3=0x1000",
            "python",
            "timeout+validation",
            "alias-ok",
            "shutdown: phase1 3.6.0",
        ],
    );
}

#[test]
fn filesystem_commands_round_trip() {
    let output = run_phase1(
        "pwd\nls /\nmkdir lab\ncd lab\npwd\necho hello world > note.txt\ncat note.txt\ncp note.txt copy.txt\ncat copy.txt\nmv copy.txt moved.txt\nls\nrm moved.txt\nls\nexit\n",
    );
    assert_contains_all(
        &output,
        &[
            "/home",
            "bin",
            "/home/lab",
            "hello world",
            "note.txt",
        ],
    );
    assert!(!output.contains("command not found"), "unexpected missing command:\n{output}");
}

#[test]
fn proc_sys_audit_and_arch_commands_work() {
    let output = run_phase1(
        "cat /proc/version\ncat /proc/cpuinfo\nps\nspawn worker --background\njobs\ncr3\nloadcr3 0x2000\ncr3\npcide on\nloadcr3 0x2001\ncr4\nlspci\npcie\naudit\nexit\n",
    );
    assert_contains_all(
        &output,
        &[
            "phase1 3.6.0",
            "phase1 virtual cpu",
            "phase1-shell",
            "spawned pid",
            "worker",
            "CR3=0x1000",
            "CR3 loaded: 0x2000",
            "CR3=0x2000",
            "PCIDE enabled",
            "CR3 loaded: 0x2001",
            "CR4: PCIDE=on",
            "Intel 82540EM network adapter",
            "sys.read path=/proc/version",
            "sys.spawn name=worker bg=true",
        ],
    );
}

#[test]
fn user_env_browser_and_sandbox_commands_work() {
    let output = run_phase1(
        "whoami\nid\nexport TEST_VAR=phase1\necho $TEST_VAR\nunset TEST_VAR\nenv\nbrowser phase1\nsandbox\ndate\nuptime\nhostname\nexit\n",
    );
    assert_contains_all(
        &output,
        &[
            "root",
            "uid=0(root)",
            "phase1",
            "PHASE1",
            "browser: disabled by safe boot profile",
            "sandbox: VFS/processes are simulated",
            "up ",
        ],
    );
}

#[test]
fn network_commands_have_stable_safe_output() {
    let output = run_phase1("ifconfig\niwconfig\nwifi-scan\nwifi-connect\nexit\n");
    assert_contains_all(
        &output,
        &[
            "lo: flags=<UP>",
            "safe-mode: host network inspection disabled",
            "safe-mode: host WiFi inspection disabled",
            "wifi-scan: disabled by safe boot profile",
            "wifi-connect: disabled by safe boot profile",
        ],
    );
}

#[test]
fn expected_errors_are_clear() {
    let output = run_phase1("cat missing.txt\ncd missing\nkill abc\nloadcr3 123\nwifi-connect\nunknowncmd\nexit\n");
    assert_contains_all(
        &output,
        &[
            "cat: no such file: missing.txt",
            "cd: no such directory",
            "usage: kill <pid>",
            "loadcr3: CR3 must be 4KiB aligned unless PCIDE is enabled",
            "wifi-connect: disabled by safe boot profile",
            "command not found: unknowncmd",
        ],
    );
}

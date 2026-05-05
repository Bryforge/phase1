use std::io::Write;
use std::process::{Command, Stdio};

fn run_phase1(script: &str) -> String {
    let binary = env!("CARGO_BIN_EXE_phase1");
    let mut child = Command::new(binary)
        .env("PHASE1_NO_COLOR", "1")
        .env("PHASE1_ASCII", "1")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn phase1 binary");

    {
        let stdin = child.stdin.as_mut().expect("phase1 stdin");
        stdin.write_all(script.as_bytes()).expect("write phase1 script");
    }

    let output = child.wait_with_output().expect("wait for phase1");
    let mut combined = String::new();
    combined.push_str(&String::from_utf8_lossy(&output.stdout));
    combined.push_str(&String::from_utf8_lossy(&output.stderr));
    assert!(output.status.success(), "phase1 exited unsuccessfully:\n{combined}");
    combined
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
            "PHASE1 // ADVANCED OPERATOR CONSOLE",
            "phase1 // command map",
            "proc",
            "python",
            "browser",
            "usage      : browser <url|phase1|about>",
            "phase1 3.5.0",
        ],
    );
}

#[test]
fn roadmap_aliases_and_capabilities_work() {
    let output = run_phase1("commands\ncaps\npy -c \"print('alias-ok')\"\nquit\n");
    assert_contains_all(
        &output,
        &[
            "phase1 // command map",
            "command        category capability",
            "wifi-connect",
            "dry-run by default",
            "python",
            "timeout+validation",
            "alias-ok",
            "shutdown: phase1 3.5.0",
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
            "phase1 3.5.0",
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
            "Terminal-first virtual OS console",
            "sandbox: VFS/processes are simulated",
            "up ",
        ],
    );
}

#[test]
fn network_commands_have_stable_output() {
    let output = run_phase1("iwconfig\nwifi-scan\nwifi-connect\nexit\n");
    assert_contains_all(&output, &["usage: wifi-connect <ssid> [password]"]);
    assert!(
        output.contains("no active WiFi interface")
            || output.contains("wifi-scan:")
            || output.contains("SSID")
            || output.contains("Current Wi-Fi Network")
            || output.contains("Preferred networks"),
        "network command output did not include expected fallback text:\n{output}"
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
            "usage: wifi-connect <ssid> [password]",
            "command not found: unknowncmd",
        ],
    );
}

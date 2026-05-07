use std::fs;
use std::io::Write;
use std::process::{self, Command, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

const EDGE_VERSION: &str = env!("CARGO_PKG_VERSION");
static RUN_COUNTER: AtomicU64 = AtomicU64::new(0);

fn run_phase1(script: &str) -> String {
    run_phase1_raw(&format!("\n{script}"))
}

fn run_phase1_raw(input: &str) -> String {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_nanos())
        .unwrap_or(0);
    let seq = RUN_COUNTER.fetch_add(1, Ordering::Relaxed);
    let run_dir =
        std::env::temp_dir().join(format!("phase1-bleeding-{}-{nonce}-{seq}", process::id()));
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
        .write_all(input.as_bytes())
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
    let output = run_phase1(
        "version --compare\nroadmap\npipeline\nupdate protocol\nupdate latest --build\nupdate test quick\nsecurity\nwasm run game status\ndash\nexit\n",
    );
    assert!(output.contains("phase1 version report"));
    assert!(output.contains("stable version  : 3.6.0"));
    assert!(
        output.contains(&format!("current version : {EDGE_VERSION}")),
        "current version report did not track package version:\n{output}"
    );
    assert!(output.contains("version scheme  : MAJOR.MINOR.PATCH[-dev]"));
    assert!(output.contains("protocol file   : UPDATE_PROTOCOL.md"));
    assert!(output.contains("Update protocol and semantic patch versioning"));
    assert!(output.contains("Capability enforcement based on command metadata"));
    assert!(output.contains("WASM/WASI plugin runtime"));
    assert!(output.contains("WASI-lite plugin runtime"));
    assert!(output.contains("metadata-backed capability enforcement"));
    assert!(output.contains("Phase1 Arena game workspace"));
    assert!(output.contains("phase1 game workspace"));
    assert!(output.contains("Configurable UI color palettes"));
    assert!(output.contains("selectable UI color palettes"));
    assert!(output.contains("System tab auto-completion"));
    assert!(output.contains("live system tab auto-completion"));
    assert!(output.contains("raw-mode input editor"));
    assert!(output.contains("Raw input editing"));
    assert!(output.contains("Full-screen TUI dashboard"));
    assert!(output.contains("In-system latest updater"));
    assert!(output.contains("Developer test kit"));
    assert!(output.contains("developer test kit"));
    assert!(output.contains("update now --trust-host"));
    assert!(output.contains("update test quick --trust-host --execute"));
    assert!(output.contains("PHASE1 FULL-SCREEN TUI DASHBOARD"));
    assert!(output.contains("panels  : core proc vfs net hw audit"));
    assert!(output.contains("capability metadata : enforced"));
    assert!(output.contains("phase1 pipelines"));
    assert!(output.contains("phase1 update protocol"));
    assert!(output.contains("third number"));
}

#[test]
fn bleeding_structured_pipelines_filter_text() {
    let output = run_phase1(
        "echo alpha > log.txt\necho beta >> log.txt\necho alpha beta >> log.txt\ncat log.txt | grep alpha | wc -l\necho c b a | cut -d ' ' -f 2\nexit\n",
    );
    assert!(
        output.contains("    2"),
        "pipeline count missing:\n{output}"
    );
    assert!(
        output.contains("b"),
        "cut pipeline output missing:\n{output}"
    );
}

#[test]
fn bleeding_wasi_lite_plugins_are_sandboxed() {
    let output = run_phase1(
        "plugins\nwasm list\nwasm inspect hello-wasi\nwasm run hello-wasi token=supersecret\nhello-wasi password=hunter2\ncomplete wa\nexit\n",
    );
    assert!(
        output.contains("wasm plugins:"),
        "missing wasm plugin list:\n{output}"
    );
    assert!(
        output.contains("hello-wasi"),
        "missing example wasm plugin:\n{output}"
    );
    assert!(
        output.contains("phase1 wasm inspect"),
        "missing inspect output:\n{output}"
    );
    assert!(
        output.contains("valid wasm"),
        "missing validation output:\n{output}"
    );
    assert!(
        output.contains("phase1 wasi run"),
        "missing run output:\n{output}"
    );
    assert!(
        output.contains("host=blocked"),
        "sandbox not reported:\n{output}"
    );
    assert!(
        output.contains("hello from phase1 wasi-lite"),
        "manifest stdout missing:\n{output}"
    );
    assert!(
        output.contains("[redacted]"),
        "secret-looking args were not redacted:\n{output}"
    );
    assert!(
        output.contains("wasm"),
        "wasm completion missing:\n{output}"
    );
    assert!(
        output.contains("wasi"),
        "wasi completion missing:\n{output}"
    );
}

#[test]
fn bleeding_theme_palettes_are_selectable() {
    let output = run_phase1(
        "theme list\ntheme matrix\ntheme\nbanner cyber\ntheme synthwave\ntheme\ntheme reset\ntheme\nexit\n",
    );
    assert!(
        output.contains("rainbow ANSI gradient"),
        "missing rainbow palette:\n{output}"
    );
    assert!(
        output.contains("matrix enabled"),
        "matrix theme did not enable:\n{output}"
    );
    assert!(
        output.contains("active : matrix"),
        "matrix theme status missing:\n{output}"
    );
    assert!(
        output.contains("display : cyber"),
        "banner cyber preview missing:\n{output}"
    );
    assert!(
        output.contains("synthwave enabled"),
        "synthwave theme did not enable:\n{output}"
    );
    assert!(
        output.contains("active : synthwave"),
        "synthwave status missing:\n{output}"
    );
    assert!(
        output.contains("reset to rainbow default"),
        "reset did not return to default:\n{output}"
    );
    assert!(
        output.contains("active : rainbow"),
        "rainbow default status missing:\n{output}"
    );
}

#[test]
fn bleeding_tab_completion_expands_commands_and_arguments() {
    let output = run_phase1("vers\t --compare\nupdate lat\t --build\ntheme ma\t\nw\t\nexit\n");
    assert!(
        output.contains("tab complete: version --compare"),
        "command tab completion missing:\n{output}"
    );
    assert!(
        output.contains("phase1 version report"),
        "completed version command did not execute:\n{output}"
    );
    assert!(
        output.contains("tab complete: update latest --build"),
        "update argument tab completion missing:\n{output}"
    );
    assert!(
        output.contains("phase1 updater // plan latest bleeding edge"),
        "completed update latest did not execute:\n{output}"
    );
    assert!(
        output.contains("tab complete: theme matrix"),
        "argument tab completion missing:\n{output}"
    );
    assert!(
        output.contains("matrix enabled"),
        "completed theme argument did not execute:\n{output}"
    );
    assert!(
        output.contains("tab matches for 'w':"),
        "ambiguous tab suggestions missing:\n{output}"
    );
    assert!(
        output.contains("wasm"),
        "wasm suggestion missing:\n{output}"
    );
    assert!(output.contains("wc"), "wc suggestion missing:\n{output}");
}

#[test]
fn bleeding_edge_boot_switch_updates_ui_channel_and_version() {
    let output =
        run_phase1_raw("e\n\nbootcfg show\nsysinfo\ntheme\nbanner edge\ndash --compact\nexit\n");
    assert!(
        output.contains("[e] EDGE") && output.contains("ON"),
        "boot switch missing:\n{output}"
    );
    assert!(
        output.contains(&format!("v{EDGE_VERSION}")),
        "boot UI did not use edge version:\n{output}"
    );
    assert!(
        output.contains("channel") && output.contains("bleeding-edge"),
        "boot UI channel missing:\n{output}"
    );
    assert!(
        output.contains("boot profile      : safe+edge"),
        "bootcfg profile not edge:\n{output}"
    );
    assert!(
        output.contains("bleeding edge     : on"),
        "bootcfg edge state missing:\n{output}"
    );
    assert!(
        output.contains("channel     : bleeding-edge"),
        "sysinfo channel missing:\n{output}"
    );
    assert!(
        output.contains(&format!("version     : {EDGE_VERSION}")),
        "sysinfo edge version missing:\n{output}"
    );
    assert!(
        output.contains(&format!("PHASE1 DASHBOARD v{EDGE_VERSION}")),
        "dash edge version missing:\n{output}"
    );
    assert!(
        output.contains("active : bleeding-edge"),
        "edge theme not automatic:\n{output}"
    );
    assert!(
        output.contains("display : bleeding-edge"),
        "banner edge preview missing:\n{output}"
    );
}

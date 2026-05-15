#[path = "arena.rs"]
mod arena;
#[path = "optics.rs"]
mod optics;
#[path = "phase.rs"]
mod phase;

use std::fs;
use std::path::{Path, PathBuf};

const WASM_MAGIC: &[u8] = b"\0asm";
const WASM_VERSION: &[u8] = b"\x01\0\0\0";
const RUNTIME: &str = "phase1-wasi-lite";

#[derive(Clone, Debug, Eq, PartialEq)]
struct WasiManifest {
    name: Option<String>,
    capability: String,
    stdout: Vec<String>,
    stderr: Vec<String>,
    exit_code: i32,
}

impl Default for WasiManifest {
    fn default() -> Self {
        Self {
            name: None,
            capability: "none".to_string(),
            stdout: Vec::new(),
            stderr: Vec::new(),
            exit_code: 0,
        }
    }
}

pub fn help() -> String {
    "phase1 wasm/wasi runtime\nusage: wasm [list|inspect|run|validate|help] [plugin] [args...]\n\nplugins live in ./plugins as .wasm artifacts with optional .wasi manifests.\nruntime is phase1-only: no host shell, no host network, no host filesystem passthrough.\nPhase1 Arena is exposed as a built-in WASI-lite game launcher: `wasm run arena demo`, `arena start`, or `game status`.\n".to_string()
}

pub fn run(plugins_dir: &Path, args: &[String]) -> String {
    match args.first().map(String::as_str) {
        None | Some("help" | "-h" | "--help") => help(),
        Some("list") => list_plugins(plugins_dir),
        Some("inspect") => match args.get(1) {
            Some(name) => inspect_plugin(plugins_dir, name),
            None => "wasm: usage: wasm inspect <plugin>\n".to_string(),
        },
        Some("validate") => match args.get(1) {
            Some(name) => validate_plugin(plugins_dir, name),
            None => "wasm: usage: wasm validate <plugin>\n".to_string(),
        },
        Some("run" | "exec") => match args.get(1) {
            Some(name) => execute_plugin(plugins_dir, name, &args[2..]),
            None => "wasm: usage: wasm run <plugin> [args...]\n".to_string(),
        },
        Some(name) => execute_plugin(plugins_dir, name, &args[1..]),
    }
}

pub fn execute_plugin(plugins_dir: &Path, name: &str, args: &[String]) -> String {
    if is_game_workspace(name) {
        return arena::game(args);
    }
    if is_arena(name) {
        return launch_arena(args);
    }
    if is_phase(name) {
        return phase_native(args);
    }
    if is_optics_status(name, args) {
        return optics_status(args);
    }
    if is_optics_device(name, args) {
        return optics_device_preview(args);
    }
    if is_optics_rails(name, args) {
        return optics_rails_preview(args);
    }

    let path = match plugin_path(plugins_dir, name) {
        Ok(path) => path,
        Err(err) => return format!("wasm: {err}\n"),
    };
    if let Err(err) = validate_wasm_file(&path) {
        return format!("wasm: {err}\n");
    }

    let manifest = read_manifest(&path);
    let plugin_name = manifest.name.clone().unwrap_or_else(|| display_name(&path));
    let mut out = String::from("phase1 wasi run\n");
    out.push_str(&format!("plugin : {plugin_name}\n"));
    out.push_str(&format!("runtime: {RUNTIME}\n"));
    out.push_str("sandbox: fs=virtual net=disabled host=blocked\n");
    out.push_str(&format!("cap    : {}\n", manifest.capability));
    if args.is_empty() {
        out.push_str("args   : none\n");
    } else {
        out.push_str(&format!("args   : {}\n", redact_args(args).join(" ")));
    }
    if manifest.stdout.is_empty() && manifest.stderr.is_empty() {
        out.push_str("module : validated wasm32-wasi artifact\n");
    }
    for line in manifest.stdout {
        out.push_str(&redact_text(&line));
        out.push('\n');
    }
    for line in manifest.stderr {
        out.push_str("stderr : ");
        out.push_str(&redact_text(&line));
        out.push('\n');
    }
    out.push_str(if manifest.exit_code == 0 {
        "status : ok\n"
    } else {
        "status : failed\n"
    });
    out.push_str(&format!("exit   : {}\n", manifest.exit_code));
    out
}

fn is_phase(name: &str) -> bool {
    name == "phase"
}

fn phase_native(args: &[String]) -> String {
    let mut out = String::from("phase1 native run\n");
    out.push_str("command: phase\n");
    if args.is_empty() {
        out.push_str("args   : none\n");
    } else {
        out.push_str(&format!("args   : {}\n", redact_args(args).join(" ")));
    }
    out.push_str(&phase::run(args));
    out.push_str("status : ok\n");
    out.push_str("exit   : 0\n");
    out
}

fn is_optics_status(name: &str, args: &[String]) -> bool {
    name == "optics" && args.first().is_some_and(|arg| arg == "status")
}

fn is_optics_device(name: &str, args: &[String]) -> bool {
    name == "optics" && args.first().is_some_and(|arg| arg == "device")
}

fn is_optics_rails(name: &str, args: &[String]) -> bool {
    name == "optics" && args.first().is_some_and(|arg| arg == "rails")
}

fn optics_status(args: &[String]) -> String {
    let mut out = String::from("phase1 wasi run\n");
    out.push_str("plugin : optics\n");
    out.push_str(&format!("runtime: {RUNTIME}\n"));
    out.push_str("sandbox: fs=virtual net=disabled host=blocked\n");
    out.push_str("cap    : none\n");
    out.push_str(&format!("args   : {}\n", redact_args(args).join(" ")));
    out.push_str("OPTICS STATUS\n");
    out.push_str("mode        : preview-only\n");
    out.push_str("renderer    : rust-static-renderer\n");
    out.push_str("top-rail    : ready-preview\n");
    out.push_str("bottom-rail : ready-preview\n");
    out.push_str(&format!(
        "devices     : {}\n",
        optics::supported_device_labels()
    ));
    out.push_str("live-hud    : disabled\n");
    out.push_str("activation  : explicit-gate-required\n");
    out.push_str("input       : raw-command-preserved\n");
    out.push_str("history     : unchanged\n");
    out.push_str("parser      : unchanged\n");
    out.push_str("non-claims  : not-compositor not-terminal-emulator not-sandbox not-security-boundary not-crypto-enforcement not-system-integrity-guarantee not-base1-boot-environment\n");
    out.push_str("OPTICS PRO SHELL RAIL CONTRACT\n");
    out.push_str(&optics::render_pro_shell_layers(
        &optics::OpticsRailState::pro_static(optics::OpticsDeviceProfile::Terminal),
        "optics status",
        false,
    ));
    out.push_str("status : ok\n");
    out.push_str("exit   : 0\n");
    out
}

fn optics_device_preview(args: &[String]) -> String {
    let mut out = String::from("phase1 wasi run\n");
    out.push_str("plugin : optics\n");
    out.push_str(&format!("runtime: {RUNTIME}\n"));
    out.push_str("sandbox: fs=virtual net=disabled host=blocked\n");
    out.push_str("cap    : none\n");
    out.push_str(&format!("args   : {}\n", redact_args(args).join(" ")));

    let Some(raw_device) = args.get(1).map(String::as_str) else {
        out.push_str("OPTICS DEVICE PREVIEW\n");
        out.push_str("result      : missing-device\n");
        out.push_str("usage       : optics device mobile|laptop|desktop|terminal\n");
        out.push_str(&format!(
            "supported   : {}\n",
            optics::supported_device_labels()
        ));
        out.push_str("status : failed\n");
        out.push_str("exit   : 1\n");
        return out;
    };

    let Some(device) = optics_device_profile(raw_device) else {
        out.push_str("OPTICS DEVICE PREVIEW\n");
        out.push_str("result      : invalid-device\n");
        out.push_str(&format!("requested   : {}\n", redact_text(raw_device)));
        out.push_str("usage       : optics device mobile|laptop|desktop|terminal\n");
        out.push_str(&format!(
            "supported   : {}\n",
            optics::supported_device_labels()
        ));
        out.push_str("status : failed\n");
        out.push_str("exit   : 1\n");
        return out;
    };

    out.push_str("OPTICS DEVICE PREVIEW\n");
    out.push_str(&format!("device      : {}\n", device.as_label()));
    out.push_str("mode        : preview-only\n");
    out.push_str("renderer    : rust-static-renderer\n");
    out.push_str("live-hud    : disabled\n");
    out.push_str(&optics::render_static_preview(device));
    out.push_str("status : ok\n");
    out.push_str("exit   : 0\n");
    out
}

fn optics_device_profile(raw: &str) -> Option<optics::OpticsDeviceProfile> {
    match raw {
        "mobile" => Some(optics::OpticsDeviceProfile::Mobile),
        "laptop" => Some(optics::OpticsDeviceProfile::Laptop),
        "desktop" => Some(optics::OpticsDeviceProfile::Desktop),
        "terminal" => Some(optics::OpticsDeviceProfile::Terminal),
        _ => None,
    }
}

fn optics_rails_preview(args: &[String]) -> String {
    let mut out = String::from("phase1 wasi run\n");
    out.push_str("plugin : optics\n");
    out.push_str(&format!("runtime: {RUNTIME}\n"));
    out.push_str("sandbox: fs=virtual net=disabled host=blocked\n");
    out.push_str("cap    : none\n");
    out.push_str(&format!("args   : {}\n", redact_args(args).join(" ")));
    out.push_str(&optics::render_static_preview(
        optics::OpticsDeviceProfile::Terminal,
    ));
    out.push_str("status : ok\n");
    out.push_str("exit   : 0\n");
    out
}

fn list_plugins(plugins_dir: &Path) -> String {
    let mut names = Vec::new();
    if let Ok(entries) = fs::read_dir(plugins_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|ext| ext.to_str()) == Some("wasm") {
                if let Some(name) = path.file_stem().and_then(|stem| stem.to_str()) {
                    if is_safe_name(name) && name != "opendoom" {
                        names.push(name.to_string());
                    }
                }
            }
        }
    }
    for built_in in ["arena", "game", "phase"] {
        if !names.iter().any(|name| name == built_in) {
            names.push(built_in.to_string());
        }
    }
    names.sort();
    if names.is_empty() {
        "phase1 wasm plugins\nno wasm plugins found\n".to_string()
    } else {
        format!("phase1 wasm plugins\n{}\n", names.join("\n"))
    }
}

fn inspect_plugin(plugins_dir: &Path, name: &str) -> String {
    if is_game_workspace(name) {
        return game_inspect();
    }
    if is_arena(name) {
        return arena_inspect();
    }
    if is_phase(name) {
        return phase_inspect();
    }

    let path = match plugin_path(plugins_dir, name) {
        Ok(path) => path,
        Err(err) => return format!("wasm: {err}\n"),
    };
    let validation = validate_wasm_file(&path);
    let manifest = read_manifest(&path);
    let size = fs::metadata(&path).map(|meta| meta.len()).unwrap_or(0);
    let mut out = String::from("phase1 wasm inspect\n");
    out.push_str(&format!("plugin : {}\n", display_name(&path)));
    out.push_str(&format!("path   : {}\n", path.display()));
    out.push_str(&format!("bytes  : {size}\n"));
    out.push_str(&format!(
        "module : {}\n",
        if validation.is_ok() {
            "valid wasm"
        } else {
            "invalid"
        }
    ));
    if let Err(err) = validation {
        out.push_str(&format!("error  : {err}\n"));
    }
    out.push_str(&format!("runtime: {RUNTIME}\n"));
    out.push_str("wasi   : sandboxed, no host shell, no host network\n");
    out.push_str(&format!("cap    : {}\n", manifest.capability));
    out
}

fn validate_plugin(plugins_dir: &Path, name: &str) -> String {
    if is_game_workspace(name) {
        return "wasm: game workspace built-in phase1-wasi-lite launcher\n".to_string();
    }
    if is_arena(name) {
        return "wasm: arena built-in phase1-wasi-lite game launcher\n".to_string();
    }
    if is_phase(name) {
        return "wasm: phase source-native status surface\n".to_string();
    }

    let path = match plugin_path(plugins_dir, name) {
        Ok(path) => path,
        Err(err) => return format!("wasm: {err}\n"),
    };
    match validate_wasm_file(&path) {
        Ok(()) => format!("wasm: {} valid wasm32-wasi artifact\n", display_name(&path)),
        Err(err) => format!("wasm: {err}\n"),
    }
}

fn launch_arena(args: &[String]) -> String {
    match args.first().map(String::as_str) {
        Some("start" | "play") => {
            arena::play();
            String::new()
        }
        _ => arena::run(args),
    }
}

fn arena_inspect() -> String {
    let mut out = String::from("phase1 wasm inspect\n");
    out.push_str("plugin : arena\n");
    out.push_str("name   : Phase1 Arena\n");
    out.push_str("module : built-in phase1 text-mode game\n");
    out.push_str(&format!("runtime: {RUNTIME}\n"));
    out.push_str("wasi   : sandboxed, no host shell, no host network\n");
    out.push_str("cap    : none\n");
    out.push_str("play   : arena start\n");
    out.push_str("dev    : docs/developers/GAME_DEV.md and scripts/test-game.sh\n");
    out
}

fn phase_inspect() -> String {
    let mut out = String::from("phase1 wasm inspect\n");
    out.push_str("plugin : phase\n");
    out.push_str("module : source-native Phase compass status surface\n");
    out.push_str("runtime: source-native\n");
    out.push_str("cap    : none\n");
    out.push_str("usage  : phase whereami | phase compass | phase path | phase map\n");
    out
}

fn game_inspect() -> String {
    let mut out = String::from("phase1 wasm inspect\n");
    out.push_str("plugin : game\n");
    out.push_str("module : Phase1 Arena development workspace\n");
    out.push_str(&format!("runtime: {RUNTIME}\n"));
    out.push_str("cap    : none\n");
    out.push_str("usage  : game status | game files | game test-plan | game roadmap\n");
    out
}

fn is_game_workspace(name: &str) -> bool {
    name == "game"
}

fn is_arena(name: &str) -> bool {
    matches!(
        name,
        "arena" | "phase-arena" | "phasearena" | "opendoom" | "open-doom" | "doom"
    )
}

fn plugin_path(plugins_dir: &Path, raw: &str) -> Result<PathBuf, String> {
    let name = raw.strip_suffix(".wasm").unwrap_or(raw);
    if !is_safe_name(name) {
        return Err(format!("invalid plugin name '{raw}'"));
    }
    let path = plugins_dir.join(format!("{name}.wasm"));
    if !path.exists() {
        return Err(format!("plugin not found: {name}"));
    }
    Ok(path)
}

fn validate_wasm_file(path: &Path) -> Result<(), String> {
    let bytes = fs::read(path).map_err(|err| format!("{}: {err}", path.display()))?;
    if bytes.len() < 8 {
        return Err(format!("{}: wasm header too short", path.display()));
    }
    if &bytes[..4] != WASM_MAGIC || &bytes[4..8] != WASM_VERSION {
        return Err(format!("{}: invalid wasm magic/version", path.display()));
    }
    Ok(())
}

fn read_manifest(wasm_path: &Path) -> WasiManifest {
    let mut manifest = WasiManifest::default();
    let manifest_path = wasm_path.with_extension("wasi");
    let Ok(raw) = fs::read_to_string(manifest_path) else {
        return manifest;
    };
    for line in raw.lines().map(str::trim) {
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let Some((key, value)) = line.split_once('=') else {
            continue;
        };
        match key.trim() {
            "name" => manifest.name = Some(value.trim().to_string()),
            "capability" | "cap" => manifest.capability = value.trim().to_string(),
            "stdout" => manifest.stdout.push(value.trim().to_string()),
            "stderr" => manifest.stderr.push(value.trim().to_string()),
            "exit" => manifest.exit_code = value.trim().parse().unwrap_or(1),
            _ => {}
        }
    }
    manifest
}

fn display_name(path: &Path) -> String {
    path.file_stem()
        .and_then(|stem| stem.to_str())
        .unwrap_or("unknown")
        .to_string()
}

fn is_safe_name(name: &str) -> bool {
    !name.is_empty()
        && name
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '_' | '-'))
}

fn redact_args(args: &[String]) -> Vec<String> {
    args.iter().map(|arg| redact_text(arg)).collect()
}

fn redact_text(value: &str) -> String {
    let lower = value.to_ascii_lowercase();
    let secret_markers = [
        "token=",
        "password=",
        "secret=",
        "key=",
        "ghp_",
        "gho_",
        "github_pat_",
        "begin private key",
    ];
    if secret_markers.iter().any(|marker| lower.contains(marker)) {
        "[redacted]".to_string()
    } else {
        value.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::{execute_plugin, inspect_plugin, run};
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::time::{SystemTime, UNIX_EPOCH};
    use std::{fs, process};

    static WASM_TEST_COUNTER: AtomicU64 = AtomicU64::new(0);

    fn temp_plugins() -> std::path::PathBuf {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|duration| duration.as_nanos())
            .unwrap_or(0);
        let seq = WASM_TEST_COUNTER.fetch_add(1, Ordering::Relaxed);
        let dir =
            std::env::temp_dir().join(format!("phase1-wasm-test-{}-{nonce}-{seq}", process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join("demo.wasm"), b"\0asm\x01\0\0\0").unwrap();
        fs::write(
            dir.join("demo.wasi"),
            "name=demo\ncapability=none\nstdout=hello wasi\n",
        )
        .unwrap();
        dir
    }

    #[test]
    fn wasm_runtime_lists_and_inspects_plugins() {
        let dir = temp_plugins();
        let listed = run(&dir, &["list".to_string()]);
        assert!(listed.contains("demo"));
        assert!(listed.contains("arena"));
        assert!(listed.contains("game"));
        assert!(listed.contains("phase"));
        let inspected = inspect_plugin(&dir, "demo");
        assert!(inspected.contains("valid wasm"));
        let game = inspect_plugin(&dir, "arena");
        assert!(game.contains("built-in phase1 text-mode game"));
        assert!(game.contains("Phase1 Arena"));
        let workspace = inspect_plugin(&dir, "game");
        assert!(workspace.contains("development workspace"));
        let phase = inspect_plugin(&dir, "phase");
        assert!(phase.contains("source-native Phase compass status surface"));
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn wasm_runtime_runs_manifest_without_host_shell() {
        let dir = temp_plugins();
        let out = execute_plugin(&dir, "demo", &["token=abc".to_string()]);
        assert!(out.contains("phase1 wasi run"));
        assert!(out.contains("hello wasi"));
        assert!(out.contains("host=blocked"));
        assert!(out.contains("[redacted]"));
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn phase_builtin_uses_source_native_status_surface() {
        let dir = temp_plugins();
        let out = execute_plugin(&dir, "phase", &["whereami".to_string()]);
        assert!(out.contains("phase1 native run"));
        assert!(out.contains("command: phase"));
        assert!(out.contains("args   : whereami"));
        assert!(out.contains("PHASE COMPASS"));
        assert!(out.contains("runtime=source-native"));
        assert!(out.contains("mutation=disabled"));
        assert!(out.contains("origin=0/0"));
        assert!(out.contains("path=ROOT>0/0"));
        assert!(!out.contains("phase1 wasi run"));
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn optics_status_route_reports_preview_mode_and_activation_gate() {
        let dir = temp_plugins();
        fs::write(dir.join("optics.wasm"), b"\0asm\x01\0\0\0").unwrap();
        let out = execute_plugin(&dir, "optics", &["status".to_string()]);
        assert!(out.contains("plugin : optics"));
        assert!(out.contains("args   : status"));
        assert!(out.contains("OPTICS STATUS"));
        assert!(out.contains("mode        : preview-only"));
        assert!(out.contains("renderer    : rust-static-renderer"));
        assert!(out.contains("top-rail    : ready-preview"));
        assert!(out.contains("bottom-rail : ready-preview"));
        assert!(out.contains("devices     : mobile,laptop,desktop,terminal"));
        assert!(out.contains("live-hud    : disabled"));
        assert!(out.contains("activation  : explicit-gate-required"));
        assert!(out.contains("OPTICS PRO SHELL RAIL CONTRACT"));
        assert!(out.contains("A TOP RAIL"));
        assert!(out.contains("B COMMAND RAIL"));
        assert!(out.contains("C STATUS HUD"));
        assert!(out.contains("D BOTTOM HUD"));
        assert!(out.contains("phase1://edge/root > optics status\n\nC STATUS HUD"));
        assert!(out.contains("parser      : unchanged"));
        assert!(out.contains("not-security-boundary"));
        assert!(out.contains("status : ok"));
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn optics_device_route_renders_requested_profile() {
        let dir = temp_plugins();
        fs::write(dir.join("optics.wasm"), b"\0asm\x01\0\0\0").unwrap();
        for device in ["mobile", "laptop", "desktop", "terminal"] {
            let out = execute_plugin(&dir, "optics", &["device".to_string(), device.to_string()]);
            assert!(out.contains("plugin : optics"));
            assert!(out.contains(&format!("args   : device {device}")));
            assert!(out.contains("OPTICS DEVICE PREVIEW"));
            assert!(out.contains(&format!("device      : {device}")));
            assert!(out.contains("mode        : preview-only"));
            assert!(out.contains("live-hud    : disabled"));
            assert!(out.contains("OPTICS HUD RAIL RENDER"));
            assert!(out.contains(&format!("device={device}")));
            assert!(out.contains("status : ok"));
        }
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn optics_device_route_rejects_invalid_profile_without_live_activation() {
        let dir = temp_plugins();
        fs::write(dir.join("optics.wasm"), b"\0asm\x01\0\0\0").unwrap();
        let out = execute_plugin(
            &dir,
            "optics",
            &["device".to_string(), "wallpaper".to_string()],
        );
        assert!(out.contains("OPTICS DEVICE PREVIEW"));
        assert!(out.contains("result      : invalid-device"));
        assert!(out.contains("requested   : wallpaper"));
        assert!(out.contains("supported   : mobile,laptop,desktop,terminal"));
        assert!(out.contains("status : failed"));
        assert!(out.contains("exit   : 1"));
        assert!(!out.contains("live-hud    : enabled"));
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn optics_rails_route_uses_renderer_without_manifest_body() {
        let dir = temp_plugins();
        fs::write(dir.join("optics.wasm"), b"\0asm\x01\0\0\0").unwrap();
        fs::write(
            dir.join("optics.wasi"),
            "name=optics\ncapability=none\nstdout=OPTICS PRO PREVIEW\n",
        )
        .unwrap();

        let out = execute_plugin(&dir, "optics", &["rails".to_string()]);
        assert!(out.contains("plugin : optics"));
        assert!(out.contains("args   : rails"));
        assert!(out.contains("OPTICS HUD RAIL RENDER"));
        assert!(out.contains("CENTER role=command-output chrome=none-permanent"));
        assert!(!out.contains("OPTICS PRO PREVIEW"));
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn arena_builtin_runs_demo() {
        let dir = temp_plugins();
        let out = execute_plugin(&dir, "arena", &["demo".to_string()]);
        assert!(out.contains("phase1 arena"));
        assert!(out.contains("original ASCII"));
        assert!(!out.contains("openDoom"));
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn game_workspace_runs_status() {
        let dir = temp_plugins();
        let out = execute_plugin(&dir, "game", &["status".to_string()]);
        assert!(out.contains("phase1 game workspace"));
        assert!(out.contains("docs/developers/GAME_DEV.md"));
        let _ = fs::remove_dir_all(dir);
    }
}

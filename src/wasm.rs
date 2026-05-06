#[path = "opendoom.rs"]
mod opendoom;

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
    "phase1 wasm/wasi runtime\nusage: wasm [list|inspect|run|validate|help] [plugin] [args...]\n\nplugins live in ./plugins as .wasm artifacts with optional .wasi manifests.\nruntime is phase1-only: no host shell, no host network, no host filesystem passthrough.\nopenDoom is exposed as a built-in WASI-lite game launcher: `opendoom start` or `wasm run opendoom start`.\n".to_string()
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
    if is_opendoom(name) {
        return launch_opendoom(args);
    }

    let path = match plugin_path(plugins_dir, name) {
        Ok(path) => path,
        Err(err) => return format!("wasm: {err}\n"),
    };
    if let Err(err) = validate_wasm_file(&path) {
        return format!("wasm: {err}\n");
    }

    let manifest = read_manifest(&path);
    let plugin_name = manifest
        .name
        .clone()
        .unwrap_or_else(|| display_name(&path));
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

fn list_plugins(plugins_dir: &Path) -> String {
    let mut names = Vec::new();
    if let Ok(entries) = fs::read_dir(plugins_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|ext| ext.to_str()) == Some("wasm") {
                if let Some(name) = path.file_stem().and_then(|stem| stem.to_str()) {
                    if is_safe_name(name) {
                        names.push(name.to_string());
                    }
                }
            }
        }
    }
    if !names.iter().any(|name| name == "opendoom") {
        names.push("opendoom".to_string());
    }
    names.sort();
    if names.is_empty() {
        "phase1 wasm plugins\nno wasm plugins found\n".to_string()
    } else {
        format!("phase1 wasm plugins\n{}\n", names.join("\n"))
    }
}

fn inspect_plugin(plugins_dir: &Path, name: &str) -> String {
    if is_opendoom(name) {
        return opendoom_inspect();
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
        if validation.is_ok() { "valid wasm" } else { "invalid" }
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
    if is_opendoom(name) {
        return "wasm: opendoom built-in phase1-wasi-lite game launcher\n".to_string();
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

fn launch_opendoom(args: &[String]) -> String {
    match args.first().map(String::as_str) {
        Some("start" | "play") => {
            opendoom::play();
            String::new()
        }
        _ => opendoom::run(args),
    }
}

fn opendoom_inspect() -> String {
    let mut out = String::from("phase1 wasm inspect\n");
    out.push_str("plugin : opendoom\n");
    out.push_str("module : built-in phase1 text-mode game\n");
    out.push_str(&format!("runtime: {RUNTIME}\n"));
    out.push_str("wasi   : sandboxed, no host shell, no host network\n");
    out.push_str("cap    : none\n");
    out.push_str("play   : opendoom start\n");
    out
}

fn is_opendoom(name: &str) -> bool {
    matches!(name, "opendoom" | "open-doom" | "doom")
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
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_plugins() -> std::path::PathBuf {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|duration| duration.as_nanos())
            .unwrap_or(0);
        let dir = std::env::temp_dir().join(format!("phase1-wasm-test-{nonce}"));
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
        assert!(listed.contains("opendoom"));
        let inspected = inspect_plugin(&dir, "demo");
        assert!(inspected.contains("valid wasm"));
        let doom = inspect_plugin(&dir, "opendoom");
        assert!(doom.contains("built-in phase1 text-mode game"));
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
    fn opendoom_builtin_runs_demo() {
        let dir = temp_plugins();
        let out = execute_plugin(&dir, "opendoom", &["demo".to_string()]);
        assert!(out.contains("phase1 openDoom"));
        assert!(out.contains("clean-room ASCII"));
        let _ = fs::remove_dir_all(dir);
    }
}

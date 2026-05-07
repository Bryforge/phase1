#[path = "../src/wasm.rs"]
mod wasm;

use std::path::Path;

#[test]
fn gina_plugin_runs_offline() {
    let out = wasm::execute_plugin(Path::new("plugins"), "gina", &["status".to_string()]);
    assert!(out.contains("phase1 wasi run"));
    assert!(out.contains("Gina"));
    assert!(out.contains("offline"));
    assert!(out.contains("host=blocked"));
    assert!(out.contains("sandbox: fs=virtual net=disabled host=blocked"));
}

#[test]
fn ai_plugin_bridges_to_gina_identity() {
    let out = wasm::execute_plugin(Path::new("plugins"), "ai", &["gina".to_string()]);
    assert!(out.contains("Phase1 AI Gina"));
    assert!(out.contains("AI integration layer"));
    assert!(out.contains("no host shell"));
    assert!(out.contains("no host network"));
    assert!(out.contains("no credential access"));
}

#[test]
fn gina_plugin_redacts_sensitive_args() {
    let out = wasm::execute_plugin(
        Path::new("plugins"),
        "gina",
        &[concat!("to", "ken=abc").to_string()],
    );
    assert!(out.contains("[redacted]"));
}

#[test]
fn gina_reports_cybersecurity_baseline() {
    let out = wasm::execute_plugin(Path::new("plugins"), "gina", &["security".to_string()]);
    assert!(out.contains("cybersecurity"));
    assert!(out.contains("safe mode first"));
    assert!(out.contains("policy gates first"));
    assert!(out.contains("redaction first"));
    assert!(out.contains("host shell blocked"));
    assert!(out.contains("host network blocked"));
    assert!(out.contains("host filesystem passthrough blocked"));
}

#[test]
fn gina_reports_optimization_and_consistency_baseline() {
    let out = wasm::execute_plugin(
        Path::new("plugins"),
        "gina",
        &["optimize".to_string(), "consistency".to_string()],
    );
    assert!(out.contains("optimization"));
    assert!(out.contains("built Phase1 binary"));
    assert!(out.contains("deterministic checks"));
    assert!(out.contains("bounded runtime"));
    assert!(out.contains("CI-backed tests"));
    assert!(out.contains("consistency"));
    assert!(out.contains("repeatable commands"));
    assert!(out.contains("test-backed behavior"));
}

#[test]
fn gina_forbids_untrusted_future_provider_defaults() {
    let out = wasm::execute_plugin(Path::new("plugins"), "gina", &["provider".to_string()]);
    assert!(out.contains("future provider-backed AI must require explicit opt-in policy gates"));
    assert!(out.contains("credentials, tokens, keys, cookies, and recovery codes must never be stored"));
    assert!(out.contains("offline"));
    assert!(out.contains("sandboxed"));
    assert!(out.contains("security-first"));
}

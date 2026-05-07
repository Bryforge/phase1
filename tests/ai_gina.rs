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
}

#[test]
fn ai_plugin_bridges_to_gina_identity() {
    let out = wasm::execute_plugin(Path::new("plugins"), "ai", &["gina".to_string()]);
    assert!(out.contains("Phase1 AI Gina"));
    assert!(out.contains("AI integration layer"));
    assert!(out.contains("no host shell"));
    assert!(out.contains("no host network"));
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

#[path = "../src/wasm.rs"]
mod wasm;

use std::fs;
use std::path::Path;

#[test]
fn gina_ai_and_assistant_manifests_are_offline_and_safe() {
    for plugin in ["gina", "ai", "assistant"] {
        let manifest = fs::read_to_string(format!("plugins/{plugin}.wasi"))
            .unwrap_or_else(|err| panic!("missing {plugin} manifest: {err}"));
        assert!(
            manifest.contains("ai.offline"),
            "{plugin} should be offline"
        );
        assert!(
            manifest.contains("no host") || manifest.contains("host shell blocked"),
            "{plugin} should describe host blocking"
        );
        assert!(
            manifest.contains("credential")
                || manifest.contains("no provider")
                || manifest.contains("no external"),
            "{plugin} should describe credential/provider safety"
        );
    }
}

#[test]
fn assistant_alias_runs_through_wasi_lite_without_host_access() {
    let out = wasm::execute_plugin(Path::new("plugins"), "assistant", &["status".to_string()]);
    assert!(out.contains("phase1 wasi run"));
    assert!(out.contains("Assistant"));
    assert!(out.contains("Gina alias active"));
    assert!(out.contains("sandbox: fs=virtual net=disabled host=blocked"));
    assert!(out.contains("cap    : ai.offline"));
}

#[test]
fn gina_and_ai_bridge_still_run_after_queue_cleanup() {
    let gina = wasm::execute_plugin(Path::new("plugins"), "gina", &["security".to_string()]);
    assert!(gina.contains("Phase1 AI operations assistant"));
    assert!(gina.contains("cybersecurity"));
    assert!(gina.contains("host=blocked"));

    let ai = wasm::execute_plugin(Path::new("plugins"), "ai", &["gina".to_string()]);
    assert!(ai.contains("Phase1 AI bridge"));
    assert!(ai.contains("operations-focused"));
    assert!(ai.contains("host=blocked"));
}

#[test]
fn gina_docs_preserve_provider_disabled_default() {
    let doc = fs::read_to_string("junk/experiments/ai/AI_GINA.md")
        .expect("read junk/experiments/ai/AI_GINA.md");
    let roadmap = fs::read_to_string("junk/experiments/ai/AI_GINA_ROADMAP.md")
        .expect("read junk/experiments/ai/AI_GINA_ROADMAP.md");
    assert!(doc.contains("offline by default"));
    assert!(doc.contains("no external provider calls"));
    assert!(doc.contains("assistant"));
    assert!(roadmap.contains("Provider-backed research"));
    assert!(roadmap.contains("provider-disabled default"));
}

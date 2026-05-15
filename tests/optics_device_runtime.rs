use std::io::Write;
use std::process::{Command, Stdio};

fn run_phase1(input: &str) -> String {
    let mut child = Command::new(env!("CARGO_BIN_EXE_phase1"))
        .env("PHASE1_TEST_MODE", "1")
        .env("PHASE1_PERSISTENT_STATE", "0")
        .env("PHASE1_COOKED_INPUT", "1")
        .env("PHASE1_NO_COLOR", "1")
        .env("PHASE1_ASCII", "1")
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
    format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    )
}

#[test]
fn optics_device_preview_routes_render_all_profiles() {
    let output = run_phase1(
        "optics device mobile\noptics device laptop\noptics device desktop\noptics device terminal\nexit\n",
    );

    for device in ["mobile", "laptop", "desktop", "terminal"] {
        for required in [
            "phase1 wasi run",
            "plugin : optics",
            "runtime: phase1-wasi-lite",
            "sandbox: fs=virtual net=disabled host=blocked",
            "cap    : none",
            "OPTICS DEVICE PREVIEW",
            "mode        : preview-only",
            "renderer    : rust-static-renderer",
            "live-hud    : disabled",
            "OPTICS HUD RAIL RENDER",
            "status : ok",
            "exit   : 0",
        ] {
            assert!(output.contains(required), "missing {required:?}:\n{output}");
        }
        assert!(
            output.contains(&format!("args   : device {device}")),
            "{output}"
        );
        assert!(
            output.contains(&format!("device      : {device}")),
            "{output}"
        );
        assert!(output.contains(&format!("device={device}")), "{output}");
    }
}

#[test]
fn optics_device_preview_rejects_missing_and_invalid_profiles() {
    let output = run_phase1("optics device\noptics device wallpaper\nexit\n");

    for required in [
        "OPTICS DEVICE PREVIEW",
        "result      : missing-device",
        "result      : invalid-device",
        "requested   : wallpaper",
        "usage       : optics device mobile|laptop|desktop|terminal",
        "supported   : mobile,laptop,desktop,terminal",
        "status : failed",
        "exit   : 1",
    ] {
        assert!(output.contains(required), "missing {required:?}:\n{output}");
    }

    for forbidden in [
        "live-hud    : enabled",
        "runtime=wired",
        "security-boundary claimed",
        "crypto-enforcement claimed",
        "system-integrity-guarantee claimed",
        "host=enabled",
    ] {
        assert!(
            !output.contains(forbidden),
            "forbidden {forbidden:?}:\n{output}"
        );
    }
}

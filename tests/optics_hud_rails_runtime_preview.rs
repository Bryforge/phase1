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
fn optics_rails_preview_runs_through_read_only_wasi_lite_surface() {
    let output = run_phase1("optics rails\nexit\n");

    for required in [
        "phase1 wasi run",
        "plugin : optics",
        "runtime: phase1-wasi-lite",
        "sandbox: fs=virtual net=disabled host=blocked",
        "cap    : none",
        "args   : rails",
        "OPTICS HUD RAIL RENDER",
        "status=static-render",
        "runtime=not-wired",
        "TOP product=Phase1 channel=edge profile=PRO",
        "ctx=root > nest:0/1 > portal:none > ghost:none",
        "origin=0/0",
        "route=ROOT",
        "TOP integrity=not-checked crypto=chain-planned",
        "safe-portal=planned",
        "rollback=available",
        "CENTER role=command-output chrome=none-permanent",
        "CENTER rule=center-remains-primary-workspace",
        "CENTER sample=phase1://edge/root > optics rails preview",
        "BOT color=bright-blue input=active mutation=none command=none task=idle result=ok",
        "BOT warning=none safe-portal=planned rollback=available copy-safe=raw-command-preserved",
        "not-security-boundary",
        "not-crypto-enforcement",
        "not-system-integrity-guarantee",
        "not-base1-boot-environment",
        "status : ok",
        "exit   : 0",
    ] {
        assert!(output.contains(required), "missing {required:?}:\n{output}");
    }

    for forbidden in [
        "OPTICS PRO PREVIEW",
        "runtime=wired",
        "security-boundary claimed",
        "crypto-enforcement claimed",
        "system-integrity-guarantee claimed",
        "base1 boot environment claimed",
        "host=enabled",
    ] {
        assert!(
            !output.contains(forbidden),
            "forbidden {forbidden:?}:\n{output}"
        );
    }
}

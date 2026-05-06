use std::fs;
use std::io::Write;
use std::process::{Command, Stdio};
use std::time::{SystemTime, UNIX_EPOCH};

fn run_phase1(script: &str) -> String {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_nanos())
        .unwrap_or(0);
    let run_dir = std::env::temp_dir().join(format!("phase1-game-{nonce}"));
    let _ = fs::remove_dir_all(&run_dir);
    fs::create_dir_all(&run_dir).expect("create game test dir");

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

    let input = format!("\n{script}");
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
fn phase1_arena_workspace_commands_are_visible() {
    let output = run_phase1(
        "wasm list\nwasm inspect arena\nwasm run arena demo\nwasm run game status\nwasm run game files\nwasm run game test-plan\nwasm run game roadmap\nexit\n",
    );

    assert!(output.contains("arena"), "arena plugin missing:\n{output}");
    assert!(output.contains("game"), "game workspace missing:\n{output}");
    assert!(
        output.contains("Phase1 Arena"),
        "game title missing:\n{output}"
    );
    assert!(
        output.contains("phase1 arena"),
        "arena renderer missing:\n{output}"
    );
    assert!(
        output.contains("original ASCII"),
        "asset safety marker missing:\n{output}"
    );
    assert!(
        output.contains("GAME_DEV.md"),
        "game dev doc missing:\n{output}"
    );
    assert!(
        output.contains("scripts/test-game.sh"),
        "game runner missing:\n{output}"
    );
    assert!(
        output.contains("src/arena.rs"),
        "game module missing:\n{output}"
    );
    assert!(
        output.contains("phase1 game roadmap"),
        "game roadmap missing:\n{output}"
    );
    assert!(
        !output.contains("phase1 openDoom"),
        "old game name leaked:\n{output}"
    );
}

#[test]
fn phase1_arena_scripted_runs_are_deterministic() {
    let output = run_phase1(
        "wasm run arena script d d d fire map\nwasm run arena script s s s s s s\nexit\n",
    );

    assert!(
        output.contains("phase1 arena scripted run"),
        "scripted run banner missing:\n{output}"
    );
    assert!(
        output.contains("arena> fire"),
        "fire command missing:\n{output}"
    );
    assert!(output.contains("@"), "player marker missing:\n{output}");
    assert!(output.contains("M"), "hostile marker missing:\n{output}");
    assert!(
        output.contains("status=live"),
        "live status missing:\n{output}"
    );
    assert!(
        !output.contains("openDoom"),
        "old game name leaked:\n{output}"
    );
}

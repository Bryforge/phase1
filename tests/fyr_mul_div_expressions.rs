use std::fs;
use std::io::Write;
use std::process::{self, Command, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

static RUN_COUNTER: AtomicU64 = AtomicU64::new(0);

fn run_phase1(input: &str) -> String {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_nanos())
        .unwrap_or(0);
    let seq = RUN_COUNTER.fetch_add(1, Ordering::Relaxed);
    let run_dir = std::env::temp_dir().join(format!(
        "phase1-fyr-mul-div-{}-{nonce}-{seq}",
        process::id()
    ));
    let _ = fs::remove_dir_all(&run_dir);
    fs::create_dir_all(&run_dir).expect("create fyr mul div test dir");

    let mut child = Command::new(env!("CARGO_BIN_EXE_phase1"))
        .current_dir(&run_dir)
        .env("PHASE1_TEST_MODE", "1")
        .env("PHASE1_PERSISTENT_STATE", "0")
        .env_remove("PHASE1_THEME")
        .env_remove("PHASE1_ALLOW_HOST_TOOLS")
        .env_remove("PHASE1_SAFE_MODE")
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
    let _ = fs::remove_dir_all(&run_dir);
    format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    )
}

#[test]
fn fyr_let_bindings_support_multiplication_expression() {
    let output = run_phase1(
        "fyr init app\n\
echo 'fn main() -> i32 { let base = 21; let answer = base * 2; assert(answer == 42); return answer; }' > app/tests/math_mul.fyr\n\
fyr check app\n\
fyr test app\n\
exit\n",
    );

    assert!(
        output.contains("fyr check: ok app/src/main.fyr"),
        "{output}"
    );
    assert!(
        output.contains("test    : app/tests/math_mul.fyr ok"),
        "{output}"
    );
    assert!(output.contains("status  : ok"), "{output}");
}

#[test]
fn fyr_let_bindings_support_division_expression() {
    let output = run_phase1(
        "fyr init app\n\
echo 'fn main() -> i32 { let total = 84; let answer = total / 2; assert_eq(answer, 42); return answer; }' > app/tests/math_div.fyr\n\
fyr check app\n\
fyr test app\n\
exit\n",
    );

    assert!(
        output.contains("fyr check: ok app/src/main.fyr"),
        "{output}"
    );
    assert!(
        output.contains("test    : app/tests/math_div.fyr ok"),
        "{output}"
    );
    assert!(output.contains("status  : ok"), "{output}");
}

#[test]
fn fyr_division_by_zero_reports_diagnostic() {
    let output = run_phase1(
        "fyr init app\n\
echo 'fn main() -> i32 { let total = 84; let answer = total / 0; assert(answer == 42); return answer; }' > app/tests/math_bad_div.fyr\n\
fyr test app\n\
exit\n",
    );

    assert!(
        output.contains("test    : app/tests/math_bad_div.fyr failed: division by zero"),
        "{output}"
    );
    assert!(output.contains("status  : failed"), "{output}");
}

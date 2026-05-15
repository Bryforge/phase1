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
        "phase1-fyr-whitespace-equivalence-{}-{nonce}-{seq}",
        process::id()
    ));
    let _ = fs::remove_dir_all(&run_dir);
    fs::create_dir_all(&run_dir).expect("create fyr whitespace equivalence test dir");

    let mut child = Command::new(env!("CARGO_BIN_EXE_phase1"))
        .current_dir(&run_dir)
        .env("PHASE1_TEST_MODE", "1")
        .env("PHASE1_PERSISTENT_STATE", "0")
        .env("PHASE1_COOKED_INPUT", "1")
        .env("PHASE1_NO_COLOR", "1")
        .env("PHASE1_ASCII", "1")
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
fn fyr_let_expression_spacing_forms_are_equivalent() {
    let output = run_phase1(
        "fyr init app\n\
echo 'fn main() -> i32 { let x=1+2; assert_eq(x,3); return x; }' > app/tests/compact.fyr\n\
echo 'fn main() -> i32 { let x = 1 + 2; assert_eq(x, 3); return x; }' > app/tests/spaced.fyr\n\
echo 'fn main() -> i32 { let   x   =   1   +   2; assert_eq( x , 3 ); return x; }' > app/tests/padded.fyr\n\
fyr check app\n\
fyr test app\n\
exit\n",
    );

    assert!(
        output.contains("fyr check: ok app/src/main.fyr"),
        "{output}"
    );
    for test in ["compact", "spaced", "padded"] {
        assert!(
            output.contains(&format!("test    : app/tests/{test}.fyr ok")),
            "{output}"
        );
    }
    assert!(output.contains("passed  : 4"), "{output}");
    assert!(output.contains("failed  : 0"), "{output}");
    assert!(output.contains("status  : ok"), "{output}");
}

#[test]
fn fyr_boolean_and_grouping_spacing_forms_are_equivalent() {
    let output = run_phase1(
        "fyr init app\n\
echo 'fn main() -> i32 { let answer=42; if (answer>40&&answer<50){return answer;} return 0; }' > app/tests/tight_bool.fyr\n\
echo 'fn main() -> i32 { let answer = 42; if ( answer > 40 && answer < 50 ) { return answer; } return 0; }' > app/tests/spaced_bool.fyr\n\
fyr check app\n\
fyr test app\n\
exit\n",
    );

    assert!(
        output.contains("fyr check: ok app/src/main.fyr"),
        "{output}"
    );
    assert!(
        output.contains("test    : app/tests/tight_bool.fyr ok"),
        "{output}"
    );
    assert!(
        output.contains("test    : app/tests/spaced_bool.fyr ok"),
        "{output}"
    );
    assert!(output.contains("passed  : 3"), "{output}");
    assert!(output.contains("failed  : 0"), "{output}");
    assert!(output.contains("status  : ok"), "{output}");
}

#[test]
fn fyr_string_literal_spacing_is_preserved() {
    let output = run_phase1(
        "echo 'fn main() -> i32 { print(\"hello   world\"); return 0; }' > literal.fyr\n\
fyr check literal.fyr\n\
fyr run literal.fyr\n\
exit\n",
    );

    assert!(output.contains("fyr check: ok literal.fyr"), "{output}");
    assert!(output.contains("hello   world"), "{output}");
}

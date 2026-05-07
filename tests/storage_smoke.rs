use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn storage_status_is_read_only_and_guarded_by_default() {
    let sandbox = TestSandbox::new("status");
    let output = phase1_storage(sandbox.path())
        .args(["storage", "status"])
        .env("PHASE1_SAFE_MODE", "1")
        .env_remove("PHASE1_ALLOW_HOST_TOOLS")
        .output()
        .expect("run phase1-storage storage status");

    assert_success(&output, "storage status");
    let text = output_text(&output);
    assert_contains_all(
        &text,
        &["storage root", "exists", "repos", "host tools", "guarded"],
    );
}

#[test]
fn mutating_storage_git_and_rust_actions_are_blocked_by_default() {
    let sandbox = TestSandbox::new("guarded");
    let cases: &[&[&str]] = &[
        &["storage", "init"],
        &["storage", "doctor"],
        &["git", "clone", "https://github.com/Bryforge/phase1.git"],
        &["rust", "version"],
        &["rust", "run", "fn main() { println!(\"blocked\"); }"],
        &["rust", "init", "demo"],
    ];

    for args in cases {
        let output = phase1_storage(sandbox.path())
            .args(*args)
            .env("PHASE1_SAFE_MODE", "1")
            .env_remove("PHASE1_ALLOW_HOST_TOOLS")
            .output()
            .expect("run guarded phase1-storage command");
        assert!(
            !output.status.success(),
            "guarded command unexpectedly succeeded: {args:?}\n{}",
            output_text(&output)
        );
        let text = output_text(&output);
        assert!(
            text.contains("disabled by safe boot profile"),
            "guarded command did not explain safe-mode block: {args:?}\n{text}"
        );
    }
}

#[test]
fn storage_init_and_rust_init_work_only_after_explicit_trust_gate() {
    let sandbox = TestSandbox::new("trusted-init");
    let storage_root = sandbox.path().join("workspace");

    let init = phase1_storage(sandbox.path())
        .args(["storage", "init"])
        .env("PHASE1_SAFE_MODE", "0")
        .env("PHASE1_ALLOW_HOST_TOOLS", "1")
        .env("PHASE1_STORAGE_ROOT", &storage_root)
        .output()
        .expect("run trusted storage init");
    assert_success(&init, "trusted storage init");
    let init_text = output_text(&init);
    assert_contains_all(
        &init_text,
        &["storage: initialized", "repos", "build", "tmp"],
    );
    assert!(storage_root.join("repos").is_dir(), "repos dir missing");
    assert!(storage_root.join("build").is_dir(), "build dir missing");
    assert!(storage_root.join("tmp").is_dir(), "tmp dir missing");

    let rust_init = phase1_storage(sandbox.path())
        .args(["rust", "init", "demo_app"])
        .env("PHASE1_SAFE_MODE", "0")
        .env("PHASE1_ALLOW_HOST_TOOLS", "1")
        .env("PHASE1_STORAGE_ROOT", &storage_root)
        .output()
        .expect("run trusted rust init");
    assert_success(&rust_init, "trusted rust init");
    let rust_text = output_text(&rust_init);
    assert!(rust_text.contains("rust: created project"), "{rust_text}");
    assert!(
        storage_root.join("repos/demo_app/Cargo.toml").is_file(),
        "Cargo.toml missing after rust init"
    );
    assert!(
        storage_root.join("repos/demo_app/src/main.rs").is_file(),
        "main.rs missing after rust init"
    );

    let list = phase1_storage(sandbox.path())
        .args(["storage", "list"])
        .env("PHASE1_STORAGE_ROOT", &storage_root)
        .output()
        .expect("list storage repositories");
    assert_success(&list, "storage list");
    assert!(output_text(&list).contains("demo_app"));
}

#[test]
fn repository_and_cargo_inputs_are_validated() {
    let sandbox = TestSandbox::new("validation");
    let storage_root = sandbox.path().join("workspace");

    let bad_project = phase1_storage(sandbox.path())
        .args(["rust", "init", "../escape"])
        .env("PHASE1_SAFE_MODE", "0")
        .env("PHASE1_ALLOW_HOST_TOOLS", "1")
        .env("PHASE1_STORAGE_ROOT", &storage_root)
        .output()
        .expect("run invalid rust init");
    assert!(
        !bad_project.status.success(),
        "invalid project name unexpectedly succeeded"
    );
    assert!(output_text(&bad_project).contains("invalid repository/project name"));

    let bad_url = phase1_storage(sandbox.path())
        .args(["git", "clone", "file:///etc/passwd"])
        .env("PHASE1_SAFE_MODE", "0")
        .env("PHASE1_ALLOW_HOST_TOOLS", "1")
        .env("PHASE1_STORAGE_ROOT", &storage_root)
        .output()
        .expect("run invalid git clone");
    assert!(
        !bad_url.status.success(),
        "invalid git URL unexpectedly succeeded"
    );
    assert!(output_text(&bad_url).contains("URL must be a normal git remote URL"));
}

#[test]
fn language_roadmap_names_major_runtime_families() {
    let sandbox = TestSandbox::new("roadmap");
    let output = phase1_storage(sandbox.path())
        .args(["lang", "roadmap"])
        .output()
        .expect("run language roadmap");

    assert_success(&output, "language roadmap");
    let text = output_text(&output);
    assert_contains_all(
        &text,
        &[
            "Rust",
            "Git",
            "Python",
            "JavaScript",
            "TypeScript",
            "Go",
            "Java",
            "C#",
            "Swift",
            "WebAssembly",
        ],
    );
}

fn phase1_storage(current_dir: &Path) -> Command {
    let mut command = Command::new(env!("CARGO_BIN_EXE_phase1-storage"));
    command.current_dir(current_dir);
    command
}

fn assert_success(output: &Output, label: &str) {
    assert!(
        output.status.success(),
        "{label} failed with status {:?}\n{}",
        output.status.code(),
        output_text(output)
    );
}

fn assert_contains_all(text: &str, needles: &[&str]) {
    for needle in needles {
        assert!(text.contains(needle), "missing {needle:?}\n{text}");
    }
}

fn output_text(output: &Output) -> String {
    let mut text = String::new();
    text.push_str(&String::from_utf8_lossy(&output.stdout));
    text.push_str(&String::from_utf8_lossy(&output.stderr));
    text
}

struct TestSandbox {
    path: PathBuf,
}

impl TestSandbox {
    fn new(label: &str) -> Self {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "phase1-storage-smoke-{label}-{}-{nonce}",
            std::process::id()
        ));
        fs::create_dir_all(&path).expect("create test sandbox");
        Self { path }
    }

    fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for TestSandbox {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}

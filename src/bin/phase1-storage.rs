use std::env;
use std::ffi::OsStr;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

const DEFAULT_STORAGE_ROOT: &str = "phase1.workspace";
const COMMAND_TIMEOUT: Duration = Duration::from_secs(60);
const RUST_RUN_TIMEOUT: Duration = Duration::from_secs(30);
const MAX_OUTPUT_BYTES: usize = 24 * 1024;

fn main() {
    let args = env::args().skip(1).collect::<Vec<_>>();
    let status = match run(&args) {
        Ok(output) => {
            print!("{output}");
            0
        }
        Err(err) => {
            eprintln!("phase1-storage: {err}");
            1
        }
    };
    std::process::exit(status);
}

fn run(args: &[String]) -> Result<String, String> {
    match args.first().map(String::as_str) {
        None | Some("help") | Some("-h") | Some("--help") => Ok(help()),
        Some("storage") | Some("store") | Some("workspace") => storage(&args[1..]),
        Some("git") => git(&args[1..]),
        Some("rust") | Some("rs") | Some("cargo") => rust(&args[1..]),
        Some("lang") | Some("languages") => language(&args[1..]),
        Some(other) => Err(format!("unknown command '{other}'\n\n{}", help())),
    }
}

fn help() -> String {
    "phase1-storage // guarded storage, Git, Rust, and language roadmap helper\n\nusage:\n  phase1-storage storage status\n  phase1-storage storage init\n  phase1-storage storage list\n  phase1-storage git clone <url> [name]\n  phase1-storage git status <name>\n  phase1-storage git pull <name>\n  phase1-storage rust version\n  phase1-storage rust run <file.rs | inline-code>\n  phase1-storage rust init <name>\n  phase1-storage rust cargo <repo> <check|build|test|run> [args...]\n  phase1-storage lang roadmap\n\nsafety:\n  Read-only status commands are always available. Mutating storage, Git, and Rust host execution require:\n    PHASE1_SAFE_MODE=0 PHASE1_ALLOW_HOST_TOOLS=1\n\nstorage root:\n  PHASE1_STORAGE_ROOT overrides the default phase1.workspace directory.\n"
        .to_string()
}

fn storage(args: &[String]) -> Result<String, String> {
    match args.first().map(String::as_str).unwrap_or("status") {
        "status" | "info" => storage_status(),
        "path" => Ok(format!("{}\n", storage_root().display())),
        "init" => {
            require_host_tools("storage init")?;
            ensure_storage_tree()?;
            Ok(format!(
                "storage: initialized {}\nrepos: {}\nbuild: {}\ntmp  : {}\n",
                storage_root().display(),
                repos_dir().display(),
                build_dir().display(),
                tmp_dir().display()
            ))
        }
        "list" | "ls" => list_repositories(),
        "doctor" => {
            require_host_tools("storage doctor")?;
            storage_doctor()
        }
        "help" | "-h" | "--help" => Ok(help()),
        other => Err(format!("storage: unknown action '{other}'")),
    }
}

fn git(args: &[String]) -> Result<String, String> {
    let Some(action) = args.first().map(String::as_str) else {
        return Err("usage: phase1-storage git <clone|status|pull|list> ...".to_string());
    };
    match action {
        "clone" => {
            require_host_tools("git clone")?;
            let url = args.get(1).ok_or("usage: phase1-storage git clone <url> [name]")?;
            validate_git_url(url)?;
            let name = match args.get(2) {
                Some(name) => validate_repo_name(name)?,
                None => derive_repo_name(url)?,
            };
            ensure_storage_tree()?;
            let destination = repos_dir().join(&name);
            if destination.exists() {
                return Err(format!("git clone: destination already exists: {}", destination.display()));
            }
            let mut cmd = Command::new("git");
            cmd.env("GIT_TERMINAL_PROMPT", "0")
                .env("GCM_INTERACTIVE", "never")
                .arg("clone")
                .arg("--depth")
                .arg("1")
                .arg(url)
                .arg(&destination);
            let output = run_command(cmd, COMMAND_TIMEOUT)?;
            Ok(format!(
                "git: cloned {url}\nrepo: {}\n{}",
                destination.display(),
                format_output(output)
            ))
        }
        "status" => {
            require_host_tools("git status")?;
            let repo = repo_path(args.get(1).ok_or("usage: phase1-storage git status <name>")?)?;
            let mut cmd = Command::new("git");
            cmd.env("GIT_TERMINAL_PROMPT", "0")
                .env("GCM_INTERACTIVE", "never")
                .arg("status")
                .arg("--short")
                .current_dir(repo);
            Ok(format_output(run_command(cmd, COMMAND_TIMEOUT)?))
        }
        "pull" => {
            require_host_tools("git pull")?;
            let repo = repo_path(args.get(1).ok_or("usage: phase1-storage git pull <name>")?)?;
            let mut cmd = Command::new("git");
            cmd.env("GIT_TERMINAL_PROMPT", "0")
                .env("GCM_INTERACTIVE", "never")
                .arg("pull")
                .arg("--ff-only")
                .current_dir(repo);
            Ok(format_output(run_command(cmd, COMMAND_TIMEOUT)?))
        }
        "list" | "ls" => list_repositories(),
        "help" | "-h" | "--help" => Ok(help()),
        other => Err(format!("git: unknown action '{other}'")),
    }
}

fn rust(args: &[String]) -> Result<String, String> {
    let Some(action) = args.first().map(String::as_str) else {
        return Err("usage: phase1-storage rust <version|run|init|cargo> ...".to_string());
    };
    match action {
        "version" | "doctor" => {
            require_host_tools("rust version")?;
            let rustc = run_command(Command::new("rustc").arg("--version"), COMMAND_TIMEOUT)?;
            let cargo = run_command(Command::new("cargo").arg("--version"), COMMAND_TIMEOUT)?;
            Ok(format!("{}{}", format_output(rustc), format_output(cargo)))
        }
        "run" => {
            require_host_tools("rust run")?;
            let source = args.get(1).ok_or("usage: phase1-storage rust run <file.rs | inline-code>")?;
            run_rust_source(source)
        }
        "init" | "new" => {
            require_host_tools("rust init")?;
            let name = validate_repo_name(args.get(1).ok_or("usage: phase1-storage rust init <name>")?)?;
            ensure_storage_tree()?;
            let project = repos_dir().join(&name);
            if project.exists() {
                return Err(format!("rust init: project already exists: {}", project.display()));
            }
            fs::create_dir_all(project.join("src")).map_err(|err| err.to_string())?;
            fs::write(
                project.join("Cargo.toml"),
                format!(
                    "[package]\nname = \"{}\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[dependencies]\n",
                    cargo_package_name(&name)
                ),
            )
            .map_err(|err| err.to_string())?;
            fs::write(
                project.join("src/main.rs"),
                "fn main() {\n    println!(\"hello from phase1 rust workspace\");\n}\n",
            )
            .map_err(|err| err.to_string())?;
            Ok(format!("rust: created project {}\n", project.display()))
        }
        "cargo" => {
            require_host_tools("cargo")?;
            let repo = repo_path(args.get(1).ok_or("usage: phase1-storage rust cargo <repo> <check|build|test|run> [args...]")?)?;
            let subcommand = args.get(2).ok_or("usage: phase1-storage rust cargo <repo> <check|build|test|run> [args...]")?;
            validate_cargo_subcommand(subcommand)?;
            let mut cmd = Command::new("cargo");
            cmd.arg(subcommand).args(&args[3..]).current_dir(repo);
            Ok(format_output(run_command(cmd, Duration::from_secs(120))?))
        }
        "help" | "-h" | "--help" => Ok(help()),
        other => Err(format!("rust: unknown action '{other}'")),
    }
}

fn language(args: &[String]) -> Result<String, String> {
    match args.first().map(String::as_str).unwrap_or("roadmap") {
        "roadmap" | "list" | "status" => Ok(language_roadmap()),
        other => Err(format!("lang: unknown action '{other}'")),
    }
}

fn storage_status() -> Result<String, String> {
    let root = storage_root();
    let exists = root.exists();
    let repo_count = if repos_dir().exists() {
        fs::read_dir(repos_dir())
            .map_err(|err| err.to_string())?
            .filter_map(Result::ok)
            .filter(|entry| entry.path().is_dir())
            .count()
    } else {
        0
    };
    Ok(format!(
        "storage root : {}\nexists       : {}\nrepos        : {}\nhost tools   : {}\n",
        root.display(),
        if exists { "yes" } else { "no" },
        repo_count,
        if host_tools_allowed() { "enabled" } else { "guarded" }
    ))
}

fn storage_doctor() -> Result<String, String> {
    let mut out = storage_status()?;
    out.push_str("\nrequired tools:\n");
    for tool in ["git", "rustc", "cargo"] {
        let mut cmd = Command::new(tool);
        cmd.arg("--version");
        match run_command(cmd, COMMAND_TIMEOUT) {
            Ok(output) => out.push_str(&format!("  {tool}: {}", first_line(&format_output(output)))),
            Err(err) => out.push_str(&format!("  {tool}: missing or blocked ({err})\n")),
        }
    }
    Ok(out)
}

fn list_repositories() -> Result<String, String> {
    let root = repos_dir();
    if !root.exists() {
        return Ok("storage: no repositories yet; run phase1-storage storage init\n".to_string());
    }
    let mut names = fs::read_dir(&root)
        .map_err(|err| err.to_string())?
        .filter_map(Result::ok)
        .filter(|entry| entry.path().is_dir())
        .filter_map(|entry| entry.file_name().to_str().map(ToOwned::to_owned))
        .collect::<Vec<_>>();
    names.sort();
    if names.is_empty() {
        Ok("storage: no repositories yet\n".to_string())
    } else {
        Ok(format!("{}\n", names.join("\n")))
    }
}

fn run_rust_source(source: &str) -> Result<String, String> {
    ensure_storage_tree()?;
    let code = if Path::new(source).exists() {
        fs::read_to_string(source).map_err(|err| err.to_string())?
    } else {
        source.to_string()
    };
    if !code.contains("fn main") {
        return Err("rust run: source must contain fn main".to_string());
    }
    let nonce = unique_nonce();
    let source_path = tmp_dir().join(format!("phase1_rust_{nonce}.rs"));
    let binary_path = build_dir().join(format!("phase1_rust_{nonce}"));
    fs::write(&source_path, code).map_err(|err| err.to_string())?;

    let mut compile = Command::new("rustc");
    compile.arg("--edition=2021").arg(&source_path).arg("-o").arg(&binary_path);
    let compile_output = run_command(compile, COMMAND_TIMEOUT)?;
    if !compile_output.status.success() {
        let _ = fs::remove_file(source_path);
        return Ok(format_output(compile_output));
    }

    let run_output = run_command(Command::new(&binary_path), RUST_RUN_TIMEOUT)?;
    let _ = fs::remove_file(source_path);
    let _ = fs::remove_file(binary_path);
    Ok(format_output(run_output))
}

fn storage_root() -> PathBuf {
    env::var_os("PHASE1_STORAGE_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(DEFAULT_STORAGE_ROOT))
}

fn repos_dir() -> PathBuf {
    storage_root().join("repos")
}

fn build_dir() -> PathBuf {
    storage_root().join("build")
}

fn tmp_dir() -> PathBuf {
    storage_root().join("tmp")
}

fn ensure_storage_tree() -> Result<(), String> {
    fs::create_dir_all(repos_dir()).map_err(|err| err.to_string())?;
    fs::create_dir_all(build_dir()).map_err(|err| err.to_string())?;
    fs::create_dir_all(tmp_dir()).map_err(|err| err.to_string())?;
    Ok(())
}

fn repo_path(name: &str) -> Result<PathBuf, String> {
    let name = validate_repo_name(name)?;
    let path = repos_dir().join(name);
    if !path.exists() {
        return Err(format!("repo not found: {}", path.display()));
    }
    Ok(path)
}

fn validate_git_url(url: &str) -> Result<(), String> {
    let allowed = url.starts_with("https://")
        || url.starts_with("http://")
        || url.starts_with("git://")
        || url.starts_with("ssh://")
        || url.starts_with("git@");
    if allowed && !url.contains(' ') && !url.contains('\n') && !url.contains('\r') {
        Ok(())
    } else {
        Err("git clone: URL must be a normal git remote URL".to_string())
    }
}

fn derive_repo_name(url: &str) -> Result<String, String> {
    let tail = url
        .rsplit(['/', ':'])
        .next()
        .ok_or("git clone: could not derive repository name")?;
    let trimmed = tail.strip_suffix(".git").unwrap_or(tail);
    validate_repo_name(trimmed)
}

fn validate_repo_name(name: &str) -> Result<String, String> {
    let valid = !name.is_empty()
        && name != "."
        && name != ".."
        && name.len() <= 80
        && name
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_' | '.'));
    if valid {
        Ok(name.to_string())
    } else {
        Err(format!("invalid repository/project name: {name}"))
    }
}

fn cargo_package_name(name: &str) -> String {
    name.chars()
        .map(|ch| if ch == '.' { '-' } else { ch })
        .collect()
}

fn validate_cargo_subcommand(subcommand: &str) -> Result<(), String> {
    match subcommand {
        "check" | "build" | "test" | "run" => Ok(()),
        other => Err(format!("cargo subcommand not allowed here: {other}")),
    }
}

fn require_host_tools(command: &str) -> Result<(), String> {
    if host_tools_allowed() {
        Ok(())
    } else if safe_mode_enabled() {
        Err(format!("{command}: disabled by safe boot profile; set PHASE1_SAFE_MODE=0 and PHASE1_ALLOW_HOST_TOOLS=1 for trusted host-backed use"))
    } else {
        Err(format!("{command}: disabled; set PHASE1_ALLOW_HOST_TOOLS=1 for trusted host-backed use"))
    }
}

fn safe_mode_enabled() -> bool {
    !matches!(env::var("PHASE1_SAFE_MODE").ok().as_deref(), Some("0" | "false" | "off" | "no"))
}

fn host_tools_allowed() -> bool {
    !safe_mode_enabled() && env::var("PHASE1_ALLOW_HOST_TOOLS").ok().as_deref() == Some("1")
}

fn run_command(mut command: Command, timeout: Duration) -> io::Result<Output> {
    let mut child = command
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    let start = Instant::now();
    loop {
        if child.try_wait()?.is_some() {
            return child.wait_with_output();
        }
        if start.elapsed() >= timeout {
            let _ = child.kill();
            let _ = child.wait();
            return Err(io::Error::new(io::ErrorKind::TimedOut, "command timed out"));
        }
        thread::sleep(Duration::from_millis(25));
    }
}

fn format_output(output: Output) -> String {
    let mut text = String::new();
    text.push_str(&String::from_utf8_lossy(&output.stdout));
    text.push_str(&String::from_utf8_lossy(&output.stderr));
    if text.len() > MAX_OUTPUT_BYTES {
        text.truncate(MAX_OUTPUT_BYTES);
        text.push_str("\n[output truncated by phase1-storage]\n");
    }
    sanitize_output(&text)
}

fn sanitize_output(raw: &str) -> String {
    raw.lines()
        .map(|line| {
            if line.to_ascii_lowercase().contains("token=")
                || line.to_ascii_lowercase().contains("password=")
                || line.to_ascii_lowercase().contains("authorization:")
            {
                "[redacted sensitive output]".to_string()
            } else {
                line.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
        + "\n"
}

fn first_line(text: &str) -> String {
    text.lines().next().unwrap_or("unknown").to_string() + "\n"
}

fn unique_nonce() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_nanos())
        .unwrap_or(0)
}

fn language_roadmap() -> String {
    "phase1 major language support roadmap\n\nTier 0 - available now\n  Rust: rustc compile/run, cargo check/build/test/run through guarded host tools\n  Git: guarded clone/status/pull into phase1.workspace/repos\n\nTier 1 - near-term first-class shells\n  Python: existing guarded python command plus package-aware project runner\n  C/C++: existing gcc/cc path, expand to clang/g++, CMake, and compile databases\n  JavaScript/TypeScript: node, npm, pnpm, bun, deno runners with lockfile awareness\n  Go: go run/test/build and module cache isolation\n\nTier 2 - managed runtimes\n  Java/Kotlin: javac, gradle, maven, kotlin compiler wrappers\n  C#: dotnet build/test/run workspace support\n  Swift: swift build/test/run where host toolchain exists\n  PHP/Ruby: composer, phpunit, bundle, rake, rspec guarded runners\n\nTier 3 - data, science, and systems expansion\n  R, Julia, Lua, Perl, Zig, Elixir/Erlang, Scala, Haskell, OCaml, Dart, and WebAssembly/WASI\n\nCross-cutting requirements\n  per-language capability metadata, timeout limits, output redaction, workspace isolation, lockfile detection, smoke tests, and docs\n"
        .to_string()
}

#[allow(dead_code)]
fn _flush_stdout() {
    let _ = io::stdout().flush();
}

#[allow(dead_code)]
fn _as_os_str(path: &Path) -> &OsStr {
    path.as_os_str()
}

#[cfg(test)]
mod tests {
    use super::{derive_repo_name, language_roadmap, sanitize_output, validate_repo_name};

    #[test]
    fn derives_safe_repo_names_from_common_git_urls() {
        assert_eq!(derive_repo_name("https://github.com/Bryforge/phase1.git").unwrap(), "phase1");
        assert_eq!(derive_repo_name("git@github.com:Bryforge/phase1.git").unwrap(), "phase1");
    }

    #[test]
    fn rejects_path_traversal_repo_names() {
        assert!(validate_repo_name("phase1").is_ok());
        assert!(validate_repo_name("../phase1").is_err());
        assert!(validate_repo_name("phase one").is_err());
    }

    #[test]
    fn redacts_sensitive_output_markers() {
        let out = sanitize_output("ok\ntoken=secret\nAuthorization: bearer nope\n");
        assert!(out.contains("ok"));
        assert!(out.contains("[redacted sensitive output]"));
        assert!(!out.contains("secret"));
        assert!(!out.contains("bearer nope"));
    }

    #[test]
    fn roadmap_names_major_language_families() {
        let roadmap = language_roadmap();
        for language in ["Rust", "Python", "JavaScript", "TypeScript", "Go", "Java", "C#", "Swift", "WebAssembly"] {
            assert!(roadmap.contains(language));
        }
    }
}

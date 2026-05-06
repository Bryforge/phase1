use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use crate::commands::Phase1Shell;

const MAX_SOURCE_BYTES: usize = 256 * 1024;
const RUN_TIMEOUT: Duration = Duration::from_secs(8);
const COMPILE_TIMEOUT: Duration = Duration::from_secs(20);
const MAX_OUTPUT_BYTES: usize = 24 * 1024;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Runner {
    Interpreted {
        tools: &'static [&'static str],
        args: &'static [&'static str],
    },
    CompileRun {
        tools: &'static [&'static str],
        compile_args: &'static [&'static str],
    },
    Rust,
    Go,
    JavaSource,
    Kotlin,
    Dotnet,
    WasmInfo,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LanguageSpec {
    pub name: &'static str,
    pub aliases: &'static [&'static str],
    pub extensions: &'static [&'static str],
    pub ecosystem: &'static str,
    pub safety: &'static str,
    runner: Runner,
}

pub const LANGUAGES: &[LanguageSpec] = &[
    spec(
        "rust",
        &["rs"],
        &["rs"],
        "systems",
        "compiled with rustc; host execution is gated",
        Runner::Rust,
    ),
    spec(
        "c",
        &["ansi-c"],
        &["c"],
        "systems",
        "compiled with cc/gcc/clang; host execution is gated",
        Runner::CompileRun {
            tools: &["cc", "gcc", "clang"],
            compile_args: &["-Wall", "-Wextra", "-O0"],
        },
    ),
    spec(
        "cpp",
        &["c++", "cplusplus"],
        &["cpp", "cc", "cxx"],
        "systems",
        "compiled with c++/g++/clang++; host execution is gated",
        Runner::CompileRun {
            tools: &["c++", "g++", "clang++"],
            compile_args: &["-Wall", "-Wextra", "-O0"],
        },
    ),
    spec(
        "go",
        &["golang"],
        &["go"],
        "systems/cloud",
        "go run in a temp workspace; host execution is gated",
        Runner::Go,
    ),
    spec(
        "zig",
        &[],
        &["zig"],
        "systems",
        "zig run; host execution is gated",
        Runner::Interpreted {
            tools: &["zig"],
            args: &["run"],
        },
    ),
    spec(
        "python",
        &["py", "python3"],
        &["py"],
        "scripting/data",
        "python3/python execution is gated",
        Runner::Interpreted {
            tools: &["python3", "python"],
            args: &[],
        },
    ),
    spec(
        "javascript",
        &["js", "node"],
        &["js", "mjs", "cjs"],
        "web",
        "node execution is gated",
        Runner::Interpreted {
            tools: &["node"],
            args: &[],
        },
    ),
    spec(
        "typescript",
        &["ts", "deno"],
        &["ts"],
        "web",
        "deno run without extra permissions; host execution is gated",
        Runner::Interpreted {
            tools: &["deno"],
            args: &["run", "--no-prompt"],
        },
    ),
    spec(
        "java",
        &[],
        &["java"],
        "jvm",
        "Java source-file mode; host execution is gated",
        Runner::JavaSource,
    ),
    spec(
        "kotlin",
        &["kt"],
        &["kt"],
        "jvm",
        "kotlinc jar build then java -jar; host execution is gated",
        Runner::Kotlin,
    ),
    spec(
        "scala",
        &[],
        &["scala", "sc"],
        "jvm",
        "scala source runner where installed; host execution is gated",
        Runner::Interpreted {
            tools: &["scala"],
            args: &[],
        },
    ),
    spec(
        "csharp",
        &["c#", "cs", "dotnet"],
        &["cs"],
        ".net",
        "dotnet console wrapper; host execution is gated",
        Runner::Dotnet,
    ),
    spec(
        "fsharp",
        &["fs", "f#"],
        &["fsx"],
        ".net",
        "dotnet fsi where installed; host execution is gated",
        Runner::Interpreted {
            tools: &["dotnet"],
            args: &["fsi"],
        },
    ),
    spec(
        "swift",
        &[],
        &["swift"],
        "apple/systems",
        "swift script runner; host execution is gated",
        Runner::Interpreted {
            tools: &["swift"],
            args: &[],
        },
    ),
    spec(
        "ruby",
        &["rb"],
        &["rb"],
        "scripting/web",
        "ruby execution is gated",
        Runner::Interpreted {
            tools: &["ruby"],
            args: &[],
        },
    ),
    spec(
        "php",
        &[],
        &["php"],
        "web",
        "php cli execution is gated",
        Runner::Interpreted {
            tools: &["php"],
            args: &[],
        },
    ),
    spec(
        "perl",
        &["pl"],
        &["pl", "pm"],
        "scripting",
        "perl execution is gated",
        Runner::Interpreted {
            tools: &["perl"],
            args: &[],
        },
    ),
    spec(
        "lua",
        &[],
        &["lua"],
        "scripting/embedded",
        "lua execution is gated",
        Runner::Interpreted {
            tools: &["lua", "lua5.4", "lua5.3"],
            args: &[],
        },
    ),
    spec(
        "r",
        &["rscript"],
        &["r"],
        "data/science",
        "Rscript execution is gated",
        Runner::Interpreted {
            tools: &["Rscript"],
            args: &[],
        },
    ),
    spec(
        "julia",
        &["jl"],
        &["jl"],
        "data/science",
        "julia execution is gated",
        Runner::Interpreted {
            tools: &["julia"],
            args: &[],
        },
    ),
    spec(
        "haskell",
        &["hs"],
        &["hs"],
        "functional",
        "runghc execution is gated",
        Runner::Interpreted {
            tools: &["runghc", "runhaskell"],
            args: &[],
        },
    ),
    spec(
        "ocaml",
        &["ml"],
        &["ml"],
        "functional",
        "ocaml interpreter execution is gated",
        Runner::Interpreted {
            tools: &["ocaml"],
            args: &[],
        },
    ),
    spec(
        "elixir",
        &["exs"],
        &["exs", "ex"],
        "beam",
        "elixir execution is gated",
        Runner::Interpreted {
            tools: &["elixir"],
            args: &[],
        },
    ),
    spec(
        "erlang",
        &["escript"],
        &["erl", "escript"],
        "beam",
        "escript execution is gated",
        Runner::Interpreted {
            tools: &["escript"],
            args: &[],
        },
    ),
    spec(
        "dart",
        &[],
        &["dart"],
        "mobile/web",
        "dart execution is gated",
        Runner::Interpreted {
            tools: &["dart"],
            args: &[],
        },
    ),
    spec(
        "bash",
        &["sh", "shell"],
        &["sh", "bash"],
        "shell",
        "shell execution is powerful and host-gated; use only for trusted scripts",
        Runner::Interpreted {
            tools: &["bash"],
            args: &["--noprofile", "--norc"],
        },
    ),
    spec(
        "wasm",
        &["wasi", "webassembly"],
        &["wasm"],
        "sandbox",
        "inspected through phase1 WASI-lite path; no host shell execution",
        Runner::WasmInfo,
    ),
];

const fn spec(
    name: &'static str,
    aliases: &'static [&'static str],
    extensions: &'static [&'static str],
    ecosystem: &'static str,
    safety: &'static str,
    runner: Runner,
) -> LanguageSpec {
    LanguageSpec {
        name,
        aliases,
        extensions,
        ecosystem,
        safety,
        runner,
    }
}

pub fn run(shell: &mut Phase1Shell, args: &[String]) -> String {
    match args.first().map(String::as_str) {
        None | Some("help") | Some("-h") | Some("--help") => help(),
        Some("list") | Some("ls") => list(),
        Some("support") | Some("matrix") => support_matrix(),
        Some("status") => status(args.get(1).map(String::as_str)),
        Some("doctor") => doctor(args.get(1).map(String::as_str)),
        Some("run") | Some("exec") => run_language(shell, &args[1..]),
        Some("detect") => detect(args.get(1).map(String::as_str)),
        Some("security") | Some("policy") => security_report(),
        Some(other) => format!("lang: unknown action '{other}'\n{}", help()),
    }
}

fn help() -> String {
    "phase1 lang // native guarded language runtime manager\n\nusage:\n  lang list\n  lang support\n  lang status [language]\n  lang doctor [language]\n  lang detect <file>\n  lang run <language|auto> <vfs-file | inline-code>\n  lang security\n\nexamples:\n  echo 'fn main() { println!(\"hi\"); }' > hello.rs\n  lang run rust hello.rs\n  lang run python 'print(\"hello\")'\n  lang detect app.ts\n\nsafety:\n  Language execution is host-backed except WASI-lite inspection and requires:\n    PHASE1_SAFE_MODE=0 PHASE1_ALLOW_HOST_TOOLS=1\n  Source is copied from the phase1 VFS or inline input into a temporary file, output is bounded, and common sensitive markers are redacted.\n"
        .to_string()
}

fn list() -> String {
    let mut out = String::from("language        ecosystem        extensions\n");
    for spec in LANGUAGES {
        out.push_str(&format!(
            "{:<15} {:<16} {}\n",
            spec.name,
            spec.ecosystem,
            spec.extensions.join(",")
        ));
    }
    out
}

fn support_matrix() -> String {
    let mut out = String::from("phase1 native language support matrix\n\n");
    for spec in LANGUAGES {
        out.push_str(&format!(
            "{:<12} aliases={:<20} ext={:<18} safety={}\n",
            spec.name,
            if spec.aliases.is_empty() {
                "-".to_string()
            } else {
                spec.aliases.join(",")
            },
            spec.extensions.join(","),
            spec.safety
        ));
    }
    out
}

fn status(language: Option<&str>) -> String {
    match language.and_then(find_language) {
        Some(spec) => format!(
            "language : {}\necosystem: {}\naliases  : {}\next      : {}\nsafety   : {}\nrunner   : {}\n",
            spec.name,
            spec.ecosystem,
            if spec.aliases.is_empty() { "none".to_string() } else { spec.aliases.join(", ") },
            spec.extensions.join(", "),
            spec.safety,
            runner_summary(spec.runner)
        ),
        None if language.is_some() => format!("lang: unsupported language '{}'\n", language.unwrap_or_default()),
        None => format!("{} languages registered. Use 'lang support' for details.\n", LANGUAGES.len()),
    }
}

fn doctor(language: Option<&str>) -> String {
    if !crate::policy::host_tools_allowed() {
        return format!("{}\n", crate::policy::host_denial_message("lang doctor"));
    }
    let specs = match language.and_then(find_language) {
        Some(spec) => vec![spec],
        None if language.is_some() => {
            return format!(
                "lang: unsupported language '{}'\n",
                language.unwrap_or_default()
            )
        }
        None => LANGUAGES.iter().collect::<Vec<_>>(),
    };

    let mut out = String::from("language        toolchain\n");
    for spec in specs {
        let tools = runner_tools(spec.runner);
        if tools.is_empty() {
            out.push_str(&format!("{:<15} phase1 internal\n", spec.name));
        } else if let Some(tool) = find_tool(tools) {
            let version = tool_version(tool);
            out.push_str(&format!("{:<15} {} {}\n", spec.name, tool, version.trim()));
        } else {
            out.push_str(&format!(
                "{:<15} missing ({})\n",
                spec.name,
                tools.join("|")
            ));
        }
    }
    out
}

fn run_language(shell: &mut Phase1Shell, args: &[String]) -> String {
    if args.len() < 2 {
        return "usage: lang run <language|auto> <vfs-file | inline-code>\n".to_string();
    }
    if !crate::policy::host_tools_allowed() && args[0] != "wasm" && args[0] != "wasi" {
        return format!("{}\n", crate::policy::host_denial_message("lang run"));
    }

    let mut language = args[0].as_str();
    let source_arg = args[1..].join(" ");
    if language == "auto" {
        language = detect_language_from_path(&source_arg).unwrap_or("unknown");
    }
    let Some(spec) = find_language(language) else {
        return format!("lang: unsupported language '{language}'\n");
    };

    if spec.runner == Runner::WasmInfo {
        return "lang wasm: use the phase1 'wasm' command for WASI-lite plugin validation and execution\n".to_string();
    }

    let source = load_source(shell, &source_arg);
    if source.len() > MAX_SOURCE_BYTES {
        return format!("lang: source is too large; limit is {MAX_SOURCE_BYTES} bytes\n");
    }

    shell.kernel.audit.record(format!(
        "host.lang.run language={} bytes={}",
        spec.name,
        source.len()
    ));
    match execute(spec, &source) {
        Ok(output) => sanitize_output(&format_output(output)),
        Err(err) => format!("lang {}: {}\n", spec.name, err),
    }
}

fn detect(path: Option<&str>) -> String {
    match path.and_then(detect_language_from_path) {
        Some(language) => format!("{language}\n"),
        None => "unknown\n".to_string(),
    }
}

fn security_report() -> String {
    "phase1 language runtime security\n\n- execution is blocked by default safe mode\n- host-backed language execution requires PHASE1_SAFE_MODE=0 and PHASE1_ALLOW_HOST_TOOLS=1\n- source comes from the phase1 VFS or explicit inline input\n- source is copied to temporary files and bounded to 256 KiB\n- compile and run commands have timeouts\n- stdout/stderr are capped and redacted for common sensitive markers\n- package install, network fetch, editor shell escapes, and background daemons are not implemented here\n- WASM/WASI remains the preferred long-term sandbox target\n"
        .to_string()
}

fn execute(spec: &LanguageSpec, source: &str) -> io::Result<Output> {
    let nonce = unique_nonce();
    let root = env::temp_dir().join(format!("phase1_lang_{nonce}"));
    fs::create_dir_all(&root)?;
    let ext = spec.extensions.first().copied().unwrap_or("txt");
    let source_path = root.join(format!("main.{ext}"));
    fs::write(&source_path, source)?;

    let result = match spec.runner {
        Runner::Interpreted { tools, args } => {
            let tool = find_tool(tools).ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::NotFound,
                    format!("missing tool: {}", tools.join("|")),
                )
            })?;
            let mut cmd = Command::new(tool);
            cmd.args(args).arg(&source_path);
            run_command(cmd, RUN_TIMEOUT)
        }
        Runner::CompileRun {
            tools,
            compile_args,
        } => {
            let tool = find_tool(tools).ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::NotFound,
                    format!("missing compiler: {}", tools.join("|")),
                )
            })?;
            let binary = root.join("main-bin");
            let mut compile = Command::new(tool);
            compile
                .args(compile_args)
                .arg(&source_path)
                .arg("-o")
                .arg(&binary);
            let compile_output = run_command(compile, COMPILE_TIMEOUT)?;
            if compile_output.status.success() {
                let run = Command::new(&binary);
                run_command(run, RUN_TIMEOUT)
            } else {
                Ok(compile_output)
            }
        }
        Runner::Rust => {
            let binary = root.join("main-rs");
            let mut compile = Command::new("rustc");
            compile
                .arg("--edition=2021")
                .arg(&source_path)
                .arg("-o")
                .arg(&binary);
            let compile_output = run_command(compile, COMPILE_TIMEOUT)?;
            if compile_output.status.success() {
                let run = Command::new(&binary);
                run_command(run, RUN_TIMEOUT)
            } else {
                Ok(compile_output)
            }
        }
        Runner::Go => {
            let mut cmd = Command::new("go");
            cmd.arg("run").arg(&source_path).current_dir(&root);
            run_command(cmd, RUN_TIMEOUT)
        }
        Runner::JavaSource => {
            let java_source = root.join("Main.java");
            fs::write(&java_source, normalize_java_source(source))?;
            let mut cmd = Command::new("java");
            cmd.arg(&java_source).current_dir(&root);
            run_command(cmd, RUN_TIMEOUT)
        }
        Runner::Kotlin => {
            let jar = root.join("main.jar");
            let mut compile = Command::new("kotlinc");
            compile
                .arg(&source_path)
                .arg("-include-runtime")
                .arg("-d")
                .arg(&jar)
                .current_dir(&root);
            let compile_output = run_command(compile, COMPILE_TIMEOUT)?;
            if compile_output.status.success() {
                let mut run = Command::new("java");
                run.arg("-jar").arg(&jar);
                run_command(run, RUN_TIMEOUT)
            } else {
                Ok(compile_output)
            }
        }
        Runner::Dotnet => {
            let project = root.join("csproj");
            fs::create_dir_all(&project)?;
            fs::write(project.join("phase1.csproj"), "<Project Sdk=\"Microsoft.NET.Sdk\"><PropertyGroup><OutputType>Exe</OutputType><TargetFramework>net8.0</TargetFramework><ImplicitUsings>enable</ImplicitUsings><Nullable>enable</Nullable></PropertyGroup></Project>")?;
            fs::write(project.join("Program.cs"), source)?;
            let mut cmd = Command::new("dotnet");
            cmd.arg("run").current_dir(project);
            run_command(cmd, Duration::from_secs(30))
        }
        Runner::WasmInfo => unreachable!(),
    };

    let _ = fs::remove_dir_all(root);
    result
}

fn load_source(shell: &mut Phase1Shell, raw: &str) -> String {
    let looks_like_path = raw.starts_with('/') || raw.contains('.') || raw.ends_with('s');
    if looks_like_path {
        if let Ok(content) = shell.kernel.sys_read(raw) {
            return content;
        }
    }
    raw.to_string()
}

fn normalize_java_source(source: &str) -> String {
    if source.contains("class Main") {
        source.to_string()
    } else {
        format!("public class Main {{ public static void main(String[] args) throws Exception {{\n{}\n}} }}\n", source)
    }
}

fn find_language(name: &str) -> Option<&'static LanguageSpec> {
    let lowered = name.to_ascii_lowercase();
    LANGUAGES
        .iter()
        .find(|spec| spec.name == lowered || spec.aliases.iter().any(|alias| *alias == lowered))
}

fn detect_language_from_path(path: &str) -> Option<&'static str> {
    let ext = Path::new(path).extension()?.to_str()?.to_ascii_lowercase();
    LANGUAGES
        .iter()
        .find(|spec| spec.extensions.iter().any(|candidate| *candidate == ext))
        .map(|spec| spec.name)
}

fn runner_tools(runner: Runner) -> &'static [&'static str] {
    match runner {
        Runner::Interpreted { tools, .. } | Runner::CompileRun { tools, .. } => tools,
        Runner::Rust => &["rustc"],
        Runner::Go => &["go"],
        Runner::JavaSource => &["java"],
        Runner::Kotlin => &["kotlinc", "java"],
        Runner::Dotnet => &["dotnet"],
        Runner::WasmInfo => &[],
    }
}

fn runner_summary(runner: Runner) -> String {
    match runner {
        Runner::Interpreted { tools, args } => format!("{} {}", tools.join("|"), args.join(" ")),
        Runner::CompileRun { tools, .. } => {
            format!("compile with {} then run binary", tools.join("|"))
        }
        Runner::Rust => "rustc --edition=2021 then run binary".to_string(),
        Runner::Go => "go run".to_string(),
        Runner::JavaSource => "java source-file mode".to_string(),
        Runner::Kotlin => "kotlinc -include-runtime then java -jar".to_string(),
        Runner::Dotnet => "dotnet run in temporary console project".to_string(),
        Runner::WasmInfo => "phase1 WASI-lite command".to_string(),
    }
}

fn find_tool(tools: &'static [&'static str]) -> Option<&'static str> {
    tools.iter().copied().find(|tool| {
        Command::new(tool)
            .arg("--version")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .is_ok()
    })
}

fn tool_version(tool: &str) -> String {
    let mut cmd = Command::new(tool);
    cmd.arg("--version");
    match run_command(cmd, Duration::from_secs(3)) {
        Ok(output) => first_line(&format_output(output)),
        Err(_) => "version unavailable".to_string(),
    }
}

fn run_command(mut cmd: Command, timeout: Duration) -> io::Result<Output> {
    let mut child = cmd
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .env("GIT_TERMINAL_PROMPT", "0")
        .env("GCM_INTERACTIVE", "never")
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
        text.push_str("\n[output truncated by phase1 lang]\n");
    }
    if text.is_empty() {
        format!("process exited with {}\n", output.status)
    } else {
        text
    }
}

fn sanitize_output(raw: &str) -> String {
    raw.lines()
        .map(|line| {
            let lower = line.to_ascii_lowercase();
            if lower.contains("token=")
                || lower.contains("password=")
                || lower.contains("authorization:")
                || lower.contains("secret=")
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
    text.lines().next().unwrap_or("unknown").to_string()
}

fn unique_nonce() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_nanos())
        .unwrap_or(0)
}

#[allow(dead_code)]
fn workspace_root() -> PathBuf {
    env::var_os("PHASE1_STORAGE_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("phase1.workspace"))
}

#[cfg(test)]
mod tests {
    use super::{
        detect_language_from_path, find_language, normalize_java_source, sanitize_output, LANGUAGES,
    };

    #[test]
    fn all_registered_languages_have_extensions() {
        assert!(LANGUAGES.len() >= 25);
        for language in LANGUAGES {
            assert!(!language.name.is_empty());
            assert!(!language.extensions.is_empty());
        }
    }

    #[test]
    fn aliases_resolve_major_languages() {
        assert_eq!(find_language("py").map(|spec| spec.name), Some("python"));
        assert_eq!(
            find_language("node").map(|spec| spec.name),
            Some("javascript")
        );
        assert_eq!(find_language("c++").map(|spec| spec.name), Some("cpp"));
        assert_eq!(find_language("c#").map(|spec| spec.name), Some("csharp"));
    }

    #[test]
    fn detects_language_from_extension() {
        assert_eq!(detect_language_from_path("app.rs"), Some("rust"));
        assert_eq!(detect_language_from_path("app.ts"), Some("typescript"));
        assert_eq!(detect_language_from_path("app.exs"), Some("elixir"));
    }

    #[test]
    fn java_source_is_wrapped_when_needed() {
        let wrapped = normalize_java_source("System.out.println(\"hi\");");
        assert!(wrapped.contains("class Main"));
        assert!(wrapped.contains("System.out.println"));
    }

    #[test]
    fn output_redaction_removes_sensitive_lines() {
        let out = sanitize_output("ok\nsecret=value\npassword=nope\n");
        assert!(out.contains("ok"));
        assert!(!out.contains("value"));
        assert!(!out.contains("nope"));
    }
}

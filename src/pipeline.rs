use crate::commands::{parse_line, Phase1Shell};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PipelineResult {
    pub output: String,
    pub success: bool,
}

impl PipelineResult {
    fn ok(output: String) -> Self {
        Self {
            output,
            success: true,
        }
    }

    fn err(output: String) -> Self {
        Self {
            output,
            success: false,
        }
    }
}

pub fn run(shell: &mut Phase1Shell, line: &str) -> Result<PipelineResult, String> {
    let stages = split_pipeline(line)?;
    if stages.len() < 2 {
        return Err("pipeline requires at least two stages separated by |".to_string());
    }

    let mut input = String::new();
    let mut success = true;
    for (idx, stage) in stages.iter().enumerate() {
        let tokens = parse_line(stage)?;
        if tokens.is_empty() {
            return Err("empty pipeline stage".to_string());
        }
        let result = run_stage(shell, &tokens, if idx == 0 { None } else { Some(&input) });
        input = result.output;
        success &= result.success;
        if !result.success {
            break;
        }
    }

    Ok(PipelineResult {
        output: input,
        success,
    })
}

pub fn help() -> String {
    "phase1 pipelines\nusage: <producer> | <filter> [| <filter>...]\n\nproducers: cat, echo, history, ps, ls, find, audit, env, version, sysinfo\nfilters  : grep [-i] [-n] [-c], wc [-l|-w|-c], head [-n N|-N], tail [-n N|-N], sort, uniq, cut -d X -f N\nexamples : cat log.txt | grep alpha | wc -l\n           history | tail -5\n           ps | grep worker\n".to_string()
}

fn run_stage(shell: &mut Phase1Shell, tokens: &[String], input: Option<&str>) -> PipelineResult {
    let cmd = tokens[0].as_str();
    let args = &tokens[1..];
    match cmd {
        "cat" => match input {
            Some(value) if args.is_empty() => PipelineResult::ok(value.to_string()),
            _ => read_files(shell, args),
        },
        "echo" => PipelineResult::ok(format!("{}\n", args.join(" "))),
        "history" => PipelineResult::ok(history_text(shell)),
        "ps" => PipelineResult::ok(shell.kernel.scheduler.ps()),
        "ls" => PipelineResult::ok(shell.kernel.vfs.ls(args.first().map(String::as_str), false)),
        "find" => PipelineResult::ok(crate::text::find(&shell.kernel.vfs, args)),
        "audit" => PipelineResult::ok(shell.kernel.audit.dump()),
        "env" => PipelineResult::ok(env_text(shell)),
        "version" => PipelineResult::ok(super::release::version_report(args)),
        "sysinfo" => PipelineResult::ok(format!(
            "version={}\nuser={}\ncwd={}\nprocesses={}\n",
            crate::kernel::VERSION,
            shell.user(),
            shell.kernel.vfs.cwd.display(),
            shell.kernel.scheduler.ps().lines().skip(1).count()
        )),
        "grep" => filter_grep(input.unwrap_or(""), args),
        "wc" => filter_wc(input.unwrap_or(""), args),
        "head" => filter_window(input.unwrap_or(""), args, true),
        "tail" => filter_window(input.unwrap_or(""), args, false),
        "sort" => filter_sort(input.unwrap_or(""), args),
        "uniq" => filter_uniq(input.unwrap_or(""), args),
        "cut" => filter_cut(input.unwrap_or(""), args),
        other => PipelineResult::err(format!("pipeline: unsupported stage '{other}'\n{}", help())),
    }
}

fn split_pipeline(line: &str) -> Result<Vec<String>, String> {
    let mut out = Vec::new();
    let mut current = String::new();
    let mut quote = None;
    let mut escaped = false;

    for ch in line.chars() {
        if escaped {
            current.push(ch);
            escaped = false;
            continue;
        }
        if ch == '\\' {
            current.push(ch);
            escaped = true;
            continue;
        }
        if let Some(q) = quote {
            current.push(ch);
            if ch == q {
                quote = None;
            }
            continue;
        }
        if ch == '\'' || ch == '"' {
            quote = Some(ch);
            current.push(ch);
            continue;
        }
        if ch == '|' {
            let stage = current.trim();
            if stage.is_empty() {
                return Err("empty pipeline stage".to_string());
            }
            out.push(stage.to_string());
            current.clear();
        } else {
            current.push(ch);
        }
    }

    if quote.is_some() {
        return Err("unterminated quote in pipeline".to_string());
    }
    let stage = current.trim();
    if stage.is_empty() {
        return Err("empty pipeline stage".to_string());
    }
    out.push(stage.to_string());
    Ok(out)
}

fn read_files(shell: &mut Phase1Shell, args: &[String]) -> PipelineResult {
    if args.is_empty() {
        return PipelineResult::err("usage: cat <file>...\n".to_string());
    }
    let mut out = String::new();
    let mut ok = true;
    for path in args {
        match shell.kernel.sys_read(path) {
            Ok(content) => out.push_str(&content),
            Err(err) => {
                out.push_str(&format!("cat: {err}\n"));
                ok = false;
            }
        }
    }
    PipelineResult {
        output: out,
        success: ok,
    }
}

fn history_text(shell: &Phase1Shell) -> String {
    let mut out = String::new();
    for (idx, line) in shell.history.iter().enumerate() {
        out.push_str(&format!("{:>4} {}\n", idx + 1, line));
    }
    out
}

fn env_text(shell: &Phase1Shell) -> String {
    let mut keys: Vec<_> = shell.env.keys().collect();
    keys.sort();
    let mut out = String::new();
    for key in keys {
        out.push_str(key);
        out.push('=');
        out.push_str(&shell.env[key]);
        out.push('\n');
    }
    out
}

fn filter_grep(input: &str, args: &[String]) -> PipelineResult {
    let mut ignore_case = false;
    let mut line_numbers = false;
    let mut count_only = false;
    let mut pattern = None;
    for arg in args {
        match arg.as_str() {
            "-i" | "--ignore-case" => ignore_case = true,
            "-n" | "--line-number" => line_numbers = true,
            "-c" | "--count" => count_only = true,
            _ => pattern = Some(arg.as_str()),
        }
    }
    let Some(pattern) = pattern else {
        return PipelineResult::err("usage: grep [-i] [-n] [-c] <pattern>\n".to_string());
    };
    let query = if ignore_case {
        pattern.to_ascii_lowercase()
    } else {
        pattern.to_string()
    };
    let mut out = String::new();
    let mut matches = 0;
    for (idx, line) in input.lines().enumerate() {
        let haystack = if ignore_case {
            line.to_ascii_lowercase()
        } else {
            line.to_string()
        };
        if haystack.contains(&query) {
            matches += 1;
            if !count_only {
                if line_numbers {
                    out.push_str(&(idx + 1).to_string());
                    out.push(':');
                }
                out.push_str(line);
                out.push('\n');
            }
        }
    }
    if count_only {
        out.push_str(&format!("{matches}\n"));
    }
    PipelineResult::ok(out)
}

fn filter_wc(input: &str, args: &[String]) -> PipelineResult {
    let mut show_lines = false;
    let mut show_words = false;
    let mut show_bytes = false;
    for arg in args {
        match arg.as_str() {
            "-l" | "--lines" => show_lines = true,
            "-w" | "--words" => show_words = true,
            "-c" | "--bytes" => show_bytes = true,
            other => return PipelineResult::err(format!("wc: unknown option '{other}'\n")),
        }
    }
    if !show_lines && !show_words && !show_bytes {
        show_lines = true;
        show_words = true;
        show_bytes = true;
    }
    let mut out = String::new();
    if show_lines {
        out.push_str(&format!("{:>5}", input.lines().count()));
    }
    if show_words {
        out.push_str(&format!("{:>5}", input.split_whitespace().count()));
    }
    if show_bytes {
        out.push_str(&format!("{:>5}", input.len()));
    }
    out.push('\n');
    PipelineResult::ok(out)
}

fn filter_window(input: &str, args: &[String], from_start: bool) -> PipelineResult {
    let mut limit = 10usize;
    let mut idx = 0;
    while idx < args.len() {
        let arg = &args[idx];
        if arg == "-n" || arg == "--lines" {
            idx += 1;
            let Some(value) = args.get(idx) else {
                return PipelineResult::err("missing line count\n".to_string());
            };
            match value.parse::<usize>() {
                Ok(value) => limit = value,
                Err(_) => return PipelineResult::err(format!("invalid line count '{value}'\n")),
            }
        } else if let Some(raw) = arg.strip_prefix('-') {
            match raw.parse::<usize>() {
                Ok(value) => limit = value,
                Err(_) => return PipelineResult::err(format!("unknown option '{arg}'\n")),
            }
        } else {
            return PipelineResult::err(format!("unknown argument '{arg}'\n"));
        }
        idx += 1;
    }

    let lines: Vec<_> = input.lines().collect();
    let selected: Vec<_> = if from_start {
        lines.into_iter().take(limit).collect()
    } else {
        let start = lines.len().saturating_sub(limit);
        lines.into_iter().skip(start).collect()
    };
    let mut out = selected.join("\n");
    if !out.is_empty() {
        out.push('\n');
    }
    PipelineResult::ok(out)
}

fn filter_sort(input: &str, args: &[String]) -> PipelineResult {
    if !args.is_empty() {
        return PipelineResult::err("sort: no options are supported yet\n".to_string());
    }
    let mut lines: Vec<_> = input.lines().collect();
    lines.sort_unstable();
    let mut out = lines.join("\n");
    if !out.is_empty() {
        out.push('\n');
    }
    PipelineResult::ok(out)
}

fn filter_uniq(input: &str, args: &[String]) -> PipelineResult {
    if !args.is_empty() {
        return PipelineResult::err("uniq: no options are supported yet\n".to_string());
    }
    let mut out = String::new();
    let mut previous: Option<&str> = None;
    for line in input.lines() {
        if previous != Some(line) {
            out.push_str(line);
            out.push('\n');
            previous = Some(line);
        }
    }
    PipelineResult::ok(out)
}

fn filter_cut(input: &str, args: &[String]) -> PipelineResult {
    let mut delimiter = ' ';
    let mut field = None;
    let mut idx = 0;
    while idx < args.len() {
        match args[idx].as_str() {
            "-d" => {
                idx += 1;
                let Some(raw) = args.get(idx) else {
                    return PipelineResult::err("cut: missing delimiter\n".to_string());
                };
                delimiter = raw.chars().next().unwrap_or(' ');
            }
            "-f" => {
                idx += 1;
                let Some(raw) = args.get(idx) else {
                    return PipelineResult::err("cut: missing field\n".to_string());
                };
                field = raw.parse::<usize>().ok();
            }
            other => return PipelineResult::err(format!("cut: unknown option '{other}'\n")),
        }
        idx += 1;
    }
    let Some(field) = field else {
        return PipelineResult::err("usage: cut -d X -f N\n".to_string());
    };
    let index = field.saturating_sub(1);
    let mut out = String::new();
    for line in input.lines() {
        if let Some(value) = line.split(delimiter).nth(index) {
            out.push_str(value);
            out.push('\n');
        }
    }
    PipelineResult::ok(out)
}

#[cfg(test)]
mod tests {
    use super::run;
    use crate::commands::Phase1Shell;

    #[test]
    fn pipelines_filter_vfs_text() {
        let mut shell = Phase1Shell::new();
        shell
            .kernel
            .vfs
            .write_file("/home/log.txt", "alpha\nbeta\nalpha beta\n", false)
            .unwrap();
        let result = run(&mut shell, "cat /home/log.txt | grep alpha | wc -l").unwrap();
        assert!(result.success);
        assert_eq!(result.output.trim(), "2");
    }

    #[test]
    fn pipelines_support_cut() {
        let mut shell = Phase1Shell::new();
        let result = run(&mut shell, "echo b a b | cut -d ' ' -f 1").unwrap();
        assert!(result.success);
        assert_eq!(result.output.trim(), "b");
    }
}

use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

const LEARN_PATH: &str = "phase1.learn";
const HISTORY_PATH: &str = "phase1.history";
const MAX_NOTES: usize = 128;
const MAX_RULES: usize = 128;
const MAX_COMMANDS: usize = 256;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct Memory {
    notes: Vec<String>,
    rules: Vec<Rule>,
    commands: BTreeMap<String, CommandStat>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Rule {
    trigger: String,
    response: String,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct CommandStat {
    seen: u64,
    ok: u64,
    fail: u64,
}

fn main() {
    let mut args: Vec<String> = env::args().skip(1).collect();
    let path = env::var("PHASE1_LEARN_PATH")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from(LEARN_PATH));
    let mut memory = Memory::load(&path).unwrap_or_else(|err| {
        eprintln!("learn: could not load {}: {err}", path.display());
        Memory::default()
    });

    let output = match args.first().map(String::as_str) {
        None | Some("help" | "-h" | "--help") => help(),
        Some("status") => memory.status(&path),
        Some("profile") => memory.profile(),
        Some("suggest") => memory.suggest(),
        Some("ask") => {
            args.remove(0);
            memory.ask(&args.join(" "))
        }
        Some("note") => {
            args.remove(0);
            let result = memory.add_note(&args.join(" "));
            save_result(&memory, &path, result)
        }
        Some("teach") => {
            args.remove(0);
            let result = memory.teach(&args.join(" "));
            save_result(&memory, &path, result)
        }
        Some("observe") => {
            args.remove(0);
            let result = observe_args(&mut memory, &args);
            save_result(&memory, &path, result)
        }
        Some("import-history") | Some("learn-history") => {
            let history_path = args
                .get(1)
                .map(PathBuf::from)
                .unwrap_or_else(|| PathBuf::from(HISTORY_PATH));
            let result = memory.import_history(&history_path);
            save_result(&memory, &path, result)
        }
        Some("forget") => {
            args.remove(0);
            let result = memory.forget(&args.join(" "));
            save_result(&memory, &path, result)
        }
        Some("export") => memory.serialize(),
        Some(other) => format!("learn: unknown command '{other}'\n{}", help()),
    };

    print!("{output}");
}

fn save_result(memory: &Memory, path: &Path, result: Result<String, String>) -> String {
    match result {
        Ok(message) => match memory.save(path) {
            Ok(()) => message,
            Err(err) => format!("learn: save failed: {err}\n"),
        },
        Err(err) => format!("learn: {err}\n"),
    }
}

fn observe_args(memory: &mut Memory, args: &[String]) -> Result<String, String> {
    if args.is_empty() {
        return Err("usage: phase1-learn observe <ok|fail|seen> -- <command>".to_string());
    }
    let status = args[0].as_str();
    let start = args
        .iter()
        .position(|arg| arg == "--")
        .map_or(1, |idx| idx + 1);
    let command = args[start..].join(" ");
    if command.trim().is_empty() {
        return Err("observe requires a command after --".to_string());
    }
    memory.observe(&command, status)
}

impl Memory {
    fn load(path: &Path) -> io::Result<Self> {
        let Ok(raw) = fs::read_to_string(path) else {
            return Ok(Self::default());
        };
        Ok(Self::parse(&raw))
    }

    fn parse(raw: &str) -> Self {
        let mut memory = Self::default();
        for line in raw.lines() {
            if line.trim().is_empty() || line.starts_with('#') {
                continue;
            }
            let parts: Vec<_> = line.split('\t').collect();
            match parts.as_slice() {
                ["NOTE", encoded] => {
                    if let Ok(text) = decode_text(encoded) {
                        memory.notes.push(text);
                    }
                }
                ["RULE", trigger, response] => {
                    if let (Ok(trigger), Ok(response)) =
                        (decode_text(trigger), decode_text(response))
                    {
                        memory.rules.push(Rule { trigger, response });
                    }
                }
                ["CMD", command, seen, ok, fail] if is_safe_command_name(command) => {
                    memory.commands.insert(
                        (*command).to_string(),
                        CommandStat {
                            seen: seen.parse().unwrap_or(0),
                            ok: ok.parse().unwrap_or(0),
                            fail: fail.parse().unwrap_or(0),
                        },
                    );
                }
                _ => {}
            }
        }
        memory.trim();
        memory
    }

    fn save(&self, path: &Path) -> io::Result<()> {
        if let Some(parent) = path
            .parent()
            .filter(|parent| !parent.as_os_str().is_empty())
        {
            fs::create_dir_all(parent)?;
        }
        fs::write(path, self.serialize())
    }

    fn serialize(&self) -> String {
        let mut out = String::from("# phase1 learning memory v1\n");
        out.push_str("# local-first, sanitized, no network, no external model\n");
        for note in &self.notes {
            out.push_str("NOTE\t");
            out.push_str(&encode_hex(note.as_bytes()));
            out.push('\n');
        }
        for rule in &self.rules {
            out.push_str("RULE\t");
            out.push_str(&encode_hex(rule.trigger.as_bytes()));
            out.push('\t');
            out.push_str(&encode_hex(rule.response.as_bytes()));
            out.push('\n');
        }
        for (command, stat) in &self.commands {
            out.push_str(&format!(
                "CMD\t{command}\t{}\t{}\t{}\n",
                stat.seen, stat.ok, stat.fail
            ));
        }
        out
    }

    fn status(&self, path: &Path) -> String {
        let total_seen: u64 = self.commands.values().map(|stat| stat.seen).sum();
        format!(
            "phase1 learning system\nstatus  : active\nmode    : local-first heuristic memory\nfile    : {}\nnotes   : {}\nrules   : {}\ncommands: {} unique, {} observed\nprivacy : sanitized; no network, no cloud model, no raw secrets\n",
            path.display(),
            self.notes.len(),
            self.rules.len(),
            self.commands.len(),
            total_seen
        )
    }

    fn add_note(&mut self, text: &str) -> Result<String, String> {
        let note = sanitize_text(text);
        if note.is_empty() {
            return Err("note requires text".to_string());
        }
        self.notes.push(note);
        self.trim();
        Ok("learn: note stored\n".to_string())
    }

    fn teach(&mut self, raw: &str) -> Result<String, String> {
        let (trigger, response) = raw
            .split_once("=>")
            .or_else(|| raw.split_once('='))
            .ok_or_else(|| "usage: phase1-learn teach <trigger> = <response>".to_string())?;
        let trigger = normalize_query(trigger);
        let response = sanitize_text(response);
        if trigger.is_empty() || response.is_empty() {
            return Err("teach requires a trigger and a response".to_string());
        }
        if let Some(rule) = self.rules.iter_mut().find(|rule| rule.trigger == trigger) {
            rule.response = response;
        } else {
            self.rules.push(Rule { trigger, response });
        }
        self.trim();
        Ok("learn: rule taught\n".to_string())
    }

    fn observe(&mut self, line: &str, status: &str) -> Result<String, String> {
        let command = command_name(line).ok_or_else(|| "could not identify command".to_string())?;
        let stat = self.commands.entry(command.clone()).or_default();
        stat.seen = stat.seen.saturating_add(1);
        match status {
            "ok" | "success" | "true" => stat.ok = stat.ok.saturating_add(1),
            "fail" | "failed" | "error" | "false" => stat.fail = stat.fail.saturating_add(1),
            "seen" | "unknown" => {}
            other => return Err(format!("unknown observe status '{other}'")),
        }
        self.trim();
        Ok(format!("learn: observed {command}\n"))
    }

    fn import_history(&mut self, path: &Path) -> Result<String, String> {
        let raw = fs::read_to_string(path).map_err(|err| format!("{}: {err}", path.display()))?;
        let mut count = 0_u64;
        for line in raw.lines() {
            let decoded = if let Some(encoded) = line.strip_prefix("H\t") {
                decode_text(encoded).unwrap_or_default()
            } else if line.starts_with('#') {
                String::new()
            } else {
                line.to_string()
            };
            if !decoded.trim().is_empty() && self.observe(&decoded, "seen").is_ok() {
                count = count.saturating_add(1);
            }
        }
        Ok(format!(
            "learn: imported {count} history entries from {}\n",
            path.display()
        ))
    }

    fn ask(&self, query: &str) -> String {
        let needle = normalize_query(query);
        if needle.is_empty() {
            return self.suggest();
        }
        for rule in &self.rules {
            if rule.trigger.contains(&needle) || needle.contains(&rule.trigger) {
                return format!(
                    "phase1 learned answer\nmatch : {}\nanswer: {}\n",
                    rule.trigger, rule.response
                );
            }
        }
        let matches = self
            .notes
            .iter()
            .filter(|note| normalize_query(note).contains(&needle))
            .map(String::as_str)
            .collect::<Vec<_>>();
        if matches.is_empty() {
            format!("phase1 learned answer\nmatch : none\n{}", self.suggest())
        } else {
            format!("phase1 learned notes\n{}\n", matches.join("\n"))
        }
    }

    fn suggest(&self) -> String {
        let mut out = String::from("phase1 smart suggestions\n");
        let top = self.top_commands(5);
        if top.is_empty() {
            out.push_str("next : help\nwhy  : no learned command patterns yet\ntry  : phase1-learn import-history\n");
            return out;
        }
        let (command, stat) = top[0];
        out.push_str(&format!("top  : {command} ({} uses)\n", stat.seen));
        out.push_str(&format!("next : {}\n", next_step(command)));
        out.push_str("why  : local command frequency plus Phase1 workflow heuristics\n");
        out.push_str("also : ");
        out.push_str(
            &top.iter()
                .map(|(name, stat)| format!("{name}:{}", stat.seen))
                .collect::<Vec<_>>()
                .join(", "),
        );
        out.push('\n');
        out
    }

    fn profile(&self) -> String {
        let mut out = String::from("phase1 learning profile\n");
        out.push_str(&format!("notes   : {}\n", self.notes.len()));
        out.push_str(&format!("rules   : {}\n", self.rules.len()));
        let top = self.top_commands(12);
        if top.is_empty() {
            out.push_str("commands: none yet\n");
        } else {
            out.push_str("commands:\n");
            for (name, stat) in top {
                out.push_str(&format!(
                    "  {:<14} seen={:<4} ok={:<4} fail={}\n",
                    name, stat.seen, stat.ok, stat.fail
                ));
            }
        }
        out
    }

    fn forget(&mut self, target: &str) -> Result<String, String> {
        let normalized = normalize_query(target);
        match normalized.as_str() {
            "" => Err("usage: phase1-learn forget <all|notes|rules|commands|query>".to_string()),
            "all" => {
                self.notes.clear();
                self.rules.clear();
                self.commands.clear();
                Ok("learn: all memory cleared\n".to_string())
            }
            "notes" => {
                self.notes.clear();
                Ok("learn: notes cleared\n".to_string())
            }
            "rules" => {
                self.rules.clear();
                Ok("learn: rules cleared\n".to_string())
            }
            "commands" => {
                self.commands.clear();
                Ok("learn: command stats cleared\n".to_string())
            }
            query => {
                let before = self.notes.len() + self.rules.len() + self.commands.len();
                self.notes
                    .retain(|note| !normalize_query(note).contains(query));
                self.rules.retain(|rule| !rule.trigger.contains(query));
                self.commands.retain(|command, _| !command.contains(query));
                let after = self.notes.len() + self.rules.len() + self.commands.len();
                Ok(format!(
                    "learn: removed {} matching entries\n",
                    before.saturating_sub(after)
                ))
            }
        }
    }

    fn top_commands(&self, limit: usize) -> Vec<(&String, &CommandStat)> {
        let mut items: Vec<_> = self.commands.iter().collect();
        items.sort_by(|a, b| b.1.seen.cmp(&a.1.seen).then_with(|| a.0.cmp(b.0)));
        items.truncate(limit);
        items
    }

    fn trim(&mut self) {
        if self.notes.len() > MAX_NOTES {
            let drop_count = self.notes.len() - MAX_NOTES;
            self.notes.drain(0..drop_count);
        }
        if self.rules.len() > MAX_RULES {
            let drop_count = self.rules.len() - MAX_RULES;
            self.rules.drain(0..drop_count);
        }
        if self.commands.len() > MAX_COMMANDS {
            let mut names: Vec<_> = self
                .commands
                .iter()
                .map(|(name, stat)| (name.clone(), stat.seen))
                .collect();
            names.sort_by(|a, b| a.1.cmp(&b.1).then_with(|| a.0.cmp(&b.0)));
            for (name, _) in names.into_iter().take(self.commands.len() - MAX_COMMANDS) {
                self.commands.remove(&name);
            }
        }
    }
}

fn command_name(line: &str) -> Option<String> {
    let first = line.split_whitespace().next()?;
    let command = first.rsplit('/').next().unwrap_or(first);
    let command = command
        .chars()
        .filter(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_'))
        .collect::<String>()
        .to_ascii_lowercase();
    if is_safe_command_name(&command) {
        Some(command)
    } else {
        None
    }
}

fn is_safe_command_name(command: &str) -> bool {
    !command.is_empty()
        && command.len() <= 48
        && command
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_'))
}

fn next_step(command: &str) -> &'static str {
    match command {
        "avim" | "ned" => "grep or cat to review edited files",
        "cat" | "grep" | "find" | "wc" => "pipeline or head/tail for deeper text inspection",
        "history" => "phase1-learn import-history to refresh the learning profile",
        "lang" | "python" | "gcc" => "lang doctor and lang security before host-backed code",
        "security" | "audit" | "opslog" => "capabilities to inspect command guard boundaries",
        "git" | "phase1-storage" => "cargo run --bin phase1-storage -- git list",
        "theme" | "banner" | "matrix" => "sysinfo to confirm the active terminal profile",
        _ => "sysinfo, security, or roadmap for the next operator view",
    }
}

fn sanitize_text(text: &str) -> String {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return String::new();
    }
    let lower = trimmed.to_ascii_lowercase();
    let risky = [
        "password",
        "passwd",
        "token",
        "secret",
        "credential",
        "cookie",
        "recovery code",
        "private key",
        "github_pat_",
        "ghp_",
        "gho_",
        "bearer ",
        "authorization:",
    ];
    if risky.iter().any(|marker| lower.contains(marker)) {
        return "[redacted-sensitive-memory]".to_string();
    }
    trimmed
        .chars()
        .filter(|ch| !ch.is_control())
        .take(600)
        .collect()
}

fn normalize_query(text: &str) -> String {
    sanitize_text(text)
        .to_ascii_lowercase()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn encode_hex(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        out.push(HEX[(byte >> 4) as usize] as char);
        out.push(HEX[(byte & 0x0f) as usize] as char);
    }
    out
}

fn decode_text(raw: &str) -> Result<String, String> {
    let bytes = decode_hex(raw)?;
    String::from_utf8(bytes).map_err(|err| err.to_string())
}

fn decode_hex(raw: &str) -> Result<Vec<u8>, String> {
    if !raw.len().is_multiple_of(2) {
        return Err("hex payload has odd length".to_string());
    }
    let mut bytes = Vec::with_capacity(raw.len() / 2);
    let chars: Vec<_> = raw.bytes().collect();
    for pair in chars.chunks(2) {
        let high = hex_value(pair[0]).ok_or_else(|| "invalid hex payload".to_string())?;
        let low = hex_value(pair[1]).ok_or_else(|| "invalid hex payload".to_string())?;
        bytes.push((high << 4) | low);
    }
    Ok(bytes)
}

fn hex_value(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}

fn help() -> String {
    "phase1 learning system\nusage:\n  phase1-learn status\n  phase1-learn import-history [phase1.history]\n  phase1-learn observe <ok|fail|seen> -- <command>\n  phase1-learn teach <trigger> = <response>\n  phase1-learn note <text>\n  phase1-learn ask <query>\n  phase1-learn suggest\n  phase1-learn profile\n  phase1-learn forget <all|notes|rules|commands|query>\n  phase1-learn export\n\nprivacy: stores sanitized local memory in phase1.learn; no network and no external AI model.\n".to_string()
}

#[cfg(test)]
mod tests {
    use super::{command_name, Memory};
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_path(name: &str) -> std::path::PathBuf {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|duration| duration.as_nanos())
            .unwrap_or(0);
        std::env::temp_dir().join(format!("phase1-learn-{name}-{nonce}"))
    }

    #[test]
    fn observes_command_frequency() {
        let mut memory = Memory::default();
        memory.observe("avim notes.rs", "ok").unwrap();
        memory.observe("avim notes.rs", "fail").unwrap();
        let profile = memory.profile();
        assert!(profile.contains("avim"));
        assert!(profile.contains("seen=2"));
        assert!(profile.contains("fail=1"));
    }

    #[test]
    fn teach_and_ask_round_trip() {
        let mut memory = Memory::default();
        memory.teach("deploy = use main for Pages deploys").unwrap();
        let answer = memory.ask("deploy");
        assert!(answer.contains("use main"));
    }

    #[test]
    fn serializes_and_restores_redacted_sensitive_notes() {
        let mut memory = Memory::default();
        memory.add_note("my token is ghp_secret").unwrap();
        let exported = memory.serialize();
        assert!(!exported.contains("ghp_secret"));
        let parsed = Memory::parse(&exported);
        assert_eq!(
            parsed.notes.first().map(String::as_str),
            Some("[redacted-sensitive-memory]")
        );
    }

    #[test]
    fn imports_phase1_history_hex_format() {
        let path = temp_path("history");
        fs::write(
            &path,
            "# phase1 persistent history v1\nH\t6176696d206e6f7465732e7273\nH\t737973696e666f\n",
        )
        .unwrap();
        let mut memory = Memory::default();
        memory.import_history(&path).unwrap();
        let profile = memory.profile();
        assert!(profile.contains("avim"));
        assert!(profile.contains("sysinfo"));
        let _ = fs::remove_file(path);
    }

    #[test]
    fn command_names_are_sanitized() {
        assert_eq!(
            command_name("/usr/bin/python demo.py").as_deref(),
            Some("python")
        );
        assert_eq!(command_name("bad$cmd arg").as_deref(), Some("badcmd"));
    }
}

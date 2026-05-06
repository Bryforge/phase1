use std::collections::VecDeque;
use std::fs;
use std::io;
use std::path::PathBuf;

pub const HISTORY_LIMIT: usize = 512;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum HistoryStore {
    Disabled,
    Disk(PathBuf),
}

impl HistoryStore {
    pub fn from_env(_persistent_state: bool) -> Self {
        match std::env::var("PHASE1_HISTORY") {
            Ok(value) if value.eq_ignore_ascii_case("off") => Self::Disabled,
            Ok(value) if !value.trim().is_empty() => Self::Disk(PathBuf::from(value)),
            _ => Self::Disabled,
        }
    }

    pub fn describe(&self) -> String {
        match self {
            Self::Disabled => "off".to_string(),
            Self::Disk(path) => path.display().to_string(),
        }
    }

    pub fn load(&self, history: &mut VecDeque<String>) -> io::Result<usize> {
        let Self::Disk(path) = self else {
            return Ok(0);
        };
        if !path.exists() {
            return Ok(0);
        }
        let raw = fs::read_to_string(path)?;
        let mut loaded = 0;
        for line in raw.lines().filter(|line| !line.trim().is_empty()) {
            push_bounded(history, line);
            loaded += 1;
        }
        Ok(loaded)
    }

    pub fn save(&self, history: &VecDeque<String>) -> io::Result<()> {
        let Self::Disk(path) = self else {
            return Ok(());
        };
        if let Some(parent) = path.parent().filter(|parent| !parent.as_os_str().is_empty()) {
            fs::create_dir_all(parent)?;
        }
        let mut out = String::new();
        for line in history.iter().filter(|line| !line.trim().is_empty()) {
            out.push_str(line);
            out.push('\n');
        }
        fs::write(path, out)
    }

    pub fn clear(&self, history: &mut VecDeque<String>) -> io::Result<()> {
        history.clear();
        match self {
            Self::Disabled => Ok(()),
            Self::Disk(path) if path.exists() => fs::write(path, ""),
            Self::Disk(_) => Ok(()),
        }
    }
}

pub fn push_bounded(history: &mut VecDeque<String>, line: &str) {
    if line.trim().is_empty() {
        return;
    }
    history.push_back(line.to_string());
    while history.len() > HISTORY_LIMIT {
        history.pop_front();
    }
}

#[cfg(test)]
mod tests {
    use super::{push_bounded, HistoryStore, HISTORY_LIMIT};
    use std::collections::VecDeque;

    #[test]
    fn bounded_history_keeps_recent_entries() {
        let mut history = VecDeque::new();
        for idx in 0..(HISTORY_LIMIT + 3) {
            push_bounded(&mut history, &format!("cmd-{idx}"));
        }
        assert_eq!(history.len(), HISTORY_LIMIT);
        assert_eq!(history.front().map(String::as_str), Some("cmd-3"));
    }

    #[test]
    fn disabled_history_describes_as_off() {
        assert_eq!(HistoryStore::Disabled.describe(), "off");
    }

    #[test]
    fn persistent_state_does_not_enable_disk_history() {
        std::env::remove_var("PHASE1_HISTORY");
        assert_eq!(HistoryStore::from_env(true), HistoryStore::Disabled);
    }
}

use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use crate::denial::DenialResponse;

/// Append-only JSON-lines log of denial events for a session.
pub struct DenialLog {
    path: PathBuf,
}

impl DenialLog {
    /// Open (or create) a denial log at `dir/denials.jsonl`.
    /// The directory is created if it does not exist.
    pub fn open(dir: &Path) -> Result<Self, std::io::Error> {
        std::fs::create_dir_all(dir)?;
        Ok(Self { path: dir.join("denials.jsonl") })
    }

    /// Append a denial as a JSON line. Silently no-ops on error (never panics).
    pub fn append(&self, denial: &DenialResponse) {
        match serde_json::to_string(denial) {
            Ok(json) => {
                if let Ok(mut f) = OpenOptions::new().create(true).append(true).open(&self.path) {
                    let _ = writeln!(f, "{json}");
                }
            },
            Err(e) => log::warn!("[ROY] could not serialize denial: {e}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn temp_dir() -> PathBuf {
        std::env::temp_dir().join(format!("roy_test_{}", std::process::id()))
    }

    #[test]
    fn creates_dir_and_writes_denial() {
        let dir = temp_dir().join("creates");
        let log = DenialLog::open(&dir).expect("open log");
        let denial = DenialResponse::new("rm -rf /", "destructive").with_rule_id("D001");
        log.append(&denial);

        let contents = fs::read_to_string(log.path).expect("read log");
        assert!(contents.contains("rm -rf /"), "log must contain blocked command");
        assert!(contents.contains("D001"), "log must contain rule id");
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn appends_multiple_denials() {
        let dir = temp_dir().join("appends");
        let log = DenialLog::open(&dir).expect("open log");
        log.append(&DenialResponse::new("cmd1", "reason1"));
        log.append(&DenialResponse::new("cmd2", "reason2"));

        let contents = fs::read_to_string(&log.path).expect("read log");
        let lines: Vec<&str> = contents.lines().collect();
        assert_eq!(lines.len(), 2, "should have 2 JSON lines");
        assert!(lines[0].contains("cmd1"));
        assert!(lines[1].contains("cmd2"));
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn each_line_is_valid_json() {
        let dir = temp_dir().join("valid_json");
        let log = DenialLog::open(&dir).expect("open log");
        log.append(&DenialResponse::new("test", "test reason").with_alternative("alt"));

        let contents = fs::read_to_string(&log.path).expect("read log");
        for line in contents.lines() {
            let parsed: serde_json::Value =
                serde_json::from_str(line).expect("each line must be valid JSON");
            assert!(parsed.get("blocked").is_some(), "must have blocked field");
        }
        let _ = fs::remove_dir_all(&dir);
    }
}

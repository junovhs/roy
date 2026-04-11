use super::*;
use crate::shell::ShellError;

impl ShellRuntime {
    pub(super) fn dispatch_cd(&mut self, args: &[&str]) -> DispatchResult {
        let Some(&raw) = args.first() else {
            self.last_exit_status = Some(0);
            return DispatchResult::CwdChanged {
                to: self.env.cwd().to_path_buf(),
            };
        };

        let raw_path = std::path::Path::new(raw);
        let absolute = if raw_path.is_absolute() {
            raw_path.to_path_buf()
        } else {
            self.env.cwd().join(raw_path)
        };

        if absolute.exists() && !self.workspace.contains(&absolute) {
            return self.deny(
                "cd",
                format!(
                    "cd: {} escapes workspace boundary (root: {})",
                    absolute.display(),
                    self.workspace.root().display()
                ),
            );
        }

        match self.env.chdir(std::path::Path::new(raw)) {
            Ok(()) => {
                self.last_exit_status = Some(0);
                DispatchResult::CwdChanged {
                    to: self.env.cwd().to_path_buf(),
                }
            }
            Err(ShellError::DirNotFound(p)) => self.cwd_error(p, "no such directory"),
            Err(ShellError::NotADirectory(p)) => self.cwd_error(p, "not a directory"),
            Err(e) => {
                let msg = format!("cd: {e}");
                self.io.write_error(&msg);
                self.last_exit_status = Some(1);
                DispatchResult::Executed {
                    output: msg,
                    exit_code: 1,
                }
            }
        }
    }

    pub(super) fn dispatch_pwd(&mut self) -> DispatchResult {
        let output = self.env.cwd().display().to_string();
        self.io.write_line(&output);
        self.last_exit_status = Some(0);
        DispatchResult::Executed {
            output,
            exit_code: 0,
        }
    }

    pub(super) fn dispatch_env(&mut self, args: &[&str]) -> DispatchResult {
        let snap = self.env.snapshot();
        let filter = args.first().copied();
        let mut lines: Vec<String> = snap
            .iter()
            .filter(|(k, _)| filter.is_none_or(|f| k.contains(f)))
            .map(|(k, v)| format!("{k}={v}"))
            .collect();
        lines.sort();
        let output = lines.join("\n");
        self.io.write_line(&output);
        self.last_exit_status = Some(0);
        DispatchResult::Executed {
            output,
            exit_code: 0,
        }
    }

    pub(super) fn dispatch_exit(&mut self, args: &[&str]) -> DispatchResult {
        let code: i32 = args.first().and_then(|s| s.parse().ok()).unwrap_or(0);
        self.last_exit_status = Some(code);
        DispatchResult::Exit { code }
    }

    pub(super) fn dispatch_help(&mut self) -> DispatchResult {
        let mut lines = vec![
            "ROY — controlled shell host".to_string(),
            "".to_string(),
            "Built-in commands:".to_string(),
        ];

        lines.extend(
            self.registry
                .public_help_lines()
                .into_iter()
                .map(|line| format!("  {line}")),
        );

        lines.extend([
            "".to_string(),
            "ROY-native commands: pending TOOL-02".to_string(),
            "Policy engine:       pending POL-01".to_string(),
            "Embedded agents:     pending AGEN-01".to_string(),
        ]);

        let output = lines.join("\n");
        self.io.write_line(&output);
        self.last_exit_status = Some(0);
        DispatchResult::Executed {
            output,
            exit_code: 0,
        }
    }

    fn cwd_error(&mut self, path: std::path::PathBuf, suffix: &str) -> DispatchResult {
        let msg = format!("cd: {}: {suffix}", path.display());
        self.io.write_error(&msg);
        self.last_exit_status = Some(1);
        DispatchResult::Executed {
            output: msg,
            exit_code: 1,
        }
    }
}

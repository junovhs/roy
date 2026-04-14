use super::*;
use crate::render::CommandResult;
use crate::schema_registry::SchemaRegistry;
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
                args,
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
                    artifacts: Vec::new(),
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
            artifacts: Vec::new(),
        }
    }

    pub(super) fn dispatch_env(&mut self, args: &[&str]) -> DispatchResult {
        let snap = self.env.snapshot();
        let filter = args.first().copied();
        let mut pairs: Vec<(String, String)> = snap
            .into_iter()
            .filter(|(k, _)| filter.is_none_or(|f| k.contains(f)))
            .collect();
        pairs.sort_by(|(a, _), (b, _)| a.cmp(b));

        // Write to transcript buffer (legacy path; renderers are the canonical
        // output surface for REND-01 consumers).
        let output = pairs
            .iter()
            .map(|(k, v)| format!("{k}={v}"))
            .collect::<Vec<_>>()
            .join("\n");
        self.io.write_line(&output);
        self.last_exit_status = Some(0);

        DispatchResult::Typed {
            result: CommandResult::env_map(pairs),
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
            format!("Workspace:  {}", self.workspace.root().display()),
            format!("Directory:  {}", self.env.cwd().display()),
            "".to_string(),
        ];

        let public = self.registry.public_commands();
        let builtins: Vec<_> = public
            .iter()
            .filter(|s| matches!(s.backend, Backend::Builtin))
            .collect();
        let native: Vec<_> = public
            .iter()
            .filter(|s| matches!(s.backend, Backend::RoyNative))
            .collect();

        lines.push("Shell built-ins:".to_string());
        lines.extend(builtins.iter().map(|s| format!("  {}", s.help_text)));

        if !native.is_empty() {
            lines.push("".to_string());
            lines.push("ROY-native commands:".to_string());
            lines.extend(native.iter().map(|s| format!("  {}", s.help_text)));
        }

        lines.extend([
            "".to_string(),
            "Policy engine:  active".to_string(),
            "Run `commands` for a machine-readable command list.".to_string(),
        ]);

        let output = lines.join("\n");
        self.io.write_line(&output);
        self.last_exit_status = Some(0);
        DispatchResult::Executed {
            output,
            exit_code: 0,
            artifacts: Vec::new(),
        }
    }

    pub(super) fn dispatch_commands(&mut self) -> DispatchResult {
        let names: Vec<&str> = self
            .registry
            .public_commands()
            .into_iter()
            .map(|s| s.name)
            .collect();
        let output = names.join("\n");
        self.io.write_line(&output);
        self.last_exit_status = Some(0);
        DispatchResult::Executed {
            output,
            exit_code: 0,
            artifacts: Vec::new(),
        }
    }

    pub(super) fn dispatch_schemas(&mut self) -> DispatchResult {
        let registry = SchemaRegistry::new();
        let output = registry
            .all()
            .iter()
            .map(|schema| schema.list_line())
            .collect::<Vec<_>>()
            .join("\n");

        self.io.write_line(&output);
        self.last_exit_status = Some(0);
        DispatchResult::Executed {
            output,
            exit_code: 0,
            artifacts: Vec::new(),
        }
    }

    pub(super) fn dispatch_schema(&mut self, args: &[&str]) -> DispatchResult {
        let Some(name) = args.first().copied() else {
            let msg =
                "schema: missing schema name — run `schemas` to list available schemas".to_string();
            self.io.write_error(&msg);
            self.last_exit_status = Some(2);
            return DispatchResult::Executed {
                output: msg,
                exit_code: 2,
                artifacts: Vec::new(),
            };
        };

        let registry = SchemaRegistry::new();
        let Some(schema) = registry.lookup(name) else {
            let msg = format!(
                "schema: unknown schema '{name}' — run `schemas` to list available schemas"
            );
            self.io.write_error(&msg);
            self.last_exit_status = Some(1);
            return DispatchResult::Executed {
                output: msg,
                exit_code: 1,
                artifacts: Vec::new(),
            };
        };

        let output = schema.full_description();
        self.io.write_line(&output);
        self.last_exit_status = Some(0);
        DispatchResult::Executed {
            output,
            exit_code: 0,
            artifacts: Vec::new(),
        }
    }

    fn cwd_error(&mut self, path: std::path::PathBuf, suffix: &str) -> DispatchResult {
        let msg = format!("cd: {}: {suffix}", path.display());
        self.io.write_error(&msg);
        self.last_exit_status = Some(1);
        DispatchResult::Executed {
            output: msg,
            exit_code: 1,
            artifacts: Vec::new(),
        }
    }
}

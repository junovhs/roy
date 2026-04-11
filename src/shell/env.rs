// API is live in tests; binary wiring pending TOOL-01 / SHEL-02.
#![allow(dead_code)]

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use super::ShellError;

/// Controlled environment for one ROY shell session.
///
/// PATH is ROY-owned and does not inherit from the OS environment.
/// All env vars start empty except for the ROY identity set; callers
/// may populate them explicitly. No OS env is leaked.
pub struct ShellEnv {
    vars: HashMap<String, String>,
    cwd: PathBuf,
}

impl ShellEnv {
    /// Create a new environment rooted at `workspace_root`.
    ///
    /// PATH is set to the ROY-controlled path list. No OS env vars are
    /// inherited — the agent sees only what ROY explicitly provides.
    pub fn new(workspace_root: PathBuf) -> Self {
        let mut vars = HashMap::new();
        vars.insert("PATH".to_string(), Self::roy_path());
        vars.insert("SHELL".to_string(), "roy".to_string());
        vars.insert("TERM".to_string(), "roy-term".to_string());
        Self {
            vars,
            cwd: workspace_root,
        }
    }

    /// ROY-controlled PATH entries.
    ///
    /// OS PATH (/usr/bin, /bin, etc.) is intentionally absent — that
    /// is the core mechanism by which ROY prevents shell-shaped fallback.
    pub fn roy_path() -> String {
        "/usr/local/lib/roy/bin".to_string()
    }

    /// Get an environment variable value.
    pub fn get(&self, key: &str) -> Option<&str> {
        self.vars.get(key).map(String::as_str)
    }

    /// Set an environment variable.
    pub fn set(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.vars.insert(key.into(), value.into());
    }

    /// Unset an environment variable.
    pub fn unset(&mut self, key: &str) {
        self.vars.remove(key);
    }

    /// Current working directory.
    pub fn cwd(&self) -> &Path {
        &self.cwd
    }

    /// Change working directory.
    ///
    /// Relative paths are resolved against current CWD. The result is
    /// always canonicalized (symlinks resolved, `..` collapsed).
    ///
    /// # Errors
    /// - `ShellError::DirNotFound` — path does not exist
    /// - `ShellError::NotADirectory` — path exists but is not a directory
    pub fn chdir(&mut self, path: &Path) -> Result<(), ShellError> {
        let target = if path.is_absolute() {
            path.to_path_buf()
        } else {
            self.cwd.join(path)
        };

        let canonical = target
            .canonicalize()
            .map_err(|_| ShellError::DirNotFound(target.clone()))?;

        if !canonical.is_dir() {
            return Err(ShellError::NotADirectory(canonical));
        }

        self.cwd = canonical;
        Ok(())
    }

    /// Snapshot all env vars for passing to capability execution.
    ///
    /// Not for passing to arbitrary OS processes — only ROY-native
    /// command execution should receive this environment.
    pub fn snapshot(&self) -> HashMap<String, String> {
        self.vars.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tmp_root() -> PathBuf {
        std::env::temp_dir()
    }

    #[test]
    fn cwd_starts_at_workspace_root() {
        let root = tmp_root();
        let env = ShellEnv::new(root.clone());
        assert_eq!(env.cwd(), root.as_path());
    }

    #[test]
    fn path_entries_exclude_os_paths() {
        let env = ShellEnv::new(tmp_root());
        let path = env.get("PATH").expect("PATH must be set");
        let entries: Vec<&str> = path.split(':').collect();
        // These are the OS PATH entries ROY must never expose.
        assert!(
            !entries.contains(&"/usr/bin"),
            "ROY must not expose /usr/bin"
        );
        assert!(!entries.contains(&"/bin"), "ROY must not expose /bin");
        assert!(
            !entries.contains(&"/usr/local/bin"),
            "ROY must not expose /usr/local/bin"
        );
    }

    #[test]
    fn shell_identity_vars_set() {
        let env = ShellEnv::new(tmp_root());
        assert_eq!(env.get("SHELL"), Some("roy"));
        assert_eq!(env.get("TERM"), Some("roy-term"));
    }

    #[test]
    fn set_and_get_var() {
        let mut env = ShellEnv::new(tmp_root());
        env.set("FOO", "bar");
        assert_eq!(env.get("FOO"), Some("bar"));
    }

    #[test]
    fn unset_removes_var() {
        let mut env = ShellEnv::new(tmp_root());
        env.set("FOO", "bar");
        env.unset("FOO");
        assert_eq!(env.get("FOO"), None);
    }

    #[test]
    fn get_missing_var_returns_none() {
        let env = ShellEnv::new(tmp_root());
        assert_eq!(env.get("NOT_SET_12345"), None);
    }

    #[test]
    fn chdir_absolute_path() {
        let root = tmp_root();
        let parent = root.parent().unwrap().to_path_buf();
        let parent_canonical = parent.canonicalize().unwrap();
        let mut env = ShellEnv::new(root);
        env.chdir(&parent_canonical).unwrap();
        assert_eq!(env.cwd(), parent_canonical.as_path());
    }

    #[test]
    fn chdir_relative_dotdot() {
        let root = tmp_root();
        let parent_canonical = root.parent().unwrap().canonicalize().unwrap();
        let mut env = ShellEnv::new(root);
        env.chdir(Path::new("..")).unwrap();
        assert_eq!(env.cwd(), parent_canonical.as_path());
    }

    #[test]
    fn chdir_nonexistent_returns_dir_not_found() {
        let mut env = ShellEnv::new(tmp_root());
        let result = env.chdir(Path::new("/nonexistent_roy_path_xyz_99991"));
        assert!(
            matches!(result, Err(ShellError::DirNotFound(_))),
            "expected DirNotFound, got {result:?}"
        );
    }

    #[test]
    fn snapshot_excludes_os_path() {
        let env = ShellEnv::new(tmp_root());
        let snap = env.snapshot();
        let path = snap.get("PATH").expect("PATH in snapshot");
        assert!(!path.split(':').any(|e| e == "/usr/bin" || e == "/bin"));
    }
}

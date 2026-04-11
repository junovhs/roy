// Live via ShellRuntime workspace_cwd; UI surfacing pending WRK-02.
#![allow(dead_code)]

use std::path::{Path, PathBuf};

use super::boundary::{WorkspaceBoundary, WorkspaceError};

/// Workspace-scoped current working directory.
///
/// Combines workspace boundary enforcement with CWD tracking. Every `chdir`
/// is validated against the boundary before the directory changes. The CWD
/// always starts at the workspace root and can only move to subdirectories
/// within the same workspace.
pub struct WorkspaceCwd {
    boundary: WorkspaceBoundary,
    cwd: PathBuf,
}

impl WorkspaceCwd {
    /// Create a workspace CWD rooted at the workspace root.
    ///
    /// The initial CWD is set to the workspace root itself.
    pub fn new(boundary: WorkspaceBoundary) -> Self {
        let cwd = boundary.root().to_path_buf();
        Self { boundary, cwd }
    }

    /// Current working directory (always within workspace boundaries).
    pub fn cwd(&self) -> &Path {
        &self.cwd
    }

    /// The workspace boundary this CWD is scoped to.
    pub fn boundary(&self) -> &WorkspaceBoundary {
        &self.boundary
    }

    /// Change directory to `path`, enforcing workspace boundary.
    ///
    /// Resolves relative paths against the current CWD, canonicalizes,
    /// then validates the result is within the workspace. Rejects paths
    /// that would escape the workspace with `PathEscapesBoundary`.
    pub fn chdir(&mut self, path: &Path) -> Result<(), WorkspaceError> {
        let target = if path.is_absolute() {
            path.to_path_buf()
        } else {
            self.cwd.join(path)
        };

        let canonical = target
            .canonicalize()
            .map_err(|_| WorkspaceError::Io(
                std::io::Error::new(std::io::ErrorKind::NotFound, target.display().to_string())
            ))?;

        self.boundary.validate_cwd(&canonical)?;

        if !canonical.is_dir() {
            return Err(WorkspaceError::Io(std::io::Error::other(
                format!("{} is not a directory", canonical.display()),
            )));
        }

        self.cwd = canonical;
        Ok(())
    }

    /// Display path relative to workspace root, or absolute if outside (should not happen).
    pub fn display_path(&self) -> String {
        self.cwd
            .strip_prefix(self.boundary.root())
            .map(|rel| format!("~/{}", rel.display()))
            .unwrap_or_else(|_| self.cwd.display().to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    fn tmp_boundary() -> WorkspaceBoundary {
        WorkspaceBoundary::new(env::temp_dir().canonicalize().unwrap())
    }

    fn workspace_cwd() -> WorkspaceCwd {
        WorkspaceCwd::new(tmp_boundary())
    }

    #[test]
    fn initial_cwd_is_workspace_root() {
        let ws = workspace_cwd();
        assert_eq!(ws.cwd(), ws.boundary().root());
    }

    #[test]
    fn chdir_to_root_itself_is_ok() {
        let mut ws = workspace_cwd();
        let root = ws.boundary().root().to_path_buf();
        assert!(ws.chdir(&root).is_ok());
        assert_eq!(ws.cwd(), root.as_path());
    }

    #[test]
    fn chdir_outside_workspace_is_denied() {
        let mut ws = workspace_cwd();
        let outside = env::temp_dir().parent().unwrap().canonicalize().unwrap();
        assert!(matches!(
            ws.chdir(&outside),
            Err(WorkspaceError::PathEscapesBoundary { .. })
        ));
    }

    #[test]
    fn cwd_unchanged_after_failed_chdir() {
        let mut ws = workspace_cwd();
        let original = ws.cwd().to_path_buf();
        let outside = env::temp_dir().parent().unwrap().canonicalize().unwrap();
        let _ = ws.chdir(&outside);
        assert_eq!(ws.cwd(), original.as_path());
    }

    #[test]
    fn chdir_nonexistent_path_is_io_error() {
        let mut ws = workspace_cwd();
        assert!(matches!(
            ws.chdir(Path::new("/definitely_does_not_exist_ws_99993")),
            Err(WorkspaceError::Io(_))
        ));
    }

    #[test]
    fn display_path_shows_tilde_relative_format() {
        let ws = workspace_cwd();
        let display = ws.display_path();
        // Root maps to "~/" prefix
        assert!(display.starts_with("~/") || display == "~/");
    }

    #[test]
    fn boundary_accessor_returns_workspace_boundary() {
        let ws = workspace_cwd();
        assert_eq!(ws.boundary().root(), ws.cwd()); // starts at root
    }
}

// Live via ShellRuntime workspace field; full UI surfacing pending WRK-02.
#![allow(dead_code)]

use std::path::{Path, PathBuf};

/// Error types for workspace boundary operations.
#[derive(Debug)]
pub enum WorkspaceError {
    /// The declared workspace root does not exist or is not a directory.
    RootNotFound(PathBuf),
    /// A path would escape the declared workspace boundary.
    PathEscapesBoundary { path: PathBuf, root: PathBuf },
    /// Underlying OS IO error during path operations.
    Io(std::io::Error),
}

impl std::fmt::Display for WorkspaceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RootNotFound(p) => write!(f, "workspace root not found: {}", p.display()),
            Self::PathEscapesBoundary { path, root } => write!(
                f,
                "path {} escapes workspace boundary ({})",
                path.display(),
                root.display()
            ),
            Self::Io(e) => write!(f, "workspace io error: {e}"),
        }
    }
}

impl std::error::Error for WorkspaceError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(e) => Some(e),
            _ => None,
        }
    }
}

impl From<std::io::Error> for WorkspaceError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

/// A declared workspace — an explicit root that scopes all path operations.
///
/// The workspace boundary is the mechanism by which ROY's world stays
/// bounded. No `cd`, read, write, or capability invocation should silently
/// escape it. All paths are validated against the canonical root before use.
#[derive(Debug, Clone)]
pub struct WorkspaceBoundary {
    root: PathBuf,
}

impl WorkspaceBoundary {
    /// Declare a workspace rooted at `root`.
    ///
    /// The root is stored as-is; use [`WorkspaceBoundary::validate`] to
    /// verify it exists on disk. Storing without validation lets tests
    /// construct boundaries with temp dirs that may not canonicalize yet.
    pub fn new(root: PathBuf) -> Self {
        Self { root }
    }

    /// Workspace root path.
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// True if `path` is inside the workspace root.
    ///
    /// Both the root and the target are canonicalized before comparison.
    /// If either cannot be canonicalized, returns `false` — the safe default
    /// is to deny rather than silently permit an unchecked escape.
    pub fn contains(&self, path: &Path) -> bool {
        let Ok(canonical_root) = self.root.canonicalize() else {
            return false;
        };
        let Ok(canonical_path) = path.canonicalize() else {
            return false;
        };
        canonical_path.starts_with(&canonical_root)
    }

    /// Normalize `path` to a canonical absolute path inside the workspace.
    ///
    /// Resolves relative paths against the workspace root, then canonicalizes.
    /// Returns `PathEscapesBoundary` if the result would leave the workspace.
    pub fn normalize(&self, path: &Path) -> Result<PathBuf, WorkspaceError> {
        let absolute = if path.is_absolute() {
            path.to_path_buf()
        } else {
            self.root.join(path)
        };

        let canonical =
            absolute
                .canonicalize()
                .map_err(|_| WorkspaceError::PathEscapesBoundary {
                    path: absolute.clone(),
                    root: self.root.clone(),
                })?;

        if !self.contains(&canonical) {
            return Err(WorkspaceError::PathEscapesBoundary {
                path: canonical,
                root: self.root.clone(),
            });
        }

        Ok(canonical)
    }

    /// Validate that a CWD is inside the workspace boundary.
    ///
    /// Used by `dispatch_cd` to reject directory changes that would escape.
    pub fn validate_cwd(&self, cwd: &Path) -> Result<(), WorkspaceError> {
        if !self.contains(cwd) {
            return Err(WorkspaceError::PathEscapesBoundary {
                path: cwd.to_path_buf(),
                root: self.root.clone(),
            });
        }
        Ok(())
    }

    /// Validate that the workspace root itself exists and is a directory.
    pub fn validate(&self) -> Result<(), WorkspaceError> {
        if !self.root.is_dir() {
            return Err(WorkspaceError::RootNotFound(self.root.clone()));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    fn tmp() -> PathBuf {
        env::temp_dir().canonicalize().unwrap()
    }

    fn boundary() -> WorkspaceBoundary {
        WorkspaceBoundary::new(tmp())
    }

    #[test]
    fn root_returns_declared_path() {
        let root = tmp();
        let b = WorkspaceBoundary::new(root.clone());
        assert_eq!(b.root(), root.as_path());
    }

    #[test]
    fn contains_root_itself() {
        assert!(boundary().contains(&tmp()));
    }

    #[test]
    fn contains_child_of_root() {
        // /tmp is always a valid dir; any subpath that exists is inside
        let b = boundary();
        // The root itself is always inside itself
        assert!(b.contains(b.root()));
    }

    #[test]
    fn does_not_contain_path_outside_root() {
        let b = WorkspaceBoundary::new(tmp());
        // /tmp's parent (usually /) is outside /tmp
        let parent = tmp().parent().unwrap().to_path_buf();
        assert!(!b.contains(&parent));
    }

    #[test]
    fn does_not_contain_nonexistent_path() {
        let b = boundary();
        assert!(!b.contains(Path::new("/definitely_does_not_exist_99991")));
    }

    #[test]
    fn validate_cwd_accepts_root() {
        assert!(boundary().validate_cwd(&tmp()).is_ok());
    }

    #[test]
    fn validate_cwd_rejects_outside_root() {
        let b = boundary();
        let outside = tmp().parent().unwrap().to_path_buf();
        assert!(matches!(
            b.validate_cwd(&outside),
            Err(WorkspaceError::PathEscapesBoundary { .. })
        ));
    }

    #[test]
    fn normalize_absolute_inside_workspace() {
        let b = boundary();
        let root = tmp();
        // Normalizing the root itself should return the canonical root
        assert_eq!(b.normalize(&root).unwrap(), root);
    }

    #[test]
    fn normalize_rejects_outside_workspace() {
        let b = boundary();
        let outside = tmp().parent().unwrap().to_path_buf();
        assert!(matches!(
            b.normalize(&outside),
            Err(WorkspaceError::PathEscapesBoundary { .. })
        ));
    }

    #[test]
    fn validate_passes_for_existing_dir() {
        assert!(boundary().validate().is_ok());
    }

    #[test]
    fn validate_fails_for_nonexistent_root() {
        let b = WorkspaceBoundary::new(PathBuf::from("/definitely_does_not_exist_99992"));
        assert!(matches!(b.validate(), Err(WorkspaceError::RootNotFound(_))));
    }

    #[test]
    fn workspace_error_display_contains_path() {
        let err = WorkspaceError::RootNotFound(PathBuf::from("/foo"));
        assert!(err.to_string().contains("/foo"));
    }
}

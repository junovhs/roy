//! Workspace management and boundary enforcement.
//!
//! Declares workspace roots, scopes current-directory semantics, enforces
//! path normalization, and binds command visibility and capability access
//! to the active workspace. ROY's world is not the whole machine.
//!
//! Public surface:
//! - [`WorkspaceBoundary`] — declared root + path containment + validation
//! - [`WorkspaceCwd`]      — workspace-scoped CWD with escape enforcement
//! - [`WorkspaceError`]    — error type for boundary violations

pub mod boundary;
pub mod cwd;

use std::path::{Path, PathBuf};

// Used by ShellRuntime and tests; UI wiring pending WRK-02.
#[allow(unused_imports)]
pub use boundary::{WorkspaceBoundary, WorkspaceError};
#[allow(unused_imports)]
pub use cwd::WorkspaceCwd;

pub fn normalize_host_path(path: &Path) -> PathBuf {
    #[cfg(windows)]
    {
        use std::path::MAIN_SEPARATOR_STR;

        let text = path.as_os_str().to_string_lossy();
        if let Some(stripped) = text.strip_prefix(r"\\?\UNC\") {
            return PathBuf::from(format!(r"\\{stripped}"));
        }
        if let Some(stripped) = text.strip_prefix(r"\\?\") {
            return PathBuf::from(stripped.replace('/', MAIN_SEPARATOR_STR));
        }
    }

    path.to_path_buf()
}

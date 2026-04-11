//! Shell host runtime.
//!
//! Owns the compatibility shell environment: controlled PATH, CWD semantics,
//! env var management, command dispatch, and IO abstraction. Agents installed
//! inside ROY see this layer as their entire world.
//!
//! Public surface:
//! - [`ShellEnv`]       — controlled environment (PATH, CWD, vars)
//! - [`ShellRuntime`]   — dispatches commands; holds a [`BufferedIo`] transcript
//! - [`ShellIo`]        — IO trait; [`BufferedIo`] is the v0.1 implementation
//! - [`DispatchResult`] — typed outcome of every command dispatch
//! - [`ShellError`]     — error type shared across shell sub-modules
//!
//! TOOL-01 will wire the command registry into [`ShellRuntime::dispatch`].

pub mod env;
pub mod io;
pub mod result;
pub mod runtime;
pub mod traps;

pub use env::ShellEnv;
pub use io::{BufferedIo, ShellIo};
// Used by test sidecars via `crate::shell::DispatchResult`; binary wiring pending SHEL-02.
#[allow(unused_imports)]
pub use result::DispatchResult;
pub use runtime::ShellRuntime;

/// Errors from the shell host layer.
// Used internally by env/runtime and in tests; binary wiring pending SHEL-02.
#[allow(dead_code)]
#[derive(Debug)]
pub enum ShellError {
    /// Target directory does not exist or is not accessible.
    DirNotFound(std::path::PathBuf),
    /// Target path exists but is not a directory.
    NotADirectory(std::path::PathBuf),
    /// Underlying OS IO error.
    Io(std::io::Error),
}

impl std::fmt::Display for ShellError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DirNotFound(p) => write!(f, "directory not found: {}", p.display()),
            Self::NotADirectory(p) => write!(f, "not a directory: {}", p.display()),
            Self::Io(e) => write!(f, "io error: {e}"),
        }
    }
}

impl std::error::Error for ShellError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(e) => Some(e),
            _ => None,
        }
    }
}

impl From<std::io::Error> for ShellError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

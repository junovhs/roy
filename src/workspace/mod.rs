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

// Used by ShellRuntime and tests; UI wiring pending WRK-02.
#[allow(unused_imports)]
pub use boundary::{WorkspaceBoundary, WorkspaceError};
#[allow(unused_imports)]
pub use cwd::WorkspaceCwd;

//! Typed capability runtime for ROY-native commands.
//!
//! Executes a small set of trusted host actions behind structured request and
//! result types. These capabilities back ROY-native commands without exposing a
//! generic shell substrate to the agent.

mod fs;
mod validation;

use std::path::PathBuf;

use crate::workspace::WorkspaceBoundary;

/// Typed request issued by a ROY-native command.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CapabilityRequest {
    Fs(FsCapability),
    Validation(ValidationCapability),
}

/// Filesystem-oriented capabilities scoped to the active workspace.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FsCapability {
    ListDir { path: Option<String> },
    ReadFile { path: String },
    WriteFile { path: String, contents: String },
}

/// Validation capabilities backed by trusted host tools.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidationCapability {
    CargoCheck,
}

/// Structured result of a capability execution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CapabilityOutput {
    DirectoryListing {
        path: PathBuf,
        entries: Vec<WorkspaceEntry>,
    },
    FileContents {
        path: PathBuf,
        contents: String,
    },
    FileWritten {
        path: PathBuf,
        bytes_written: usize,
    },
    ValidationRun {
        command: String,
        cwd: PathBuf,
        exit_code: i32,
        stdout: String,
        stderr: String,
    },
}

impl CapabilityOutput {
    /// Exit code surfaced back through the shell runtime.
    pub fn exit_code(&self) -> i32 {
        match self {
            Self::ValidationRun { exit_code, .. } => *exit_code,
            _ => 0,
        }
    }

    /// Primary user-facing output rendered to the shell transcript.
    pub fn primary_text(&self) -> String {
        match self {
            Self::DirectoryListing { path, entries } => {
                let mut lines = vec![format!("{}:", path.display())];
                lines.extend(
                    entries
                        .iter()
                        .map(|entry| format!("{:<4} {}", entry.kind, entry.name)),
                );
                lines.join("\n")
            }
            Self::FileContents { contents, .. } => contents.clone(),
            Self::FileWritten {
                path,
                bytes_written,
            } => format!("wrote {bytes_written} bytes to {}", path.display()),
            Self::ValidationRun {
                command,
                cwd,
                exit_code,
                stdout,
                stderr,
            } => {
                if *exit_code == 0 {
                    if stdout.trim().is_empty() {
                        format!("{command} passed in {}", cwd.display())
                    } else {
                        stdout.clone()
                    }
                } else if !stderr.trim().is_empty() {
                    stderr.clone()
                } else {
                    format!("{command} failed with exit code {exit_code} in {}", cwd.display())
                }
            }
        }
    }

    /// Secondary stderr channel, when distinct from the primary text.
    pub fn error_text(&self) -> Option<&str> {
        match self {
            Self::ValidationRun {
                exit_code,
                stderr,
                stdout,
                ..
            } if *exit_code != 0 && !stderr.trim().is_empty() && stderr != stdout => {
                Some(stderr)
            }
            _ => None,
        }
    }
}

/// Entry rendered in a workspace listing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkspaceEntry {
    pub name: String,
    pub kind: &'static str,
}

/// Error returned when a capability cannot execute.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapabilityError {
    message: String,
}

impl CapabilityError {
    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl std::fmt::Display for CapabilityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for CapabilityError {}

/// Capability executor bound to one workspace root and cwd.
pub struct CapabilityRuntime {
    workspace: WorkspaceBoundary,
    cwd: PathBuf,
}

impl CapabilityRuntime {
    pub fn new(workspace: WorkspaceBoundary, cwd: PathBuf) -> Self {
        Self { workspace, cwd }
    }

    pub fn execute(
        &self,
        request: &CapabilityRequest,
    ) -> Result<CapabilityOutput, CapabilityError> {
        match request {
            CapabilityRequest::Fs(capability) => self.execute_fs(capability),
            CapabilityRequest::Validation(capability) => self.execute_validation(capability),
        }
    }
}

#[cfg(test)]
#[path = "capability_tests.rs"]
mod tests;

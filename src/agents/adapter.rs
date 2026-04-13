#![allow(dead_code)]

//! Embedded-agent adapter contract — types and traits for AGEN-02+ adapters.

use std::io::Write;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

// ── identity ──────────────────────────────────────────────────────────────────

/// Which terminal-native agent product is hosted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AgentKind {
    ClaudeCode,
    Codex,
    Custom { name: String },
}

impl AgentKind {
    /// Short stable identifier used in logs and session records.
    pub fn id(&self) -> &str {
        match self {
            Self::ClaudeCode => "claude-code",
            Self::Codex => "codex",
            Self::Custom { name } => name.as_str(),
        }
    }
}

/// Metadata about an installed agent — version and install location.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentMeta {
    pub kind: AgentKind,
    /// Semver or tool-reported version string.
    pub version: String,
    /// Absolute path to the agent binary.
    pub install_path: PathBuf,
}

// ── auth ──────────────────────────────────────────────────────────────────────

/// How the agent authenticates to its upstream service.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AgentAuthMethod {
    /// No authentication required (e.g. local-only or pre-configured).
    None,
    /// Token is read from an environment variable named `key`.
    EnvVar { key: String },
    /// OAuth device-flow handshake is needed before launching.
    OauthDevice,
    /// Either device auth or an injected API token can be used.
    OauthDeviceOrEnvVar { key: String },
}

// ── launch config ─────────────────────────────────────────────────────────────

/// Configuration for launching an agent process inside ROY.
#[derive(Debug, Clone)]
pub struct LaunchConfig {
    pub workspace_root: PathBuf,
    pub session_id: u64,
    pub env_overrides: Vec<(String, String)>,
}

// ── supervision events ────────────────────────────────────────────────────────

/// Supervision event from a running agent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SupervisionEvent {
    AgentStarted { pid: u32 },
    OutputLine { text: String },
    ErrorLine { text: String },
    OutputChunk { bytes: Vec<u8>, is_stderr: bool },
    CommandAttempt { command: String, args: Vec<String> },
    ProcessExited { code: i32 },
}

// ── adapter trait ─────────────────────────────────────────────────────────────

/// Contract for an embedded-agent adapter — stateless, one kind per impl.
pub trait AgentAdapter: Send + Sync {
    fn meta(&self) -> &AgentMeta;
    fn auth_method(&self) -> AgentAuthMethod;
    /// Spawn the agent; returns a handle for polling supervision events.
    fn launch(&self, config: LaunchConfig) -> Result<AgentHandle, AgentError>;
}

// ── handle ────────────────────────────────────────────────────────────────────

/// Running (or recently-exited) agent process owned by the ROY shell host.
///
/// Attach a supervision queue via `set_pending`; drain via `drain_pending`.
pub struct AgentHandle {
    pub meta: AgentMeta,
    pub session_id: u64,
    events: Vec<SupervisionEvent>,
    exit_code: Option<i32>,
    pending: Option<Arc<Mutex<Vec<SupervisionEvent>>>>,
    stdin: Option<SharedAgentInput>,
}

type SharedAgentInput = Arc<Mutex<Box<dyn Write + Send>>>;

impl AgentHandle {
    /// Create a handle for a newly-started agent process.
    pub fn new(meta: AgentMeta, session_id: u64) -> Self {
        Self {
            meta,
            session_id,
            events: Vec::new(),
            exit_code: None,
            pending: None,
            stdin: None,
        }
    }

    /// Record a supervision event; exit codes from ProcessExited are captured.
    pub fn push_event(&mut self, event: SupervisionEvent) {
        if let SupervisionEvent::ProcessExited { code } = &event {
            self.exit_code = Some(*code);
        }
        self.events.push(event);
    }

    /// All supervision events recorded so far, in arrival order.
    pub fn events(&self) -> &[SupervisionEvent] {
        &self.events
    }

    pub fn exit_code(&self) -> Option<i32> {
        self.exit_code
    }

    pub fn has_exited(&self) -> bool {
        self.exit_code.is_some()
    }

    /// Attach the supervision queue produced by a concrete adapter's threads.
    pub fn set_pending(&mut self, queue: Arc<Mutex<Vec<SupervisionEvent>>>) {
        self.pending = Some(queue);
    }

    /// Attach a writable input stream for forwarding user text to the agent.
    pub fn set_stdin(&mut self, writer: SharedAgentInput) {
        self.stdin = Some(writer);
    }

    /// Take all buffered events, leaving the log empty.
    ///
    /// Exit state is preserved in `exit_code` regardless.
    /// Used by the runtime poll loop to consume new output each tick.
    pub fn take_events(&mut self) -> Vec<SupervisionEvent> {
        std::mem::take(&mut self.events)
    }

    /// Drain pending events from supervision threads into the event log.
    ///
    /// Safe to call on a polling loop; exit codes are captured automatically.
    pub fn drain_pending(&mut self) {
        let Some(queue) = &self.pending else { return };
        let drained = {
            let mut locked = queue.lock().expect("supervision queue lock poisoned");
            locked.drain(..).collect::<Vec<_>>()
        };
        for event in drained {
            self.push_event(event);
        }
    }

    pub fn send_input(&self, input: &str) -> Result<(), AgentError> {
        self.send_raw_bytes(input.as_bytes())
    }

    /// Forward raw bytes to the running agent without modification (PTY passthrough).
    pub fn send_raw_bytes(&self, bytes: &[u8]) -> Result<(), AgentError> {
        let Some(writer) = &self.stdin else {
            return Err(AgentError::io_error("agent input is unavailable"));
        };
        let mut locked = writer
            .lock()
            .map_err(|_| AgentError::io_error("agent input lock poisoned"))?;
        locked
            .write_all(bytes)
            .map_err(|e| AgentError::io_error(e.to_string()))?;
        locked
            .flush()
            .map_err(|e| AgentError::io_error(e.to_string()))?;
        Ok(())
    }
}

// ── error ─────────────────────────────────────────────────────────────────────

/// Error produced during agent launch or supervision.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentError {
    kind: AgentErrorKind,
    message: String,
}

/// Classification of agent errors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AgentErrorKind {
    /// Agent binary not found or not installed.
    NotInstalled,
    /// Process could not be started.
    LaunchFailed,
    /// Authentication is required before the agent can be launched.
    AuthRequired,
    /// An I/O error occurred during supervision.
    IoError,
}

impl AgentError {
    pub fn not_installed(name: &str) -> Self {
        Self {
            kind: AgentErrorKind::NotInstalled,
            message: format!("{name} is not installed or not found on PATH"),
        }
    }

    pub fn launch_failed(message: impl Into<String>) -> Self {
        Self {
            kind: AgentErrorKind::LaunchFailed,
            message: message.into(),
        }
    }

    pub fn auth_required(message: impl Into<String>) -> Self {
        Self {
            kind: AgentErrorKind::AuthRequired,
            message: message.into(),
        }
    }

    pub fn io_error(message: impl Into<String>) -> Self {
        Self {
            kind: AgentErrorKind::IoError,
            message: message.into(),
        }
    }

    /// Classification of this error.
    pub fn kind(&self) -> &AgentErrorKind {
        &self.kind
    }
}

impl std::fmt::Display for AgentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for AgentError {}

// ── tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
#[path = "adapter_tests.rs"]
mod tests;

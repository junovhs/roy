#![allow(dead_code)]

use std::io::Write;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AgentKind {
    ClaudeCode,
    Codex,
    Custom { name: String },
}

impl AgentKind {
    pub fn id(&self) -> &str {
        match self {
            Self::ClaudeCode => "claude-code",
            Self::Codex => "codex",
            Self::Custom { name } => name.as_str(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentMeta {
    pub kind: AgentKind,
    pub version: String,
    pub install_path: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AgentAuthMethod {
    None,
    EnvVar { key: String },
    OauthDevice,
    OauthDeviceOrEnvVar { key: String },
}

#[derive(Debug, Clone)]
pub struct LaunchConfig {
    pub workspace_root: PathBuf,
    pub session_id: u64,
    pub env_overrides: Vec<(String, String)>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SupervisionEvent {
    AgentStarted { pid: u32 },
    OutputLine { text: String },
    ErrorLine { text: String },
    OutputChunk { bytes: Vec<u8>, is_stderr: bool },
    CommandAttempt { command: String, args: Vec<String> },
    ProcessExited { code: i32 },
}

pub trait AgentAdapter: Send + Sync {
    fn meta(&self) -> &AgentMeta;
    fn auth_method(&self) -> AgentAuthMethod;
    fn launch(&self, config: LaunchConfig) -> Result<AgentHandle, AgentError>;
}

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

    pub fn push_event(&mut self, event: SupervisionEvent) {
        if let SupervisionEvent::ProcessExited { code } = &event {
            self.exit_code = Some(*code);
        }
        self.events.push(event);
    }

    pub fn events(&self) -> &[SupervisionEvent] {
        &self.events
    }

    pub fn exit_code(&self) -> Option<i32> {
        self.exit_code
    }

    pub fn has_exited(&self) -> bool {
        self.exit_code.is_some()
    }

    pub fn set_pending(&mut self, queue: Arc<Mutex<Vec<SupervisionEvent>>>) {
        self.pending = Some(queue);
    }

    pub fn set_stdin(&mut self, writer: SharedAgentInput) {
        self.stdin = Some(writer);
    }

    pub fn take_events(&mut self) -> Vec<SupervisionEvent> {
        std::mem::take(&mut self.events)
    }

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentError {
    kind: AgentErrorKind,
    message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AgentErrorKind {
    NotInstalled,
    LaunchFailed,
    AuthRequired,
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

#[cfg(test)]
#[path = "adapter_tests.rs"]
mod tests;

pub mod agent;
pub mod config;
pub mod denial;
pub mod interceptor;
pub mod policy;
pub mod session;
pub mod session_log;

pub use agent::{AgentAdapter, AgentHost, ClaudeCodeAdapter, CodexAdapter};
pub use config::{RoyConfig, load_default as load_config};
pub use denial::DenialResponse;
pub use interceptor::{Disposition, LineBuffer, RoyInterceptor};
pub use policy::PolicyEngine;
pub use session::{RoySession, session_from_env};
pub use session_log::DenialLog;

//! Policy engine for command and capability execution.
//!
//! Every resolved command passes through policy before execution. Policy
//! determines allowance, blocking, approval requirements, logging level,
//! output transforms, and rate limits. Attached to sessions and workspaces,
//! not globally.
//!
//! Public surface:
//! - [`PolicyEngine`]    — evaluates commands against a profile
//! - [`PolicyOutcome`]   — Allow / Deny / ApprovalPending
//! - [`PolicyProfile`]   — named rule set (permissive / read_only / dev)
//! - [`PolicyPermission`]— per-command permission value

pub mod engine;
pub mod profile;

// Used by ShellRuntime dispatch; unused_imports suppressed pending wiring.
#[allow(unused_imports)]
pub use engine::{PolicyEngine, PolicyOutcome};
#[allow(unused_imports)]
pub use profile::{PolicyPermission, PolicyProfile};

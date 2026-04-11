//! Command resolution and substitution registry.
//!
//! Resolves command names to ROY-native capabilities, wrapped utilities,
//! compatibility traps, or explicit denials. This is the control-plane
//! mechanism that prevents shell-shaped fallback.
//!
//! Public surface:
//! - [`CommandSchema`]   — full metadata for one command entry
//! - [`CommandRegistry`] — the explicit substitution table
//! - [`schema`]          — type definitions (Backend, RiskLevel, Visibility)

pub mod registry;
pub mod schema;

// Used by ShellRuntime dispatch and tests; additional wiring pending TOOL-02.
#[allow(unused_imports)]
pub use registry::CommandRegistry;
#[allow(unused_imports)]
pub use schema::CommandSchema;

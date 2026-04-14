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

pub mod ast;
pub mod fs;
pub mod plan;
pub mod registry;
pub mod schema;
pub mod validation;

use crate::capabilities::CapabilityRequest;

// Used by ShellRuntime dispatch and tests; additional wiring pending TOOL-02.
#[allow(unused_imports)]
pub use registry::CommandRegistry;
#[allow(unused_imports)]
pub use schema::CommandSchema;

pub(crate) fn parse_native_request(
    name: &str,
    args: &[&str],
) -> Result<Option<CapabilityRequest>, String> {
    if let Some(request) = fs::parse_request(name, args) {
        return request.map(Some);
    }

    if let Some(request) = validation::parse_request(name, args) {
        return request.map(Some);
    }

    Ok(None)
}

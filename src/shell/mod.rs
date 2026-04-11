//! Shell host runtime.
//!
//! Owns the compatibility shell environment: process model, PATH shaping,
//! command lookup, IO streams, and working-directory semantics. Agents
//! installed inside ROY see this layer as their world.
//!
//! Populated by SHEL-01.

//! SQLite persistence layer.
//!
//! Stores sessions, shell traces, installed-agent metadata, workspace
//! bindings, policy profiles, and artifact references. Local and boring
//! by design — correctness and inspectability over cleverness.
//!
//! Primary entry point: [`sqlite::RoyStore`].

pub mod sqlite;

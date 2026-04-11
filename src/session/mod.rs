//! Session and transcript event ledger.
//!
//! Persists typed events beyond raw scrollback: user inputs, agent outputs,
//! command invocations, policy denials, approval pauses, artifacts, and
//! host-level notices. Provides replayability and debuggability.
//!
//! Public surface:
//! - [`Session`]      — event ledger for one shell session
//! - [`SessionEvent`] — all typed events that can occur
//! - [`Timestamp`]    — `u64` ms since UNIX epoch
//!
//! STOR-01 will wire persistence; SES-02 integrates with `ShellRuntime`.

pub mod engine;
pub mod events;

// Used by tests; binary wiring pending SES-02.
#[allow(unused_imports)]
pub use engine::Session;
#[allow(unused_imports)]
pub use events::{SessionEvent, Timestamp};

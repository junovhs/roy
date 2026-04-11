//! Session and transcript event ledger.
//!
//! Persists typed events beyond raw scrollback: user inputs, agent outputs,
//! command invocations, policy denials, approval pauses, artifacts, and
//! host-level notices. Provides replayability and debuggability.
//!
//! Populated by SES-01.

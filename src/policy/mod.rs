//! Policy engine for command and capability execution.
//!
//! Every resolved command passes through policy before execution. Policy
//! determines allowance, blocking, approval requirements, logging level,
//! output transforms, and rate limits. Attached to sessions and workspaces,
//! not globally.
//!
//! Populated by POL-01.

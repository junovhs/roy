# ROY Architecture — Control-Plane Seam

## Status

ARCH-01 full seam specification. Updated from FORK-01 stub.

---

## Integration Boundary

### Seam location

**`alacritty/src/event.rs` → `ActionContext::write_to_pty`**

This is the single point where all user input bytes pass before reaching the PTY process. It has access to the full terminal state (`self.terminal`) needed to detect raw-mode programs, making it the correct place to decide whether interception applies.

### Where ROY state lives

`roy/` — a new top-level workspace crate. Never upstreamed.

```
roy/
  Cargo.toml
  src/
    lib.rs            ← public surface re-exports
    interceptor.rs    ← RoyInterceptor trait + line buffer
    policy.rs         ← allow/deny/redirect rule engine
    denial.rs         ← DenialResponse (structured, serializable)
    workspace.rs      ← workspace config (roy.toml), session state
    session.rs        ← session artifact ledger
```

Alacritty's tracked crates (`alacritty/`, `alacritty_terminal/`, etc.) are touched minimally:
- **`alacritty/src/window_context.rs`**: add `Option<Arc<dyn RoyInterceptor>>` field to `WindowContext`. Wired at session spawn from config.
- **`alacritty/src/event.rs`**: add `Option<&'a Arc<dyn RoyInterceptor>>` field to `ActionContext`. Call interceptor in `write_to_pty`. That is the full extent of the ROY coupling on Alacritty core.

No ROY types leak beyond these two files.

---

## Trait Signatures

```rust
// roy/src/interceptor.rs

use std::borrow::Cow;

/// Outcome returned by the interceptor for each PTY write.
pub enum Disposition {
    /// Pass bytes through to the shell unchanged.
    Passthrough,
    /// Block the write. Print a DenialResponse to the terminal.
    Denied(DenialResponse),
    /// Replace the write with different bytes (redirect to owned command).
    Redirect(Cow<'static, [u8]>),
}

/// The hook ROY injects into the PTY write path.
///
/// Implementations must be `Send + Sync` — `WindowContext` is shared across
/// winit event handlers.
pub trait RoyInterceptor: Send + Sync {
    /// Called before bytes are sent to the PTY.
    ///
    /// `bytes` is the raw input (may be a partial line during interactive typing).
    /// `in_raw_mode` — true if the running process has set the PTY to raw mode
    ///   (vim, less, interactive programs). When true, return Passthrough immediately.
    fn intercept(&self, bytes: &[u8], in_raw_mode: bool) -> Disposition;
}
```

```rust
// roy/src/denial.rs

use serde::{Deserialize, Serialize};

/// Structured denial event — printed inline, written to session log.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DenialResponse {
    /// The blocked input (redacted if sensitive).
    pub blocked: String,
    /// Human-readable reason (policy rule name or description).
    pub reason: String,
    /// The owned alternative the agent should use instead.
    pub alternative: Option<String>,
    /// Policy rule ID that matched, for log correlation.
    pub rule_id: Option<String>,
}

impl DenialResponse {
    /// Render as a terminal message string (ANSI-styled ROY prefix).
    pub fn render(&self) -> String {
        // Implemented in PTY-01
        todo!()
    }
}
```

---

## Hook Wiring — Session Spawn

```rust
// alacritty/src/window_context.rs (sketch — implemented in PTY-01)

pub struct WindowContext {
    // ... existing fields ...
    roy_interceptor: Option<Arc<dyn RoyInterceptor>>,
}

// At WindowContext::initial():
let roy_interceptor = if config.roy.enabled {
    Some(Arc::new(RoySession::from_config(&config.roy)) as Arc<dyn RoyInterceptor>)
} else {
    None
};
```

```rust
// alacritty/src/event.rs (sketch — implemented in PTY-01)

pub struct ActionContext<'a, N, T> {
    // ... existing fields ...
    pub roy_interceptor: Option<&'a Arc<dyn RoyInterceptor>>,
}

impl<'a, N: Notify + 'a, T: EventListener> input::ActionContext<T> for ActionContext<'a, N, T> {
    fn write_to_pty<B: Into<Cow<'static, [u8]>>>(&self, val: B) {
        let bytes = val.into();
        if let Some(interceptor) = self.roy_interceptor {
            let in_raw_mode = self.terminal.mode().contains(TermMode::RAW_MODE_FLAGS);
            match interceptor.intercept(&bytes, in_raw_mode) {
                Disposition::Passthrough => self.notifier.notify(bytes),
                Disposition::Denied(resp) => {
                    // Write denial message back to terminal display (PTY-01 / DENY-01)
                    self.notifier.notify(resp.render().into_bytes());
                },
                Disposition::Redirect(new_bytes) => self.notifier.notify(new_bytes),
            }
        } else {
            self.notifier.notify(bytes);
        }
    }
}
```

---

## Raw-Mode Detection

`alacritty_terminal::term::TermMode` has mode flags. The relevant combination for "running process owns the terminal":

```rust
// alacritty_terminal/src/term/mod.rs — TermMode flags to check
const RAW_MODE_FLAGS: TermMode =
    TermMode::APPLICATION_CURSOR   // cursor app mode — set by interactive programs
    | TermMode::MOUSE_MODE;        // any mouse tracking — strong signal for raw mode
```

This is a heuristic. A process in raw mode with no mouse tracking won't be caught by this — but in that case the process is either a simple full-screen program or an agent tool. Agents write complete newline-terminated commands; interactive programs in raw mode are protected by the mouse/cursor check. Refinement is PTY-01's problem.

---

## Line Buffer

Individual keystrokes arrive as separate `write_to_pty` calls (single bytes). The interceptor needs its own line buffer to assemble complete commands before policy evaluation.

```rust
// Sketch — lives in RoySession/interceptor impl

struct LineBuffer {
    buf: Vec<u8>,
}

impl LineBuffer {
    /// Returns Some(complete_line) when a newline is received, else None.
    fn push(&mut self, bytes: &[u8]) -> Option<Vec<u8>> {
        self.buf.extend_from_slice(bytes);
        if self.buf.contains(&b'\n') || self.buf.contains(&b'\r') {
            Some(std::mem::take(&mut self.buf))
        } else {
            None
        }
    }
}
```

The interceptor returns `Passthrough` for partial lines and evaluates the policy only on complete lines. Partial lines are buffered internally and not blocked. This means denial is delivered after the user hits Enter (or the agent sends the newline) — correct UX.

---

## Upstream Sync Strategy

| Path | Ownership | Notes |
|------|-----------|-------|
| `alacritty/` | Upstream-trackable | ROY edits: two fields added (WindowContext + ActionContext), one call site in `write_to_pty`. Merge conflicts expected only in `event.rs` and `window_context.rs`. |
| `alacritty_terminal/` | Upstream-trackable | No ROY edits planned. |
| `alacritty_config/` | Upstream-trackable | No ROY edits planned. |
| `alacritty_config_derive/` | Upstream-trackable | No ROY edits planned. |
| `roy/` | Local-only | Never submitted upstream. |
| `CLAUDE.md`, `AGENTS.md`, `neti.toml`, `ARCHITECTURE.md`, `rustfmt.toml` | Local-only | Doctrine tooling config. |

Upstream sync: cherry-pick or merge from upstream `alacritty/alacritty`. Expect periodic conflict resolution in `alacritty/src/event.rs` around the `roy_interceptor` field and `write_to_pty` body.

---

## Issue Dependency Map

```
FORK-01 (done)
  └─ ARCH-01 (this doc)
       ├─ PTY-01   — implement RoyInterceptor + hook write_to_pty
       ├─ CFG-01   — roy.toml config integration
       └─ CORE-01  — policy engine, workspace core
            ├─ DENY-01  — DenialResponse rendering + session log
            ├─ STOR-01  — session artifacts + diagnostics
            └─ AGEN-01  — agent supervision
```

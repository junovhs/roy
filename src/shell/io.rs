// API is live in tests; binary wiring pending TOOL-01 / SHEL-02.
#![allow(dead_code)]

use super::ShellEnv;

/// IO surface for a ROY shell session.
///
/// Decouples the shell runtime from concrete output targets so the
/// Dioxus UI pane, tests, and future CLI modes can all be IO targets
/// without the runtime knowing which one it addresses.
pub trait ShellIo {
    /// Write a line of output text (stdout channel).
    fn write_line(&mut self, text: &str);
    /// Write a line of error text (stderr channel).
    fn write_error(&mut self, text: &str);
    /// Produce the prompt string shown before user input.
    fn prompt_str(&self, env: &ShellEnv) -> String;
}

/// Buffered IO — collects output into in-memory vecs.
///
/// Used by [`ShellRuntime`](super::ShellRuntime) as its session
/// transcript buffer and by tests to inspect dispatch output without
/// touching any real file descriptor.
pub struct BufferedIo {
    pub output: Vec<String>,
    pub errors: Vec<String>,
}

impl BufferedIo {
    pub fn new() -> Self {
        Self { output: Vec::new(), errors: Vec::new() }
    }
}

impl Default for BufferedIo {
    fn default() -> Self {
        Self::new()
    }
}

impl ShellIo for BufferedIo {
    fn write_line(&mut self, text: &str) {
        self.output.push(text.to_string());
    }

    fn write_error(&mut self, text: &str) {
        self.errors.push(text.to_string());
    }

    fn prompt_str(&self, env: &ShellEnv) -> String {
        format!("roy:{}\u{276f} ", env.cwd().display())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn make_env() -> ShellEnv {
        ShellEnv::new(PathBuf::from("/tmp"))
    }

    #[test]
    fn buffered_io_captures_output_lines() {
        let mut io = BufferedIo::new();
        io.write_line("hello");
        io.write_line("world");
        assert_eq!(io.output, vec!["hello", "world"]);
        assert!(io.errors.is_empty());
    }

    #[test]
    fn buffered_io_captures_errors_separately() {
        let mut io = BufferedIo::new();
        io.write_line("ok");
        io.write_error("bad");
        assert_eq!(io.output, vec!["ok"]);
        assert_eq!(io.errors, vec!["bad"]);
    }

    #[test]
    fn buffered_io_prompt_includes_cwd() {
        let io = BufferedIo::new();
        let env = make_env();
        let prompt = io.prompt_str(&env);
        assert!(prompt.contains("/tmp"), "prompt must show cwd");
    }

    #[test]
    fn buffered_io_default_is_empty() {
        let io = BufferedIo::default();
        assert!(io.output.is_empty());
        assert!(io.errors.is_empty());
    }
}

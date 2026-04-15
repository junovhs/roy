use std::thread::{Builder, JoinHandle};

/// Like `thread::spawn`, but with a `name` argument.
pub fn spawn_named<F, T, S>(name: S, f: F) -> JoinHandle<T>
where
    F: FnOnce() -> T + Send + 'static,
    T: Send + 'static,
    S: Into<String>,
{
    match Builder::new().name(name.into()).spawn(f) {
        Ok(handle) => handle,
        Err(err) => panic!("failed to spawn named thread: {err}"),
    }
}

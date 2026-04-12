use crate::session::{Session, SessionEvent, Timestamp};

pub(crate) fn now_millis() -> Timestamp {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as Timestamp
}

pub(crate) fn short_path_label(path: &std::path::Path) -> String {
    path.file_name()
        .and_then(std::ffi::OsStr::to_str)
        .map(str::to_string)
        .unwrap_or_else(|| path.display().to_string())
}

pub(crate) fn relative_scope_label(root: &std::path::Path, cwd: &std::path::Path) -> String {
    cwd.strip_prefix(root)
        .ok()
        .and_then(|path| {
            if path.as_os_str().is_empty() {
                None
            } else {
                Some(format!("/{}", path.display()))
            }
        })
        .unwrap_or_else(|| "/".to_string())
}

pub(crate) fn is_session_active(session: &Session) -> bool {
    !matches!(
        session.events().last(),
        Some(SessionEvent::SessionEnded { .. })
    )
}

pub(super) fn build_cockpit_session(session_root: std::path::PathBuf, ts: Timestamp) -> Session {
    let mut s = Session::new(ts, session_root, ts);
    s.push(SessionEvent::HostNotice {
        message: "ROY shell cockpit ready".to_string(),
        ts: ts + 1,
    });
    s
}

pub(super) fn drawer_selected(open_drawer: Option<&str>, drawer: &str) -> bool {
    open_drawer == Some(drawer)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::{Path, PathBuf};
    use std::time::{SystemTime, UNIX_EPOCH};

    fn session() -> Session {
        Session::new(7, PathBuf::from("/tmp/roy-layout-tests"), 10)
    }

    #[test]
    fn now_millis_returns_current_epoch_millis() {
        let before = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("before")
            .as_millis() as Timestamp;
        let ts = now_millis();
        let after = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("after")
            .as_millis() as Timestamp;

        assert!(ts >= before);
        assert!(ts <= after);
        assert!(ts > 1_000_000_000_000);
    }

    #[test]
    fn short_path_label_prefers_file_name() {
        assert_eq!(
            short_path_label(Path::new("/tmp/demo/file.txt")),
            "file.txt"
        );
    }

    #[test]
    fn short_path_label_falls_back_to_display_for_root_like_paths() {
        assert_eq!(short_path_label(Path::new("/")), "/");
    }

    #[test]
    fn relative_scope_label_is_slash_for_workspace_root() {
        assert_eq!(
            relative_scope_label(Path::new("/tmp/project"), Path::new("/tmp/project")),
            "/"
        );
    }

    #[test]
    fn relative_scope_label_formats_child_path() {
        assert_eq!(
            relative_scope_label(
                Path::new("/tmp/project"),
                Path::new("/tmp/project/src/ui/layout")
            ),
            "/src/ui/layout"
        );
    }

    #[test]
    fn relative_scope_label_denies_outside_paths_with_root_marker() {
        assert_eq!(
            relative_scope_label(Path::new("/tmp/project"), Path::new("/outside")),
            "/"
        );
    }

    #[test]
    fn is_session_active_only_until_end_event() {
        let mut session = session();
        assert!(is_session_active(&session));

        session.end(9, 20);
        assert!(!is_session_active(&session));
    }

    #[test]
    fn build_cockpit_session_adds_host_notice_after_start_event() {
        let session = build_cockpit_session(PathBuf::from("/tmp/cockpit"), 42);
        let events = session.events();

        assert!(matches!(events[0], SessionEvent::SessionStarted { ts: 42 }));
        assert!(matches!(
            &events[1],
            SessionEvent::HostNotice { message, ts }
            if message == "ROY shell cockpit ready" && *ts == 43
        ));
    }

    #[test]
    fn drawer_selected_matches_only_the_active_drawer() {
        assert!(drawer_selected(Some("diag"), "diag"));
        assert!(!drawer_selected(Some("diag"), "review"));
        assert!(!drawer_selected(None, "diag"));
    }
}

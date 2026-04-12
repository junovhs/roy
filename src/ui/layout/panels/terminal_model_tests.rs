use super::*;
use crate::session::{Session, SessionEvent};
use crate::shell::{DispatchResult, ShellRuntime};

mod test_support {
    use super::{Session, ShellLine, ShellRuntime, SubmitContext};
    use dioxus::{
        core::RuntimeGuard,
        prelude::{Element, ScopeId, Signal, VNode, VirtualDom},
    };

    pub(super) fn make_session() -> Session {
        Session::new(101, std::env::temp_dir(), 10)
    }

    pub(super) fn with_runtime<T>(f: impl FnOnce() -> T) -> T {
        fn app() -> Element {
            VNode::empty()
        }

        let mut dom = VirtualDom::new(app);
        dom.rebuild_to_vec();
        let _guard = RuntimeGuard::new(dom.runtime());
        f()
    }

    pub(super) type TestCtx = (
        Signal<ShellRuntime>,
        Signal<Session>,
        Signal<Vec<ShellLine>>,
        Signal<String>,
        SubmitContext,
    );

    pub(super) fn make_ctx() -> TestCtx {
        let runtime = Signal::new_in_scope(ShellRuntime::new(std::env::temp_dir()), ScopeId::APP);
        let session = Signal::new_in_scope(make_session(), ScopeId::APP);
        let lines = Signal::new_in_scope(Vec::new(), ScopeId::APP);
        let input_text = Signal::new_in_scope("pending".to_string(), ScopeId::APP);
        let ctx = SubmitContext {
            runtime,
            session,
            lines,
            input_text,
        };
        (runtime, session, lines, input_text, ctx)
    }
}

#[path = "terminal_model_tests_parse.rs"]
mod parse;
#[path = "terminal_model_tests_submit.rs"]
mod submit;

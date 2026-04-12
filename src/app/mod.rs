use dioxus::prelude::*;

use crate::ui::layout::Cockpit;

/// Root application component. Bootstraps fonts, CSS animations, and the ROY
/// shell cockpit.
#[component]
pub fn App() -> Element {
    // Inject Google Fonts + keyframe animations into the webview <head> once
    // on mount. The eval is fire-and-forget; results are not needed.
    use_effect(move || {
        let _ = document::eval(
            "var r=document.createElement('style');\
             r.textContent='html,body{margin:0;padding:0;overflow:hidden;box-sizing:border-box;}';\
             document.head.appendChild(r);\
             var s=document.createElement('style');\
             s.textContent='\
               @keyframes pulse{0%,100%{opacity:1;transform:scale(1)}50%{opacity:.5;transform:scale(.85)}}\
               @keyframes blink{50%{opacity:0}}\
             ';\
             document.head.appendChild(s);\
             var l=document.createElement('link');\
             l.rel='stylesheet';\
             l.href='https://fonts.googleapis.com/css2?family=Geist:wght@300;400;500;600\
               &family=JetBrains+Mono:wght@300;400;500\
               &family=Fraunces:ital,opsz,wght@0,9..144,300;1,9..144,300;1,9..144,400\
               &display=swap';\
             document.head.appendChild(l);",
        );
    });
    rsx! {
        Cockpit {}
    }
}

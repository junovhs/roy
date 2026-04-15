use log::{debug, warn};
use winit::raw_window_handle::RawDisplayHandle;

use alacritty_terminal::term::ClipboardType;

#[cfg(any(feature = "x11", target_os = "macos", windows))]
use copypasta::ClipboardContext;
use copypasta::ClipboardProvider;
use copypasta::nop_clipboard::NopClipboardContext;
#[cfg(all(feature = "wayland", not(any(target_os = "macos", windows))))]
use copypasta::wayland_clipboard;
#[cfg(all(feature = "x11", not(any(target_os = "macos", windows))))]
use copypasta::x11_clipboard::{Primary as X11SelectionClipboard, X11ClipboardContext};

pub struct Clipboard {
    clipboard: Box<dyn ClipboardProvider>,
    selection: Option<Box<dyn ClipboardProvider>>,
}

fn nop_clipboard() -> Box<dyn ClipboardProvider> {
    match NopClipboardContext::new() {
        Ok(clipboard) => Box::new(clipboard),
        Err(err) => {
            warn!("Unable to initialize nop clipboard provider: {err}");
            Box::new(NopClipboardContext)
        },
    }
}

impl Clipboard {
    pub unsafe fn new(display: RawDisplayHandle) -> Self {
        match display {
            #[cfg(all(feature = "wayland", not(any(target_os = "macos", windows))))]
            RawDisplayHandle::Wayland(display) => {
                // SAFETY: `display.display` comes from winit's live Wayland display handle and
                // remains valid for clipboard provider construction during this call.
                let (selection, clipboard) = unsafe {
                    wayland_clipboard::create_clipboards_from_external(display.display.as_ptr())
                };
                Self { clipboard: Box::new(clipboard), selection: Some(Box::new(selection)) }
            },
            _ => Self::default(),
        }
    }

    /// Used for tests, to handle missing clipboard provider when built without the `x11`
    /// feature, and as default clipboard value.
    pub fn new_nop() -> Self {
        Self { clipboard: nop_clipboard(), selection: None }
    }
}

impl Default for Clipboard {
    fn default() -> Self {
        #[cfg(any(target_os = "macos", windows))]
        return match ClipboardContext::new() {
            Ok(clipboard) => Self { clipboard: Box::new(clipboard), selection: None },
            Err(err) => {
                warn!("Unable to initialize clipboard provider: {err}");
                Self::new_nop()
            },
        };

        #[cfg(all(feature = "x11", not(any(target_os = "macos", windows))))]
        return match (ClipboardContext::new(), X11ClipboardContext::<X11SelectionClipboard>::new())
        {
            (Ok(clipboard), Ok(selection)) => {
                Self { clipboard: Box::new(clipboard), selection: Some(Box::new(selection)) }
            },
            (clipboard_result, selection_result) => {
                if let Err(err) = clipboard_result {
                    warn!("Unable to initialize X11 clipboard provider: {err}");
                }
                if let Err(err) = selection_result {
                    warn!("Unable to initialize X11 selection provider: {err}");
                }
                Self::new_nop()
            },
        };

        #[cfg(not(any(feature = "x11", target_os = "macos", windows)))]
        return Self::new_nop();
    }
}

impl Clipboard {
    pub fn store(&mut self, ty: ClipboardType, text: impl Into<String>) {
        let clipboard = match (ty, &mut self.selection) {
            (ClipboardType::Selection, Some(provider)) => provider,
            (ClipboardType::Selection, None) => return,
            _ => &mut self.clipboard,
        };

        if let Err(err) = clipboard.set_contents(text.into()) {
            warn!("Unable to store text in clipboard: {err}");
        }
    }

    pub fn load(&mut self, ty: ClipboardType) -> String {
        let clipboard = match (ty, &mut self.selection) {
            (ClipboardType::Selection, Some(provider)) => provider,
            _ => &mut self.clipboard,
        };

        match clipboard.get_contents() {
            Err(err) => {
                debug!("Unable to load text from clipboard: {err}");
                String::new()
            },
            Ok(text) => text,
        }
    }
}

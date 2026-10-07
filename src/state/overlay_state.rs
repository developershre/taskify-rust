use dioxus::prelude::*;

// ============================================================
// Overlay State — completely separate from the global AppState
// ============================================================

/// Coordinates open dialogs/modals across the UI.
///
/// This lives in its own context and is provided once at the app root
/// (`App` in main.rs), so none of its signals leak into `AppState`.
#[derive(Clone, Copy, PartialEq)]
pub struct OverlayState {
    dialog_open: Signal<bool>,
}

impl OverlayState {
    pub fn set_dialog_open(mut self, open: bool) {
        self.dialog_open.set(open);
    }

    pub fn is_dialog_open(self) -> bool {
        *self.dialog_open.read()
    }
}

/// Provides the overlay state at the app root. Safe to call once per mount;
/// consumers retrieve it with `use_context::<OverlayState>()`.
pub fn use_overlay_state() -> OverlayState {
    let dialog_open = use_signal(|| false);
    use_context_provider(move || OverlayState { dialog_open })
}

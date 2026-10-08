use dioxus::prelude::*;

/// Open/close status of the right-hand calendar & notes sidebar.
///
/// Deliberately kept separate from the global `AppState` and defaults to
/// closed (`false`).
#[derive(Clone, Copy, PartialEq)]
pub struct CalendarSidebarState {
    pub open: Signal<bool>,
}

impl CalendarSidebarState {
    pub fn is_open(self) -> bool {
        *self.open.read()
    }

    pub fn set_open(mut self, open: bool) {
        self.open.set(open);
    }

    pub fn toggle(mut self) {
        let cur = *self.open.read();
        self.open.set(!cur);
    }
}

/// Provision hook — called once at the app root inside `use_init_app_state`.
pub fn init_calendar_sidebar_state() -> CalendarSidebarState {
    let open = use_signal(|| false);
    use_context_provider(move || CalendarSidebarState { open })
}

/// Consumer hook — read the shared calendar sidebar state.
pub fn use_calendar_sidebar_state() -> CalendarSidebarState {
    use_context::<CalendarSidebarState>()
}

use dioxus::prelude::*;

use crate::components::ui::{
    Input, InputGroup, InputGroupAddon, InputGroupAddonPosition, InputGroupControl,
};
use crate::icons::{CloseIcon, MaximizeIcon, MinimizeIcon, SearchIcon};

#[component]
pub fn TitleBar() -> Element {
    let desktop = dioxus::desktop::window();

    let minimize_window = desktop.clone();
    let maximize_window = desktop.clone();
    let close_window = desktop.clone();
    let drag_window = desktop.clone();

    rsx! {
        div {
            class: "
                w-screen
                h-12
                flex
                items-center
                p-2
                select-none
                bg-background
            ",

            // ============================================================
            // Logo
            // ============================================================
            div {
                class: "
                    flex
                    items-center
                    gap-4
                    px-2
                    cursor-default
                ",

                img {
                    src: asset!("/assets/logo.png"),
                    class: "size-4 pointer-events-none",
                }
            }

            // ============================================================
            // Draggable Area
            // ============================================================
            div {
                class: "
                    flex-1
                    h-full
                    flex
                    items-center
                    justify-center
                    cursor-default
                ",

                onmousedown: move |_| {
                    drag_window.drag();
                },

                // Search itself does NOT trigger dragging
                div {
                    class: "titlebar-search w-full max-w-md",

                    InputGroup {
                        InputGroupAddon {
                            position: InputGroupAddonPosition::InlineStart,

                            SearchIcon {
                                class: "size-4",
                            },
                        }

                        InputGroupControl {
                            Input {
                                class: "
                                    h-6
                                    rounded-none
                                    border-0
                                    bg-transparent
                                    px-2
                                    shadow-none
                                    focus-visible:ring-0
                                "
                                .to_string(),

                                placeholder: "Search...".to_string(),
                            }
                        }
                    }
                }
            }

            // ============================================================
            // Window Controls
            // ============================================================
            div {
                class: "flex items-center",

                // Minimize
                button {
                    class: "p-2",

                    onclick: move |_| {
                        minimize_window.window.set_minimized(true);
                    },

                    MinimizeIcon {
                        class: "size-3".to_string(),
                    }
                }

                // Maximize
                button {
                    class: "p-2",
                    onclick: move |_| {
                        maximize_window.toggle_maximized();
                    },

                    MaximizeIcon {
                        class: "size-3".to_string(),
                    }
                }

                // Close
                button {
                    class: "p-2",

                    onclick: move |_| {
                        close_window.close();
                    },

                    CloseIcon {
                        size: "24".to_string(),
                    }
                }
            }
        }
    }
}

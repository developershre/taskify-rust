use dioxus::prelude::*;

use crate::components::title_search::TitleSearch;
use crate::icons::{CloseIcon, MaximizeIcon, MinimizeIcon};

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
                flex
                items-center
                p-2
                select-none
                bg-background
            ",
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
                    class: "size-6 pointer-events-none",
                }
            }
            div {
                class: "",
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

                TitleSearch{},
            }

            // ============================================================
            // Window Controls
            // ============================================================
            div {
                class: "flex items-center gap-2",

                // Minimize
                button {
                    class: "p-2 hover:bg-gray-100 rounded-sm",

                    onclick: move |_| {
                        minimize_window.window.set_minimized(true);
                    },

                    MinimizeIcon {
                        class: "size-3".to_string(),
                    }
                }

                // Maximize
                button {
                    class: "p-2 hover:bg-gray-100 rounded-sm",
                    onclick: move |_| {
                        maximize_window.toggle_maximized();
                    },

                    MaximizeIcon {
                        class: "size-3".to_string(),
                    }
                }

                // Close
                button {
                    class: "p-2 hover:bg-red-500 hover:text-white rounded-sm",

                    onclick: move |_| {
                        close_window.close();
                    },

                    CloseIcon {
                        class: "size-3".to_string(),
                    }
                }
            }
        }
    }
}

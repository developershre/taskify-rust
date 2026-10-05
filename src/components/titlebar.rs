use dioxus::prelude::*;

use crate::components::ui::{
    DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuSeparator,
    DropdownMenuShortcut, DropdownMenuTrigger,
};
use crate::components::TitleSearch;
use crate::icons::{CloseIcon, MaximizeIcon, MinimizeIcon};

#[component]
pub fn TitleBar() -> Element {
    let desktop = dioxus::desktop::window();

    let minimize_window = desktop.clone();
    let maximize_window = desktop.clone();
    let close_window = desktop.clone();
    let drag_window = desktop.clone();
    let exit_window = desktop.clone();
    let max_window = desktop.clone();

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
            // ============================================================
            // Logo
            // ============================================================
            div {
                class: "
                    flex
                    items-center
                    gap-2
                    px-2
                    cursor-default
                    shrink-0
                ",

                img {
                    src: asset!("/assets/logo.png"),
                    class: "size-6 pointer-events-none",
                }
            }

            // ============================================================
            // Titlebar Menu (File, Edit, View, Help)
            // ============================================================
            div {
                class: "flex items-center gap-0.5 shrink-0",

                // File
                DropdownMenu {
                    DropdownMenuTrigger {
                        class: "px-2 py-1 text-xs text-muted-foreground hover:text-foreground hover:bg-muted/80 rounded transition-colors cursor-pointer",
                        "File"
                    }
                    DropdownMenuContent {
                        class: "w-48",
                        DropdownMenuItem {
                            onclick: move |_| println!("File > New Task"),
                            span { "New Task" }
                            DropdownMenuShortcut { "Ctrl+N" }
                        }
                        DropdownMenuItem {
                            onclick: move |_| println!("File > Open"),
                            span { "Open..." }
                            DropdownMenuShortcut { "Ctrl+O" }
                        }
                        DropdownMenuItem {
                            onclick: move |_| println!("File > Save"),
                            span { "Save" }
                            DropdownMenuShortcut { "Ctrl+S" }
                        }
                        DropdownMenuItem {
                            onclick: move |_| println!("File > Save As"),
                            span { "Save As..." }
                            DropdownMenuShortcut { "Ctrl+A" }
                        }
                        DropdownMenuSeparator {}
                        DropdownMenuItem {
                            variant: "destructive".to_string(),
                            onclick: move |_| exit_window.close(),
                            span { "Exit" }
                            DropdownMenuShortcut { "Ctrl+Q" }
                        }
                    }
                }

                // Edit
                DropdownMenu {
                    DropdownMenuTrigger {
                        class: "px-2 py-1 text-xs text-muted-foreground hover:text-foreground hover:bg-muted/80 rounded transition-colors cursor-pointer",
                        "Edit"
                    }
                    DropdownMenuContent {
                        class: "w-44",
                        DropdownMenuItem {
                            onclick: move |_| println!("Edit > Undo"),
                            span { "Undo" }
                            DropdownMenuShortcut { "Ctrl+Z" }
                        }
                        DropdownMenuItem {
                            onclick: move |_| println!("Edit > Redo"),
                            span { "Redo" }
                            DropdownMenuShortcut { "Ctrl+Y" }
                        }
                        DropdownMenuSeparator {}
                        DropdownMenuItem {
                            onclick: move |_| println!("Edit > Cut"),
                            span { "Cut" }
                            DropdownMenuShortcut { "Ctrl+X" }
                        }
                        DropdownMenuItem {
                            onclick: move |_| println!("Edit > Copy"),
                            span { "Copy" }
                            DropdownMenuShortcut { "Ctrl+C" }
                        }
                        DropdownMenuItem {
                            onclick: move |_| println!("Edit > Paste"),
                            span { "Paste" }
                            DropdownMenuShortcut { "Ctrl+V" }
                        }
                    }
                }

                // View
                DropdownMenu {
                    DropdownMenuTrigger {
                        class: "px-2 py-1 text-xs text-muted-foreground hover:text-foreground hover:bg-muted/80 rounded transition-colors cursor-pointer",
                        "View"
                    }
                    DropdownMenuContent {
                        class: "w-48",
                        DropdownMenuItem {
                            onclick: move |_| println!("View > Zoom In"),
                            span { "Zoom In" }
                            DropdownMenuShortcut { "Ctrl++" }
                        }
                        DropdownMenuItem {
                            onclick: move |_| println!("View > Zoom Out"),
                            span { "Zoom Out" }
                            DropdownMenuShortcut { "Ctrl+-" }
                        }
                        DropdownMenuSeparator {}
                        DropdownMenuItem {
                            onclick: move |_| max_window.toggle_maximized(),
                            span { "Toggle Maximize" }
                            DropdownMenuShortcut { "F11" }
                        }
                    }
                }

                // Help
                DropdownMenu {
                    DropdownMenuTrigger {
                        class: "px-2 py-1 text-xs text-muted-foreground hover:text-foreground hover:bg-muted/80 rounded transition-colors cursor-pointer",
                        "Help"
                    }
                    DropdownMenuContent {
                        class: "w-44",
                        DropdownMenuItem {
                            onclick: move |_| println!("Help > Documentation"),
                            span { "Documentation" }
                        }
                        DropdownMenuSeparator {}
                        DropdownMenuItem {
                            onclick: move |_| println!("Help > About Taskify"),
                            span { "About Taskify" }
                        }
                    }
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

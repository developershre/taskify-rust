use dioxus::prelude::*;

use crate::components::ui::{
    DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuSeparator,
    DropdownMenuShortcut, DropdownMenuTrigger,
};
use crate::components::{SearchContext, TitleSearch};
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

    let mut search_open = use_signal(|| false);
    use_context_provider(|| SearchContext { open: search_open });

    // Register all application keyboard shortcuts
    use_effect(move || {
        let mut eval = document::eval(
            r#"
            window.addEventListener('keydown', (e) => {
                const isInput = e.target.tagName === 'INPUT' || e.target.tagName === 'TEXTAREA' || e.target.isContentEditable;
                const isCtrlOrMeta = e.ctrlKey || e.metaKey;

                // Ctrl+K / Cmd+K: Always toggle search palette
                if (isCtrlOrMeta && e.key.toLowerCase() === 'k') {
                    e.preventDefault();
                    dioxus.send('search.toggle');
                    return;
                }

                // Escape: Always close palettes/menus
                if (e.key === 'Escape') {
                    dioxus.send('escape');
                    return;
                }

                // F11: Toggle maximize
                if (e.key === 'F11') {
                    e.preventDefault();
                    dioxus.send('window.toggle_maximize');
                    return;
                }

                // Ctrl+Q: Exit app
                if (isCtrlOrMeta && e.key.toLowerCase() === 'q') {
                    e.preventDefault();
                    dioxus.send('window.close');
                    return;
                }

                // Other shortcuts when not typing in an input
                if (isCtrlOrMeta && !isInput) {
                    const key = e.key.toLowerCase();
                    if (key === 'n') {
                        e.preventDefault();
                        dioxus.send('file.new');
                    } else if (key === 'o') {
                        e.preventDefault();
                        dioxus.send('file.open');
                    } else if (key === 's') {
                        e.preventDefault();
                        dioxus.send('file.save');
                    } else if (key === 'a') {
                        e.preventDefault();
                        dioxus.send('file.save_as');
                    } else if (key === 'z') {
                        e.preventDefault();
                        dioxus.send('edit.undo');
                    } else if (key === 'y') {
                        e.preventDefault();
                        dioxus.send('edit.redo');
                    } else if (e.key === '=' || e.key === '+') {
                        e.preventDefault();
                        dioxus.send('view.zoom_in');
                    } else if (e.key === '-') {
                        e.preventDefault();
                        dioxus.send('view.zoom_out');
                    }
                }
            });
            "#,
        );

        spawn(async move {
            let win = dioxus::desktop::window();
            while let Ok(msg) = eval.recv::<String>().await {
                match msg.as_str() {
                    "search.toggle" => {
                        let current = *search_open.read();
                        search_open.set(!current);
                    }
                    "escape" => {
                        search_open.set(false);
                    }
                    "window.close" => {
                        win.close();
                    }
                    "window.toggle_maximize" => {
                        win.toggle_maximized();
                    }
                    "file.new" => {
                        println!("Action: New Task (Ctrl+N)");
                    }
                    "file.open" => {
                        println!("Action: Open File (Ctrl+O)");
                    }
                    "file.save" => {
                        println!("Action: Save (Ctrl+S)");
                    }
                    "file.save_as" => {
                        println!("Action: Save As (Ctrl+A)");
                    }
                    "edit.undo" => {
                        println!("Action: Undo (Ctrl+Z)");
                    }
                    "edit.redo" => {
                        println!("Action: Redo (Ctrl+Y)");
                    }
                    "view.zoom_in" => {
                        println!("Action: Zoom In (Ctrl++)");
                    }
                    "view.zoom_out" => {
                        println!("Action: Zoom Out (Ctrl+-)");
                    }
                    _ => {}
                }
            }
        });
    });

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

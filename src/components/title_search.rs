use dioxus::prelude::*;

use crate::components::ui::{
    Command, CommandDialog, CommandEmpty, CommandGroup, CommandInput, CommandItem, CommandList,
    CommandShortcut,
};
use crate::icons::SearchIcon;

#[component]
pub fn TitleSearch() -> Element {
    let mut open = use_signal(|| false);
    let mut search = use_signal(String::new);

    let query = search().to_lowercase();
    let q = query.trim();

    let nav_items = [
        ("Dashboard", "G D"),
        ("Tasks", "G T"),
        ("Projects", "G P"),
        ("Settings", "G S"),
    ];

    let action_items = [
        ("New Task", "Ctrl+N"),
        ("Open File", "Ctrl+O"),
        ("Save", "Ctrl+S"),
    ];

    let filtered_nav: Vec<_> = nav_items
        .into_iter()
        .filter(|(name, _)| q.is_empty() || name.to_lowercase().contains(q))
        .collect();

    let filtered_actions: Vec<_> = action_items
        .into_iter()
        .filter(|(name, _)| q.is_empty() || name.to_lowercase().contains(q))
        .collect();

    let has_results = !filtered_nav.is_empty() || !filtered_actions.is_empty();

    rsx! {
        div {
            class: "relative",

            // Trigger button inside the TitleBar
            button {
                r#type: "button",
                class: "flex items-center justify-between gap-3 px-3 py-1 text-xs \
                        text-muted-foreground bg-muted/40 hover:bg-muted/70 \
                        rounded-md border border-border/40 transition-colors \
                        w-64 cursor-pointer select-none",
                onclick: move |e| {
                    e.stop_propagation();
                    open.set(true);
                },
                onmousedown: move |e| {
                    e.stop_propagation();
                },

                div {
                    class: "flex items-center gap-2",
                    SearchIcon {
                        class: "size-3.5 opacity-60",
                    }
                    span { "Search..." }
                }

                kbd {
                    class: "pointer-events-none inline-flex h-4 select-none \
                            items-center gap-1 rounded border bg-muted \
                            px-1.5 font-mono text-[10px] font-medium \
                            text-muted-foreground",
                    "Ctrl K"
                }
            }

            // Command palette modal dialog
            CommandDialog {
                open: open,
                title: "Search".to_string(),
                description: "Search for commands, tasks, and navigation...".to_string(),

                Command {
                    CommandInput {
                        placeholder: "Type a command or search...".to_string(),
                        value: search,
                    }

                    CommandList {
                        if !has_results {
                            CommandEmpty {
                                "No results found."
                            }
                        }

                        if !filtered_nav.is_empty() {
                            CommandGroup {
                                heading: "Navigation".to_string(),
                                for (name, shortcut) in filtered_nav {
                                    CommandItem {
                                        key: "{name}",
                                        onclick: move |_| {
                                            open.set(false);
                                            search.write().clear();
                                        },
                                        span { "{name}" }
                                        CommandShortcut { "{shortcut}" }
                                    }
                                }
                            }
                        }

                        if !filtered_actions.is_empty() {
                            CommandGroup {
                                heading: "Actions".to_string(),
                                for (name, shortcut) in filtered_actions {
                                    CommandItem {
                                        key: "{name}",
                                        onclick: move |_| {
                                            open.set(false);
                                            search.write().clear();
                                        },
                                        span { "{name}" }
                                        CommandShortcut { "{shortcut}" }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

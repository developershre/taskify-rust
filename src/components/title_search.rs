use dioxus::prelude::*;

use crate::components::NewTaskForm;
use crate::components::ui::{
    Command, CommandDialog, CommandEmpty, CommandGroup, CommandInput, CommandItem, CommandList,
    CommandShortcut,
};
use crate::icons::SearchIcon;
use crate::state::{use_app_state, AppState};

#[derive(Clone, Copy, PartialEq)]
pub struct SearchContext {
    pub open: Signal<bool>,
}

fn run_action(name: &str, mut app_state: AppState, mut new_task: NewTaskForm) {
    match name {
        "New Task" => new_task.open.set(true),
        "New Note" => {
            app_state.add_note(
                "Untitled note".to_string(),
                "Write something worth keeping.".to_string(),
            );
            let notes_open = *app_state.calendar_sidebar_open.read();
            if !notes_open {
                app_state.toggle_calendar_sidebar();
            }
        }
        "Toggle Theme" => app_state.toggle_theme(),
        _ => {}
    }
}

#[component]
pub fn TitleSearch() -> Element {
    let mut open = match try_use_context::<SearchContext>() {
        Some(ctx) => ctx.open,
        None => use_signal(|| false),
    };
    let mut search = use_signal(String::new);
    let new_task = use_context::<NewTaskForm>();
    let app_state = use_app_state();
    let router = router();

    let query = search().to_lowercase();
    let q = query.trim();

    let nav_items = [
        ("Dashboard", "/"),
        ("All Tasks", "/tasks/all"),
        ("Urgent Tasks", "/tasks/urgent"),
        ("Projects", "/projects"),
        ("Calendar", "/calendar"),
        ("Analytics", "/analytics"),
        ("Settings", "/settings"),
    ];

    let action_items = [("New Task", ""), ("New Note", ""), ("Toggle Theme", "")];

    let filtered_nav: Vec<_> = nav_items
        .into_iter()
        .filter(|(name, _)| q.is_empty() || name.to_lowercase().contains(q))
        .collect();

    let filtered_actions: Vec<_> = action_items
        .into_iter()
        .filter(|(name, _)| q.is_empty() || name.to_lowercase().contains(q))
        .collect();

    let has_results = !filtered_nav.is_empty() || !filtered_actions.is_empty();

    let first_match = filtered_nav
        .first()
        .copied()
        .or_else(|| filtered_actions.first().copied());

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
                        onkeydown: move |e: KeyboardEvent| {
                            if e.key() == Key::Enter {
                                if let Some((name, path)) = first_match {
                                    if !path.is_empty() {
                                        let _ = router.push(path);
                                    } else {
                                        run_action(name, app_state, new_task);
                                    }
                                    open.set(false);
                                    search.write().clear();
                                }
                            } else if e.key() == Key::Escape {
                                open.set(false);
                            }
                        },
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
                                for (name, path) in filtered_nav {
                                    CommandItem {
                                        key: "{name}",
                                        onclick: move |_| {
                                            let _ = router.push(path);
                                            open.set(false);
                                            search.write().clear();
                                        },
                                        span { "{name}" }
                                        CommandShortcut { "Enter" }
                                    }
                                }
                            }
                        }

                        if !filtered_actions.is_empty() {
                            CommandGroup {
                                heading: "Actions".to_string(),
                                for (name, _) in filtered_actions {
                                    CommandItem {
                                        key: "{name}",
                                        onclick: move |_| {
                                            run_action(name, app_state, new_task);
                                            open.set(false);
                                            search.write().clear();
                                        },
                                        span { "{name}" }
                                        CommandShortcut { "Enter" }
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

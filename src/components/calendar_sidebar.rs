use dioxus::prelude::*;
use crate::components::ui::Calendar;
use crate::state::use_app_state;

#[derive(Props, Clone, PartialEq)]
pub struct CalendarSidebarProps {
    #[props(default)]
    pub class: String,
}

#[component]
pub fn CalendarSidebar(props: CalendarSidebarProps) -> Element {
    let mut app_state = use_app_state();
    let is_open = *app_state.calendar_sidebar_open.read();

    // New note inline modal/form state
    let mut show_new_note = use_signal(|| false);
    let mut new_note_title = use_signal(String::new);
    let mut new_note_desc = use_signal(String::new);

    // Active action menu id
    let mut active_action_id = use_signal(|| Option::<String>::None);

    if !is_open {
        return rsx! {};
    }

    let notes = app_state.notes.read();

    rsx! {
        aside {
            class: format!(
                "w-72 sm:w-80 shrink-0 rounded-2xl border border-border/40 bg-card p-4 flex flex-col h-full overflow-y-auto select-none shadow-xs gap-5 transition-all duration-300 {}",
                props.class,
            ),

            // ============================================================
            // Calendar Widget (using UI Calendar component)
            // ============================================================
            Calendar {
                selected: app_state.selected_date,
                onselect: move |date: (u32, u32, u32)| {
                    app_state.selected_date.set(Some(date));
                },
            }

            // ============================================================
            // Notes Section
            // ============================================================
            div { class: "flex flex-col gap-3 mt-1",

                // Notes Header
                div { class: "flex items-center justify-between px-1",
                    div { class: "flex items-center gap-2",
                        // Notepad / Edit icon
                        svg {
                            class: "size-4 text-foreground",
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            path { d: "M17 3a2.85 2.83 0 1 1 4 4L7.5 20.5 2 22l1.5-5.5Z" }
                            path { d: "m15 5 4 4" }
                        }
                        span { class: "text-sm font-semibold text-foreground tracking-tight",
                            "Notes"
                        }
                    }

                    // Add Note Button (+)
                    button {
                        class: "size-6 rounded flex items-center justify-center text-muted-foreground hover:text-foreground hover:bg-sidebar-accent transition-colors cursor-pointer",
                        title: "Add Note",
                        onclick: move |_| {
                            let cur = *show_new_note.read();
                            show_new_note.set(!cur);
                        },
                        svg {
                            class: "size-4",
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            path { d: "M12 5v14M5 12h14" }
                        }
                    }
                }

                // Inline New Note Creator Form
                if *show_new_note.read() {
                    div { class: "p-3 rounded-xl border border-border bg-card shadow-sm flex flex-col gap-2.5",
                        input {
                            class: "w-full text-xs font-semibold bg-transparent border-b border-border/60 pb-1 outline-none text-foreground placeholder:text-muted-foreground/50",
                            placeholder: "Note title...",
                            value: "{new_note_title}",
                            oninput: move |e| new_note_title.set(e.value()),
                        }
                        textarea {
                            class: "w-full text-[11px] bg-transparent resize-none h-14 outline-none text-foreground placeholder:text-muted-foreground/50",
                            placeholder: "A simple item with title and description...",
                            value: "{new_note_desc}",
                            oninput: move |e| new_note_desc.set(e.value()),
                        }
                        div { class: "flex items-center justify-end gap-2 pt-1 border-t border-border/40",
                            button {
                                class: "px-2 py-1 text-[11px] rounded text-muted-foreground hover:text-foreground cursor-pointer",
                                onclick: move |_| show_new_note.set(false),
                                "Cancel"
                            }
                            button {
                                class: "px-2.5 py-1 text-[11px] rounded bg-primary text-primary-foreground font-medium hover:opacity-90 transition-opacity cursor-pointer",
                                onclick: move |_| {
                                    let t = new_note_title.read().clone();
                                    let d = new_note_desc.read().clone();
                                    if !t.trim().is_empty() {
                                        app_state
                                            .add_note(
                                                t,
                                                if d.trim().is_empty() {
                                                    "A simple item with title and description.".to_string()
                                                } else {
                                                    d
                                                },
                                            );
                                        new_note_title.set(String::new());
                                        new_note_desc.set(String::new());
                                        show_new_note.set(false);
                                    }
                                },
                                "Add Note"
                            }
                        }
                    }
                }

                // Notes Cards List matching the screenshot
                div { class: "flex flex-col gap-2.5",

                    if notes.is_empty() {
                        div { class: "p-4 text-center text-xs text-muted-foreground border border-dashed border-border rounded-xl",
                            "No notes created yet."
                        }
                    }

                    for note in notes.iter() {
                        {
                            let is_menu_open = active_action_id.read().as_deref() == Some(note.id.as_str());

                            rsx! {
                                div {
                                    key: "{note.id}",
                                    class: "relative rounded-xl p-3.5 bg-card/60 hover:bg-card border border-border/50 hover:border-border transition-colors flex items-center justify-between gap-3 shadow-2xs group",

                                    // Action Dropdown Menu
                                    div { class: "flex flex-col min-w-0 flex-1",
                                        span { class: "font-semibold text-xs text-foreground truncate", "{note.title}" }
                                        p { class: "text-[11px] text-muted-foreground mt-0.5 leading-relaxed truncate",
                                            "{note.description}"
                                        }
                                    }
        
                                    button {
                                        r#type: "button",
                                        class: "px-2.5 py-1 text-xs rounded-md bg-secondary hover:bg-secondary/80 text-secondary-foreground font-medium shrink-0 cursor-pointer border border-border/30 transition-colors shadow-2xs",
                                        onclick: {
                                            let nid = note.id.clone();
                                            move |_| {
                                                if active_action_id.read().as_deref() == Some(nid.as_str()) {
                                                    active_action_id.set(None);
                                                } else {
                                                    active_action_id.set(Some(nid.clone()));
                                                }
                                            }
                                        },
                                        "Actions"
                                    }
        
                                    if is_menu_open {
                                        div { class: "absolute right-3 top-10 z-30 w-28 rounded-lg border border-border bg-popover text-popover-foreground shadow-md p-1 flex flex-col gap-0.5 animate-in fade-in-50 zoom-in-95",
                                            button {
                                                class: "w-full text-left px-2 py-1 text-xs rounded hover:bg-muted text-foreground transition-colors cursor-pointer",
                                                onclick: move |_| {
                                                    active_action_id.set(None);
                                                },
                                                "View"
                                            }
                                            button {
                                                class: "w-full text-left px-2 py-1 text-xs rounded hover:bg-destructive/10 text-destructive transition-colors cursor-pointer",
                                                onclick: {
                                                    let nid = note.id.clone();
                                                    move |_| {
                                                        app_state.delete_note(&nid);
                                                        active_action_id.set(None);
                                                    }
                                                },
                                                "Delete"
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
    }
}

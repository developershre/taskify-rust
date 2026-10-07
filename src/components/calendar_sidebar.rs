use crate::components::ui::{Calendar, Item, ItemActions, ItemContent, ItemDescription, ItemTitle};
use crate::state::{use_app_state, OverlayState};
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct CalendarSidebarProps {
    #[props(default)]
    pub class: String,
}

use crate::icons::EditIcon;

#[component]
pub fn CalendarSidebar(props: CalendarSidebarProps) -> Element {
    let mut app_state = use_app_state();
    let is_open = *app_state.calendar_sidebar_open.read();
    let overlay = use_context::<OverlayState>();

    // Self-dim while any dialog is open: the sidebar subscribes to the shared
    // OverlayState and recedes (opacity + inertness) instead of relying on
    // z-ordering against the dialog scrim/content.
    let dialog_class = if overlay.is_dialog_open() {
        " opacity-50 pointer-events-none"
    } else {
        ""
    };

    // Active action menu id
    let mut active_action_id = use_signal(|| Option::<String>::None);

    // The sidebar itself stays visible while a dialog is open; only close a
    // dangling note popup so it never floats next to the dialog.
    use_effect(move || {
        if overlay.is_dialog_open() {
            active_action_id.set(None);
        }
    });

    let action_menu_anchor = active_action_id;
    use_effect(move || {
        if let Some(id) = action_menu_anchor.read().clone() {
            super::ui::popup::position_popup(
                &format!("note-{id}"),
                "[data-popup-anchor]",
                "[data-popup-trigger]",
                "bottom",
                "end",
                6,
                false,
            );
        }
    });

    if !is_open {
        return rsx! {};
    }

    let notes = app_state.notes.read();

    rsx! {
        aside {
            class: format!(
                "w-60 sm:w-72 shrink-0 rounded-2xl border border-border/40 bg-card p-4 flex flex-col h-full overflow-y-auto select-none shadow-xs gap-5 transition-all duration-300 isolate max-xl:fixed max-xl:inset-y-0 max-xl:right-0 max-xl:z-40 max-xl:rounded-none max-xl:w-72 max-xl:shadow-2xl {}{}",
                props.class,
                dialog_class,
            ),

            // ============================================================
            // Calendar Widget (using UI Calendar component)
            // ============================================================
            Calendar {
                selected: app_state.selected_date,
                onselect: move |date: (u32, u32, u32)| {
                    app_state.selected_date.set(Some(date));
                },
                events: app_state.calendar_events.read().clone(),
            }

            // ============================================================
            // Notes Section
            // ============================================================
            div { class: "flex flex-col gap-3 mt-1",

                // Notes Header
                div { class: "flex items-center justify-between px-1",
                    div { class: "flex items-center gap-2",
                        // Notepad / Edit icon
                        EditIcon { class: "size-4" }
                        span { class: "text-sm font-semibold text-foreground tracking-tight",
                            "Notes"
                        }
                    }
                    // Add Note Button (+)
                    button {
                        class: "size-6 rounded flex items-center justify-center text-muted-foreground hover:text-foreground hover:bg-sidebar-accent transition-colors cursor-pointer",
                        title: "Add Note",
                        onclick: move |_| {
                            app_state.new_note_open.set(true);
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
                                    "data-popup-anchor": "true",

                                    Item {
                                        ItemContent {
                                            ItemTitle { "{note.title}" }
                                            ItemDescription { "{note.description}" }
                                        }
                                        ItemActions {
                                            button {
                                                r#type: "button",
                                                "data-popup-trigger": "true",
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
                                        }
                                    }

                                    if is_menu_open {
                                        div { "data-popup": format!("note-{}", note.id),
                                            class: "fixed left-0 top-0 z-50 invisible",
                                            div { class: "w-28 rounded-lg border border-border bg-popover text-popover-foreground p-1 flex flex-col gap-0.5 animate-in fade-in-50 zoom-in-95",
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
}

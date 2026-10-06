use dioxus::prelude::*;

use crate::components::ui::Calendar as CalendarWidget;
use crate::state::{use_app_state, TaskItem};
use crate::views::{EmptyState, PageHeader, TaskRow};

#[component]
pub fn Calendar() -> Element {
    let app_state = use_app_state();
    let selected = app_state.selected_date;

    let (due_today, upcoming): (Vec<TaskItem>, Vec<TaskItem>) = {
        let sel = *selected.read();
        let mut today = Vec::new();
        let mut later = Vec::new();
        for task in app_state.tasks.read().iter() {
            match task.due_date {
                Some(date) if Some(date) == sel => today.push(task.clone()),
                Some(date) if sel.is_some_and(|s| date > s) => later.push(task.clone()),
                _ => {}
            }
        }
        later.sort_by_key(|t| t.due_date);
        (today, later)
    };

    let selected_label = match *selected.read() {
        Some((y, m, d)) => format!("{:04}-{:02}-{:02}", y, m, d),
        None => "No date selected".to_string(),
    };

    rsx! {
        div { class: "flex flex-col",
            PageHeader {
                title: "Calendar",
                description: "Plan your schedule and review tasks due on each day.",
            }

            div { class: "flex flex-col gap-4 lg:flex-row",
                // Calendar widget
                div { class: "rounded-xl border border-border/40 bg-card p-4 shadow-xs lg:w-96 lg:shrink-0",
                    CalendarWidget { selected: Some(selected) }
                }

                // Agenda
                div { class: "flex min-w-0 flex-1 flex-col gap-4",
                    div { class: "flex flex-col gap-3 rounded-xl border border-border/40 bg-card p-4 shadow-xs",
                        div { class: "flex items-center justify-between",
                            h2 { class: "text-sm font-semibold tracking-tight text-foreground",
                                "Due on this day"
                            }
                            span { class: "text-xs text-muted-foreground", "{selected_label}" }
                        }

                        if due_today.is_empty() {
                            p { class: "py-4 text-center text-xs text-muted-foreground",
                                "No tasks due on this day."
                            }
                        } else {
                            div { class: "flex flex-col gap-2",
                                for task in due_today {
                                    TaskRow { key: "{task.id}", task: task }
                                }
                            }
                        }
                    }

                    div { class: "flex flex-col gap-3 rounded-xl border border-border/40 bg-card p-4 shadow-xs",
                        h2 { class: "text-sm font-semibold tracking-tight text-foreground",
                            "Upcoming deadlines"
                        }

                        if upcoming.is_empty() {
                            EmptyState {
                                title: "Nothing scheduled",
                                description: "Tasks with a later due date will show up here.",
                                icon: rsx! {
                                    svg {
                                        class: "size-5",
                                        view_box: "0 0 24 24",
                                        fill: "none",
                                        stroke: "currentColor",
                                        stroke_width: "2",
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        rect {
                                            width: "18",
                                            height: "18",
                                            x: "3",
                                            y: "4",
                                            rx: "2",
                                        }
                                        path { d: "M16 2v4M8 2v4M3 10h18" }
                                    }
                                },
                            }
                        } else {
                            div { class: "flex flex-col gap-2",
                                for task in upcoming {
                                    TaskRow { key: "{task.id}", task: task }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

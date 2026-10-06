use dioxus::prelude::*;

use crate::state::{use_app_state, TaskItem};
use crate::views::{EmptyState, PageHeader, StatCard, TaskRow};

#[component]
pub fn Tasks() -> Element {
    let app_state = use_app_state();
    let router = router();

    let total = app_state.total_tasks_count();
    let pending = app_state.pending_tasks_count();
    let completed = app_state.completed_tasks_count();

    let completion_pct = (completed * 100).checked_div(total).unwrap_or(0);

    let upcoming: Vec<TaskItem> = app_state
        .tasks
        .read()
        .iter()
        .filter(|t| !t.completed)
        .take(5)
        .cloned()
        .collect();

    rsx! {
        div { class: "flex flex-col",
            PageHeader {
                title: "Tasks",
                description: "Track, prioritize and ship your work.",
                actions: rsx! {
                    button {
                        r#type: "button",
                        class: "inline-flex h-8 items-center gap-1.5 rounded-md bg-primary px-3 text-xs font-medium text-primary-foreground shadow-xs hover:bg-primary/90 transition-colors cursor-pointer",
                        onclick: move |_| {
                            let _ = router.push("/tasks/all");
                        },
                        svg {
                            class: "size-3.5",
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            path { d: "M5 12h14M12 5v14" }
                        }
                        "View all"
                    }
                },
            }

            div { class: "grid grid-cols-1 gap-3 pb-6 sm:grid-cols-3",
                StatCard {
                    label: "Total",
                    value: "{total}",
                    hint: "tasks in this workspace",
                }
                StatCard {
                    label: "Pending",
                    value: "{pending}",
                    hint: "still in progress",
                }
                StatCard {
                    label: "Completed",
                    value: "{completed}",
                    hint: "{completion_pct}% completion rate",
                }
            }

            h2 { class: "pb-2 text-sm font-semibold tracking-tight text-foreground",
                "Upcoming"
            }

            if upcoming.is_empty() {
                EmptyState {
                    title: "No pending tasks",
                    description: "You are all caught up. New tasks will show up here.",
                    icon: rsx! {
                        svg {
                            class: "size-5",
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            path { d: "M20 6 9 17l-5-5" }
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

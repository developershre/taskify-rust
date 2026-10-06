use dioxus::prelude::*;

use crate::state::{use_app_state, TaskItem};
use crate::views::{EmptyState, PageHeader, TaskRow};

#[component]
pub fn TasksArchived() -> Element {
    let app_state = use_app_state();

    let archived: Vec<TaskItem> = app_state
        .tasks
        .read()
        .iter()
        .filter(|t| t.completed)
        .cloned()
        .collect();
    let count = archived.len();

    rsx! {
        div { class: "flex flex-col",
            PageHeader {
                title: "Archived",
                description: if count == 1 {
                    "1 completed task".to_string()
                } else {
                    format!("{count} completed tasks")
                },
            }

            if archived.is_empty() {
                EmptyState {
                    title: "Nothing archived yet",
                    description: "Completed tasks are archived here automatically.",
                    icon: rsx! {
                        svg {
                            class: "size-5",
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            path { d: "M21 8v13H3V8M1 3h22v5H1zM10 12h4" }
                        }
                    },
                }
            } else {
                div { class: "flex flex-col gap-2",
                    for task in archived {
                        TaskRow { key: "{task.id}", task: task }
                    }
                }
            }
        }
    }
}

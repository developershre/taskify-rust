use dioxus::prelude::*;

use crate::state::{use_app_state, TaskItem, TaskPriority};
use crate::views::{EmptyState, PageHeader, TaskRow};

#[component]
pub fn TasksUrgent() -> Element {
    let app_state = use_app_state();

    let urgent: Vec<TaskItem> = app_state
        .tasks
        .read()
        .iter()
        .filter(|t| !t.completed && matches!(t.priority, TaskPriority::Urgent | TaskPriority::High))
        .cloned()
        .collect();
    let count = urgent.len();

    rsx! {
        div { class: "flex flex-col",
            PageHeader {
                title: "Urgent",
                description: if count == 1 {
                    "1 high priority task".to_string()
                } else {
                    format!("{count} high priority tasks")
                },
            }

            if urgent.is_empty() {
                EmptyState {
                    title: "No urgent tasks",
                    description: "High and urgent priority tasks will show up here.",
                    icon: rsx! {
                        svg {
                            class: "size-5",
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            path { d: "M12 2 2 20h20L12 2z" }
                            path { d: "M12 9v4M12 17h.01" }
                        }
                    },
                }
            } else {
                div { class: "flex flex-col gap-2",
                    for task in urgent {
                        TaskRow { key: "{task.id}", task: task }
                    }
                }
            }
        }
    }
}

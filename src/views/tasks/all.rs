use dioxus::prelude::*;

use crate::components::ui::Switch;
use crate::state::use_app_state;
use crate::views::{EmptyState, PageHeader, TaskRow};

#[component]
pub fn TasksAll() -> Element {
    let app_state = use_app_state();
    let show_completed = app_state.show_completed;

    let tasks = app_state.filtered_tasks();
    let count = tasks.len();

    rsx! {
        div { class: "flex flex-col",
            PageHeader {
                title: "All Tasks",
                description: if count == 1 {
                    "1 task".to_string()
                } else {
                    format!("{count} tasks")
                },
                actions: rsx! {
                    div { class: "flex items-center gap-2 pt-2",
                        span { class: "text-xs font-medium text-muted-foreground",
                            "Show completed"
                        }
                        Switch { checked: show_completed }
                    }
                },
            }

            if tasks.is_empty() {
                EmptyState {
                    title: "No tasks here",
                    description: "Tasks you create will appear in this list.",
                    icon: rsx! {
                        svg {
                            class: "size-5",
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            path { d: "M12 5v14M5 12h14" }
                        }
                    },
                }
            } else {
                div { class: "flex flex-col gap-2",
                    for task in tasks {
                        TaskRow { key: "{task.id}", task: task }
                    }
                }
            }
        }
    }
}

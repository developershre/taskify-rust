use dioxus::prelude::*;

use crate::state::{use_app_state, TaskItem};
use crate::views::{EmptyState, PageHeader, StatCard, TaskRow};

fn project_tasks(app_state: crate::state::AppState, name: &str) -> Vec<TaskItem> {
    app_state
        .tasks
        .read()
        .iter()
        .filter(|t| t.project == name)
        .cloned()
        .collect()
}

#[component]
pub fn ProjectsPersonal() -> Element {
    let app_state = use_app_state();

    let tasks = project_tasks(app_state, "Personal");
    let total = tasks.len();
    let done = tasks.iter().filter(|t| t.completed).count();
    let open = total - done;

    rsx! {
        div { class: "flex flex-col",
            PageHeader {
                title: "Personal",
                description: "Tasks in your personal project.",
            }

            div { class: "grid grid-cols-1 gap-3 pb-6 sm:grid-cols-2",
                StatCard {
                    label: "Total",
                    value: "{total}",
                    hint: "tasks in this project",
                }
                StatCard {
                    label: "Open",
                    value: "{open}",
                    hint: "{done} completed",
                }
            }

            if tasks.is_empty() {
                EmptyState {
                    title: "No tasks in this project",
                    description: "Tasks assigned to the Personal project will appear here.",
                    icon: rsx! {
                        svg {
                            class: "size-5",
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            path { d: "M20 20a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.9a2 2 0 0 1-1.69-.9L9.6 3.9A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13a2 2 0 0 0 2 2Z" }
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

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
pub fn ProjectsGithub() -> Element {
    let app_state = use_app_state();

    let tasks = project_tasks(app_state, "Github Repos");
    let total = tasks.len();
    let done = tasks.iter().filter(|t| t.completed).count();
    let open = total - done;

    rsx! {
        div { class: "flex flex-col",
            PageHeader {
                title: "Github Repos",
                description: "Tasks linked to your GitHub repositories.",
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
                    description: "Tasks linked to GitHub repositories will appear here.",
                    icon: rsx! {
                        svg {
                            class: "size-5",
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            path { d: "M15 22v-4a4.8 4.8 0 0 0-1-3.5c3 0 6-2 6-5.5.08-1.25-.27-2.48-1-3.5.28-1.15.28-2.35 0-3.5 0 0-1 0-3 1.5-2.64-.5-5.36-.5-8 0C6 2 5 2 5 2c-.3 1.15-.3 2.35 0 3.5A5.4 5.4 0 0 0 4 9c0 3.5 3 5.5 6 5.5-.39.49-.68 1.05-.85 1.65-.17.6-.22 1.23-.15 1.85v4" }
                            path { d: "M9 18c-4.51 2-5-2-7-2" }
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

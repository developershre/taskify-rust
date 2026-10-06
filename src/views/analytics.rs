use dioxus::prelude::*;

use crate::state::{use_app_state, TaskPriority};
use crate::views::{PageHeader, StatCard};

#[component]
pub fn Analytics() -> Element {
    let app_state = use_app_state();

    let tasks = app_state.tasks.read();
    let total = tasks.len();
    let completed = tasks.iter().filter(|t| t.completed).count();
    let pending = total - completed;
    let pct = (completed * 100).checked_div(total).unwrap_or(0);

    let low = tasks
        .iter()
        .filter(|t| t.priority == TaskPriority::Low)
        .count();
    let medium = tasks
        .iter()
        .filter(|t| t.priority == TaskPriority::Medium)
        .count();
    let high = tasks
        .iter()
        .filter(|t| t.priority == TaskPriority::High)
        .count();
    let urgent = tasks
        .iter()
        .filter(|t| t.priority == TaskPriority::Urgent)
        .count();

    let priorities: Vec<(&str, usize, &str)> = vec![
        ("Low", low, "bg-muted-foreground/50"),
        ("Medium", medium, "bg-yellow-500"),
        ("High", high, "bg-orange-500"),
        ("Urgent", urgent, "bg-destructive"),
    ];

    let mut projects: std::collections::BTreeMap<String, (usize, usize)> =
        std::collections::BTreeMap::new();
    for task in tasks.iter() {
        let entry = projects.entry(task.project.clone()).or_insert((0, 0));
        entry.0 += 1;
        if task.completed {
            entry.1 += 1;
        }
    }
    let projects: Vec<(String, usize, usize)> = projects
        .into_iter()
        .map(|(name, (count, done))| (name, count, done))
        .collect();
    let projects_len = projects.len();

    rsx! {
        div { class: "flex flex-col",
            PageHeader {
                title: "Analytics",
                description: "A snapshot of your workspace activity.",
            }

            div { class: "grid grid-cols-1 gap-3 pb-6 sm:grid-cols-2 lg:grid-cols-4",
                StatCard {
                    label: "Total tasks",
                    value: "{total}",
                    hint: "across all projects",
                }
                StatCard {
                    label: "Completed",
                    value: "{completed}",
                    hint: "{pct}% of all tasks",
                }
                StatCard {
                    label: "Pending",
                    value: "{pending}",
                    hint: "awaiting completion",
                }
                StatCard {
                    label: "Projects",
                    value: "{projects_len}",
                    hint: "active projects",
                }
            }

            div { class: "flex flex-col gap-4 lg:flex-row",
                // Priority breakdown
                div { class: "flex flex-1 flex-col gap-3 rounded-xl border border-border/40 bg-card p-4 shadow-xs",
                    h2 { class: "text-sm font-semibold tracking-tight text-foreground",
                        "Tasks by priority"
                    }
                    div { class: "flex flex-col gap-3 pt-1",
                        for (label, count, color) in priorities {
                            {
                                let width = (count * 100).checked_div(total).unwrap_or(0);
                                rsx! {
                                    div { key: "{label}", class: "flex flex-col gap-1.5",
                                        div { class: "flex items-center justify-between text-xs",
                                            span { class: "font-medium text-foreground", "{label}" }
                                            span { class: "text-muted-foreground",
                                                "{count} · {width}%"
                                            }
                                        }
                                        div { class: "h-2 w-full overflow-hidden rounded-full bg-muted",
                                            div { class: "h-full rounded-full transition-all {color}",
                                                style: "width: {width}%",
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // Project breakdown
                div { class: "flex flex-1 flex-col gap-3 rounded-xl border border-border/40 bg-card p-4 shadow-xs",
                    h2 { class: "text-sm font-semibold tracking-tight text-foreground",
                        "Tasks by project"
                    }
                    div { class: "flex flex-col gap-2 pt-1",
                        for (name, count, done) in projects {
                            {
                                let width = (done * 100).checked_div(count).unwrap_or(0);
                                rsx! {
                                    div { key: "{name}", class: "flex items-center justify-between rounded-lg border border-border/40 px-3 py-2",
                                        span { class: "truncate text-xs font-medium text-foreground",
                                            "{name}"
                                        }
                                        span { class: "shrink-0 text-xs text-muted-foreground",
                                            "{done}/{count} · {width}%"
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

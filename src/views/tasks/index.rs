use dioxus::prelude::*;

use crate::components::{EmptyState, NewTaskForm, PageHeader};
use crate::icons::{AudioWaveformIcon, CalendarIcon, LayersIcon, PlusIcon};
use crate::state::{get_real_current_date, use_app_state, TaskItem, TaskPriority, TaskStatus};

use super::list::TaskRow;

// ============================================================
// Stat card
// ============================================================

#[component]
fn StatCard(
    title: String,
    value: String,
    hint: String,
    icon_class: String,
    icon: Element,
) -> Element {
    rsx! {
        div { class: "flex items-center gap-3 rounded-xl border border-border/40 bg-card p-4 shadow-xs",
            div { class: "flex size-10 shrink-0 items-center justify-center rounded-lg [&_svg]:size-5 {icon_class}",
                {icon}
            }
            div { class: "min-w-0 flex-1",
                p { class: "text-xl font-bold tracking-tight text-foreground", "{value}" }
                p { class: "truncate text-xs font-medium text-foreground", "{title}" }
                p { class: "truncate text-[10px] text-muted-foreground", "{hint}" }
            }
        }
    }
}

// ============================================================
// Tasks mini dashboard
// ============================================================

#[component]
pub fn Tasks() -> Element {
    let app_state = use_app_state();
    let mut new_task = use_context::<NewTaskForm>();
    let router = router();

    let tasks: Vec<TaskItem> = {
        let list = app_state.tasks.read();
        let proj = app_state.selected_project.read();
        let show_all =
            proj.is_empty() || proj.as_str() == "All Projects" || proj.as_str() == "All Tasks";
        list.iter()
            .filter(|t| show_all || t.project == *proj)
            .cloned()
            .collect()
    };

    let today = get_real_current_date();
    let total = tasks.len();
    let in_progress = tasks
        .iter()
        .filter(|t| t.status == TaskStatus::InProgress)
        .count();
    let completed = tasks.iter().filter(|t| t.completed).count();
    let overdue = tasks
        .iter()
        .filter(|t| !t.completed && t.due_date.is_some_and(|d| d < today))
        .count();
    let pct = (completed * 100)
        .checked_div(total)
        .map(|v| v as u32)
        .unwrap_or(0);

    let priority_rows = [
        (
            "Urgent",
            tasks
                .iter()
                .filter(|t| t.priority == TaskPriority::Urgent)
                .count(),
            "bg-red-500",
        ),
        (
            "High",
            tasks
                .iter()
                .filter(|t| t.priority == TaskPriority::High)
                .count(),
            "bg-amber-500",
        ),
        (
            "Medium",
            tasks
                .iter()
                .filter(|t| t.priority == TaskPriority::Medium)
                .count(),
            "bg-blue-500",
        ),
        (
            "Low",
            tasks
                .iter()
                .filter(|t| t.priority == TaskPriority::Low)
                .count(),
            "bg-zinc-400",
        ),
    ];

    let recent: Vec<TaskItem> = tasks.iter().take(4).cloned().collect();

    rsx! {
        div { class: "flex max-w-6xl flex-col gap-6 pb-8",

            PageHeader {
                title: "Tasks".to_string(),
                description: "A quick overview of everything you're working on.".to_string(),
                actions: rsx! {
                    button {
                        r#type: "button",
                        class: "inline-flex h-9 items-center gap-2 rounded-lg bg-primary px-3.5 text-xs font-medium text-primary-foreground shadow-xs hover:opacity-90 transition-opacity cursor-pointer",
                        onclick: move |_| new_task.open.set(true),
                        PlusIcon { class: "size-4" }
                        "New Task"
                    }
                },
            }

            // ---- Stat cards ----
            div { class: "grid grid-cols-2 gap-4 md:grid-cols-4",
                StatCard {
                    title: "Total tasks".to_string(),
                    value: total.to_string(),
                    hint: "across all projects".to_string(),
                    icon_class: "bg-blue-500/10 text-blue-500".to_string(),
                    icon: rsx! { LayersIcon {} },
                }
                StatCard {
                    title: "In progress".to_string(),
                    value: in_progress.to_string(),
                    hint: "being worked on".to_string(),
                    icon_class: "bg-amber-500/10 text-amber-500".to_string(),
                    icon: rsx! { AudioWaveformIcon { class: "size-5" } },
                }
                StatCard {
                    title: "Completed".to_string(),
                    value: completed.to_string(),
                    hint: "done & dusted".to_string(),
                    icon_class: "bg-emerald-500/10 text-emerald-500".to_string(),
                    icon: rsx! {
                        svg {
                            class: "size-5",
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            path { d: "M22 11.08V12a10 10 0 1 1-5.93-9.14" }
                            path { d: "m9 11 3 3L22 4" }
                        }
                    },
                }
                StatCard {
                    title: "Overdue".to_string(),
                    value: overdue.to_string(),
                    hint: "past their due date".to_string(),
                    icon_class: "bg-red-500/10 text-red-500".to_string(),
                    icon: rsx! { CalendarIcon { class: "size-5" } },
                }
            }

            if total == 0 {
                EmptyState {
                    title: "No tasks yet".to_string(),
                    description: "Create your first task and your dashboard will come to life."
                        .to_string(),
                    icon: rsx! {
                        LayersIcon {}
                    },
                }
            } else {
                // ---- Progress + priority breakdown ----
                div { class: "grid gap-4 md:grid-cols-2",

                    div { class: "rounded-xl border border-border/40 bg-card p-4 shadow-xs",
                        div { class: "flex items-center justify-between",
                            h3 { class: "text-sm font-semibold text-foreground", "Completion" }
                            span { class: "text-[11px] text-muted-foreground",
                                "{completed} of {total}"
                            }
                        }
                        div { class: "mt-2 flex items-end justify-between",
                            span { class: "text-2xl font-bold tracking-tight text-foreground",
                                "{pct}%"
                            }
                            span { class: "text-[11px] text-muted-foreground",
                                "tasks completed"
                            }
                        }
                        div { class: "mt-3 h-2 overflow-hidden rounded-full bg-muted",
                            div { class: "h-2 rounded-full bg-emerald-500 transition-all",
                                style: "width: {pct}%"
                            }
                        }
                    }

                    div { class: "rounded-xl border border-border/40 bg-card p-4 shadow-xs",
                        div { class: "flex items-center justify-between",
                            h3 { class: "text-sm font-semibold text-foreground",
                                "Priority breakdown"
                            }
                            span { class: "text-[11px] text-muted-foreground", "by share" }
                        }
                        div { class: "mt-3 flex flex-col gap-2.5",
                            for (label, count, color) in priority_rows {
                                {
                                    let row_pct =
                                        (count * 100).checked_div(total).unwrap_or(0);
                                    rsx! {
                                        div { key: "{label}",
                                            div { class: "flex items-center justify-between text-[11px]",
                                                span { class: "font-medium text-foreground",
                                                    "{label}"
                                                }
                                                span { class: "text-muted-foreground", "{count}" }
                                            }
                                            div { class: "mt-1 h-1.5 overflow-hidden rounded-full bg-muted",
                                                div {
                                                    class: "h-1.5 rounded-full {color}",
                                                    style: "width: {row_pct}%",
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // ---- Recent tasks ----
                div { class: "rounded-xl border border-border/40 bg-card p-4 shadow-xs",
                    div { class: "flex items-center justify-between pb-3",
                        h3 { class: "text-sm font-semibold text-foreground", "Recent tasks" }
                        button {
                            r#type: "button",
                            class: "cursor-pointer text-xs font-medium text-primary hover:underline transition-colors",
                            onclick: move |_| {
                                let _ = router.push("/tasks/all");
                            },
                            "View all →"
                        }
                    }
                    div { class: "flex flex-col gap-2",
                        for task in recent {
                            TaskRow { key: "{task.id}", task }
                        }
                    }
                }
            }
        }
    }
}

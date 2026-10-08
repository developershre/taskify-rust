use dioxus::prelude::*;

use crate::components::{EmptyState, NewTaskForm, PageHeader};
use crate::icons::ZapIcon;
use crate::state::{use_app_state, TaskItem, TaskPriority, TaskStatus};

use super::kanban::KanbanBoard;
use super::list::TaskList;
use super::table::TaskTable;

// ============================================================
// Shared primitives
// ============================================================

#[derive(Clone, Copy, PartialEq)]
pub enum TaskView {
    Board,
    List,
    Table,
}

/// Circular completion checkbox shared by board, list and table.
#[component]
pub fn TaskCheck(id: String, completed: bool) -> Element {
    let mut app_state = use_app_state();

    rsx! {
        button {
            r#type: "button",
            class: if completed {
                "flex size-4 shrink-0 cursor-pointer items-center justify-center rounded-full bg-primary text-primary-foreground transition-colors hover:opacity-90"
            } else {
                "flex size-4 shrink-0 cursor-pointer items-center justify-center rounded-full border border-border text-transparent transition-colors hover:border-primary hover:text-primary/40"
            },
            title: if completed { "Mark incomplete" } else { "Mark complete" },
            onclick: move |e| {
                e.stop_propagation();
                app_state.toggle_task(&id);
            },
            svg {
                class: "size-2.5",
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "3",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                path { d: "M20 6 9 17l-5-5" }
            }
        }
    }
}

pub fn format_due(due: (u32, u32, u32)) -> String {
    const MONTHS: [&str; 12] = [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];
    let (y, m, d) = due;
    let month = MONTHS
        .get((m as usize).saturating_sub(1))
        .copied()
        .unwrap_or("");
    format!("{month} {d}, {y}")
}

// ============================================================
// Task page filters (All / Urgent / Archived)
// ============================================================

#[derive(Clone, Copy, PartialEq)]
pub enum TaskFilter {
    All,
    Urgent,
    Archived,
}

impl TaskFilter {
    pub fn matching(self, tasks: &[TaskItem]) -> Vec<TaskItem> {
        tasks
            .iter()
            .filter(|t| match self {
                Self::All => true,
                Self::Urgent => t.priority == TaskPriority::Urgent,
                Self::Archived => t.completed || t.status == TaskStatus::Completed,
            })
            .cloned()
            .collect()
    }
}

// ============================================================
// Full board page: header + view switcher + board/list/table
// ============================================================

#[component]
pub fn TaskBoardPage(title: String, description: String, filter: TaskFilter) -> Element {
    let app_state = use_app_state();
    let mut new_task = use_context::<NewTaskForm>();
    let mut view = use_signal(|| TaskView::Board);

    let tasks = {
        let list = app_state.tasks.read();
        filter.matching(&list)
    };
    let total = tasks.len();

    rsx! {
        div { class: "flex max-w-6xl flex-1 min-h-0 flex-col gap-6",

            PageHeader {
                title: title,
                description: description,
                actions: rsx! {
                    div { class: "flex items-center gap-3",
                        button {
                            r#type: "button",
                            class: "inline-flex h-8 cursor-pointer items-center gap-1.5 rounded-md bg-primary px-3 text-xs font-medium text-primary-foreground transition-opacity hover:opacity-90",
                            onclick: move |_| new_task.open.set(true),
                            svg {
                                class: "size-3",
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "2",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                path { d: "M12 5v14" }
                                path { d: "M5 12h14" }
                            }
                            "New task"
                        }

                        span { class: "text-xs text-muted-foreground", "{total} tasks" }

                        div { class: "flex items-center gap-1 rounded-lg border border-border/40 bg-muted/40 p-1",
                            for (target, label) in [
                                (TaskView::Board, "Board"),
                                (TaskView::List, "List"),
                                (TaskView::Table, "Table"),
                            ] {
                                button {
                                    key: "{label}",
                                    r#type: "button",
                                    class: if view() == target {
                                        "cursor-pointer rounded-md bg-background px-3 py-1.5 text-xs font-medium text-foreground shadow-xs"
                                    } else {
                                        "cursor-pointer rounded-md px-3 py-1.5 text-xs font-medium text-muted-foreground transition-colors hover:text-foreground"
                                    },
                                    onclick: move |_| view.set(target),
                                    "{label}"
                                }
                            }
                        }
                    }
                },
            }

            if tasks.is_empty() {
                div { class: "flex flex-1 items-center justify-center",
                    EmptyState {
                        title: "No tasks found".to_string(),
                        description: "Nothing matches this view right now. Create a task from the title bar."
                            .to_string(),
                        icon: rsx! {
                            ZapIcon { class: "size-5 text-amber-500" }
                        },
                    }
                }
            } else {
                { match view() {
                    TaskView::Board => rsx! { KanbanBoard { tasks } },
                    TaskView::List => rsx! { TaskList { tasks } },
                    TaskView::Table => rsx! { TaskTable { tasks } },
                } }
            }
        }
    }
}

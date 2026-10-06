use dioxus::prelude::*;

use crate::components::ui::{
    Badge, Dialog, DialogContent, DialogDescription, DialogHeader, DialogTitle, Switch,
};
use crate::state::{use_app_state, TaskItem, TaskStatus};
use crate::views::{EmptyState, TaskCard, TaskDnd, TaskRow};

#[derive(Clone, Copy, PartialEq)]
enum TaskView {
    Board,
    List,
    Table,
}

#[component]
pub fn Tasks() -> Element {
    let mut app_state = use_app_state();
    let mut view = use_signal(|| TaskView::Board);

    let mut dragging = use_signal(|| Option::<String>::None);
    use_context_provider(|| TaskDnd { dragging });

    let mut manage_open = use_signal(|| false);
    let show_completed_col = use_signal(|| true);
    let show_progress_col = use_signal(|| true);
    let show_backlog_col = use_signal(|| true);

    let columns = [
        (TaskStatus::Completed, "Completed", show_completed_col),
        (TaskStatus::InProgress, "In Progress", show_progress_col),
        (TaskStatus::Backlog, "Backlog", show_backlog_col),
    ];

    let mut tasks: Vec<TaskItem> = app_state.tasks.read().iter().cloned().collect();
    tasks.sort_by_key(|t| t.completed);

    let current_view = *view.read();

    rsx! {
        div { class: "flex flex-col",
            div { class: "flex items-center justify-between gap-3 py-4",
                div { class: "inline-flex items-center rounded-lg border border-border/40 bg-muted/30 p-0.5",
                    for (mode, label) in [
                        (TaskView::Board, "Board"),
                        (TaskView::List, "List"),
                        (TaskView::Table, "Table"),
                    ] {
                        {
                            let active_class = if current_view == mode {
                                "bg-background text-foreground shadow-xs"
                            } else {
                                "text-muted-foreground hover:text-foreground"
                            };
                            rsx! {
                                button {
                                    key: "{label}",
                                    r#type: "button",
                                    class: "rounded-md px-3 py-1.5 text-xs font-medium transition-colors cursor-pointer {active_class}",
                                    onclick: move |_| {
                                        view.set(mode);
                                    },
                                    "{label}"
                                }
                            }
                        }
                    }
                }

                if current_view == TaskView::Board {
                    button {
                        r#type: "button",
                        class: "inline-flex h-8 items-center gap-1.5 rounded-md border border-border bg-transparent px-3 text-xs font-medium text-foreground transition-colors hover:bg-muted cursor-pointer",
                        onclick: move |_| {
                            manage_open.set(true);
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
                        "Manage Boards"
                    }
                }
            }

            if tasks.is_empty() {
                EmptyState {
                    title: "No tasks yet",
                    description: "Create your first task and it will show up in any view.",
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
            } else if current_view == TaskView::Board {
                div { class: "grid grid-cols-1 gap-4 pb-6 sm:grid-cols-2 lg:grid-cols-3",
                    for (status, name, visible) in columns {
                        {
                            let mut visible = visible;
                            if *visible.read() {
                                let column: Vec<TaskItem> = tasks
                                    .iter()
                                    .filter(|t| t.status == status)
                                    .cloned()
                                    .collect();
                                let column_empty = column.is_empty();

                                rsx! {
                                    div { key: "{name}",
                                        class: "flex min-h-72 flex-col gap-3 rounded-xl border border-border/40 bg-muted/20 p-3",
                                        ondragover: move |e| {
                                            e.prevent_default();
                                        },
                                        ondrop: move |_| {
                                            let id = dragging.read().clone();
                                            dragging.set(None);
                                            if let Some(id) = id {
                                                app_state.set_task_status(&id, status);
                                            }
                                        },

                                        div { class: "flex items-center justify-between gap-2",
                                            h3 { class: "text-sm font-semibold text-foreground",
                                                "{name}"
                                            }
                                            button {
                                                r#type: "button",
                                                class: "flex size-6 shrink-0 items-center justify-center rounded-md text-muted-foreground transition-colors hover:bg-muted hover:text-foreground cursor-pointer",
                                                title: "Hide {name}",
                                                onclick: move |_| {
                                                    visible.set(false);
                                                },
                                                svg {
                                                    class: "size-3.5",
                                                    view_box: "0 0 24 24",
                                                    fill: "none",
                                                    stroke: "currentColor",
                                                    stroke_width: "2",
                                                    stroke_linecap: "round",
                                                    path { d: "M18 6 6 18M6 6l12 12" }
                                                }
                                            }
                                        }

                                        for task in column {
                                            TaskCard { key: "{task.id}", task: task }
                                        }

                                        if column_empty {
                                            div { class: "rounded-lg border border-dashed border-border/60 px-3 py-5 text-center text-xs text-muted-foreground",
                                                "Drop tasks here"
                                            }
                                        }
                                    }
                                }
                            } else {
                                rsx! {
                                    button { key: "{name}",
                                        r#type: "button",
                                        class: "flex min-h-72 items-center justify-center rounded-xl border border-dashed border-border/60 text-xs font-medium text-muted-foreground transition-colors hover:border-border hover:text-foreground cursor-pointer",
                                        onclick: move |_| {
                                            visible.set(true);
                                        },
                                        "Show {name}"
                                    }
                                }
                            }
                        }
                    }
                }
            } else if current_view == TaskView::List {
                div { class: "flex flex-col gap-2 pb-6",
                    for task in tasks.iter().cloned() {
                        TaskRow { key: "{task.id}", task: task }
                    }
                }
            } else {
                div { class: "overflow-x-auto rounded-xl border border-border/40",
                    table { class: "w-full text-left",
                        thead {
                            tr { class: "border-b border-border/40 bg-muted/40",
                                th { class: "px-4 py-2.5 text-xs font-medium text-muted-foreground",
                                    "Task"
                                }
                                th { class: "px-4 py-2.5 text-xs font-medium text-muted-foreground",
                                    "Project"
                                }
                                th { class: "px-4 py-2.5 text-xs font-medium text-muted-foreground",
                                    "Priority"
                                }
                                th { class: "px-4 py-2.5 text-xs font-medium text-muted-foreground",
                                    "Status"
                                }
                                th { class: "px-4 py-2.5 text-xs font-medium text-muted-foreground",
                                    "Due"
                                }
                            }
                        }
                        tbody {
                            for task in tasks.iter().cloned() {
                                {
                                    let task_id = task.id.clone();
                                    let completed = task.completed;
                                    let due_cell = match task.due_date {
                                        Some((y, m, d)) => format!("{}/{}/{}", m, d, y),
                                        None => "—".to_string(),
                                    };
                                    let title_class = if completed {
                                        "text-sm font-medium text-muted-foreground line-through"
                                    } else {
                                        "text-sm font-medium text-foreground"
                                    };
                                    let toggle_class = if completed {
                                        "flex size-4 shrink-0 items-center justify-center rounded border border-primary bg-primary text-primary-foreground cursor-pointer transition-colors"
                                    } else {
                                        "flex size-4 shrink-0 items-center justify-center rounded border border-input bg-transparent hover:border-primary cursor-pointer transition-colors"
                                    };

                                    rsx! {
                                        tr { key: "{task.id}",
                                            class: "border-t border-border/30 transition-colors hover:bg-muted/30",
                                            td { class: "px-4 py-2.5",
                                                div { class: "flex items-center gap-2.5",
                                                    button {
                                                        r#type: "button",
                                                        class: toggle_class,
                                                        "aria-label": "Toggle task",
                                                        onclick: move |_| {
                                                            app_state.toggle_task(&task_id);
                                                        },
                                                        if completed {
                                                            svg {
                                                                class: "size-3",
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
                                                    span { class: title_class, "{task.title}" }
                                                }
                                            }
                                            td { class: "px-4 py-2.5 text-xs text-muted-foreground",
                                                "{task.project}"
                                            }
                                            td { class: "px-4 py-2.5",
                                                Badge {
                                                    variant: task.priority.badge_variant(),
                                                    size: "sm",
                                                    "{task.priority.as_str()}"
                                                }
                                            }
                                            td { class: "px-4 py-2.5",
                                                Badge {
                                                    variant: task.status.badge_variant(),
                                                    size: "sm",
                                                    "{task.status.as_str()}"
                                                }
                                            }
                                            td { class: "whitespace-nowrap px-4 py-2.5 text-xs text-muted-foreground",
                                                "{due_cell}"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            Dialog { open: manage_open,
                DialogContent { class: "max-w-sm",
                    DialogHeader {
                        DialogTitle { "Manage Boards" }
                        DialogDescription { "Choose which columns appear on your board." }
                    }

                    div { class: "flex flex-col gap-4",
                        for (status, name, visible) in columns {
                            {
                                let count = tasks.iter().filter(|t| t.status == status).count();
                                rsx! {
                                    div { key: "{name}",
                                        class: "flex items-center justify-between gap-4",
                                        div { class: "flex flex-col gap-0.5",
                                            span { class: "text-sm font-medium text-foreground",
                                                "{name}"
                                            }
                                            span { class: "text-xs text-muted-foreground",
                                                "{count} tasks"
                                            }
                                        }
                                        Switch { checked: visible }
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

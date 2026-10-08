use dioxus::prelude::*;

use crate::components::TaskDetailContext;
use crate::components::ui::Badge;
use crate::icons::TrashIcon;
use crate::state::{use_app_state, TaskItem};

use super::shared::{format_due, TaskCheck};

#[component]
fn TableRow(task: TaskItem) -> Element {
    let mut app_state = use_app_state();
    let detail = use_context::<TaskDetailContext>();

    let title = task.title.clone();
    let desc = task.description.clone();
    let project = task.project.clone();
    let id = task.id.clone();
    let open_id = task.id.clone();
    let completed = task.completed;
    let priority = task.priority.as_str();
    let prio_variant = task.priority.badge_variant().to_string();
    let status_label = task.status.as_str();
    let status_variant = task.status.badge_variant().to_string();
    let due_label = task.due_date.map(format_due);

    rsx! {
        tr { class: "group cursor-pointer transition-colors hover:bg-muted/40",
            onclick: move |_| detail.show(open_id.clone()),

            td { class: "px-4 py-3",
                div { class: "flex items-start gap-2.5",
                    TaskCheck { id: id.clone(), completed }
                    div { class: "min-w-0",
                        p { class: if completed {
                                "text-xs font-medium text-foreground line-through"
                            } else {
                                "text-xs font-medium text-foreground"
                            },
                            "{title}"
                        }
                        if !desc.is_empty() {
                            p { class: "mt-0.5 line-clamp-1 text-[11px] text-muted-foreground", "{desc}" }
                        }
                    }
                }
            }

            td { class: "px-4 py-3 text-xs text-muted-foreground", "{project}" }

            td { class: "px-4 py-3",
                Badge { variant: prio_variant, size: "sm".to_string(), "{priority}" }
            }

            td { class: "px-4 py-3",
                Badge { variant: status_variant, size: "sm".to_string(), "{status_label}" }
            }

            td { class: "px-4 py-3 text-xs text-muted-foreground",
                if let Some(due) = &due_label {
                    "{due}"
                } else {
                    "—"
                }
            }

            td { class: "px-4 py-3",
                div { class: "flex justify-end opacity-0 transition-opacity group-hover:opacity-100",
                    button {
                        r#type: "button",
                        class: "flex size-7 cursor-pointer items-center justify-center rounded-md text-muted-foreground transition-colors hover:bg-destructive/10 hover:text-destructive",
                        title: "Delete task",
                        onclick: move |e| {
                            e.stop_propagation();
                            app_state.delete_task(&id);
                        },
                        TrashIcon { class: "size-4" }
                    }
                }
            }
        }
    }
}

/// Data table of all tasks.
#[component]
pub fn TaskTable(tasks: Vec<TaskItem>) -> Element {
    rsx! {
        div { class: "overflow-x-auto rounded-xl border border-border/50 bg-card shadow-xs",
            table { class: "w-full border-collapse text-left",
                thead { class: "border-b border-border/60 bg-muted/40",
                    tr {
                        th { class: "px-4 py-2.5 text-[11px] font-medium uppercase tracking-wide text-muted-foreground",
                            "Task"
                        }
                        th { class: "px-4 py-2.5 text-[11px] font-medium uppercase tracking-wide text-muted-foreground",
                            "Project"
                        }
                        th { class: "px-4 py-2.5 text-[11px] font-medium uppercase tracking-wide text-muted-foreground",
                            "Priority"
                        }
                        th { class: "px-4 py-2.5 text-[11px] font-medium uppercase tracking-wide text-muted-foreground",
                            "Status"
                        }
                        th { class: "px-4 py-2.5 text-[11px] font-medium uppercase tracking-wide text-muted-foreground",
                            "Due date"
                        }
                        th { class: "px-4 py-2.5 text-right text-[11px] font-medium uppercase tracking-wide text-muted-foreground",
                            "Actions"
                        }
                    }
                }
                tbody { class: "divide-y divide-border/50",
                    for task in tasks {
                        TableRow { key: "{task.id}", task }
                    }
                }
            }
        }
    }
}

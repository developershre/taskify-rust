use dioxus::prelude::*;

use crate::components::ui::{
    Badge, Dialog, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogTitle,
};
use crate::icons::CloseIcon;
use crate::state::{use_app_state, AppState, TaskItem};
use crate::views::tasks::shared::format_due;

/// Trim + push a new subtask, then clear the input.
fn commit_subtask(mut app_state: AppState, task_id: String, mut new_sub: Signal<String>) {
    let title = new_sub.read().trim().to_string();
    if title.is_empty() {
        return;
    }
    app_state.add_subtask(&task_id, title);
    new_sub.set(String::new());
}

/// Which task is open in the detail dialog (provided by TitleBar).
#[derive(Clone, Copy, PartialEq)]
pub struct TaskDetailContext {
    pub open: Signal<bool>,
    pub task_id: Signal<Option<String>>,
}

impl TaskDetailContext {
    /// Open the dialog for a task.
    pub fn show(mut self, id: String) {
        self.task_id.set(Some(id));
        self.open.set(true);
    }
}

#[component]
pub fn TaskDetailDialog() -> Element {
    let mut detail = use_context::<TaskDetailContext>();
    let mut app_state = use_app_state();
    let mut new_sub = use_signal(String::new);

    // Live lookup so the dialog always reflects edits/deletes made elsewhere.
    let task: Option<TaskItem> = {
        let id = detail.task_id.read().clone();
        let list = app_state.tasks.read();
        id.and_then(|tid| list.iter().find(|t| t.id == tid).cloned())
    };

    rsx! {
        Dialog {
            open: detail.open,

            if let Some(task) = task {
                {
                    let title = task.title.clone();
                    let desc = task.description.clone();
                    let project = task.project.clone();
                    let id = task.id.clone();
                    let completed = task.completed;
                    let priority = task.priority.as_str();
                    let prio_variant = task.priority.badge_variant().to_string();
                    let status_label = task.status.as_str();
                    let status_variant = task.status.badge_variant().to_string();
                    let due_label = task.due_date.map(format_due);
                    let sub_total = task.subtasks.len();
                    let sub_done = task.subtasks.iter().filter(|s| s.completed).count();
                    let task_tid = task.id.clone();
                    let add_tid = task.id.clone();

                    rsx! {
                        DialogContent { class: "max-w-lg max-h-[85vh] overflow-y-auto",
                            DialogHeader {
                                DialogTitle { "{title}" }
                                DialogDescription { "Task details and current status." }
                            }

                            div { class: "flex flex-wrap items-center gap-1.5",
                                Badge {
                                    variant: prio_variant,
                                    size: "sm".to_string(),
                                    "{priority}"
                                }
                                Badge {
                                    variant: status_variant,
                                    size: "sm".to_string(),
                                    "{status_label}"
                                }
                                Badge {
                                    variant: "outline".to_string(),
                                    size: "sm".to_string(),
                                    "{project}"
                                }
                            }

                            div { class: "mt-1 rounded-lg border border-border/50 bg-muted/30 p-3",
                                p { class: "text-[10px] font-medium uppercase tracking-wide text-muted-foreground",
                                    "Description"
                                }
                                p { class: "mt-1 text-xs leading-relaxed text-foreground",
                                    if desc.is_empty() {
                                        "No description provided."
                                    } else {
                                        "{desc}"
                                    }
                                }
                            }

                            div { class: "grid grid-cols-2 gap-3",
                                div { class: "flex flex-col gap-1 rounded-lg border border-border/50 p-3",
                                    span { class: "text-[10px] font-medium uppercase tracking-wide text-muted-foreground",
                                        "Due date"
                                    }
                                    span { class: "text-xs font-medium text-foreground",
                                        if let Some(due) = &due_label {
                                            "{due}"
                                        } else {
                                            "No due date"
                                        }
                                    }
                                }
                                div { class: "flex flex-col gap-1 rounded-lg border border-border/50 p-3",
                                    span { class: "text-[10px] font-medium uppercase tracking-wide text-muted-foreground",
                                        "Completed"
                                    }
                                    span { class: "text-xs font-medium text-foreground",
                                        if completed { "Yes" } else { "No" }
                                    }
                                }
                            }

                            // Subtasks
                            div { class: "mt-1",
                                div { class: "flex items-center justify-between",
                                    p { class: "text-[10px] font-medium uppercase tracking-wide text-muted-foreground",
                                        "Subtasks"
                                    }
                                    if sub_total > 0 {
                                        span { class: "text-[10px] text-muted-foreground",
                                            "{sub_done}/{sub_total} done"
                                        }
                                    }
                                }

                                if sub_total > 0 {
                                    div { class: "mt-1.5 flex flex-col gap-1",
                                        for (i, sub) in task.subtasks.iter().enumerate() {
                                            {
                                                let sid = task_tid.clone();
                                                let rid = task_tid.clone();
                                                let sub_title = sub.title.clone();
                                                let sub_done_flag = sub.completed;
                                                rsx! {
                                                    div { key: "{i}",
                                                        class: "group/sub flex items-center gap-2.5 rounded-md border border-border/50 bg-card px-2.5 py-1.5",
                                                        button {
                                                            r#type: "button",
                                                            class: if sub_done_flag {
                                                                "flex size-4 shrink-0 cursor-pointer items-center justify-center rounded-full bg-primary text-primary-foreground transition-colors hover:opacity-90"
                                                            } else {
                                                                "flex size-4 shrink-0 cursor-pointer items-center justify-center rounded-full border border-border text-transparent transition-colors hover:border-primary hover:text-primary/40"
                                                            },
                                                            title: if sub_done_flag {
                                                                "Mark incomplete"
                                                            } else {
                                                                "Mark complete"
                                                            },
                                                            onclick: move |_| app_state.toggle_subtask(&sid, i),
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
                                                        span { class: if sub_done_flag {
                                                                "min-w-0 flex-1 truncate text-xs text-muted-foreground line-through"
                                                            } else {
                                                                "min-w-0 flex-1 truncate text-xs text-foreground"
                                                            },
                                                            "{sub_title}"
                                                        }
                                                        button {
                                                            r#type: "button",
                                                            class: "flex size-5 shrink-0 cursor-pointer items-center justify-center rounded text-muted-foreground opacity-0 transition-all hover:bg-destructive/10 hover:text-destructive group-hover/sub:opacity-100",
                                                            title: "Remove subtask",
                                                            onclick: move |_| app_state.remove_subtask(&rid, i),
                                                            CloseIcon { class: "size-3" }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }

                                div { class: "mt-2 flex gap-2",
                                    input {
                                        r#type: "text",
                                        class: "h-8 min-w-0 flex-1 rounded-md border border-input bg-transparent px-2.5 text-xs text-foreground placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring",
                                        placeholder: "Add a subtask...",
                                        value: "{new_sub}",
                                        oninput: move |e| new_sub.set(e.value()),
                                        onkeydown: move |e: KeyboardEvent| {
                                            if e.key() == Key::Enter {
                                                commit_subtask(app_state, add_tid.clone(), new_sub);
                                            }
                                        },
                                    }
                                    button {
                                        r#type: "button",
                                        class: "inline-flex h-8 shrink-0 items-center rounded-md border border-border px-3 text-xs font-medium text-foreground hover:bg-muted transition-colors cursor-pointer",
                                        onclick: move |_| commit_subtask(app_state, task_tid.clone(), new_sub),
                                        "Add"
                                    }
                                }
                            }

                            DialogFooter {
                                button {
                                    r#type: "button",
                                    class: "inline-flex h-8 items-center rounded-md border border-border px-3 text-xs font-medium text-foreground hover:bg-muted transition-colors cursor-pointer",
                                    onclick: move |_| app_state.toggle_task(&id),
                                    if completed {
                                        "Mark incomplete"
                                    } else {
                                        "Mark complete"
                                    }
                                }
                                button {
                                    r#type: "button",
                                    class: "inline-flex h-8 items-center rounded-md bg-primary px-3 text-xs font-medium text-primary-foreground hover:opacity-90 transition-opacity cursor-pointer",
                                    onclick: move |_| detail.open.set(false),
                                    "Close"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

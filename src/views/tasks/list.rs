use dioxus::prelude::*;

use crate::components::TaskDetailContext;
use crate::components::ui::{
    Badge, Select, SelectContent, SelectItem, SelectTrigger, SelectValue,
};
use crate::icons::TrashIcon;
use crate::state::{use_app_state, TaskItem};

use super::shared::{format_due, TaskCheck};

#[component]
pub fn TaskRow(task: TaskItem) -> Element {
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
        div { class: "group flex cursor-pointer items-center gap-3 rounded-xl border border-border/50 bg-card px-3.5 py-3 shadow-xs transition-colors hover:border-border",
            onclick: move |_| detail.show(open_id.clone()),

            TaskCheck { id: id.clone(), completed }

            div { class: "min-w-0 flex-1",
                p { class: if completed {
                        "text-xs font-medium text-foreground line-through"
                    } else {
                        "text-xs font-medium text-foreground"
                    },
                    "{title}"
                }
                if !desc.is_empty() {
                    p { class: "mt-0.5 line-clamp-1 text-xs text-muted-foreground", "{desc}" }
                }
            }

            span { class: "hidden w-24 shrink-0 truncate text-[11px] text-muted-foreground md:block",
                "{project}"
            }

            div { class: "flex shrink-0 items-center gap-1.5",
                Badge { variant: prio_variant, size: "sm".to_string(), "{priority}" }
                Badge { variant: status_variant, size: "sm".to_string(), "{status_label}" }
            }

            span { class: "w-24 shrink-0 text-right text-[11px] text-muted-foreground",
                if let Some(due) = &due_label {
                    "{due}"
                } else {
                    "—"
                }
            }

            button {
                r#type: "button",
                class: "flex size-7 shrink-0 cursor-pointer items-center justify-center rounded-md text-muted-foreground opacity-0 transition-all hover:bg-destructive/10 hover:text-destructive group-hover:opacity-100",
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

/// Vertical list of task rows with search + priority/status filters.
#[component]
pub fn TaskList(tasks: Vec<TaskItem>) -> Element {
    let app_state = use_app_state();
    let mut query = use_signal(String::new);
    let mut prio_filter = use_signal(|| "All priorities".to_string());
    let mut stat_filter = use_signal(|| "All statuses".to_string());

    let q = query.read().to_lowercase();
    let prio = prio_filter.read().clone();
    let stat = stat_filter.read().clone();
    let stat_options: Vec<String> = app_state.board_columns.read().clone();

    let filtered: Vec<TaskItem> = tasks
        .iter()
        .filter(|t| {
            let text_match = q.is_empty()
                || t.title.to_lowercase().contains(&q)
                || t.description.to_lowercase().contains(&q)
                || t.project.to_lowercase().contains(&q);
            let prio_match = prio == "All priorities" || t.priority.as_str() == prio;
            let stat_match = stat == "All statuses" || t.status.as_str() == stat;
            text_match && prio_match && stat_match
        })
        .cloned()
        .collect();

    let filtered_count = filtered.len();
    let has_filters = !q.is_empty() || prio != "All priorities" || stat != "All statuses";

    rsx! {
        div { class: "flex flex-col gap-3",

            // Search + filter bar.
            div { class: "flex flex-wrap items-center gap-2",
                input {
                    r#type: "text",
                    class: "h-9 w-64 rounded-md border border-input bg-background px-3 text-xs text-foreground placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring",
                    placeholder: "Search tasks...",
                    value: "{query}",
                    oninput: move |e| query.set(e.value()),
                }

                div { class: "w-44",
                    Select {
                        value: prio_filter,
                        SelectTrigger {
                            SelectValue { placeholder: "All priorities" }
                        }
                        SelectContent {
                            SelectItem { value: "All priorities".to_string(), "All priorities" }
                            SelectItem { value: "Low".to_string(), "Low" }
                            SelectItem { value: "Medium".to_string(), "Medium" }
                            SelectItem { value: "High".to_string(), "High" }
                            SelectItem { value: "Urgent".to_string(), "Urgent" }
                        }
                    }
                }

                div { class: "w-44",
                    Select {
                        value: stat_filter,
                        SelectTrigger {
                            SelectValue { placeholder: "All statuses" }
                        }
                        SelectContent {
                            SelectItem { value: "All statuses".to_string(), "All statuses" }
                            for opt in stat_options {
                                SelectItem { key: "{opt}", value: opt.clone(), "{opt}" }
                            }
                        }
                    }
                }

                if has_filters {
                    button {
                        r#type: "button",
                        class: "h-9 cursor-pointer rounded-md border border-border px-3 text-xs font-medium text-muted-foreground transition-colors hover:bg-muted hover:text-foreground",
                        onclick: move |_| {
                            query.set(String::new());
                            prio_filter.set("All priorities".to_string());
                            stat_filter.set("All statuses".to_string());
                        },
                        "Reset"
                    }
                }
            }

            if filtered_count == 0 {
                div { class: "rounded-xl border border-dashed border-border/50 bg-muted/20 px-4 py-10 text-center",
                    p { class: "text-xs font-medium text-foreground",
                        if has_filters { "No tasks match your filters" } else { "No tasks yet" }
                    }
                    if has_filters {
                        p { class: "mt-1 text-[11px] text-muted-foreground",
                            "Adjust the search or filters to see more results."
                        }
                    }
                }
            } else {
                div { class: "flex flex-col gap-2",
                    for task in filtered {
                        TaskRow { key: "{task.id}", task }
                    }
                }
            }
        }
    }
}

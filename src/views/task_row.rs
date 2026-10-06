use dioxus::prelude::*;

use crate::components::ui::{Badge, Item, ItemActions, ItemContent, ItemDescription, ItemTitle};
use crate::state::{use_app_state, TaskItem};

#[derive(Props, Clone, PartialEq)]
pub struct TaskRowProps {
    pub task: TaskItem,
}

#[component]
pub fn TaskRow(props: TaskRowProps) -> Element {
    let mut app_state = use_app_state();
    let task_id = props.task.id.clone();
    let completed = props.task.completed;
    let priority = props.task.priority;

    let priority_label = priority.as_str();

    let due_label = match props.task.due_date {
        Some((y, m, d)) => format!("Due {:04}-{:02}-{:02}", y, m, d),
        None => "No due date".to_string(),
    };

    let toggle_class = if completed {
        "flex size-4 shrink-0 items-center justify-center rounded border border-primary bg-primary text-primary-foreground cursor-pointer transition-colors"
    } else {
        "flex size-4 shrink-0 items-center justify-center rounded border border-input bg-transparent hover:border-primary cursor-pointer transition-colors"
    };

    rsx! {
        Item { class: "border-border/40",
            div { class: "flex min-w-0 flex-1 items-center gap-3",
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
                ItemContent {
                    ItemTitle {
                        class: if completed { "line-through text-muted-foreground" } else { "" },
                        "{props.task.title}"
                    }
                    ItemDescription { "{props.task.project} · {due_label}" }
                }
            }
            ItemActions {
                Badge { variant: priority.badge_variant(), "{priority_label}" }
            }
        }
    }
}

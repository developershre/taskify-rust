use dioxus::prelude::*;

use crate::components::ui::Badge;
use crate::state::TaskItem;

#[derive(Clone, Copy, PartialEq)]
pub struct TaskDnd {
    pub dragging: Signal<Option<String>>,
}

#[derive(Props, Clone, PartialEq)]
pub struct TaskCardProps {
    pub task: TaskItem,
}

#[component]
pub fn TaskCard(props: TaskCardProps) -> Element {
    let mut dragging = use_context::<TaskDnd>().dragging;
    let task_id = props.task.id.clone();
    let is_dragging = dragging.read().as_deref() == Some(task_id.as_str());

    let priority_label = props.task.priority.as_str();
    let priority_variant = props.task.priority.badge_variant();
    let due_label = match props.task.due_date {
        Some((y, m, d)) => format!("Due {}/{}/{}", m, d, y),
        None => "No due date".to_string(),
    };

    let card_class = if is_dragging {
        "flex cursor-grab items-start gap-2 rounded-lg border border-border/40 bg-card p-3 shadow-xs opacity-50 active:cursor-grabbing"
    } else {
        "flex cursor-grab items-start gap-2 rounded-lg border border-border/40 bg-card p-3 shadow-xs transition-colors hover:border-border active:cursor-grabbing"
    };

    rsx! {
        div {
            "draggable": "true",
            class: card_class,
            ondragstart: move |e| {
                let _ = e.data_transfer().set_data("text/plain", &task_id);
                dragging.set(Some(task_id.clone()));
            },
            ondragend: move |_| {
                dragging.set(None);
            },

            svg {
                class: "mt-0.5 size-3.5 shrink-0 text-muted-foreground/60",
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                xmlns: "http://www.w3.org/2000/svg",
                path { d: "M9 5h.01M9 12h.01M9 19h.01M15 5h.01M15 12h.01M15 19h.01" }
            }

            div { class: "flex min-w-0 flex-1 flex-col gap-1",
                span { class: "truncate text-sm font-medium text-foreground",
                    "{props.task.title}"
                }
                span { class: "truncate text-xs text-muted-foreground",
                    "{props.task.project}"
                }
                div { class: "flex items-center justify-between gap-2 pt-0.5",
                    Badge {
                        variant: priority_variant,
                        size: "sm",
                        class: "shrink-0",
                        "{priority_label}"
                    }
                    span { class: "shrink-0 text-[11px] text-muted-foreground",
                        "{due_label}"
                    }
                }
            }
        }
    }
}

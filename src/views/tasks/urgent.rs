use dioxus::prelude::*;

use super::shared::{TaskBoardPage, TaskFilter};

#[component]
pub fn TasksUrgent() -> Element {
    rsx! {
        TaskBoardPage {
            title: "Urgent Tasks".to_string(),
            description: "High-priority work that needs attention right now.".to_string(),
            filter: TaskFilter::Urgent,
        }
    }
}

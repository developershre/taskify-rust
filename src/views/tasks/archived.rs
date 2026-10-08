use dioxus::prelude::*;

use super::shared::{TaskBoardPage, TaskFilter};

#[component]
pub fn TasksArchived() -> Element {
    rsx! {
        TaskBoardPage {
            title: "Archived Tasks".to_string(),
            description: "Completed work you've archived for reference.".to_string(),
            filter: TaskFilter::Archived,
        }
    }
}

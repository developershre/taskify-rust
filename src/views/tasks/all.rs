use dioxus::prelude::*;

use super::shared::{TaskBoardPage, TaskFilter};

#[component]
pub fn TasksAll() -> Element {
    rsx! {
        TaskBoardPage {
            title: "All Tasks".to_string(),
            description: "Every task across your projects on a single board.".to_string(),
            filter: TaskFilter::All,
        }
    }
}
